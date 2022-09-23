mod cli;
mod action_doc;

use std::fs;
use std::path::Path;
use action_doc::GithubAction;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::foo();

    match args.command {
        cli::Commands::Action { path } => {
            let gha = GithubAction::parse(&path)
                .expect("Unable to parse Github action.yaml");
            let readme_path = Path::new(&path).join("README.md");
            fs::write(readme_path.to_str().unwrap(), gha.to_markdown()).expect("Unable to write readme");
        }
    }

    Ok(())
}
