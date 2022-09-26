#![deny(warnings)]
#![forbid(unsafe_code)]

mod cli;
mod config;
mod error;
mod filebuf;
mod lister;
mod manual;
mod runner;
mod testcases;

use crate::error::Result;

fn main() {
    use std::error::Error as _;
    if let Err(err) = execute(cli::application()) {
        eprintln!("Error: {}", err);
        let mut err = err.source();
        while let Some(cause) = err {
            eprintln!("Caused by:\n\t{}", cause);
            err = cause.source();
        }
        std::process::exit(config::FAILURE);
    }
}

use crate::cli::{Application, Command};

fn execute(app: Application) -> Result<()> {
    testcases::verify()?;

    match app.cmd {
        Command::Run { select, exclude } => runner::execute(select, exclude),

        Command::List { select, exclude } => lister::execute(select, exclude),

        Command::Man {} => manual::execute(),
    }
}
