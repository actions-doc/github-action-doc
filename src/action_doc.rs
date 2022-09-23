use std::collections::HashMap;
use std::fmt::{Formatter};
use std::fmt::Write;
use std::fs::File;
use std::path::Path;
use serde::{Deserialize};

#[derive(Debug, Deserialize, PartialEq)]
pub struct GithubActionInput {
    description: String,
    default: Option<String>,
    required: bool
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct GithubActionOutput {
    description: String
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct GithubAction {
    name: String,
    description: String,
    author: Option<String>,
    inputs: Option<HashMap<String, GithubActionInput>>,
    outputs: Option<HashMap<String, GithubActionOutput>>
}

impl GithubAction {
    pub fn parse(path: &String) -> Result<GithubAction, Box<dyn std::error::Error>> {
        let p = Path::new(path).join("action.yml");
        // println!("Opening: {}", p);
        let f = File::open(p).unwrap();
        let action: GithubAction = serde_yaml::from_reader(f)?;
        println!("{}", action);
        Ok(action)
    }

    fn inputs_to_markdown(inputs: Option<HashMap<String, GithubActionInput>>) -> String {
        match inputs {
            Some(inputs) => {
                let mut mdown = String::new();

                writeln!(mdown, "| Input | Description | Required | Default Value |").unwrap();
                writeln!(mdown, "| :---- | :---------- | :------- |:--------------|").unwrap();

                for (name, input) in inputs {
                    let input_default = match input.default {
                        Some(x) => x,
                        None => " ".to_string()
                    };
                    writeln!(mdown, "| {} | {} | {} | {} |",
                             name, input.description, input.required, input_default
                    ).unwrap();
                }

                return mdown;
            }
            None => "No inputs.".to_string()
        }
    }

    fn outputs_to_markdown(outputs: Option<HashMap<String, GithubActionOutput>>) -> String {
        match outputs {
            Some(inputs) => {
                let mut mdown = String::new();

                writeln!(mdown, "| Output | Description |").unwrap();
                writeln!(mdown, "| :----- | :----------- |").unwrap();

                for (name, output) in inputs {
                    writeln!(mdown, "| {} | {} |", name, output.description).unwrap();
                }

                return mdown;
            }
            None => "No outputs.".to_string()
        }
    }
    pub fn to_markdown(self) -> String {
        let mut  mdown = String::new();

        writeln!(mdown, "# {}", self.name).unwrap();
        writeln!(mdown).unwrap();
        mdown += &self.description;

        writeln!(mdown).unwrap();
        writeln!(mdown).unwrap();
        writeln!(mdown, "## Inputs").unwrap();
        writeln!(mdown).unwrap();
        writeln!(mdown, "{}", GithubAction::inputs_to_markdown(self.inputs)).unwrap();

        writeln!(mdown).unwrap();
        writeln!(mdown, "## Outputs").unwrap();
        writeln!(mdown).unwrap();
        writeln!(mdown, "{}", GithubAction::outputs_to_markdown(self.outputs)).unwrap();

        return mdown
    }

}

impl std::fmt::Display for GithubAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(name: {}, description: {})", self.name, self.description)
    }
}
