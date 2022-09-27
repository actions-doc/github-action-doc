mod jobs;
mod triggers;

use std::collections::HashMap;
use std::fs::File;
use heck::ToSnakeCase;
use serde::{Deserialize};
use crate::github::workflow::{GithubWorkflowTrigger, GithubWorkflowTriggerPayload, WorkflowJob };
use crate::markdown::Markdown;
use crate::MarkdownDocumented;

#[derive(Debug, Deserialize, PartialEq)]
pub struct GitHubWorkflow {
    pub name: String,
    pub on: HashMap<GithubWorkflowTrigger, GithubWorkflowTriggerPayload>,
    pub jobs: HashMap<String, WorkflowJob>
}

struct Pair<F, S> {
    first: F,
    second: S
}

impl GitHubWorkflow {
    pub fn parse(path: &String) -> Result<GitHubWorkflow, Box<dyn std::error::Error>> {
        let f = File::open(path).unwrap();
        let action: GitHubWorkflow = serde_yaml::from_reader(f)?;
        Ok(action)
    }
}

impl MarkdownDocumented for GitHubWorkflow {
    fn to_markdown(&self) -> Markdown {
        let mut doc = Markdown::new();

        doc.append_heading(&self.name);

        // triggers
        doc.append_line("## Triggers\n");
        let trigger_items = &self.on.keys().map(|t| t.to_string().to_snake_case()).collect();
        doc.append_list(trigger_items);
        doc.append_new_lines(1);

        for (trigger, payload) in &self.on {
            let pair = Pair{ first: trigger, second: payload };
            doc.append_text(&pair.to_markdown().to_string());
        }

        // jobs
        doc.append_line("## Jobs\n");
        for (_name, job) in &self.jobs {
            doc.append_text(&job.to_markdown().to_string());
        }

        return doc
    }
}
