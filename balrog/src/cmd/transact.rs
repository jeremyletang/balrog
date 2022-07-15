use clap::Args;

/// Send a transaction to a network
#[derive(Args, Debug)]
pub struct Transact {
    /// The address of the wallet to be used to start the transaction
    #[clap(short, long)]
    pub address: Option<String>,
    /// The network we will send the transaction to
    #[clap(short, long)]
    pub network: Option<String>,
}
