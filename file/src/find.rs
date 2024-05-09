/*
    Find specific lines containing the string instances wanted
*/
use crate::tool::{self, Constructor};

pub struct Find {
    pub lines: Vec<(usize, String)>,
}

impl Find {
    pub fn new(lines: Vec<(usize, String)>) -> Self {
        Self { lines }
    }

    /// Returns the lines that contain the string given
    pub fn find_lines(&self, string: &str) -> Vec<(usize, String)> {
        let mut instances: Vec<(usize, String)> = Vec::new();
        for line in &self.lines {
            // accessing string in tuple (number, string)
            if line.1.contains(string) {
                instances.push(line.clone());
            }
        }
        instances
    }
}

impl Constructor for Find {
    fn construct(&self, lines : &[(usize, String)]) -> String {
        let mut interface: Vec<String> = Vec::new();
        let max_char_count = tool::len(&lines[lines.len() - 1].0) + 1;
        for (index, line) in lines {
            let space = " ".repeat(max_char_count - tool::len(index));
            interface.push(format!("{}:{}{}", index, space, line));
        }
        interface.join("\n")
    }
}
