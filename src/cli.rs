use std::fmt::Display;
use std::path::{Path, PathBuf};

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};

use crate::error::{Error, Result};
use crate::subcommand::Subcommand;
use crate::DEFAULT_CONFIG;

// #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
// pub struct ChangesConfig {
//     pub prompt: Option<String>,
// }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    pub prompt: Option<String>,
}

#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), about, version, author)]
pub struct Cli {
    #[clap(subcommand)]
    command: Subcommand,
    #[clap(
    	short,
    	long,
    	value_name = "PATH",
    	default_value = DEFAULT_CONFIG,
    )]
    config: PathBuf,
    #[clap(short, long, env, value_name = "PATH")]
    workdir: Option<PathBuf>,
    #[clap(short, long, env, value_name = "PATH")]
    pub repository: Option<PathBuf>,
}

impl Config {
    /// Parses the config file and returns the values.
    pub fn parse(path: &Path) -> Result<Config> {
        Ok(config::Config::builder()
            .add_source(config::File::from(path))
            .add_source(config::Environment::with_prefix("CHANGES").separator("_"))
            .build()?
            .try_deserialize()?)
    }
}

impl Cli {
    pub fn execute(&mut self) -> Result<()> {
        if let Some(workdir) = &self.workdir {
            self.config = workdir.join(&self.config);
            self.repository = match &self.repository {
                Some(repository) => Some(workdir.join(repository)),
                None => Some(workdir.clone()),
            };
        }

        // Parse the configuration file.
        let mut path = self.config.clone();
        if !path.exists() {
            if let Some(config_path) = dirs_next::config_dir()
                .map(|dir| dir.join(env!("CARGO_PKG_NAME")).join(DEFAULT_CONFIG))
            {
                path = config_path;
            }
        }

        // Load the default configuration if necessary.
        let config = if path.exists() {
            Config::parse(&path)?
        } else {
            todo!()
        };

        match &self.command {
            Subcommand::Add(cmd) => cmd.execute(&config),
            Subcommand::Generate(cmd) => cmd.execute(&config),
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
