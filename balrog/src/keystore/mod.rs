use account::Account;
use errors::Error;
use serde::{Deserialize, Serialize};

mod account;
mod aes;
mod errors;

pub fn generate(home: &str, passphrase: &str) -> Result<KeystoreGenerated, Error> {
    let (_, kg) = Keystore::generate(passphrase)?;
    return Ok(kg);
}

pub fn import(home: &str, mnemonic: &str, passphrase: &str) -> Result<KeystoreGenerated, Error> {
    let (_, kg) = Keystore::import(mnemonic, passphrase)?;
    return Ok(kg);
}

// pub fn load(home: &str, address: &str, passphrase: &str) -> Result<Keystore, Error> {

// }

const KEYSTORE_V1: u16 = 1;

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
        };
        return Ok((Keystore::from_account(acc, passphrase), kg));
    }

    pub fn generate(passphrase: &str) -> Result<(Keystore, KeystoreGenerated), Error> {
        let (acc, m) = Account::generate()?;
        let kg = KeystoreGenerated {
            address: acc.address(),
            mnemonic: m,
        };
        return Ok((Keystore::from_account(acc, passphrase), kg));
    }

    fn from_account(account: Account, passphrase: &str) -> Keystore {
        return Keystore {
            address: account.address(),
            version: KEYSTORE_V1,
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

    // pub fn
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Crypto {
    #[serde(rename = "aes-128")]
    AES128(aes::Aes128Cypher),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serde_keystore() {
        let seed = hex::decode("92a5b23c0b8a99e37d07df3fb9966917f5d06e02ddbd909c7e184371463e9fc92e69929e00b5ab250f49c3fb1c12f252de4fed2c1db88387094a0f8c4c9ccd6c").unwrap();
        let ks = Keystore::new("saduibadsiucxabvsuifdiwud", 1, &seed, "secure passphrase");

        let s = serde_json::to_string(&ks).unwrap();
        println!("{}", s);
    }
}
