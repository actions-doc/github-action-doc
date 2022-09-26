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
