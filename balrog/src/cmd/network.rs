use clap::{Args, Subcommand};

/// Manage networks
#[derive(Subcommand, Debug)]
pub enum Network {
    #[clap(version = "1.0")]
    Import(NetworkImport),
    #[clap(version = "1.0")]
    Info(NetworkInfo),
}

/// Import a network
#[derive(Args, Debug)]
pub struct NetworkImport {}

/// Display informations about a network
#[derive(Args, Debug)]
pub struct NetworkInfo {
    /// The name of the network
    #[clap(short, long)]
    pub network: Option<String>,
}
