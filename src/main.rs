use std::{env, process};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = rsgrep::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(e) = rsgrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
