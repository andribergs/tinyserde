use std::collections::HashMap;

#[derive(Debug)]
pub struct JsonParser {
    pub input: String,
    pub cursor: usize
}

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(i64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>)
}

impl Clone for JsonValue {
    fn clone(&self) -> Self {
        match &self {
            JsonValue::Null => JsonValue::Null,
            JsonValue::Bool(val) => JsonValue::Bool(val.clone()),
            JsonValue::Number(val) => JsonValue::Number(val.clone()),
            JsonValue::String(val) => JsonValue::String(val.clone()),
            JsonValue::Array(array) => JsonValue::Array(array.clone()),
            JsonValue::Object(map) => JsonValue::Object(map.clone()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    ParseError(&'static str, usize),
}

#[derive(Debug, PartialEq)]
enum ParseType {
    Null,
    Boolean,
    Number,
    String,
    Array,
    Object,
    Unknown,
}

fn is_whitespace(c: char) -> bool {
    return match c {
        '\t' => true,
        '\n' => true,
        '\r' => true,
        ' ' => true,
        _ => false,
    }
}

fn is_string_terminating_symbol(c: char) -> bool {
    return match c {
        '}' => true,
        ']' => true,
        ':' => true,
        ',' => true,
        _ => false,
    }
}

fn is_numeric_char(c: char) -> bool { 
    return match c {
        '-' => true,
        '0' => true,
        '1' => true,
        '2' => true,
        '3' => true,
        '4' => true,
        '5' => true,
        '6' => true,
        '7' => true,
        '8' => true,
        '9' => true,
        _ => false,
    }
}

fn determine_parse_type(c: char) -> ParseType {
    if c == '{' {
        ParseType::Object
    } 
    else if c =='[' {
        ParseType::Array
    }
    else if is_numeric_char(c) {
        ParseType::Number
        
    } else if c == '"' {
        ParseType::String
    } else if c == 't' || c == 'f' {
        ParseType::Boolean
    } else if c == 'n' {
        ParseType::Null
    } else {
        ParseType::Unknown
    }
}

impl JsonParser {
    pub fn parse(&mut self) -> Result<JsonValue, ParserError> {
        let value = self.parse_helper();
        self.skip_whitespace();
        if !self.eof() {
            return Err(ParserError::ParseError("Could not parse JSON.", self.cursor.clone()))
        }
        value
    }

    fn eof(&self) -> bool {
        return self.cursor >= self.input.chars().count();
    }

    fn peek(&self) -> char {
        if self.eof() {
            return '|';
        }
        // FIXME: This feels like an inefficient way to do this,
        // i.e. we always have to do a linear scan up to the nth 
        // character at self.cursor whenever we call peek().
        self.input.chars().nth(self.cursor).unwrap()
    }

    fn peek_after_white_space(&mut self) -> char {
        if self.eof() {
            return '|';
        }
        let mut temp_cursor = self.cursor.clone();
        temp_cursor += 1;
        while !self.eof() {
            if !is_whitespace(self.input.chars().nth(temp_cursor).unwrap()) {
                break;
            }
            temp_cursor += 1;
        }
        self.input.chars().nth(temp_cursor).unwrap()
    }

    fn skip_whitespace(&mut self) {
        while !self.eof() {
            if !is_whitespace(self.input.chars().nth(self.cursor).unwrap()) {
                break;
            }
            self.cursor += 1;
        }
    }

    fn consume_specific(&mut self, expected: char) -> bool {
        if self.peek() != expected {
            return false;
        }
        self.cursor += 1;
        true
    }

    fn consume_and_unescape_string(&mut self) -> Result<String, ParserError> {
        if !self.consume_specific('"') {
            return Err(ParserError::ParseError("Expected '\"' ", self.cursor));
        }
        let mut builder = String::new();
        loop {
            match self.peek() {
                '"' => {
                    if !is_string_terminating_symbol(self.peek_after_white_space()){
                        builder.push('"');
                        self.consume_specific('"');
                    } else {
                        self.consume_specific('"');
                        return Ok(builder);
                    }
                },
                '|' => {
                    return Ok(builder);
                },
                ch => {
                    builder.push(ch);
                    self.cursor += 1;
                }
            }
        }
    }

    fn parse_helper(&mut self) -> Result<JsonValue, ParserError> {
        self.skip_whitespace();
        let type_to_parse: ParseType = determine_parse_type(self.peek());
        return match type_to_parse {
            ParseType::Object => self.parse_object(),
            ParseType::Number => self.parse_number(),
            ParseType::String => self.parse_string(),
            ParseType::Boolean => self.parse_bool(),
            ParseType::Null => self.parse_null(),
            ParseType::Array => self.parse_array(),
            _ => Err(ParserError::ParseError("parse_helper failed", self.cursor)),
        };
    }

    fn parse_object(&mut self) -> Result<JsonValue, ParserError> {
        if !self.consume_specific('{') {
            return Err(ParserError::ParseError("Expected '{'", self.cursor));
        }
        let mut values: HashMap<String, JsonValue> = HashMap::new();
        loop {
            self.skip_whitespace();
            if self.peek() == '}' {
                return Err(ParserError::ParseError("'}' in an invalid position", self.cursor));
            }
            self.skip_whitespace();

            // Get the property key.
            let key = self.consume_and_unescape_string().unwrap();

            self.skip_whitespace();
            if !self.consume_specific(':') {
                return Err(ParserError::ParseError("Expected ':'", self.cursor));
            }
            self.skip_whitespace();

            // Get the property value.
            let value = self.parse_helper().unwrap();
            values.insert(key, value);

            self.skip_whitespace();
            if self.peek() == '}' {
                break;
            }
            if !self.consume_specific(',') {
                return Err(ParserError::ParseError("Expected ','", self.cursor));
            }
            self.skip_whitespace();
            if self.peek() == '}' {
                return Err(ParserError::ParseError("'}' in an invalid position", self.cursor));
            }
        }
        if !self.consume_specific('}') {
            return Err(ParserError::ParseError("Expected '}'", self.cursor));
        }
        Ok(JsonValue::Object(values))
    }

    fn parse_array(&mut self) -> Result<JsonValue, ParserError> {
        if !self.consume_specific('[') {
            return Err(ParserError::ParseError("Expected '['", self.cursor));
        }
        let mut array = vec![];
        while self.peek() != ']' {
            self.skip_whitespace();
            let element = self.parse_helper().unwrap();
            array.push(element);
            self.skip_whitespace();
            if !self.consume_specific(',') && !(self.peek() == ']') {
                return Err(ParserError::ParseError("Expected ',' or ']'", self.cursor));
            }
        }
        if !self.consume_specific(']') {
            return Err(ParserError::ParseError("Expected ']'", self.cursor));
        }
        Ok(JsonValue::Array(array))
    }

    fn parse_bool(&mut self) -> Result<JsonValue, ParserError> {
        let value: bool;
        if &self.input[self.cursor..self.cursor+4] == "true" {
            value = true;
            self.cursor += 4;
        } else if &self.input[self.cursor..self.cursor+5] == "false" {
            value = false;
            self.cursor += 5;
        } else {
            return Err(ParserError::ParseError("Expected either true or false", self.cursor));
        }
        Ok(JsonValue::Bool(value))
    }

    fn parse_null(&mut self) -> Result<JsonValue, ParserError> {
        if &self.input[self.cursor..self.cursor+4] == "null" {
            self.cursor += 4;
        } else {
            return Err(ParserError::ParseError("Expected null", self.cursor));
        }
        Ok(JsonValue::Null)
    }

    fn parse_string(&mut self) -> Result<JsonValue, ParserError> {
        let value = self.consume_and_unescape_string().unwrap();
        Ok(JsonValue::String(value))
    }

    fn parse_number(&mut self) -> Result<JsonValue, ParserError> {
        let mut value: i64 = 0;
        while !self.eof() {
            let ch = self.peek();
            if !(ch as u8 > b'0' && ch as u8 <= b'9') {
                break;
            }
            value *= 10;
            value += (ch as u8 - b'0') as i64;
            self.cursor += 1;
        }
        Ok(JsonValue::Number(value))
    }
}

#[test]
fn test_parse_json_obj_with_number() {
    let json_input = "{ \"foo\": 123 \n, \"bar\":    456 }".to_string();
    let mut parser = JsonParser {
        input: json_input, 
        cursor: 0,
    };
    let expected_value = JsonValue::Object(HashMap::from([("foo".to_string(), JsonValue::Number(123)), ("bar".to_string(), JsonValue::Number(456))]));
    match parser.parse() {
        Ok(value) => assert_eq!(value, expected_value),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parse_json_obj_with_string() {
    let json_input = "{ \"foo\": \"abcde\" }".to_string();
    let mut parser = JsonParser {
        input: json_input, 
        cursor: 0,
    };
    let expected_value = JsonValue::Object(HashMap::from([("foo".to_string(), JsonValue::String("abcde".to_string()))]));
    match parser.parse() {
        Ok(value) => assert_eq!(value, expected_value),
        Err(_) => assert!(false),
    };
}

#[test]
fn test_parse_json_obj_with_bool() {
    let json_input = "{ \"foo\": false }".to_string();
    let mut parser = JsonParser {
        input: json_input, 
        cursor: 0,
    };
    let expected_value = JsonValue::Object(HashMap::from([("foo".to_string(), JsonValue::Bool(false))]));
    match parser.parse() {
        Ok(value) => assert_eq!(value, expected_value),
        Err(_) => assert!(false),
    };
}

#[test]
fn test_parse_json_obj_with_null() {
    let json_input = "{ \"foo\": null }".to_string();
    let mut parser = JsonParser {
        input: json_input, 
        cursor: 0,
    };
    let expected_value = JsonValue::Object(HashMap::from([("foo".to_string(), JsonValue::Null)]));
    match parser.parse() {
        Ok(value) => assert_eq!(value, expected_value),
        Err(_) => assert!(false),
    };
}

#[test]
fn test_parse_json_obj_with_array() {
    let json_input = "
[
  	{
		\"foo\": null
	},
	{
		\"bar\": 123
	},
	{
		\"baz\": \"abcde\"
	},
	345,
	\"efgh\",
	null,
	false
]
".to_string();
    let mut parser = JsonParser {
        input: json_input, 
        cursor: 0,
    };
    let expected_value = JsonValue::Array(vec![
        JsonValue::Object(HashMap::from([("foo".to_string(), JsonValue::Null)])),
        JsonValue::Object(HashMap::from([("bar".to_string(), JsonValue::Number(123))])),
        JsonValue::Object(HashMap::from([("baz".to_string(), JsonValue::String("abcde".to_string()))])),
        JsonValue::Number(345),
        JsonValue::String("efgh".to_string()),
        JsonValue::Null,
        JsonValue::Bool(false),
    ]);
    match parser.parse() {
        Ok(value) => assert_eq!(value, expected_value),
        Err(_) => assert!(false),
    };
}

#[test]
fn test_parse_json_obj_with_nested_string() {
    let json_input = "{ \"foo\": \"abcde and a nested string \"nested\" right in the middle of the sentence.\" }".to_string();
    let mut parser = JsonParser {
        input: json_input, 
        cursor: 0,
    };
    let expected_value = JsonValue::Object(HashMap::from([("foo".to_string(), JsonValue::String("abcde and a nested string \"nested\" right in the middle of the sentence.".to_string()))]));
    match parser.parse() {
        Ok(value) => assert_eq!(value, expected_value),
        Err(_) => assert!(false),
    };
}
