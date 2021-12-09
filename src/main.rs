use changes::Cli;
use clap::Parser;
use std::process;

fn main() {
    let args = Cli::parse();
    match changes::run(args) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    }
}
