use crate::json::{JSON_TYPE, JSONValue};
use crate::json::object::{FromJSON, JSON, ToJSON};
use crate::json::property::JSONProperty;
use crate::symbol::SYMBOL;

#[test]
fn convert_to_and_from_json_string_to_object_with_property_of_a_bool_type() {
    // declare object with String property
    struct SomeObject {
        prop_a: bool,
    }

    // implement FromJSON and ToJSON traits
    impl FromJSON for SomeObject {
        fn parse_json_to_properties(&self, json_string: String) -> Result<Vec<(JSONProperty, JSONValue)>, String> {
            let boxed_parse = JSON::parse_as_properties(json_string);
            if boxed_parse.is_err() {
                let message = boxed_parse.err().unwrap();
                return Err(message)
            }
            let properties = boxed_parse.unwrap();
            Ok(properties)
        }
        fn set_properties(&mut self, properties: Vec<(JSONProperty, JSONValue)>) -> Result<(), String> {
            for (property, value) in properties {
                if property.property_name == "prop_a" {
                    self.prop_a = value.bool.unwrap();
                }
            }
            Ok(())
        }
        fn parse(&mut self, json_string: String) -> Result<(), String> {
            let boxed_properties = self.parse_json_to_properties(json_string);
            if boxed_properties.is_err() {
                let message = boxed_properties.err().unwrap();
                return Err(message);
            }
            let properties = boxed_properties.unwrap();
            let boxed_set = self.set_properties(properties);
            if boxed_set.is_err() {
                let message = boxed_set.err().unwrap();
                return Err(message);
            }
            Ok(())
        }
    }

    impl ToJSON for SomeObject {
        fn list_properties() -> Vec<JSONProperty> {
            let mut list = vec![];

            let property = JSONProperty {
                property_name: "prop_a".to_string(),
                property_type: JSON_TYPE.boolean.to_string()
            };
            list.push(property);

            list
        }

        fn get_property(&self, property_name: String) -> JSONValue {
            let mut value = JSONValue::new();

            if property_name == "prop_a".to_string() {
                let boolean : bool = self.prop_a.to_owned();
                value.bool = Some(boolean);
            }

            value
        }

        fn to_json_string(&self) -> String {
            let mut json_list = vec![];
            json_list.push(SYMBOL.opening_curly_bracket.to_string());


            let mut properties_list = vec![];

            let properties = SomeObject::list_properties();
            for property in properties {
                let value = self.get_property(property.property_name.to_string());

                if &property.property_type == JSON_TYPE.boolean {
                    if value.bool.is_some() {
                        let raw_value = value.bool.unwrap();
                        let formatted_property = format!("  \"{}\": {}", &property.property_name, raw_value);
                        properties_list.push(formatted_property.to_string());
                    }
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


    // convert SomeObject to json string
    let mut obj = SomeObject { prop_a: true };
    let json_string = obj.to_json_string();

    let expected_json_string = "{\r\n  \"prop_a\": true\r\n}";
    assert_eq!(expected_json_string, json_string);


    // convert json string to SomeObject
    let json_string = "{\r\n  \"prop_a\": false\r\n}";
    obj.parse(json_string.to_string()).unwrap();
    assert_eq!(false, obj.prop_a);
}