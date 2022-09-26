use std::collections::HashMap;
use std::fmt::Formatter;
use serde::{Deserialize};

#[derive(Debug, Deserialize, PartialEq)]
pub struct GithubWorkflowInput {
    pub description: String,
    pub default: Option<String>,
    #[serde(default)]
    pub required: bool,
    #[serde(alias = "type")]
    pub input_type: String
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct GithubWorkflowSecret {
    pub description: String,
    #[serde(default)]
    pub required: bool
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

#[derive(Debug, Deserialize, PartialEq)]
pub struct WorkflowJob {
    pub name: String,
    pub uses: Option<String>,
    #[serde(default)]
    pub needs: Vec<String>,
    #[serde(default)]
    pub steps: Vec<WorkflowJobStep>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct WorkflowJobStep {
    pub id: Option<String>,
    pub name: Option<String>,
    pub run: Option<String>,
    pub uses: Option<String>
}

impl std::fmt::Display for GithubWorkflowTrigger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
