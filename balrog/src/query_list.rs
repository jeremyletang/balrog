use crate::client::DatanodeV2BlockingClient;
use crate::errors::Error;
use crate::util::format_balance;
use tabled::{object::Segment, Alignment, Modify, Table, Tabled};
use vega_protobufs::vega::NodeStatus;

#[derive(Tabled)]
struct Node {
    id: String,
    name: String,
    staked_total: String,
    staked_by_operator: String,
    staked_by_delegates: String,
    status: String,
}

pub fn show(network: &str) -> Result<(), Error> {
    let mut clt = DatanodeV2BlockingClient::connect(network.to_string())?;
    let res = clt.get_nodes()?;
    let mut nodes = vec![];

    for node in res.nodes.unwrap().edges.iter() {
        let n = node.node.as_ref().unwrap();
        nodes.push(Node {
            id: n.id.clone(),
            name: n.name.clone(),
            staked_total: format_balance(n.staked_total.clone(), 18),
            staked_by_delegates: format_balance(n.staked_by_delegates.clone(), 18),
            staked_by_operator: format_balance(n.staked_by_operator.clone(), 18),
            status: status_to_string(NodeStatus::from_i32(n.status).unwrap()),
        })
    }

    print!(
        "{}",
        Table::new(nodes)
            .with(Modify::new(Segment::all()).with(Alignment::left()))
            .to_string()
    );

    return Ok(());
}

fn status_to_string(nstatus: NodeStatus) -> String {
    match nstatus {
        NodeStatus::NonValidator => "non-validator".to_string(),
        NodeStatus::Validator => "validator".to_string(),
        _ => "unsupported".to_string(),
    }
}
