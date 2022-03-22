mod change;
mod cli;
mod config;
mod error;
mod release;
mod subcommand;
mod template;

pub const DEFAULT_CONFIG: &str = "changes.toml";
// TODO: move to config
pub const CHANGES_DIR: &str = ".test_changes/";
#[doc(hidden)]
pub use cli::Cli;
