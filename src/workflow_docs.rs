mod jobs;
mod triggers;

use heck::ToSnakeCase;
use crate::github::workflow::{GitHubWorkflow };
use crate::markdown::Markdown;
use crate::MarkdownDocumented;

impl MarkdownDocumented for GitHubWorkflow {
    fn to_markdown(&self) -> Markdown {
        let mut doc = Markdown::new();

        doc.append_heading(&self.name);

        // triggers
        doc.append_line("## Triggers\n");
        let trigger_items = &self.on.keys().map(|t| t.to_string().to_snake_case()).collect();
        doc.append_list(trigger_items);
        doc.append_new_lines(1);

        for trigger in &self.on {
            doc.append_text(&trigger.to_markdown().to_string());
        }

        // jobs
        doc.append_line("## Jobs\n");
        for (_name, job) in &self.jobs {
            doc.append_text(&job.to_markdown().to_string());
        }

        return doc
    }
}
