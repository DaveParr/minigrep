use std::env;
use std::process;

use minigrep::Config;

/// The main function is the entry point for the program.
/// Rust doesn't allow for testing the main function, so we
/// need to extract the functionality into a module we can test.

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
