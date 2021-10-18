use clap::Parser;
use cmd::{Opts, SubCommands};
use serde::Serialize;

mod cmd;
mod errors;
mod init;
mod keystore;

fn main() {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommands::Account(acc) => handle_account_cmd(&opts.home.path(), acc),
        SubCommands::List(t) => {
            println!("list subcommand {:#?}", t)
        }
        SubCommands::Network(t) => {
            println!("list subcommand {:#?}", t)
        }
        SubCommands::Init(args) => match init::run(&opts.home.path(), args.force) {
            Ok(output) => print_success(&output),
            Err(e) => {
                print_error(&e.desc());
            }
        },
    }
}

fn handle_account_cmd(home: &str, acc: cmd::account::Account) {
    use cmd::account::Account;
    let pass = match rpassword::read_password_from_tty(Some("passphrase: ")) {
        Ok(p) => p,
        Err(e) => {
            print_error(&e.to_string());
            return;
        }
    };

    match acc {
        Account::Generate(generate) => match generate.address {
            Some(addr) => {}
            None => match keystore::generate(home, &pass) {
                Ok(output) => print_success(&output),
                Err(e) => print_error(&e.desc()),
            },
        },
        Account::Import(import) => match keystore::import(home, &import.mnemonic, &pass) {
            Ok(output) => print_success(&output),
            Err(e) => print_error(&e.desc()),
        },
        _ => panic!("unsupported"),
    }
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
