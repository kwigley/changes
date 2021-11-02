use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "changes")]
pub struct Opt {
    #[structopt(help = "add a changelog entry")]
    pub Add {
        #[structopt(short)]
        interactive: bool,
    },
    #[structopt(help = "edit a changelog entry")]
    Edit {
        #[structopt(short)]
        interactive: bool,
    },
}
