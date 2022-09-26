use crate::github::workflow::WorkflowJob;
use crate::markdown::Markdown;

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
