extern crate clap;

use account::Account;
use clap::Parser;
use init::Init;
use list::List;
use network::Network;

pub mod account;
pub mod init;
pub mod list;
pub mod network;

const DEFAULT_CONFIG_FOLDER: &'static str = ".balrog";

/// The balrog wallet. An implementation of wallet for the vega network.
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Jeremy Letang me@jeremyletang.com>")]
//#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    /// Sets a custom home directory for balrog
    #[clap(default_value_t, short, long)]
    pub home: Home,

    // #[clap(short, long, default_value = "$HOME")]
    // pub home: String,
    #[clap(subcommand)]
    pub subcmd: SubCommands,
}

#[derive(Parser, Debug)]
//#[clap(setting = AppSettings::ColoredHelp)]
pub enum SubCommands {
    #[clap(subcommand)]
    Account(Account),
    Init(Init),
    #[clap(subcommand)]
    List(List),
    #[clap(subcommand)]
    Network(Network),
}

#[derive(Debug)]
pub struct Home(String);

impl Home {
    pub fn path(&self) -> String {
        return self.0.clone();
    }
}

impl Default for Home {
    fn default() -> Self {
        let mut path = dirs::home_dir().unwrap();
        path.push(DEFAULT_CONFIG_FOLDER);
        return Home(path.to_str().unwrap().into());
    }
}

impl std::string::ToString for Home {
    fn to_string(&self) -> String {
        return self.0.clone();
    }
}

impl ::std::str::FromStr for Home {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Home(s.into()));
    }
}
