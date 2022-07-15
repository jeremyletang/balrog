use crate::errors::Error;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use vega_rust_sdk::vega::commands::v1::VoteSubmission;
use vega_rust_sdk::vega::vote::Value;

fn run() -> Result<VoteSubmission, Error> {
    let pid = "".to_string();
    let vote_for = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to continue?")
        .interact()
        .unwrap();

    let value = match vote_for {
        true => Value::Yes,
        false => Value::No,
    };

    return Ok(Vote {
        proposal_id: pid,
        value: value.into(),
    });
    v.proposal_id;
}
