use clap::{AppSettings, Clap};

/// Manage accounts
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub enum Account {
    #[clap(version = "1.0")]
    Generate(AccountGenerate),

    #[clap(version = "1.0")]
    Transact(AccountTransact),

    #[clap(version = "1.0")]
    Info(AccountInfo),
}

/// Generate new accounts an keypairs
#[derive(Clap, Debug)]
pub struct AccountGenerate {
    /// An address for which we want to derive new keys
    #[clap(short, long)]
    pub address: Option<String>,
}

/// Display informations about an account
#[derive(Clap, Debug)]
pub struct AccountInfo {
    /// An address for which we want to derive new keys
    #[clap(short, long)]
    pub address: Option<String>,
}

/// Emit transaction to the network using an account and pubkey
#[derive(Clap, Debug)]
pub struct AccountTransact {
    /// An address for which we want to derive new keys
    #[clap(short, long)]
    pub address: Option<String>,

    /// An address for which we want to derive new keys
    #[clap(short, long)]
    pub pubkey: String,
}
