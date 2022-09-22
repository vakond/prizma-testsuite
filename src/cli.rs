//! cli.rs

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Application {
    #[command(subcommand)]
    pub cmd: Command,
}

//use std::path::PathBuf;

#[derive(clap::Subcommand)]
pub enum Command {
    #[clap(about = "Run testcases")]
    Run {},

    #[clap(about = "Show available testcases")]
    List {},

    #[clap(about = "Show manual")]
    Man {},
}

/// Parse arguments.
pub fn application() -> Application {
    Application::parse()
}
