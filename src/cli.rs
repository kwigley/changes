use std::fmt::Display;
use std::path::{Path, PathBuf};

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};

use crate::error::{Error, Result};
use crate::subcommand::Subcommand;
use crate::DEFAULT_CONFIG;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ConfigFile {
    #[serde(default)]
    prompt: Option<String>,
}

impl ConfigFile {
    /// Parses the config file and returns the values.
    pub fn parse(path: &Path) -> Result<ConfigFile> {
        Ok(config::Config::builder()
            .add_source(config::File::from(path))
            .add_source(config::Environment::with_prefix("CHANGES").separator("_"))
            .build()?
            .try_deserialize()?)
    }
}

pub struct Config {
    pub prompt: String,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            prompt: "What type of change is this?".to_string(),
        }
    }
}

impl From<ConfigFile> for Config {
    fn from(c: ConfigFile) -> Self {
        Config {
            prompt: c.prompt.unwrap_or(Config::default().prompt),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), about, version, author)]
pub struct Cli {
    #[clap(subcommand)]
    command: Subcommand,
    #[clap(
        name = "config",
    	short,
    	long,
    	value_name = "PATH",
    	default_value = DEFAULT_CONFIG,
    )]
    config_path: PathBuf,
}

impl Cli {
    pub fn execute(&mut self) -> Result<()> {
        let config = if self.config_path.exists() {
            Config::from(ConfigFile::parse(&self.config_path)?)
        } else {
            // TODO: display error that config file is missing
            todo!()
        };

        match &self.command {
            Subcommand::Add(cmd) => cmd.execute(&config.prompt),
            Subcommand::Generate(cmd) => cmd.execute(),
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
