use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
//#[clap(setting = AppSettings::ColoredHelp)]
pub enum List {
    #[clap(version = "1.0")]
    Account(ListAccount),
}

/// List entities managed by balrog
#[derive(Args, Debug)]
pub struct ListAccount {}
