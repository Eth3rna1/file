/*
    Append certain content to a file
*/
use crate::tool::{self, Constructor};

pub struct Append {
    lines: Vec<String>,
}

impl Append {
    fn new(lines: Vec<String>) -> Self {
        Self { lines }
    }

    /// Returns the original lines plus the new lines, equalling the new content
    pub fn append_lines(&self, new_lines: &[String]) -> Vec<String> {
        let mut lines = self.lines.clone();
        lines.extend(new_lines.to_owned());
        lines
    }
}

impl Constructor for Append {}
