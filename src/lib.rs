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

    println!("\nFile content:\n{}", content);

    Ok(())
}

#[test]
fn test_get_args() {
    let args = vec!["rsgrep".into(), "pattern".into(), "filename.txt".into()];

    let config = Config::build(&args).expect("Failed to parse arguments");
    assert_eq!(config.pattern, "pattern");
    assert_eq!(config.filename, "filename.txt");
}
