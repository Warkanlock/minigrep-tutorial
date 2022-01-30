use std::{error::Error, fs};

const BASE_PATH: &str = "content/";

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("Not enough parameters");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{}", config.filename);
    let filepath = format!("{}{}", BASE_PATH, config.filename);
    let content = fs::read_to_string(filepath)?;

    println!("With text:\n{}", content);

    // for each line found in the text this will print
    for line in search(&config.query, &content) {
        println!("Found -> {}", line)
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "safe";
        let contents = "\
    Rust:\nis a safe fast productive tool\n";

        assert_eq!(
            vec!["is a safe fast productive tool"],
            search(query, contents)
        )
    }
}
