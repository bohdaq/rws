use std::io;
use std::io::{BufRead, Read};
use crate::json::key_value::parse_json_property;
use crate::symbol::SYMBOL;

#[cfg(test)]
mod tests;
mod key_value;

// TODO: wip

pub struct JSON;

impl  JSON {
    pub fn parse_as_properties(json_string: String) -> Result<Vec<(JSONProperty, JSONValue)>, String> {
        let mut properties = vec![];

        let data = json_string.as_bytes();
        let mut cursor = io::Cursor::new(data);
        let mut bytes_read : i128 = 0;
        let total_bytes : i128 = data.len() as i128;

        // read obj start '{'
        let mut buf = vec![];
        let mut boxed_read = cursor.read_until(b'{', &mut buf);
        if boxed_read.is_err() {
            let message = boxed_read.err().unwrap().to_string();
            return Err(message);
        }
        bytes_read = bytes_read + boxed_read.unwrap() as i128;

        let mut b : &[u8] = &buf;

        let mut boxed_line = String::from_utf8(Vec::from(b));
        if boxed_line.is_err() {
            let error_message = boxed_line.err().unwrap().to_string();
            return Err(error_message);
        }
        let mut line = boxed_line.unwrap();


        let mut is_there_a_key_value = true;
        while is_there_a_key_value {
            // read until key starts '"', save to buffer
            // it will work for first and consecutive key value pair
            let mut key_value_pair : String = "".to_string();


            buf = vec![];
            boxed_read = cursor.read_until(b'\"', &mut buf);
            if boxed_read.is_err() {
                let message = boxed_read.err().unwrap().to_string();
                return Err(message);
            }
            bytes_read = bytes_read + boxed_read.unwrap() as i128;
            b  = &buf;

            boxed_line = String::from_utf8(Vec::from(b));
            if boxed_line.is_err() {
                let error_message = boxed_line.err().unwrap().to_string();
                return Err(error_message);
            }

            line = boxed_line.unwrap();
            key_value_pair = [key_value_pair, line].join(SYMBOL.empty_string);



            // read until key ends '"', append to buffer
            buf = vec![];
            boxed_read = cursor.read_until(b'\"', &mut buf);
            if boxed_read.is_err() {
                let message = boxed_read.err().unwrap().to_string();
                return Err(message);
            }
            bytes_read = bytes_read + boxed_read.unwrap() as i128;
            b = buf.as_slice();

            boxed_line = String::from_utf8(Vec::from(b));
            if boxed_line.is_err() {
                let error_message = boxed_line.err().unwrap().to_string();
                return Err(error_message);
            }
            line = boxed_line.unwrap();
            key_value_pair = [key_value_pair, line].join(SYMBOL.empty_string);


            // read until delimiter ':', append to buffer
            buf = vec![];
            boxed_read = cursor.read_until(b':', &mut buf);
            if boxed_read.is_err() {
                let message = boxed_read.err().unwrap().to_string();
                return Err(message);
            }
            bytes_read = bytes_read + boxed_read.unwrap() as i128;
            b = buf.as_slice();

            boxed_line = String::from_utf8(Vec::from(b));
            if boxed_line.is_err() {
                let error_message = boxed_line.err().unwrap().to_string();
                return Err(error_message);
            }
            line = boxed_line.unwrap();
            key_value_pair = [key_value_pair, line].join(SYMBOL.empty_string);

            // read in a while loop until char is not ascii control char and not whitespace, append to buffer
            let mut is_whitespace = true;

            while is_whitespace {
                let bytes_to_read = 1;
                let mut char_buffer = vec![bytes_to_read];

                cursor.read_exact(&mut char_buffer).unwrap();
                bytes_read = bytes_read + bytes_to_read as i128;
                let char = String::from_utf8(char_buffer).unwrap();

                if char != " " {
                    let is_string = char == "\"";
                    if is_string {
                        key_value_pair = [key_value_pair, char.to_string()].join(SYMBOL.empty_string);

                        // read till non escaped '"'
                        let mut not_end_of_string_property_value = true;
                        while not_end_of_string_property_value {

                            char_buffer = vec![bytes_to_read];
                            cursor.read_exact(&mut char_buffer).unwrap();
                            bytes_read = bytes_read + bytes_to_read as i128;
                            let _char = String::from_utf8(char_buffer).unwrap();
                            let last_char_in_buffer = key_value_pair.chars().last().unwrap().to_string();
                            not_end_of_string_property_value = _char != "\"" && last_char_in_buffer != "\\";
                            key_value_pair = [key_value_pair, _char].join(SYMBOL.empty_string);
                        }
                    }

                    let is_null = char == "n";
                    if is_null {
                        // read 'ull'
                        key_value_pair = [key_value_pair, char.to_string()].join(SYMBOL.empty_string);
                        let byte = 0;
                        let mut char_buffer = vec![byte, byte, byte];
                        let length = char_buffer.len();
                        cursor.read_exact(&mut char_buffer).unwrap();
                        bytes_read = bytes_read + length as i128;
                        let remaining_bool = String::from_utf8(char_buffer).unwrap();
                        if remaining_bool != "ull" {
                            let message = format!("Unable to parse null: {}", key_value_pair);
                            return Err(message)
                        }
                        key_value_pair = [key_value_pair, remaining_bool].join(SYMBOL.empty_string);
                    }

                    let is_boolean_true = char == "t";
                    if is_boolean_true {
                        // read 'rue'
                        key_value_pair = [key_value_pair, char.to_string()].join(SYMBOL.empty_string);
                        let byte = 0;
                        let mut char_buffer = vec![byte, byte, byte];
                        let length = char_buffer.len();
                        cursor.read_exact(&mut char_buffer).unwrap();
                        bytes_read = bytes_read + length as i128;
                        let remaining_bool = String::from_utf8(char_buffer).unwrap();
                        if remaining_bool != "rue" {
                            let message = format!("Unable to parse boolean: {}", key_value_pair);
                            return Err(message)
                        }
                        key_value_pair = [key_value_pair, remaining_bool].join(SYMBOL.empty_string);
                    }

                    let is_boolean_false = char == "f";
                    if is_boolean_false {
                        // read 'alse'
                        key_value_pair = [key_value_pair, char.to_string()].join(SYMBOL.empty_string);
                        let byte = 0;
                        let mut char_buffer = vec![byte, byte, byte, byte];
                        let length = char_buffer.len();
                        cursor.read_exact(&mut char_buffer).unwrap();
                        bytes_read = bytes_read + length as i128;
                        let remaining_bool = String::from_utf8(char_buffer).unwrap();
                        if remaining_bool != "alse" {
                            let message = format!("Unable to parse boolean: {}", key_value_pair);
                            return Err(message)
                        }
                        key_value_pair = [key_value_pair, remaining_bool].join(SYMBOL.empty_string);
                    }

                    let is_array = char == "[";
                    if is_array {
                        // read the array (including nested objects and arrays)
                    }

                    let is_object = char == "{";
                    if is_object {
                        // read the object (including nested objects and arrays)
                    }

                    let is_number =
                        !is_string &&
                            !is_null &&
                            !is_boolean_true &&
                            !is_boolean_false &&
                            !is_array &&
                            !is_object;
                    if is_number {
                        // read until char is not number and decimal point, minus, exponent
                    }

                    is_whitespace = false;
                }


            }

            // attempt to read till comma, indicates presence of another key-value pair
            buf = vec![];
            boxed_read = cursor.read_until(b',', &mut buf);
            if boxed_read.is_err() {
                let message = boxed_read.err().unwrap().to_string();
                return Err(message);
            }
            bytes_read = bytes_read + boxed_read.unwrap() as i128;
            if bytes_read == total_bytes {
                is_there_a_key_value = false;
            };

            let (property, value) = parse_json_property(&key_value_pair).unwrap();


            properties.push((property, value));

        }
        Ok(properties)
    }

