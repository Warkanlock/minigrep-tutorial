use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error trying to execute the program: {}", err);
        process::exit(1);
    });

    println!("Search query: {}", config.query);
    println!("In content: {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
