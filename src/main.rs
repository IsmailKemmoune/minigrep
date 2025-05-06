use std::env;
use minigrep::{read_file, Config};

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


