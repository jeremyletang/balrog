use crate::errors::Error;
use crate::keystore;
use crate::pow;
use ed25519_compact::SecretKey;
use prost::Message;
use rand::{thread_rng, Rng};
use sha3::{Digest, Sha3_256};
use vega_rust_sdk::vega::commands::v1::input_data::Command;
use vega_rust_sdk::vega::commands::v1::transaction::From;
use vega_rust_sdk::vega::commands::v1::Signature;
use vega_rust_sdk::vega::commands::v1::{InputData, ProofOfWork, Transaction};

pub fn build_and_sign(
    command: Command,
    kp: &keystore::Keypair,
    pubkey: String,
    block_height: u64,
    // pow stuff
    block_hash: &str,
    difficulty: usize,
) -> Result<Transaction, Error> {
    let input_data = InputData {
        nonce: 1,
        block_height,
        command: Some(command),
    }
    .encode_to_vec();

    let txid = random_hash();
    let (pow_nonce, _) = pow::make(block_hash, &txid, difficulty)?;
    let signature = sign(&&input_data, kp)?;

    let tx = Transaction {
        from: Some(From::PubKey(pubkey)),
        version: 1,
        input_data,
        signature: Some(Signature {
            value: signature,
            algo: "vega/ed25519".into(),
            version: 1,
        }),
        pow: Some(ProofOfWork {
            tid: txid,
            nonce: pow_nonce,
            hash_function: pow::HASH_FUNCTION.to_string(),
        }),
    };

    return Ok(tx);
}

fn sign(msg: &[u8], kp: &keystore::Keypair) -> Result<String, Error> {
    let mut hasher = Sha3_256::new();
    hasher.update(msg);
    let h = hasher.finalize().to_vec();
    let key = SecretKey::from_slice(&kp.secret)?;
    return Ok(hex::encode(key.sign(h, None).as_ref()));
}

fn random_hash() -> String {
    let msg = thread_rng()
        .sample_iter::<u8, _>(rand::distributions::Standard)
        .take(10)
        .collect::<Vec<u8>>();
    let mut hasher = Sha3_256::new();
    hasher.update(msg);
    let h = hasher.finalize().to_vec();
    return hex::encode(h).to_uppercase();
}
