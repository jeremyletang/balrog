use clap::Clap;
use cmd::{Opts, SubCommands};

mod cmd;

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:#?}", opts);

    match opts.subcmd {
        SubCommands::Account(t) => {
            println!("account subcommand {:#?}", t)
        }
        SubCommands::List(t) => {
            println!("list subcommand {:#?}", t)
        }
        SubCommands::Network(t) => {
            println!("list subcommand {:#?}", t)
        }
        SubCommands::Init(t) => {
            println!("init subcommand {:#?}", t)
        }
    }
}
