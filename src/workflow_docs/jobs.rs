use serde::{Deserialize};
use crate::markdown::Markdown;

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

impl WorkflowJob {
   pub fn to_markdown(&self) -> String {
      let mut d = Markdown::new();

      d.append_text(&format!("### {} ", &self.name));
      d.append_new_lines(2);

      match &self.uses {
         Some(uses) => {
           d.append_line(&format!("Uses the callable workflow: `{}`", uses));
           d.append_new_lines(1);
         }
         _ => {}
      }

      if !self.steps.is_empty() {
         d.append_line(&format!("**Steps:**"));
         for step in &self.steps {
            let uses = match &step.uses {
               Some(uses) => { uses }
               None => { "" }
            };

            let name = match &step.name {
               Some(name) => { name }
               None => {
                  match &step.id {
                     Some(id) => { id }
                     None => { uses }
                  }
               }
            };

            d.append_line(&format!("* {} (`{}`)", name, match uses.is_empty() { true => "script", false => uses } ));
         }
         d.append_new_lines(1);
      }

      return d.to_string()
   }
}
