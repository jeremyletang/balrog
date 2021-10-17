use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub enum List {
    #[clap(version = "1.0")]
    Account(ListAccount),
}

/// List entities managed by balrog
#[derive(Clap, Debug)]
pub struct ListAccount {}
