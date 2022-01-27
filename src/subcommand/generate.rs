use crate::App;
use carli::prelude::cmd::*;

#[derive(clap::Parser, Debug)]
pub struct Subcommand {
    version: String,
}

impl Execute<App> for Subcommand {
    fn execute(&self, _context: &App) -> Result<()> {
        Ok(())
    }
}
