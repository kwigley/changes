mod cmds;
mod error;

use clap::Parser;
use cmds::{add, check, generate};
use error::Result;

#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), about, version, author)]
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
        Cli::Generate => generate(),
        Cli::Check => check(),
    }
}
