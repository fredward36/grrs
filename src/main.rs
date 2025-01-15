use clap::Parser;
use std::io::{BufRead, BufReader, self, Write};
use std::fs::File;
use log::info;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    env_logger::init();
    info!("Loading the application.");

    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout.lock()); // optional: wrap that handle in a buffer

    let args = Cli::parse();
    let file = File::open(&args.path).expect("could not read file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("could not read line");
        if line.contains(&args.pattern) {
            writeln!(handle, "{}", line).unwrap(); // write line to stdout instead of using println! macro, less expensive operation
        }
    }
    info!("Closing the application.");
}
