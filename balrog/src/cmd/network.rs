use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
//#[clap(setting = AppSettings::ColoredHelp)]
pub enum Network {
    #[clap(version = "1.0")]
    Import(NetworkImport),
}

/// List entities managed by balrog
#[derive(Args, Debug)]
//#[clap(setting = AppSettings::ColoredHelp)]
pub struct NetworkImport {}
