use clap::Parser;
use std::io::{BufRead, BufReader,};
use std::fs::File;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
pub struct Cli {
    /// The pattern to look for
    pub pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

pub struct Config {
    pub args: Cli,
    pub reader: BufReader<File>,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let args = Cli::parse();
        let file = File::open(&args.path)?;
        let reader = BufReader::new(file.try_clone()?);
        Ok(Config { args, reader,})
    }
}

pub fn search_string(reader: BufReader<File>, pattern: &String) -> Result<String, &'static str> {
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => return Err("Could not read line"),
        };
        if line.contains(pattern) {
            return Ok(line);
        }
    }
    Err("Pattern not found!")
}