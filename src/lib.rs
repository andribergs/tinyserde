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
    String(&'static str),
    Array(Box<[JsonValue]>),
    Object(HashMap<String, JsonValue>)
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    ConsumeInputNotFinished(usize),
    ParseHelperFailed(String),
    ParseError(String),
    InvalidJson(String),
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

impl JsonParser {
    pub fn parse(&mut self) -> Result<JsonValue, ParserError> {
        let value = self.parse_helper();
        if !self.eof() {
            return Err(ParserError::ConsumeInputNotFinished(self.cursor.clone()))
        }
        value
    }

    fn peek(&self) -> char {
        if self.eof() {
            return '0';
        }
        self.input.chars().nth(self.cursor).unwrap()
    }

    fn eof(&self) -> bool {
        return self.cursor >= self.input.chars().count();
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
            return Err(ParserError::ParseError("Expected '\"' ".to_string()));
        }
        let mut builder = String::new();
        while self.peek() != '"' {
            builder.push(self.peek());
            self.cursor += 1;
        }
        self.cursor += 1;
        Ok(builder)
    }

    fn parse_helper(&mut self) -> Result<JsonValue, ParserError> {
        self.skip_whitespace();
        let result = match self.peek() {
            // FIXME: We should be able to improve the way parse_number is done.
            '{' => self.parse_object(),
            '-' => self.parse_number(),
            '0' => self.parse_number(),
            '1' => self.parse_number(),
            '2' => self.parse_number(),
            '3' => self.parse_number(),
            '4' => self.parse_number(),
            '5' => self.parse_number(),
            '6' => self.parse_number(),
            '7' => self.parse_number(),
            '8' => self.parse_number(),
            '9' => self.parse_number(),
            _ => Err(ParserError::ParseHelperFailed("ParseHelper failed.".to_string())),
        };
        result
    }

    fn parse_object(&mut self) -> Result<JsonValue, ParserError> {
        if !self.consume_specific('{') {
            return Err(ParserError::ParseError("Expected '{'".to_string()));
        }
        let mut values: HashMap<String, JsonValue> = HashMap::new();
        loop {
            self.skip_whitespace();
            if self.peek() == '}' {
                return Err(ParserError::InvalidJson("Invalid JSON.".to_string()));
            }
            self.skip_whitespace();
            let name = self.consume_and_unescape_string().unwrap();
            self.skip_whitespace();
            if !self.consume_specific(':') {
                return Err(ParserError::ParseError("Expected ':'".to_string()));
            }
            self.skip_whitespace();
            let value = self.parse_helper().unwrap();
            values.insert(name, value);
            self.skip_whitespace();
            if self.peek() == '}' {
                break;
            }
            if !self.consume_specific(',') {
                return Err(ParserError::ParseError("Expected ','".to_string()));
            }
            self.skip_whitespace();
            if self.peek() == '}' {
                return Err(ParserError::InvalidJson("Invalid JSON.".to_string()));
            }
        }
        if !self.consume_specific('}') {
            return Err(ParserError::ParseError("Expected '}'".to_string()));
        }
        Ok(JsonValue::Object(values))
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
fn test_parse_json() {
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