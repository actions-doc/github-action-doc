use std::collections::HashMap;
use std::fmt::{Formatter};
use indoc::indoc;
use serde::{Deserialize};
use super::inputs::{GithubWorkflowInput, GithubWorkflowSecret};

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

    fn doc_workflow_call(&self) -> String {
        let mut mdown = String::new();
        mdown.push_str(indoc!("
            ### Workflow Call

            This workflow can be [called](https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#workflow_call)
            from other workflows.

            #### Inputs

        "));

        if !self.inputs.is_empty() {
            mdown.push_str(indoc!("
                | Input              | Description | Type     | Required | Default |
                |:-------------------|:------------|:---------|:---------|:--------|
            "));
            for (name, input) in &self.inputs {
                let input_default = match &input.default {
                    Some(x) => format!("`{}`", x),
                    None => " ".to_string()
                };
                mdown.push_str(&format!(
                    "| {:18} | {:18} | {:10} | {:10} | {:18} |\n",
                    name,
                    input.description,
                    input.input_type,
                    match input.required { true => "yes", false => "no" },
                    input_default
                ));
            }
        } else {
            mdown.push_str("No inputs.")
        }

        mdown.push_str("\n\n#### Secrets\n");
        if !self.secrets.is_empty() {
            mdown.push_str(indoc!("
                | Secret             | Description     | Required |
                |:-------------------|:----------------|:---------|
            "));
            for (name, secret) in &self.secrets {
                mdown.push_str(&format!(
                    "| {:18} | {:18} | {:10} |\n",
                    name,
                    secret.description,
                    match secret.required { true => "yes", false => "no" }
                ));
            }
        } else {
            mdown.push_str("No secrets.")
        }

        return mdown
    }

    pub fn to_markdown(&self, trigger: &GithubWorkflowTrigger) -> String {
        return match trigger {
            GithubWorkflowTrigger::PullRequest => {
                self.doc_pull_request()
            },
            GithubWorkflowTrigger::WorkflowCall => {
                self.doc_workflow_call()
            }
            _ => { format!("") }
        }
    }
}
