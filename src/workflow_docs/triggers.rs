use indoc::indoc;
use crate::markdown::{backtick, Markdown};
use crate::github::workflow::{GithubWorkflowTrigger, GithubWorkflowTriggerPayload};
use crate::MarkdownDocumented;

impl GithubWorkflowTriggerPayload {

    fn doc_pull_request(&self) -> String {
        let mut doc = Markdown::new();

        doc.append_line("### Pull Request");
        doc.append_new_lines(1);

        if !self.branches.is_empty() {
            doc.append_line("Branches:");
            doc.append_list(&self.branches);
        }

        if !self.paths.is_empty() {
            doc.append_line("Paths:");
            let paths = &self.paths.iter().map(|t| backtick(t)).collect();
            doc.append_list(paths);
        }

        return doc.to_string();
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

// impl MarkdownDocumented for GithubWorkflowTriggerPayload {
//     fn to_markdown(&self) -> Markdown {
//         todo!()
//     }
// }
