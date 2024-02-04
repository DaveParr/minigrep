use std::error::Error;
use std::fs;

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

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = std::env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn find_and_format<'a>(query: &str, line: &'a str) -> Option<String> {
    if let Some(start) = line.find(query) {
        let end = start + query.len();
        let formatted_line = format!(
            "{}{}{}{}{}",
            &line[..start],
            "\x1b[1m", // Start bold
            &line[start..end],
            "\x1b[0m", // End bold
            &line[end..],
        );
        Some(formatted_line)
    } else {
        None
    }
}

fn search_lines<F>(contents: &str, f: F) -> Vec<String>
where
    F: Fn(&str) -> Option<String>,
{
    contents.lines().filter_map(f).collect()
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<String> {
    search_lines(contents, |line| find_and_format(query, line))
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<String> {
    let query = query.to_lowercase();
    search_lines(contents, |line| {
        find_and_format(&query, &line.to_lowercase())
    })
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

        assert_eq!(
            vec!["safe, fast, pro\u{1b}[1mduct\u{1b}[0mive."],
            search(query, contents)
        );
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
            vec!["\u{1b}[1mrust\u{1b}[0m:", "t\u{1b}[1mrust\u{1b}[0m me."],
            search_case_insensitive(query, contents)
        );
    }
}
