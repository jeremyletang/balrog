use crate::client::{CoreBlockingClient, DatanodeV2BlockingClient};
use crate::errors::Error;
use crate::{keystore, network};
use dialoguer::{theme::ColorfulTheme, Select};
use url::Url;
use vega_rust_sdk::vega::commands::v1::input_data::Command;
use vega_rust_sdk::vega::commands::v1::{ProofOfWork, Signature};

mod governance_vote;
mod stake_delegation;
mod tx;

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
// https://explorer.fairground.wtf/txs/0x80C87FDE932170B11333CC7AE3911D47D3DA9E24CFE069BFC6E06A70C2F07003
//#[derive(Serialize, Deserialize, Debug)]
#[derive(Debug)]
pub struct TransactionResult {
    pub success: bool,
    pub explorer_link: String,
    pub signature: Signature,
    pub proof_of_work: ProofOfWork,
    pub command: Command,
}

pub fn transact(
    ks: keystore::Keystore,
    _n: network::Config,
    passphrase: &str,
    clt: &mut DatanodeV2BlockingClient,
    coreclt: &mut CoreBlockingClient,
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

    let command_select = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("select a command")
        .default(0)
        .items(COMMANDS)
        .interact()
        .unwrap();

    let command = match COMMANDS[command_select] {
        STAKE_DELEGATION => stake_delegation::run(clt)?,
        STAKE_UNDELEGATION => unimplemented!(),
        GOVERNANCE_VOTE => governance_vote::run(clt)?,
        GOVERNANCE_PROPOSAL => unimplemented!(),
        ORDER_SUBMISSION => unimplemented!(),
        ORDER_CANCELLATION => unimplemented!(),
        ORDER_AMENDMENT => unimplemented!(),
        _ => unreachable!(),
    };

    let res = coreclt.last_block_height()?;
    let tx = tx::build_and_sign(
        command.clone(),
        &kps[pkey],
        pks[pkey].clone(),
        res.get_ref().height,
        &res.get_ref().hash,
        res.get_ref().spam_pow_difficulty as usize,
    )?;

    let resp = coreclt.submit_transaction(tx.clone())?;

    let tres = TransactionResult {
        success: resp.success,
        explorer_link: make_explorer_link(_n.block_explorer, resp.tx_hash).to_string(),
        signature: tx.signature.unwrap(),
        proof_of_work: tx.pow.unwrap(),
        command,
    };

    println!("{:#?}", tres);

    return Ok(tres);
}

fn make_explorer_link(base_url: Url, hash: String) -> Url {
    base_url
        .join("txs/")
        .unwrap()
        .join(&("0x".to_string() + &hash))
        .unwrap()
}
