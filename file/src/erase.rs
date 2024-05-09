/*
    Erase certain content lines
*/
use crate::tool::{self, Constructor};

pub struct Erase {
    lines: Vec<(usize, String)>,
}

impl Erase {
    fn new(lines: Vec<(usize, String)>) -> Self {
        Self { lines }
    }

    /// Returns a the original lines minus the lines specified given their index
    fn erase_lines(&self, indexes: &[usize]) -> Vec<(usize, String)> {
        let original_lines = self.lines.clone();
        let mut lines: Vec<(usize, String)> = Vec::new();
        for (index, line) in original_lines.iter() {
            if indexes.contains(&index) {
                continue;
            }
            lines.push((*index, line.to_owned()));
        }
        lines
    }
}

impl Constructor for Erase {
    /// Meant to be given the unwanted lines which then reorganizes the line order
    fn construct(&self, result: &[(usize, String)]) -> String {
        let lines = self.lines.clone();
        let mut new_lines: Vec<String> = Vec::new();
        for (index, line) in lines.iter() {
            if result.iter().any(|(inde, _)| inde == index) {
                continue;
            }
            new_lines.push(line.to_owned());
        }
        let enumerated_lines: Vec<(usize, String)> = new_lines.iter().enumerate().map(|(index, line)| (index + 1, line.to_owned())).collect();
        let max_char_count = tool::len(&enumerated_lines[enumerated_lines.len() - 1].0) + 1;
        let mut interface: Vec<String> = Vec::new();
        for (index, string) in enumerated_lines.iter() {
            let space = " ".repeat(max_char_count - tool::len(index));
            interface.push(format!("{}:{}{}", index, space, string));
        }
        interface.join("\n")
    }
}
