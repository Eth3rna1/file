/*
    Inject content within content given the string lines
*/
use crate::tool::{self, Constructor};
use std::collections::HashMap;

pub struct Inject {
    lines: Vec<(usize, String)>,
}

impl Inject {
    pub fn new(lines: Vec<(usize, String)>) -> Self {
        Self { lines }
    }

    fn join_lines(&self, _lines: &[(usize, String)]) -> Vec<(usize, String)> {
        let mut lines: HashMap<usize, String> = HashMap::from_iter(self.lines.clone());
        let new_lines: HashMap<usize, String> = HashMap::from_iter(_lines.to_vec());
        lines.extend(new_lines);
        let max: usize = *lines.keys().max().unwrap();
        let mut joined_lines: Vec<(usize, String)> = Vec::new();
        for i in 0..=max {
            joined_lines.push((i, lines[&i].to_owned()));
        }
        joined_lines
    }

    /// Returns the original content plus the newly injected conted within with updated indexes
    pub fn inject_lines(
        &mut self,
        index: usize,
        lines: &[(usize, String)],
        with_indentation: bool,
    ) -> Result<Vec<(usize, String)>, &'static str> {
        if self.lines.iter().all(|(_index, _)| *_index != index) {
            // meaning index does not exist
            return Err("Index out of bounds");
        }
        let mut original_lines = self.lines.clone();
        let new_lines: Vec<(usize, String)>;
        if with_indentation {
            let indent_size =
                tool::measure_indent_size(&original_lines[original_lines.len() - 1].1);
            let mut modified_lines: Vec<(usize, String)> = Vec::new();
            lines.into_iter().for_each(|(ind, stri)| {
                modified_lines.push((*ind, format!("{}{}", " ".repeat(indent_size), stri)))
            });
            new_lines = self.join_lines(&modified_lines);
        } else {
            new_lines = self.join_lines(lines);
        }
        Ok(new_lines)
    }
}

impl Constructor for Inject {}
