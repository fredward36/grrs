use clap::Parser;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::fs::{File, OpenOptions};
use std::collections::HashSet;

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

    pub fn search_string(&mut self, format: bool) -> Result<String, &'static str> {
        if format {
            match self.format_by_sentence() {
                Ok(_) => (),
                Err(_) => return Err("Could not format file!"),
            };
        }
        for line in self.reader.by_ref().lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => return Err("Could not read line"),
            };
            if line.contains(&self.args.pattern) {
                return Ok(line);
            }
        }
        Err("Pattern not found!")
    }

    // In the file, seperate each sentence onto a new line for better readability
    pub fn format_by_sentence(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(&self.args.path)?;
        let reader = BufReader::new(file);
        let mut content = String::new();
        let delimiters: HashSet<char> = HashSet::from(['.', '?', '!']);
        for line in reader.lines() {
            let line = line?;
            for character in line.chars() {
                content.push(character);
                if delimiters.contains(&character) {
                    content.push('\n');
                }
            }
            content.push('\n'); // Add a new line at the end of each line
        }
        // Reopen the file in write mode to overwrite it with the formatted content
        let mut writer = BufWriter::new(OpenOptions::new().write(true).truncate(true).open(&self.args.path)?);
        writer.write_all(content.as_bytes())?;
        writer.flush()?;

        Ok(())
    }
}
