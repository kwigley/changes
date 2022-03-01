use crate::error::Result;
use crate::subcommand::Subcommand;
use clap::Parser;

// TODO: move to config
pub const CHANGES_DIR: &str = ".test_changes/";

#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), about, version, author)]
pub struct App {
    #[clap(subcommand)]
    command: Subcommand,
}

impl App {
    pub fn execute(&self) -> Result<()> {
        match &self.command {
            Subcommand::Add(cmd) => cmd.execute(self),
            Subcommand::Generate(cmd) => cmd.execute(self),
        }
    }
}
