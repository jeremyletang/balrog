use clap::{Args, Subcommand};

/// List entities managed by balrog
#[derive(Subcommand, Debug)]
pub enum List {
    /// List all existing wallets
    #[clap(version = "1.0")]
    Accounts(ListAccounts),

    /// List all registerd networks
    #[clap(version = "1.0")]
    Networks(ListNetworks),
}

#[derive(Args, Debug)]
pub struct ListAccounts {}

#[derive(Args, Debug)]
pub struct ListNetworks {}
