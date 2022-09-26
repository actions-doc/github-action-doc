mod inputs;
mod jobs;
mod triggers;

use std::collections::HashMap;
use std::fs::File;
use heck::ToSnakeCase;
use serde::{Deserialize};
use triggers::{GithubWorkflowTrigger, GithubWorkflowTriggerPayload };
use jobs::WorkflowJob;
use crate::markdown::Markdown;

#[derive(Debug, Deserialize, PartialEq)]
pub struct GitHubWorkflow {
    pub name: String,
    pub on: HashMap<GithubWorkflowTrigger, GithubWorkflowTriggerPayload>,
    pub jobs: HashMap<String, WorkflowJob>
}

impl GitHubWorkflow {
    pub fn parse(path: &String) -> Result<GitHubWorkflow, Box<dyn std::error::Error>> {
        let f = File::open(path).unwrap();
        let action: GitHubWorkflow = serde_yaml::from_reader(f)?;
        Ok(action)
    }

    pub fn to_markdown(self) -> String {
        let mut doc = Markdown::new();

        doc.append_heading(&self.name);

        // triggers
        doc.append_line("## Triggers\n");
        let trigger_items = &self.on.keys().map(|t| t.to_string().to_snake_case()).collect();
        doc.append_list(trigger_items);
        doc.append_new_lines(1);

        for (trigger, payload) in &self.on {
            doc.append_text(&payload.to_markdown(trigger));
        }

        // jobs
        doc.append_line("## Jobs\n");
        for (_name, job) in &self.jobs {
            doc.append_text(&job.to_markdown());
        }

        return doc.to_string();
    }
}
