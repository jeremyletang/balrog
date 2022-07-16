use clap::Parser;
use client::{CoreBlockingClient, DatanodeV2BlockingClient};
use cmd::{Opts, SubCommands};
use dialoguer::{theme::ColorfulTheme, Password};
use errors::Error;
use serde::Serialize;

pub mod balances;
mod client;
mod cmd;
mod config;
mod errors;
mod init;
pub mod keystore;
mod list;
mod network;
pub mod paths;
mod pow;
mod query_list;
mod transact;
mod util;

fn main() {
    let opts: Opts = Opts::parse();
    let home = opts.home.path();

    let err = match opts.subcmd {
        SubCommands::Account(args) => handle_account_cmd(&home, args),
        SubCommands::List(args) => handle_list_cmd(&home, args),
        SubCommands::Info(args) => handle_info_cmd(&home, args),
        SubCommands::Network(args) => handle_network_cmd(&home, args),
        SubCommands::Init(args) => handle_init_cmd(&home, args),
        SubCommands::Set(args) => handle_set_cmd(&home, args),
        SubCommands::Query(args) => handle_query(&home, args),
        SubCommands::Transact(args) => handle_transact(&home, args),
    };

    if let Err(e) = err {
        print_error(&e.desc());
        std::process::exit(1);
    }
}

fn handle_transact(home: &str, args: cmd::transact::Transact) -> Result<(), Error> {
    let network = network(home, args.network)?;
    let address = address(home, None)?;
    let passphrase = passphrase("enter passphrase: ")?;
    let ks = keystore::load(home, &address, &passphrase)?;
    let mut clt = DatanodeV2BlockingClient::connect(network.primary_rpc.to_string())?;
    let mut coreclt = CoreBlockingClient::connect(network.primary_rpc.to_string())?;
    transact::transact(ks, network, &passphrase, &mut clt, &mut coreclt)?;
    return Ok(());
}

fn handle_query(home: &str, args: cmd::query::Query) -> Result<(), Error> {
    use cmd::query::Query;
    let _ = match args {
        Query::Balances(qb) => {
            let network = network(home, qb.network)?;
            if qb.public_keys.len() > 0 {
                let _ = balances::show_pks(&network.primary_rpc.as_str(), qb.public_keys)?;
            } else {
                let address = address(home, None)?;
                let passphrase = passphrase("enter passphrase: ")?;
                let account_info = keystore::info(home, &address, &passphrase)?;
                let _ = balances::show(&network.primary_rpc.as_str(), &account_info)?;
            }
        }
        Query::List(subc) => handle_query_list(home, subc)?,
    };

    return Ok(());
}

fn handle_query_list(home: &str, args: cmd::query::QueryList) -> Result<(), Error> {
    use cmd::query::QueryList;
    let _ = match args {
        QueryList::Nodes(args) => {
            let network = network(home, args.network)?;
            let _ = query_list::show(&network.primary_rpc.as_str())?;
        }
    };
    return Ok(());
}

fn handle_set_cmd(home: &str, args: cmd::set::Set) -> Result<(), Error> {
    use cmd::set::Set;
    let v = match args {
        Set::Account(v) => config::set_account(home, &v.value)?,
        Set::Network(v) => config::set_network(home, &v.value)?,
    };
    print_success(&v);
    return Ok(());
}

fn handle_init_cmd(home: &str, args: cmd::init::Init) -> Result<(), Error> {
    print_success(&init::run(&home, args.force)?);
    return Ok(());
}

fn handle_info_cmd(home: &str, _args: cmd::info::Info) -> Result<(), Error> {
    print_success(&paths::info(&home));
    return Ok(());
}

fn handle_list_cmd(home: &str, ls: cmd::list::List) -> Result<(), Error> {
    use cmd::list::List;
    match ls {
        List::Accounts(_) => print_success(&list::accounts(home)?),
        List::Networks(_) => print_success(&list::networks(home)?),
    };

    return Ok(());
}

fn handle_account_cmd(home: &str, acc: cmd::account::Account) -> Result<(), Error> {
    use cmd::account::Account;
    match acc {
        Account::Create(_) => {
            let passphrase = passphrase_with_confirmation()?;
            let ks = keystore::generate(home, &passphrase)?;
            print_success(&ks);
            save_account_if_first_on_config(home, &ks.address)?;
        }
        Account::Import(import) => {
            let passphrase = passphrase_with_confirmation()?;
            let ks = keystore::import(home, &import.mnemonic, &passphrase)?;
            print_success(&ks);
            save_account_if_first_on_config(home, &ks.address)?;
        }
        Account::Info(info) => {
            let address = address(home, info.address)?;
            let passphrase = passphrase("enter passphrase: ")?;
            print_success(&keystore::info(home, &address, &passphrase)?);
        }
    };

    return Ok(());
}

fn save_account_if_first_on_config(home: &str, address: &str) -> Result<(), Error> {
    // update the config if it's the first one
    let mut cfg = config::load(home)?;
    if cfg.account.len() == 0 {
        cfg.account = address.to_string();
    }
    let _ = config::save(home, &cfg)?;
    return Ok(());
}

fn handle_network_cmd(home: &str, net: cmd::network::Network) -> Result<(), Error> {
    use cmd::network::Network;
    match net {
        Network::Import(_) => {}
        Network::Info(info) => {
            let network = network(home, info.network)?;
            print_success(&network);
        }
    };

    return Ok(());
}

fn address(home: &str, maybe_address: Option<String>) -> Result<String, Error> {
    let address = match maybe_address {
        Some(address) => address,
        None => {
            let cfg = config::load(home)?;
            cfg.account
        }
    };

    match address.is_empty() {
        true => Err(Error::NoAccountSpecified),
        false => Ok(address),
    }
}

fn network(home: &str, maybe_network: Option<String>) -> Result<network::Config, Error> {
    let network = match maybe_network {
        Some(n) => n,
        None => {
            let cfg = config::load(home)?;
            cfg.network
        }
    };

    match network.is_empty() {
        true => Err(Error::NoNetworkSpecified),
        false => network::load(home, &network),
    }
}

// fn passphrase_with_confirmation() -> Result<String, Error> {
//     let pass1 = passphrase("enter passphrase: ")?;
//     let pass2 = passphrase("confirm passphrase: ")?;
//     if pass1 == pass2 {
//         return Ok(pass1);
//     }
//     return Err(Error::PassphrasesDoesNotMatch);
// }

fn passphrase_with_confirmation() -> Result<String, Error> {
    return Ok(Password::with_theme(&ColorfulTheme::default())
        .with_prompt("enter passphrase")
        .allow_empty_password(true)
        .with_confirmation("confirm passphrase", "error: the passwords don't match.")
        .interact()?);
}

fn passphrase(text: &str) -> Result<String, Error> {
    return Ok(Password::with_theme(&ColorfulTheme::default())
        .with_prompt(text)
        .allow_empty_password(true)
        .interact()?);
}

#[derive(Serialize)]
struct ErrorDump<'l> {
    error: &'l str,
}

fn print_error(e: &str) {
    let err = ErrorDump { error: e };
    let serialized = serde_json::to_string_pretty(&err).unwrap();
    print!("{}", serialized);
}

fn print_success<T: Serialize>(output: &T) {
    let serialized = serde_json::to_string_pretty(output).unwrap();
    println!("{}", serialized);
}
