use std::io::{self, Write};
use log::info;

use grrs::{Config, search_string};

fn main() {
    env_logger::init();
    info!("Loading the application.");

    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout.lock()); // optional: wrap that handle in a buffer

    let cfg = Config::new().unwrap();

    match search_string(cfg.reader, &cfg.args.pattern) {
        Ok(result) => writeln!(handle, "{}", result).unwrap(), // write line to stdout instead of using println! macro, less expensive operation,
        Err(_) => writeln!(handle, "Error!").unwrap(),
    };

    info!("Closing the application.");
}
