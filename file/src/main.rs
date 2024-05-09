/*
    CLI tool to mutate files
*/
mod append;
mod enumerate;
mod erase;
mod find;
mod indent;
mod inject;
mod replace;
mod tool;
use append::Append;
use clap::Parser;
use enumerate::Enumerate;
use erase::Erase;
use find::Find;
use indent::Indent;
use inject::Inject;
use replace::Replace;
use std::fs;
use std::io::Read;
use std::io::Result;
use std::process::exit;
use tool::Constructor;

const AVAILABLE_COMMANDS: &[&str; 7] = &[
    "replace",
    "inject",
    "enumerate",
    "find",
    "indent",
    "append",
    "erase",
];

const SAMPLE: &str = "src/test.txt";

fn read_file_lines(_file: &str) -> Result<Vec<String>> {
    let mut file = fs::File::open(_file)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let content: Vec<String> = content
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|string| string.to_string())
        .collect();
    Ok(content)
}

fn main() {
    let lines = read_file_lines(SAMPLE).unwrap();
    let context = Enumerate::new(lines);
    let result = context.enumerate_lines();
    let interface = context.construct(&result);
    println!("{}", interface);
}
