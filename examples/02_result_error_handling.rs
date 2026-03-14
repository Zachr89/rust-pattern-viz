//! Example: Result Error Handling
//! 
//! Demonstrates Result<T, E> pattern matching for robust error handling.

use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
enum AppError {
    IoError(io::Error),
    ParseError(String),
    NotFound,
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::IoError(err)
    }
}

fn read_config_file(path: &str) -> Result<String, AppError> {
    // Pattern 1: Early return with ? operator
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_number(input: &str) -> Result<i32, AppError> {
    // Pattern 2: Match with error transformation
    match input.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(AppError::ParseError(format!("Invalid number: {}", input))),
    }
}

fn process_data(path: &str) -> Result<i32, AppError> {
    // Pattern 3: Chained Result operations
    let contents = read_config_file(path)?;
    let number = parse_number(contents.trim())?;
    Ok(number * 2)
}

fn handle_result(result: Result<i32, AppError>) {
    // Pattern 4: Exhaustive error matching
    match result {
        Ok(value) => println!("Success: {}", value),
        Err(AppError::IoError(e)) => eprintln!("IO Error: {}", e),
        Err(AppError::ParseError(msg)) => eprintln!("Parse Error: {}", msg),
        Err(AppError::NotFound) => eprintln!("Resource not found"),
    }
}

fn main() {
    handle_result(process_data("config.txt"));
    handle_result(parse_number("42"));
    handle_result(parse_number("invalid"));
}
