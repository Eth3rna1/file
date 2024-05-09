/*
    ENUMERATE command functionality
*/
use crate::tool::{self, Constructor};
use std::fs;
use std::io::{Read, Result};
use std::path::PathBuf;

pub struct Enumerate {
    lines : Vec<String>
}

impl Enumerate {
    pub fn new(lines : Vec<String>) -> Self {
        Self {
            lines
        }
    }

    /// Returns an enumerated array of strings
    pub fn enumerate_lines(&self) -> Vec<(usize, String)> {
        let lines = self.lines.clone();
        let mut enumerated_lines: Vec<(usize, String)> = Vec::new();
        for (index, line) in lines.iter().enumerate() {
            enumerated_lines.push((index + 1, line.to_owned()));
        }
        enumerated_lines
    }
}

impl Constructor for Enumerate {}
