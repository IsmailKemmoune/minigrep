use minigrep::{run, Config};
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::new(&args);

    match config {
        Ok(config) => {
            println!("query: {}, filename: {}", config.query, config.filename);

            match  run(config) {
                Ok(()) => {},
                Err(err) => println!("Error while reading the file: {}", err),
            }
        }
        Err(err) => println!("Error while parsing arguments: {}", err),
    }
}
