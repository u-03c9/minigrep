//! # minigrep
//!
//! `minigrep` is a collection of utilities to make performing searches
//! on an input file
use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

/// Starts the search given a Config struct containing the pattern
/// and the file name, and then prints out the results.
///
/// ## Panics
/// It could panic if the file doesn't exist or unable to read.
/// ```
/// let config = minigrep::Config{
///     query: "to".to_string(),
///     filename: "file_that_does_not_exists.txt".to_string(),
///     case_sensitive: false,
/// };
///
/// assert!(minigrep::run(config).is_err());
///
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

/// Searches for the query in contents with case sensitivity.
///
/// ## Example
/// ```
/// let query = "to";
/// let contents = "\
///     To here\n\
///     but not there.\n\
///     here to there.";
///
/// assert_eq!(vec!["here to there."], minigrep::search(query, &contents));
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Searches for the query in contents with case insensitivity.
///
/// ## Example
/// ```
/// let query = "tO";
/// let contents = "\
///     To here\n\
///     but not there.\n\
///     here to there.";
///
/// assert_eq!(vec!["To here", "here to there."],
///     minigrep::search_case_insensitive(query, &contents));
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
            Rust:\n\
            safe, fast, productive.\n\
            Pick three.\n\
            Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
            Rust:\n\
            safe, fast, productive.\n\
            Pick three.\n\
            Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
