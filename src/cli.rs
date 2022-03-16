use std::fmt::Display;
// use std::path::PathBuf;

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};

use crate::error::{Error, Result};
use crate::subcommand::Subcommand;
// use crate::DEFAULT_CONFIG;

#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), about, version, author)]
pub struct Cli {
    #[clap(subcommand)]
    command: Subcommand,
    //#[clap(
    // 	short,
    // 	long,
    // 	value_name = "PATH",
    // 	default_value = DEFAULT_CONFIG,
    // )]
    // config: PathBuf,
}

impl Cli {
    pub fn execute(&self) -> Result<()> {
        // TODO: parse config

        match &self.command {
            Subcommand::Add(cmd) => cmd.execute(self),
            Subcommand::Generate(cmd) => cmd.execute(self),
        }
    }
}

pub fn select_input<T: Display>(choices: &[T], prompt: String) -> Result<&T> {
    let idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(choices)
        .interact()?;
    choices
        .get(idx)
        .ok_or_else(|| Error::InvalidChangeType("None".to_owned()))
}
