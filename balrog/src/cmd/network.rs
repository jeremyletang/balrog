use clap::{Args, Subcommand};

/// Manage networks
#[derive(Subcommand, Debug)]
pub enum Network {
    #[clap(version = "1.0")]
    Import(NetworkImport),
}

/// Import a network
#[derive(Args, Debug)]
pub struct NetworkImport {}
