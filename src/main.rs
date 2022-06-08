use tinyserde::JsonParser;
use tinyserde::JsonValue;
use tinyserde::ParserError;

fn main() {
    let json_input = "{ \"foo\": 123 \n, \"bar\":    456 }".to_string();
    let mut parser = JsonParser {
        input: json_input, 
        cursor: 0,
    };
    let value: Result<JsonValue, ParserError> = parser.parse();
    match value {
        Ok(value) => println!("The parsed value is: {:?}", value),
        Err(_) => panic!("Could not parse JSON."),
    };
}
