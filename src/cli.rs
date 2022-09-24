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
    #[clap(about = "Run all or selected testcases")]
    Run {
        #[arg(
            short,
            long,
            help = "Names or numbers of tests to run, comma-delimited",
            default_value_t = String::default()
        )]
        select: String,

        #[arg(
            short = 'x',
            long,
            help = "Names or numbers of tests to exclude from run, comma-delimited",
            default_value_t = String::default()
        )]
        exclude: String,
    },

    #[clap(about = "Show available testcases")]
    List {},

    #[clap(about = "Show manual")]
    Man {},
}

/// Parse arguments.
pub fn application() -> Application {
    Application::parse()
}
