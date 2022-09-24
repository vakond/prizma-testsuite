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
            help = "Optional list of names or numbers of tests to run, comma-delimited"
        )]
        select: Option<String>,

        #[arg(
            short = 'x',
            long,
            help = "Optional list of names or numbers of tests to exclude from run, comma-delimited"
        )]
        exclude: Option<String>,
    },

    #[clap(about = "Show available testcases")]
    List {
        #[arg(
            short,
            long,
            help = "Optional list of names or numbers of tests to show, comma-delimited"
        )]
        select: Option<String>,

        #[arg(
            short = 'x',
            long,
            help = "Optional list of names or numbers of tests to hide, comma-delimited"
        )]
        exclude: Option<String>,
    },

    #[clap(about = "Show manual")]
    Man {},
}

/// Parse arguments.
pub fn application() -> Application {
    Application::parse()
}
