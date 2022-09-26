use std::collections::HashMap;
use std::fs::File;
use serde::{Deserialize};

#[derive(Debug, Deserialize, PartialEq)]
pub struct GithubActionInput {
    pub description: String,
    pub default: Option<String>,
    #[serde(default)]
    pub required: bool
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct GithubActionOutput {
    pub description: String
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct GithubAction {
    pub name: String,
    pub description: String,
    pub author: Option<String>,
    pub inputs: Option<HashMap<String, GithubActionInput>>,
    pub outputs: Option<HashMap<String, GithubActionOutput>>
}

impl GithubAction {
    pub fn parse(path: &String) -> Result<GithubAction, Box<dyn std::error::Error>> {
        let f = File::open(path).unwrap();
        let action: GithubAction = serde_yaml::from_reader(f)?;
        Ok(action)
    }
}
