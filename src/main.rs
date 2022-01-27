use carli::command::Main;
use changes::App;
use clap::Parser;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    let app = App::parse();
    if let Err(error) = app.execute() {
        error.exit();
    }
    Ok(())
}
