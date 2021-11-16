mod add;
mod error;

use crate::add::add;
use crate::error::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"))]
pub enum Cli {
    /// add a changelog entry
    Add,
    Generate,
    Check,
}

pub fn run(cmds: Cli) -> Result<()> {
    // TODO: read config, pass to fn
    match cmds {
        Cli::Add => add(),
        Cli::Generate => todo!(),
        Cli::Check => todo!(),
    }
}
