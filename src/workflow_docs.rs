use std::collections::HashMap;
use std::fmt::{Formatter};
use std::fs::File;
use heck::ToSnakeCase;
use serde::{Deserialize};

#[derive(Debug, Deserialize, PartialEq)]
pub struct GithubWorkflowInput {
    description: String,
    default: Option<String>,
    #[serde(default)]
    required: bool,
    #[serde(alias = "type")]
    input_type: String
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct GithubWorkflowSecret {
    description: String,
    #[serde(default)]
    required: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct GithubWorkflowTriggerPayload {
    #[serde(default)]
    pub branches: Vec<String>,
    #[serde(default)]
    pub paths: Vec<String>,
    #[serde(default)]
    pub inputs: HashMap<String, GithubWorkflowInput>,
    #[serde(default)]
    pub secrets: HashMap<String, GithubWorkflowSecret>
}

#[derive(Debug, Deserialize, Eq, PartialEq, Hash)]
pub enum GithubWorkflowTrigger {
    #[serde(alias = "pull_request")]
    PullRequest,
    #[serde(alias = "push")]
    Push,
    #[serde(alias = "workflow_call")]
    WorkflowCall,
    #[serde(alias = "workflow_dispatch")]
    WorkflowDispatch
}

impl std::fmt::Display for GithubWorkflowTrigger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl GithubWorkflowTriggerPayload {

    fn doc_pull_request(&self) -> String {
        let mut mdown = format!("### Pull Request\n\n");

        if self.branches.is_empty() {
            mdown.push_str("Branches: all")
        } else {
            mdown.push_str("Branches: ");
            mdown.push_str(&self.branches.join(", "));
            mdown.push_str("\n\n");
        }

        if !self.paths.is_empty() {
            mdown.push_str("Paths:\n");
            for path in &self.paths {
                mdown.push_str(&format!("* `{}`", path));
            }
            mdown.push_str("\n\n")
        }

        return mdown;
    }

    fn to_markdown(&self, trigger: &GithubWorkflowTrigger) -> String {
        return match trigger {
            GithubWorkflowTrigger::PullRequest => {
                self.doc_pull_request()
            }
            _ => { format!("") }
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct GitHubWorkflow {
    pub name: String,
    pub on: HashMap<GithubWorkflowTrigger, GithubWorkflowTriggerPayload>
}

impl GitHubWorkflow {
    pub fn parse(path: &String) -> Result<GitHubWorkflow, Box<dyn std::error::Error>> {
        let f = File::open(path).unwrap();
        let action: GitHubWorkflow = serde_yaml::from_reader(f)?;
        Ok(action)
    }

    pub fn to_markdown(self) -> String {
        let mut mdown = String::new();

        mdown.push_str(&format!("# {}\n\n", self.name));

        mdown.push_str("## Triggers\n\n");
        for (trigger, _payload) in &self.on {
            mdown.push_str(&format!("* {}\n", trigger.to_string().to_snake_case()));
        }
        mdown.push_str("\n");

        for (trigger, payload) in &self.on {
            mdown.push_str(&payload.to_markdown(trigger));
        }

        return mdown;
    }
}
