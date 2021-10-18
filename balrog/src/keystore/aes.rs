use super::errors::Error;
use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, NewBlockCipher};
use aes::Aes128;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

#[derive(Serialize, Deserialize, Debug)]
pub struct Aes128Cypher {
    pub cypher_text: String,
    pub salt: String,
}

impl Aes128Cypher {
    pub fn from_seed_and_passhprase(seed: &[u8], passphrase: &str) -> Aes128Cypher {
        let mut hasher = Sha3_256::new();
        let salt = rand::thread_rng().gen::<[u8; 32]>();
        hasher.update(salt);
        hasher.update(passphrase.as_bytes());
        let key = hasher.finalize();

        let c1 = Aes128::new(GenericArray::from_slice(&key[..16]));
        let c2 = Aes128::new(GenericArray::from_slice(&key[16..]));

        let mut encrypted = vec![];

        let mut block = GenericArray::clone_from_slice(&seed[..16]);
        c1.encrypt_block(&mut block);
        encrypted.extend_from_slice(block.as_slice());

        block.clone_from_slice(&seed[16..32]);
        c1.encrypt_block(&mut block);
        encrypted.extend_from_slice(block.as_slice());

        block.clone_from_slice(&seed[32..48]);
        c2.encrypt_block(&mut block);
        encrypted.extend_from_slice(block.as_slice());

        block.clone_from_slice(&seed[48..64]);
        c2.encrypt_block(&mut block);
        encrypted.extend_from_slice(block.as_slice());

        return Aes128Cypher {
            cypher_text: hex::encode(encrypted),
            salt: hex::encode(salt),
        };
    }

    pub fn recover_seed(&self, passphrase: &str) -> Result<Vec<u8>, Error> {
        let mut hasher = Sha3_256::new();
        let salt = match hex::decode(&self.salt) {
            Ok(s) => s,
            Err(_) => return Err(Error::InvalidSalt),
        };
        hasher.update(salt);
        hasher.update(passphrase.as_bytes());
        let key = hasher.finalize();
        let c1 = Aes128::new(GenericArray::from_slice(&key[..16]));
        let c2 = Aes128::new(GenericArray::from_slice(&key[16..]));

        let mut decrypted = vec![];
        let bytes = match hex::decode(&self.cypher_text) {
            Ok(b) => b,
            Err(_) => return Err(Error::InvalidHexCypherText),
        };

        let mut block = GenericArray::clone_from_slice(&bytes[..16]);
        c1.decrypt_block(&mut block);
        decrypted.extend_from_slice(block.as_slice());

        block.clone_from_slice(&bytes[16..32]);
        c1.decrypt_block(&mut block);
        decrypted.extend_from_slice(block.as_slice());

        block.clone_from_slice(&bytes[32..48]);
        c2.decrypt_block(&mut block);
        decrypted.extend_from_slice(block.as_slice());

        block.clone_from_slice(&bytes[48..64]);
        c2.decrypt_block(&mut block);
        decrypted.extend_from_slice(block.as_slice());

        return Ok(decrypted);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aes128_round_tryp() {
        let seed = hex::decode("92a5b23c0b8a99e37d07df3fb9966917f5d06e02ddbd909c7e184371463e9fc92e69929e00b5ab250f49c3fb1c12f252de4fed2c1db88387094a0f8c4c9ccd6c").unwrap();
        let passphrase = "SECUREP@33PHRASE";

        // create our aes128 stuff, then marshal it
        let enc = Aes128Cypher::from_seed_and_passhprase(&seed, passphrase);
        let serialized = serde_json::to_string(&enc).unwrap();

        // now we deserialize and get the seed
        let dec: Aes128Cypher = serde_json::from_str(&serialized).unwrap();
        assert_eq!(seed, dec.recover_seed(passphrase).unwrap());
    }
}
