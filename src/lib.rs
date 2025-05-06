use std::fs;
use std::io;


pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments, please provide a query and a filename");
        }

        let config = Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        };

        Ok(config)
    }
}

pub fn read_file(file_name: &str) -> io::Result<String> {
    let contents = fs::read_to_string(file_name)?;

    Ok(contents)
}