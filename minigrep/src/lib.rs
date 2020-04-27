use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
Safe, fast, productive.
Pick three.";

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
}
