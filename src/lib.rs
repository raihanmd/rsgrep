use std::{error::Error, fs};

pub struct Config<'a> {
    pub pattern: &'a String,
    pub filename: &'a String,
}

impl<'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Usage: rsgrep <pattern> <filename>");
        }
        Ok(Config {
            pattern: &args[1],
            filename: &args[2],
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    for line in search(&config.pattern, &content) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
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
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, &contents));
    }
}
