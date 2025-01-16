use std::io::{self, Write};
use log::info;

use grrs::Config;

fn main() {
    env_logger::init();
    info!("Loading the application.");

    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout.lock()); // optional: wrap that handle in a buffer

    let mut cfg = Config::new().unwrap();
    let format = false; // Can remove if each line is not too long for the bugger

    match cfg.search_string(format) {
        Ok(result) => writeln!(handle, "{}", result).unwrap(), // write line to stdout instead of using println! macro, less expensive operation,
        Err(_) => writeln!(handle, "Error!").unwrap(),
    };

    info!("Closing the application.");
}
