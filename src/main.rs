use std::{env, fs};
use std::panic::set_hook;
use tinyserde::parser::JsonParser;
use tinyserde::parser::JsonValue;
use tinyserde::parser::ParserError;
use tinyserde::deserializer::Deserializer;

const RESULT_FILE: &str = "data/combined.json";
const MISSING_PATH_MESSAGE: &str = r#"Missing path parameter"#;
const INCORRECT_USAGE_MESSAGE: &str = r#"Incorrect usage of tinyserde, please see tinyserde --help for more details"#;
const HELP_MESSAGE: &str = r#"
Tinyserde is a simple program that parses json files that reside in the same directory and combines them into a single file.

The resulting output will be written to the file "data/combined.json".

USAGE:
    tinyserde [OPTIONS] --path <PATH>

OPTIONS:
    -h, --help             Print help information.
    -p, --path <PATH>      Directory containing json files to combine.
"#;

fn combine_json(path: &str) -> () {
    let files = fs::read_dir(path).expect("Could not find directory.");
    let mut parsed_objects: Vec<JsonValue> = vec![];
    for file in files {
        let contents = fs::read_to_string(file.unwrap().path()).expect("Could not read file.");
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
    let deserializer: Deserializer = Deserializer {
        input: JsonValue::Array(parsed_objects)
    };
    let json_as_string = deserializer.deserialize();
    fs::write(RESULT_FILE, json_as_string).expect("Unable to write to file.");
}

fn main() {
    set_hook(Box::new(|info| {
        if let Some(s) = info.payload().downcast_ref::<String>() {
            println!("{}", s);
        }
    }));
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("{}", INCORRECT_USAGE_MESSAGE);
        return
    }
    let flag = &args[1];
    if flag == "-h" || flag == "--help" {
        println!("{}", HELP_MESSAGE);
    } else if flag == "-p" || flag == "--path" {
        if args.len() < 3 {
            println!("{}", MISSING_PATH_MESSAGE);
            return
        }
        let path = &args[2];
        combine_json(path);
    } else {
        println!("{}", INCORRECT_USAGE_MESSAGE);
    }
}
