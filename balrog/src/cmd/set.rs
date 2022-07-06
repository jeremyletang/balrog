use clap::{Args, Subcommand};

/// Setup configuration
#[derive(Subcommand, Debug)]
//#[clap(setting = AppSettings::ColoredHelp)]
pub enum Set {
    /// Set a default account
    #[clap(version = "1.0")]
    Account(SetValue),

    /// Set a default network
    #[clap(version = "1.0")]
    Network(SetValue),
}

#[derive(Args, Debug)]
pub struct SetValue {
    /// The new value for this setting
    #[clap(short, long)]
    pub value: String,
}
