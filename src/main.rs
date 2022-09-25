extern crate core;

mod cli;
mod action_doc;
mod markdown;
mod workflow_docs;

use std::fs;
use std::path::Path;
use action_doc::GithubAction;
use crate::workflow_docs::GitHubWorkflow;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::parse_args();

    match args.command {
        cli::Commands::Action { action_file } => {
            let gha = GithubAction::parse(&action_file)
                .expect("Unable to parse Github action");
            let readme_path = Path::new(&action_file).to_path_buf().parent().unwrap().join("README.md");
            fs::write(readme_path.to_str().unwrap(), gha.to_markdown()).expect("Unable to write readme");
        }
        cli::Commands::Workflow { workflow_file} => {
            let workflow = GitHubWorkflow::parse(&workflow_file)
                .expect("Unable to parse workflow");
            println!("Parsed workflow: {}", workflow.name);

            println!("{}", workflow.to_markdown());
        }
    }

    Ok(())
}
