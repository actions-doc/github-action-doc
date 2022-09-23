use clap::{Subcommand};
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "gha-doc")]
#[clap(about = "A GitHub action readme writer", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(arg_required_else_help = true)]
    Action {
        #[clap(required = true, value_parser)]
        path: String
    }
}

pub fn foo() -> Cli {
    return Cli::parse();
}
