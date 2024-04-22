mod change;
mod cli;
mod error;
mod release;
mod subcommand;
mod template;

#[doc(hidden)]
pub use cli::Cli;

const DEFAULT_CONFIG: &str = "changes.toml";
