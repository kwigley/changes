mod add;

use crate::app::App;
use carli::prelude::cmd::*;

/// The subcommands that are offered by the application.
#[derive(clap::Subcommand, Debug)]
pub enum Subcommand {
    Add(add::Subcommand),
}

impl Execute<App> for Subcommand {
    fn execute(&self, context: &App) -> Result<()> {
        match self {
            Self::Add(cmd) => cmd.execute(context),
        }
    }
}
