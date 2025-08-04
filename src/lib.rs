use std::{env, error::Error, fs};

#[derive(Debug)]
pub struct Config<'a> {
    pub pattern: &'a String,
    pub filename: &'a String,
    pub ignore_cases: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Usage: rsgrep <pattern> <filename>");
        }

        Ok(Config {
            pattern: &args[1],
            filename: &args[2],
            ignore_cases: args.contains(&String::from("-i"))
                || env::var("IGNORE_CASE").unwrap_or_default() == true.to_string(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let result = if config.ignore_cases {
        search_case_insensitive(config.pattern, &content)
    } else {
        search_case_sensitive(config.pattern, &content)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_args() {
        let args = vec!["rsgrep".into(), "pattern".into(), "filename.txt".into()];

        let config = Config::build(&args).expect("Failed to parse arguments");
        assert_eq!(config.pattern, "pattern");
        assert_eq!(config.filename, "filename.txt");
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUST";
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
