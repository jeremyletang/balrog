use crate::paths;
use account::Account;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub use account::Keypair;
pub use errors::Error;

mod account;
mod aes;
mod errors;

const KEYSTORE_V2: u16 = 2;

#[derive(Serialize, Deserialize, Debug)]
pub struct Keystore {
    pub address: String,
    pub version: u16,
    pub index: u64,
    pub crypto: Crypto,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeystoreGenerated {
    pub address: String,
    pub mnemonic: String,
    pub path: String,
}

impl Keystore {
    pub fn import(
        mnemonic: &str,
        passphrase: &str,
    ) -> Result<(Keystore, KeystoreGenerated), Error> {
        let acc = Account::import(mnemonic)?;
        let kg = KeystoreGenerated {
            address: acc.address(),
            mnemonic: mnemonic.to_string(),
            path: String::new(),
        };
        return Ok((Keystore::from_account(acc, passphrase), kg));
    }

    pub fn generate(passphrase: &str) -> Result<(Keystore, KeystoreGenerated), Error> {
        let (acc, m) = Account::generate()?;
        let kg = KeystoreGenerated {
            address: acc.address(),
            mnemonic: m,
            path: String::new(),
        };
        return Ok((Keystore::from_account(acc, passphrase), kg));
    }

    fn from_account(account: Account, passphrase: &str) -> Keystore {
        return Keystore {
            address: account.address(),
            version: KEYSTORE_V2,
            index: account.index_max(),
            crypto: Crypto::AES128(aes::Aes128Cypher::from_seed_and_passhprase(
                &account.seed(),
                passphrase,
            )),
        };
    }

    pub fn verify(&self) -> Result<(), Error> {
        if self.index == 0 {
            return Err(Error::InvalidIndex);
        }

        return Ok(());
    }

    pub fn account(&self, passphrase: &str) -> Result<Account, Error> {
        let seed = match &self.crypto {
            Crypto::AES128(aes128) => aes128.recover_seed(passphrase)?,
        };
        return Ok(Account::from_seed(seed, self.index));
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Crypto {
    #[serde(rename = "aes-128")]
    AES128(aes::Aes128Cypher),
}

pub fn save(home: &str, ks: &Keystore) -> Result<PathBuf, Error> {
    let path = paths::accounts_directory(home).join(Path::new(&(ks.address.clone() + ".json")));
    let serialized = serde_json::to_string(ks).unwrap();
    let mut f = File::create(&path)?;
    write!(f, "{}", serialized)?;
    return Ok(path);
}

pub fn generate(home: &str, passphrase: &str) -> Result<KeystoreGenerated, Error> {
    let (ks, mut kg) = Keystore::generate(passphrase)?;
    let path = save(home, &ks)?;
    kg.path = path.to_str().unwrap().to_string();
    return Ok(kg);
}

pub fn import(home: &str, mnemonic: &str, passphrase: &str) -> Result<KeystoreGenerated, Error> {
    let (ks, mut kg) = Keystore::import(mnemonic, passphrase)?;
    let path = save(home, &ks)?;
    kg.path = path.to_str().unwrap().to_string();
    return Ok(kg);
}

pub fn load(home: &str, address: &str, _passphrase: &str) -> Result<Keystore, Error> {
    let path = paths::accounts_directory(home).join(Path::new(&(address.to_string() + ".json")));
    let contents = std::fs::read_to_string(path)?;
    let ks = serde_json::from_str(&contents)?;
    return Ok(ks);
}

pub fn info(home: &str, address: &str, passphrase: &str) -> Result<KeystoreInfo, Error> {
    let path = paths::accounts_directory(home).join(Path::new(&(address.to_string() + ".json")));
    let contents = std::fs::read_to_string(path)?;
    let ks: Keystore = serde_json::from_str(&contents)?;
    let keypairs = ks.account(passphrase)?.keypairs()?;
    return Ok(KeystoreInfo {
        account: ks,
        keypairs,
    });
}

#[derive(Serialize, Deserialize)]
pub struct KeystoreInfo {
    pub account: Keystore,
    pub keypairs: Vec<account::Keypair>,
}

#[cfg(test)]
mod test {

    #[test]
    fn serde_keystore() {
        let _seed = hex::decode("92a5b23c0b8a99e37d07df3fb9966917f5d06e02ddbd909c7e184371463e9fc92e69929e00b5ab250f49c3fb1c12f252de4fed2c1db88387094a0f8c4c9ccd6c").unwrap();
        // let ks = Keystore::new("saduibadsiucxabvsuifdiwud", 1, &seed, "secure passphrase");

        // let s = serde_json::to_string(&ks).unwrap();
        // println!("{}", s);
    }
}
