use changes::Opt;
use std::process;
use structopt::StructOpt;

fn main() {
    let args = Opt::from_args();
    match changes::run(args) {
        Ok(_) => process::exit(0),
        Err(e) => {
            println!("{:?}", e);
            process::exit(1)
        }
    }
}
