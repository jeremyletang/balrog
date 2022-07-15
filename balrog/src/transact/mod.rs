use crate::errors::Error;
use crate::{keystore, network};
use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};

const STAKE_DELEGATION: &str = "stake delegation";
const STAKE_UNDELEGATION: &str = "stake undelegation";
const GOVERNANCE_VOTE: &str = "governance vote";
const GOVERNANCE_PROPOSAL: &str = "governance proposal";
const ORDER_SUBMISSION: &str = "order submission";
const ORDER_CANCELLATION: &str = "order cancellation";
const ORDER_AMENDMENT: &str = "order amendment";
const COMMANDS: &[&str] = &[
    STAKE_DELEGATION,
    STAKE_UNDELEGATION,
    GOVERNANCE_VOTE,
    GOVERNANCE_PROPOSAL,
    ORDER_SUBMISSION,
    ORDER_CANCELLATION,
    ORDER_AMENDMENT,
];

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionResult {
    pub transaction: String,
    pub hash: String,
}

pub fn transact(
    ks: keystore::Keystore,
    n: network::Config,
    passphrase: &str,
) -> Result<TransactionResult, Error> {
    let kps = ks.account(passphrase)?.keypairs()?;
    let mut pks = vec![];
    for kp in kps.iter() {
        pks.push(hex::encode(&kp.public));
    }

    let pkey = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("select a public key")
        .default(0)
        .items(&pks[..])
        .interact()
        .unwrap();

    let command = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("select a command")
        .default(0)
        .items(COMMANDS)
        .interact()
        .unwrap();

    return Ok(TransactionResult {
        transaction: "lol".to_string(),
        hash: "lol2".to_string(),
    });
}
