use std::{env, fs};
use tinyserde::parser::JsonParser;
use tinyserde::parser::JsonValue;
use tinyserde::parser::ParserError;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let paths = fs::read_dir(path).unwrap();
    let mut parsed_objects: Vec<JsonValue> = vec![];
    for path in paths {
        let contents = fs::read_to_string(path.unwrap().path()).unwrap_or("".to_string());
        let mut parser = JsonParser {
            input: contents, 
            cursor: 0,
        };
        let value: Result<JsonValue, ParserError> = parser.parse();
        match value {
            Ok(value) => parsed_objects.push(value),
            Err(_) => panic!("Could not parse JSON: \n {}", parser.input),
        };
    }
    println!("{:?}", parsed_objects);
}
