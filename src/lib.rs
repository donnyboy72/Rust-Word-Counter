use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let count = count_words(&contents);
    println!("Total word count: {}", count);
    Ok(())
}

fn count_words(file: &str) -> usize {
    let words: Vec<&str> = file.split_whitespace().collect();
    words.len()
}

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(args) => args,
            None => return Err("Didn't get a file"),
        };

        Ok(Config { filename })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
