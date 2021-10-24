use clap::Parser;
use cmd::{Opts, SubCommands};
use errors::Error;
use rpassword;
use serde::Serialize;

mod cmd;
mod errors;
mod init;
pub mod keystore;
mod list;
pub mod paths;

fn main() {
    let opts: Opts = Opts::parse();
    let home = opts.home.path();

    let err = match opts.subcmd {
        SubCommands::Account(args) => handle_account_cmd(&home, args),
        SubCommands::List(args) => handle_list_cmd(&home, args),
        SubCommands::Info(args) => handle_info_cmd(&home, args),
        SubCommands::Network(t) => {
            println!("list subcommand {:#?}", t);
            Ok(())
        }
        SubCommands::Init(args) => handle_init_cmd(&home, args),
    };

    if let Err(e) = err {
        print_error(&e.desc());
        std::process::exit(1);
    }
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
        List::Networks(_) => unimplemented!("unimplemented command"),
    };

    return Ok(());
}

fn handle_account_cmd(home: &str, acc: cmd::account::Account) -> Result<(), Error> {
    use cmd::account::Account;
    match acc {
        Account::Generate(generate) => {
            let passphrase = passphrase_with_confirmation()?;
            match generate.address {
                Some(_addr) => return Err(Error::HomePathIsNotADir),
                None => print_success(&keystore::generate(home, &passphrase)?),
            }
        }
        Account::Import(import) => {
            let passphrase = passphrase_with_confirmation()?;
            print_success(&keystore::import(home, &import.mnemonic, &passphrase)?);
        }
        Account::Info(info) => {
            let passphrase = passphrase("enter passphrase: ")?;
            print_success(&keystore::info(home, &info.address, &passphrase)?);
        }
        _ => panic!("unsupported"),
    };

    return Ok(());
}

fn passphrase_with_confirmation() -> Result<String, Error> {
    let pass1 = passphrase("enter passphrase: ")?;
    let pass2 = passphrase("confirm passphrase: ")?;
    if pass1 == pass2 {
        return Ok(pass1);
    }
    return Err(Error::PassphrasesDoesNotMatch);
}

fn passphrase(text: &str) -> Result<String, Error> {
    return Ok(rpassword::read_password_from_tty(Some(text))?);
}

#[derive(Serialize)]
struct ErrorDump<'l> {
    error: &'l str,
}

fn print_error(e: &str) {
    let err = ErrorDump { error: e };
    let serialized = serde_json::to_string_pretty(&err).unwrap();
    println!("{}", serialized);
}

fn print_success<T: Serialize>(output: &T) {
    let serialized = serde_json::to_string_pretty(output).unwrap();
    println!("{}", serialized);
}
