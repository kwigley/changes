pub mod add;
use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;

#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), about, version, author)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// add a changelog entry
    Add,
    /// generate a changelog section
    Generate,
    Check,
}

impl App {
    pub fn exec(self) -> Result<()> {
        match &self.command {
            Command::Add => add::run(),
            Command::Generate => generate(),
            Command::Check => check(),
        }
        Ok(())
    }
}

pub trait Main: io::Shared + Sized {
    /// Executes the requested subcommand for the application.
    ///
    /// The implementation of this method should be very simple in that it should only execute the
    /// requested subcommand and return its result. Any additional steps required to put the context
    /// in a more usable state for the subcommand should probably be done elsewhere in order to keep
    /// testing simple.
    fn execute(&self) -> error::Result<()> {
        self.subcommand().execute(self)
    }

    /// Returns the subcommand to be executed.
    fn subcommand(&self) -> &dyn Execute<Self>;
}

pub trait Execute<T>
where
    T: io::Shared,
{
    /// Executes the command using the given context.
    ///
    /// The command may mutably borrow any stream from the context in order to read input, or
    /// write to the error or global output streams. When the command encounters an error, it
    /// returns an [`Err`] containing [`crate::error::Error`]. If the command is successful,
    /// then it will return [`Ok`].
    fn execute(&self, context: &T) -> error::Result<()>;
}
