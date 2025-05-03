use std::env;
use std::fs;
use std::io::Result;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let query = &args[1];
    let filename = &args[2];

    println!("query: {}, filename: {}", query, filename);

    let contents = read_file(filename);

    match contents {
        Ok(contents) => println!("file content returned:\n{contents}"),
        Err(err) => println!("Error while reading {}: {}", filename, err),
    }
}

fn read_file(file_name: &str) -> Result<String> {
    let contents = fs::read_to_string(file_name)?;

    Ok(contents)
}
