use clap::Args;

/// Initialize balrog
#[derive(Args, Debug)]
pub struct Init {
    /// Force erasing existing configurations
    #[clap(short, long)]
    pub force: bool,
}
