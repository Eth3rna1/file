/*
    Replace the contents of lines given
*/
use crate::tool::{self, Constructor};
use std::collections::HashMap;

pub struct Replace {
    lines: Vec<(usize, String)>,
}

impl Replace {
    pub fn new(lines: Vec<(usize, String)>) -> Self {
        Self {
            lines,
        }
    }

    /// Returns the original content but with some lines being modified
    pub fn replace_lines(
        &self,
        original_text: &str,
        replacement: &str,
        matches: bool,
        keep_lines: Option<&[usize]>,
    ) -> Vec<(usize, String)> {
        let lines = &self.lines.clone();
        let binding = Vec::new();
        let excluded_lines = keep_lines.unwrap_or(&binding);
        let mut modified_lines: Vec<(usize, String)> = Vec::new();
        if matches {
            for (index, line) in lines.iter() {
                if excluded_lines.contains(index) {
                    continue;
                }
                let splitted_line: Vec<&str> = line.split(" ").collect();
                let mut new_line: Vec<&str> = Vec::new();
                for word in splitted_line.iter() {
                    if *word == original_text {
                        new_line.push(replacement);
                        continue;
                    }
                    new_line.push(word);
                }
                modified_lines.push((*index, new_line.join(" ")));
            }
        } else {
            for (index, line) in lines.iter() {
                if excluded_lines.contains(index) {
                    continue;
                }
                let new_line = line.replace(original_text, replacement);
                modified_lines.push((*index, new_line));
            }
        }
        modified_lines
    }
}

impl Constructor for Replace {
    fn construct(&self, result: &[(usize, String)]) -> String {
        let mut lines: HashMap<usize, String> = HashMap::from_iter(self.lines.clone());
        let result: HashMap<usize, String> = HashMap::from_iter(result.to_vec());
        lines.extend(result);
        let max_index: usize = *lines.keys().max().unwrap();
        let mut new_lines: Vec<(usize, String)> = Vec::new();
        for i in 1..=max_index {
            new_lines.push((i, lines[&i].to_owned()));
        }
        let max_char_count = tool::len(&new_lines[new_lines.len() - 1].0) + 1;
        let mut lines: Vec<String> = Vec::new();
        for (index, line) in new_lines.iter() {
            let space = " ".repeat(max_char_count - tool::len(index));
            lines.push(format!("{}:{}{}", index, space, line));
        }
        lines.join("\n")
    }
}
