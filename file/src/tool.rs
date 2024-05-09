/*
    Module containing irrelevant but necessary functions for the API's
*/
use std::io::{self, Write};

/// Returns the length of character of a number given
pub(crate) fn len(number: &usize) -> usize {
    number.to_string().len()
}

/// Trait used for prettifying the result after lines have been processed
pub(crate) trait Constructor {
    fn construct(&self, result: &[(usize, String)]) -> String {
        let max_char_amount = len(&result[result.len() - 1].0) + 1; // 1 to make up for the `:` char/separator
        let mut lines: Vec<String> = Vec::new();
        for (index, line) in result {
            let space = " ".repeat(max_char_amount - len(index));
            lines.push(format!("{}:{}{}", index, space, line));
        }
        lines.join("\n")
    }
}

/// Output a prompt before obtaining client input
pub(crate) fn input(prompt: &str) -> String {
    let mut handle = io::stdout().lock();
    write!(handle, "{}", prompt).unwrap();
    let _ = handle.flush();
    let mut response = String::new();
    let _ = io::stdin().read_line(&mut response);
    response = response.replace(['\n', '\r'], "");
    response
}

/// Measure the indent size of string. Tab = 4 units | Space = 1 unit
pub(crate) fn measure_indent_size(line: &str) -> usize {
    let mut space = 0;
    let line = line.replace('\t', &" ".repeat(4));
    for character in line.chars() {
        if line == " " {
            space += 1;
        }
    }
    space
}
