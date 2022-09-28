extern crate core;

mod cli;
mod action_doc;
mod github;
mod markdown;
mod workflow_docs;

use std::fs;
use std::path::Path;
use github::action::GithubAction;
use github::workflow::GitHubWorkflow;
use crate::markdown::MarkdownDocumented;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::parse_args();

    match args.command {
        cli::Commands::Action { action_file } => {
            let gha = GithubAction::parse(&action_file)
                .expect("Unable to parse Github action");
            let readme_path = Path::new(&action_file).to_path_buf().parent().unwrap().join("README.md");
            fs::write(readme_path.to_str().unwrap(), gha.to_markdown().to_string()).expect("Unable to write readme");
        }
        cli::Commands::Workflow { workflow_file, output_file } => {
            let workflow = GitHubWorkflow::parse(&workflow_file)
                .expect("Unable to parse workflow");
            let wf_path = Path::new(&workflow_file).to_path_buf();
            let readme_path = match output_file {
                Some(out) => { Path::new(&out).to_path_buf() },
                None => {
                    wf_path
                        .parent()
                        .unwrap()
                        .join(&format!("{}.md", wf_path.file_stem().unwrap().to_str().unwrap()))
                }
            };

            println!("Writing workflow readme {:?}", &readme_path);
            fs::write(readme_path.to_str().unwrap(), workflow.to_markdown().to_string()).expect("Unable to write readme");
        }
    }

    Ok(())
}
