/*
    Inject content within content given the string lines
*/
use crate::tool::{self, Constructor};
use std::collections::HashMap;

fn tab_size(line : &str) -> usize {
    let mut size: usize = 0;
    for character in line.chars() {
        match character {
            ' ' => size += 1,
            '\t' => size += 4,
            _ => break,
        };
    }
    size
}

fn indent_line(indent_size : usize, line : &str) -> String {
    let mut interface = " ".repeat(indent_size);
    interface += line;
    interface
}

pub struct Inject {
    lines: Vec<(usize, String)>,
}

impl Inject {
    pub fn new(lines: Vec<(usize, String)>) -> Self {
        Self { lines }
    }

    /// Returns the original content plus the newly injected conted within with updated indexes
    pub fn inject_lines(&self, injection_index : usize, lines : &[String], consider_indentation : bool) -> Vec<(usize, String)> {
        let original_lines = self.lines.clone();
        let mut new_lines: Vec<String> = Vec::new();
        if injection_index == 0 { // in case the client wants to inject at the top of the file
            lines.iter().for_each(|string| new_lines.push(string.to_owned()));
            original_lines.iter().for_each(|(_, string)| new_lines.push(string.to_owned()));
        } else if injection_index >= original_lines.len() { // in case the client wants to inject at the bottom of the file
            original_lines.iter().for_each(|(_, string)| new_lines.push(string.to_owned()));
            let last_line_indent_size = tab_size(&original_lines[original_lines.len() - 1].1);
            if consider_indentation {
                lines.iter().for_each(|string| new_lines.push(indent_line(last_line_indent_size, string)));
            } else {
                lines.iter().for_each(|string| new_lines.push(string.to_owned()));
            }
        } else {
            for (index, line) in original_lines.iter() {
                if *index + 1 == injection_index { // adding +1 for injection that makes more sense. EX. when specifying 1 as injection index, it SHOULD inject in the #1 indexed line
                    if consider_indentation {
                        let indentation_size = tab_size(&line);
                        new_lines.push(indent_line(indentation_size, &line));
                        continue;
                    }
                    new_lines.push(line.to_owned());
                    continue;
                }
                new_lines.push(line.to_owned());
            }
        }
        let enumerated_lines: Vec<(usize, String)> = new_lines
            .iter()
            .enumerate()
            .map(|(index, string)| (index + 1, string.to_owned()))
            .collect();
        enumerated_lines
    }
}

impl Constructor for Inject {}
