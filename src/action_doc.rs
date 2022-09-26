use std::collections::HashMap;
use std::fmt::{Formatter, Write};
use crate::github::action::{GithubAction, GithubActionInput, GithubActionOutput};
use crate::markdown::{Markdown, MarkdownDocumented};

impl MarkdownDocumented for GithubActionOutput {
    fn to_markdown(&self) -> Markdown {
        todo!()
    }
}

impl GithubAction {
    fn clean(s: &String) -> String {
        return s.trim()
            .replace("-", "‑")
            .replace(" ", " ")
    }

    fn inputs_to_markdown(inputs: &Option<HashMap<String, GithubActionInput>>) -> String {
        match inputs {
            Some(inputs) => {
                let mut mdown = String::new();

                mdown.push_str("| Input                | Description | Required | Default Value |\n");
                mdown.push_str("| :------------------- | :---------- | :------- |:--------------|\n");

                for (name, input) in inputs {
                    let input_default = match &input.default {
                        Some(x) => format!("`{}`", x),
                        None => " ".to_string()
                    };
                    writeln!(mdown, "| {:18} | {} | {} | {} |",
                             Self::clean(name),
                             input.description.replace("\n", "<br>"),
                             match input.required { true => "yes", false => "no" },
                             Self::clean(&input_default)
                    ).unwrap();
                }

                return mdown;
            }
            None => "No inputs.".to_string()
        }
    }

    fn outputs_to_markdown(outputs: &Option<HashMap<String, GithubActionOutput>>) -> String {
        match outputs {
            Some(inputs) => {
                let mut mdown = String::new();

                mdown.push_str(&format!("| {:18} | {:18} |\n", "Output", "Description"));
                mdown.push_str(&format!("| {:-<18} | {:-<18} |\n", ":", ":"));

                for (name, output) in inputs {
                    mdown.push_str(&format!("| {:18} | {:18} |", name, output.description));
                }

                return mdown;
            }
            None => "No outputs.".to_string()
        }
    }

}

impl MarkdownDocumented for GithubAction {
    fn to_markdown(&self) -> Markdown {
        let mut doc = Markdown::new();

        doc.append_heading(&self.name);

        doc += &self.description;

        doc += "\n\n## Inputs\n";
        doc += &Self::inputs_to_markdown(&self.inputs);

        doc += "\n## Outputs\n\n";
        doc += &Self::outputs_to_markdown(&self.outputs);

        return doc
    }
}

impl std::fmt::Display for GithubAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(name: {}, description: {})", self.name, self.description.trim())
    }
}
