mod add;
mod error;

use crate::add::add;
use error::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "changes")]
pub enum Opt {
    #[structopt(help = "add a changelog entry")]
    Add,
}

pub fn run(args: Opt) -> Result<()> {
    // read config, pass to fn
    match args {
        Opt::Add => add(),
    }
}
