mod cli;
mod action_doc;

use std::fs;
use std::path::Path;
use action_doc::GithubAction;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::parse_args();

    match args.command {
        cli::Commands::Action { action_file } => {
            let gha = GithubAction::parse(&action_file)
                .expect("Unable to parse Github action");
            let readme_path = Path::new(&action_file).to_path_buf().parent().unwrap().join("README.md");
            fs::write(readme_path.to_str().unwrap(), gha.to_markdown()).expect("Unable to write readme");
        }
    }

    Ok(())
}
