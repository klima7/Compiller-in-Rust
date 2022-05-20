use std::{env, io};
use std::fs::File;
use std::io::Read;
use colored::*;

mod parse;
mod lex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        handle_error("File path not passed".to_string());
    }

    let path = args.get(1).unwrap();

    let contents = read_file_contents(path);

    let contents = match contents {
        Ok(contents) => contents,
        Err(e) => handle_error(e),
    };

    println!("{}", contents);
}

fn read_file_contents(filename: &str) -> Result<String, String> {
    let file = File::open(filename);

    let mut file = match file {
        Ok(file) => file,
        Err(_) => return Err(format!("Unable to read file \"{}\"", filename))
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return Ok(contents);
}

fn handle_error(e: String) -> ! {
    let text = format!("Error: {}", e);
    println!("{}", text.red());
    std::process::exit(1);
}
