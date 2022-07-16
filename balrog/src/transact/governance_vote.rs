use crate::client::DatanodeV2BlockingClient;
use crate::errors::Error;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use vega_rust_sdk::vega::commands::v1::input_data::Command;
use vega_rust_sdk::vega::commands::v1::VoteSubmission;
use vega_rust_sdk::vega::vote::Value;

pub fn run(clt: &mut DatanodeV2BlockingClient) -> Result<Command, Error> {
    // first get list of proposals, if none are available to vote on, return.
    let resp = clt.get_proposals()?;
    let mut proposals = vec![];
    for p in resp.data.iter() {
        proposals.push(p.proposal.as_ref().unwrap().id.clone())
    }

    if proposals.len() == 0 {
        return Err(Error::NoProposalsOpen);
    }

    let pid = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("select a proposal to vote on")
        .default(0)
        .items(&proposals)
        .interact()
        .unwrap();

    let vote_for = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("vote in favour of thi proposal?")
        .interact()
        .unwrap();

    let value = match vote_for {
        true => Value::Yes,
        false => Value::No,
    };

    return Ok(Command::VoteSubmission(VoteSubmission {
        proposal_id: proposals[pid].clone(),
        value: value.into(),
    }));
}
