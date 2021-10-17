use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub enum Network {
    #[clap(version = "1.0")]
    Import(NetworkImport),
}

/// List entities managed by balrog
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct NetworkImport {}
