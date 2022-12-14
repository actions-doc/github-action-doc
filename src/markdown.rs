#![allow(dead_code)]
use std::fmt::{Display, Formatter};
use std::ops;

// Simple markdown document container.
pub struct Markdown {
    doc: String
}

impl Markdown {

    // Returns a new [`Markdown`].
    pub fn new() -> Markdown {
        Markdown { doc: String::new() }
    }

    pub fn append_heading(&mut self, text: &str) {
        self.doc.push_str(&format!("# {}\n\n", text));
    }

    pub fn append_new_lines(&mut self, n: u8) {
        for _i in 0..n {
            self.doc.push_str("\n")
        }
    }

    pub fn append_text(&mut self, text: &str) {
        self.doc.push_str(text);
    }

    pub fn append_line(&mut self, text: &str) {
        self.doc.push_str(text);
        self.append_new_lines(1);
    }

    pub fn append_list(&mut self, items: &Vec<String>) {
        self.doc.push_str(&Self::make_list(items))
    }

    pub fn make_list(items: &Vec<String>) -> String {
        let mut m = String::new();

        for item in items {
            m.push_str(&format!("* {}\n", item));
        }

        m.push_str("\n");

        return m;
    }
}

impl Display for Markdown {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       f.write_str(&self.doc)
    }
}

impl ops::Add<&str> for Markdown {
    type Output = Markdown;

    fn add(mut self, rhs: &str) -> Self::Output {
        self.append_text(rhs);
        self
    }
}

impl ops::AddAssign<&str> for Markdown {
    fn add_assign(&mut self, rhs: &str) {
        self.append_text(rhs)
    }
}

pub trait MarkdownDocumented {
    fn to_markdown(&self) -> Markdown;
}

pub fn backtick(s: &String) -> String {
    format!("`{}`", s)
}
