use clap::{Args, Subcommand};

/// Manage accounts
#[derive(Subcommand, Debug)]
//#[clap(setting = AppSettings::ColoredHelp)]
pub enum Account {
    #[clap(version = "1.0")]
    Generate(AccountGenerate),

    #[clap(version = "1.0")]
    Transact(AccountTransact),

    #[clap(version = "1.0")]
    Info(AccountInfo),

    #[clap(version = "1.0")]
    Import(AccountImport),
}

/// Generate new account
#[derive(Args, Debug)]
pub struct AccountGenerate {
    /// An address for which we want to derive new keys
    #[clap(short, long)]
    pub address: Option<String>,
}

/// Import an account using a keypair
#[derive(Args, Debug)]
pub struct AccountImport {
    /// The mnemonic to be imported
    #[clap(short, long)]
    pub mnemonic: String,
}

/// Display informations about an account
#[derive(Args, Debug)]
pub struct AccountInfo {
    /// The address from which we want to get information about
    #[clap(short, long)]
    pub address: String,
}

/// Emit transaction to the network using an account and pubkey
#[derive(Args, Debug)]
pub struct AccountTransact {
    /// An address for which we want to derive new keys
    #[clap(short, long)]
    pub address: Option<String>,

    /// An address for which we want to derive new keys
    #[clap(short, long)]
    pub pubkey: String,
}
