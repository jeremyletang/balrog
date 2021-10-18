use clap::Args;

#[derive(Args, Debug)]
//#[clap(setting = AppSettings::ColoredHelp)]
pub struct Init {
    /// Force erasing existing configurations
    #[clap(short, long)]
    pub force: bool,
}
