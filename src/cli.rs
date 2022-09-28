use clap::{Subcommand};
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "github-action-doc", version)]
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
        /// Action YAML file
        #[clap(required = true, value_parser)]
        action_file: String
    },
    #[clap(arg_required_else_help = true)]
    #[clap(about = "Generate documentation for a Github workflow", long_about = None)]
    Workflow {
        /// Full with to the output file to write workflow documentation to
        #[clap(short = 'o', value_name = "OUTPUT_FILE", value_parser)]
        output_file: Option<String>,

        /// Workflow YAML file
        #[clap(required = true, value_parser)]
        workflow_file: String
    }
}

pub fn parse_args() -> Cli {
    return Cli::parse();
}
