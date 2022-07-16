use crate::client::DatanodeV2BlockingClient;
use crate::errors::Error;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use vega_rust_sdk::vega::commands::v1::input_data::Command;
use vega_rust_sdk::vega::commands::v1::DelegateSubmission;

pub fn run(clt: &mut DatanodeV2BlockingClient) -> Result<Command, Error> {
    // first get list of proposals, if none are available to vote on, return.
    let resp = clt.get_nodes()?;
    let mut nodes = vec![];
    for n in resp.get_ref().nodes.iter() {
        nodes.push(n.id.clone())
    }

    if nodes.len() == 0 {
        return Err(Error::NoProposalsOpen);
    }

    let nid = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("select a node for which to delegate")
        .default(0)
        .items(&nodes)
        .interact()
        .unwrap();

    let value: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("what amount to delegate")
        .interact_text()
        .unwrap();

    return Ok(Command::DelegateSubmission(DelegateSubmission {
        node_id: nodes[nid].clone(),
        amount: value.into(),
    }));
}
