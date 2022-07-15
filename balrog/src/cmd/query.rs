use clap::{Args, Subcommand};

/// Query for resources on the network
#[derive(Subcommand, Debug)]
pub enum Query {
    #[clap(version = "1.0")]
    Balances(QueryBalances),
    #[clap(subcommand, version = "1.0")]
    List(QueryList),
}

/// Get balances on the network
#[derive(Args, Debug)]
pub struct QueryBalances {
    /// An optional list of vega public keys for which to get balances on the network
    #[clap(value_parser)]
    pub public_keys: Vec<String>,
    /// The address of a data-node from the network
    #[clap(short, long)]
    pub network: Option<String>,
}

/// List resources from the network
#[derive(Subcommand, Debug)]
pub enum QueryList {
    #[clap(version = "1.0")]
    Nodes(QueryListNodes),
}

/// Get nodes of the network
#[derive(Args, Debug)]
pub struct QueryListNodes {
    /// The address of a data-node from the network
    #[clap(short, long)]
    pub network: Option<String>,
}
