use changes::Cli;
use clap::Parser;
use color_eyre::eyre::Result;
use std::process::exit;

fn main() -> Result<()> {
    color_eyre::install()?;

    let app = Cli::parse();
    if let Err(error) = app.execute() {
        eprintln!("{error}");
        exit(1)
    }
    Ok(())
}
