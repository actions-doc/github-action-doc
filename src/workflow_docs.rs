mod inputs;
mod triggers;

use std::collections::HashMap;
use std::fs::File;
use heck::ToSnakeCase;
use serde::{Deserialize};
use triggers::{GithubWorkflowTrigger, GithubWorkflowTriggerPayload };

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
