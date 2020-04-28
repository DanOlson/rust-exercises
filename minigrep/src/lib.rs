use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results= if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in haystack.lines() {
        if line.contains(needle) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    let query = needle.to_lowercase();
    let mut results = Vec::new();

    for line in haystack.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
Safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["Safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn no_results() {
        let query = "foo";
        let contents = "\
When I wake up,
well I know I'm gonna be,";

        assert_eq!(vec![] as Vec<&str>, search(query, contents));
    }

    #[test]
    fn multiple_results() {
        let query = "wake";
        let contents = "\
When I wake up,
well I know I'm gonna be
I'm gonna be the man who wakes up next to you.";
        let expected = vec![
            "When I wake up,",
            "I'm gonna be the man who wakes up next to you."
        ];

        assert_eq!(expected, search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let expected = vec![
            "Rust:",
            "Trust me."
        ];
        assert_eq!(expected, search_case_insensitive(query, contents))
    }
}
