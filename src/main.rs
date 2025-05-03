use std::env;
use std::fs;
use std::io;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::new(&args);

    match config {
        Ok(config) => {
            println!("query: {}, filename: {}", config.query, config.filename);
            let contents = read_file(&config.filename);

            match contents {
                Ok(contents) => println!("file content returned:\n{contents}"),
                Err(err) => println!("Error while reading {}: {}", &config.filename, err),
            }
        }
        Err(err) => println!("Error while parsing arguments: {}", err),
    }
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
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

fn read_file(file_name: &str) -> io::Result<String> {
    let contents = fs::read_to_string(file_name)?;

    Ok(contents)
}