    pub fn to_json_string(key_value_list: Vec<(JSONProperty, JSONValue)>) -> String {
        let mut json_list = vec![];
        json_list.push(SYMBOL.opening_curly_bracket.to_string());


        let mut properties_list = vec![];

        for (property, value) in key_value_list {

            if &property.property_type == "String" {
                let raw_value = value.String.unwrap();
                let formatted_property = format!("  \"{}\": \"{}\"", &property.property_name, raw_value);
                properties_list.push(formatted_property.to_string());
            }

            if &property.property_type == "bool" {
                let raw_value = value.bool.unwrap();
                let formatted_property = format!("  \"{}\": {}", &property.property_name, raw_value);
                properties_list.push(formatted_property.to_string());
            }
        }


        let comma_new_line_carriage_return = format!("{}{}", SYMBOL.comma, SYMBOL.new_line_carriage_return);
        let properties = properties_list.join(&comma_new_line_carriage_return);

        json_list.push(properties);
        json_list.push(SYMBOL.closing_curly_bracket.to_string());
        let json= json_list.join(SYMBOL.new_line_carriage_return);
        json
    }
}

pub struct JSONType {
    pub string: &'static str,
    pub boolean: &'static str,
    pub object: &'static str,
    pub array: &'static str,
    pub integer: &'static str,
    pub number: &'static str,
    pub null: &'static str,
}

pub const JSON_TYPE: JSONType = JSONType{
    string: "String",
    boolean: "bool",
    object: "object",
    array: "array",
    integer: "i128",
    number: "f64",
    null: "null",
};

pub struct JSONProperty {
    pub property_name: String,
    pub property_type: String,
}

pub struct JSONValue {
    pub f64: Option<f64>,
    pub i128: Option<i128>,
    pub String: Option<String>,
    pub bool: Option<bool>,
    pub null: Option<Null>,
}

pub struct Null {}

pub trait ToJSON {
    fn list_properties() -> Vec<JSONProperty>;

    fn get_property(&self, property_name: String) -> JSONValue;

    fn to_json_string(&self) -> String;
}

pub trait FromJSON {
    fn parse_json_to_properties(&self, json_string: String) -> Result<Vec<(JSONProperty, JSONValue)>, String>;
    fn set_properties(&mut self, properties: Vec<(JSONProperty, JSONValue)>) -> Result<(), String>;
    fn parse(&mut self, json_string: String) -> Result<(), String>;
}

