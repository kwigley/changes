use crate::subcommand;
use carli::prelude::app::*;
use clap::Parser;
use std::cell::RefCell;
use std::io::{stderr, stdin, stdout};

// TODO(kw): move to config
pub const CHANGES_DIR: &str = ".test_changes/";

#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), about, version, author)]
pub struct App {
    /// The error output stream.
    #[clap(skip = RefCell::new(stderr().into()))]
    error: RefCell<Stream>,

    /// The input stream.
    #[clap(skip = RefCell::new(stdin().into()))]
    input: RefCell<Stream>,

    /// The global output stream.
    #[clap(skip = RefCell::new(stdout().into()))]
    output: RefCell<Stream>,

    #[clap(subcommand)]
    command: subcommand::Subcommand,
}

impl Main for App {
    fn subcommand(&self) -> &dyn Execute<Self> {
        &self.command
    }
}

impl Shared for App {
    fn error(&self) -> std::cell::RefMut<Stream> {
        self.error.borrow_mut()
    }

    fn input(&self) -> std::cell::RefMut<Stream> {
        self.input.borrow_mut()
    }

    fn output(&self) -> std::cell::RefMut<Stream> {
        self.output.borrow_mut()
    }
}
