use clap::{Subcommand};
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "github_action_doc")]
#[clap(about = "A GitHub action & workflow readme writer", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(arg_required_else_help = true)]
    #[clap(about = "Generate documentation for a Github action", long_about = None)]
    Action {
        #[clap(required = true, value_parser)]
        action_file: String
    }
}

pub fn parse_args() -> Cli {
    return Cli::parse();
}
