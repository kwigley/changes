mod cmds;
mod error;

use clap::Parser;
use cmds::{add, check};
use error::Result;

use crate::cmds::generate;

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
        Cli::Generate => generate(),
        Cli::Check => check(),
    }
}
