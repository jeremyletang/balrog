extern crate clap;

use account::Account;
use clap::Parser;
use info::Info;
use init::Init;
use list::List;
use network::Network;
use query::Query;
use set::Set;
use transact::Transact;

pub mod account;
pub mod info;
pub mod init;
pub mod list;
pub mod network;
pub mod query;
pub mod set;
pub mod transact;

const DEFAULT_CONFIG_FOLDER: &'static str = ".balrog";

/// The balrog wallet. An alternative implementation of a wallet for the vega network.
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Jeremy Letang me@jeremyletang.com>")]
#[clap(name = "balrog")]
#[clap(about = "An alternative vega wallet", long_about = None)]
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
    #[clap(subcommand)]
    Set(Set),
    Init(Init),
    Info(Info),
    #[clap(subcommand)]
    List(List),
    #[clap(subcommand)]
    Network(Network),
    #[clap(subcommand)]
    Query(Query),
    Transact(Transact),
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
