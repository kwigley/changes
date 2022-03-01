mod add;
mod generate;

#[derive(clap::Subcommand, Debug)]
pub enum Subcommand {
    Add(add::Subcommand),
    Generate(generate::Subcommand),
}
