use crate::parser::JsonValue;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Deserializer {
    pub input: JsonValue
}

impl Deserializer {
    pub fn deserialize(&self) -> String {
        let value: String = self.deserialize_helper(self.input.clone());
        value
    }

    fn deserialize_helper(&self, value: JsonValue) -> String {
        return match value {
            JsonValue::Array(array) => self.deserialize_array(array),
            JsonValue::Object(map) => self.deserialize_object(map),
            JsonValue::String(value) => value,
            _ => "".to_string(),
        }
    }

    fn deserialize_object(&self, map: HashMap<String, JsonValue>) -> String {
        let mut builder = String::new();
        builder.push('{');
        for (k, v) in map.into_iter() {
            builder.push(' ');
            builder.push('"');
            builder.push_str(k.as_str());
            builder.push('"');
            builder.push(':');
            builder.push(' ');
            let deserialized_value: String = self.deserialize_helper(v);
            builder.push('"');
            builder.push_str(deserialized_value.as_str());
            builder.push('"');
            builder.push(',');
            builder.push(' ');
        }
        builder.pop();
        builder.pop();
        builder.push(' ');
        builder.push('}');
        builder
    }

    fn deserialize_array(&self, array: Vec<JsonValue>) -> String {
        let mut builder = String::new();
        builder.push('[');
        for v in array {
            let deserialized_value: String = self.deserialize_helper(v);
            builder.push_str(deserialized_value.as_str());
            builder.push(',');
            builder.push(' ');
        }
        builder.pop();
        builder.pop();
        builder.push(']');
        builder.push('\n');
        builder
    }
    
}


