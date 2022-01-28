mod add;
mod generate;

use crate::app::App;
use carli::prelude::cmd::*;

#[derive(clap::Subcommand, Debug)]
pub enum Subcommand {
    Add(add::Subcommand),
    Generate(generate::Subcommand),
}

impl Execute<App> for Subcommand {
    fn execute(&self, context: &App) -> Result<()> {
        match self {
            Self::Add(cmd) => cmd.execute(context),
            Self::Generate(cmd) => cmd.execute(context),
        }
    }
}
