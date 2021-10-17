use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Init {
    /// Force erasing existing configurations
    #[clap(short, long)]
    pub force: bool,
}
