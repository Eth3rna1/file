/*
    Find specific lines containing the string instances wanted
*/
use crate::tool::Constructor;

pub struct Find {
    pub lines: Vec<(usize, String)>,
}

impl Find {
    pub fn new(lines: Vec<(usize, String)>) -> Self {
        Self { lines }
    }

    /// Returns the lines that contain the string given
    pub fn find_lines(&mut self, string: &str) -> Option<Vec<(usize, String)>> {
        let mut instances: Vec<(usize, String)> = Vec::new();
        for line in &self.lines {
            if line.1.contains(string) {
                // accessing string in tuple (number, string)
                instances.push(line.clone());
            }
        }
        if instances.is_empty() {
            return None;
        }
        Some(instances)
    }
}

impl Constructor for Find {}
