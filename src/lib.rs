//! # daveparr_minigrep
//!
//! `daveparr_minigrep` is a simple command line program that searches for a string in a file.
//! It is based on the example in ["The Rust Book"](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).

use std::error::Error;
use std::fs;

/// The run function is the entry point for the program.
/// It takes a Config instance and returns a Result.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

/// The Config struct holds the values of the command line arguments.
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

/// The Config struct has an implementation of the build method.
impl Config {
    /// The build method takes an iterator of strings and returns a Result of Config or a string slice.
    /// # Errors
    /// Returns an error if the query string or file path are not provided.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip the program name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = std::env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

/// The search function takes a query string and a string slice and returns a vector of string slices that match the query.
/// # Example
/// ```
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Duct tape.";
/// assert_eq!(vec!["safe, fast, productive."], daveparr_minigrep::search(query, contents));
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// The search_case_insensitive function takes a query string and a string slice and returns a vector of string slices that match the query, ignoring case.
/// # Example
/// ```
/// let query = "rUsT";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Trust me.";
/// assert_eq!(vec!["Rust:", "Trust me."], daveparr_minigrep::search_case_insensitive(query, contents));
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
