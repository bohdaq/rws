use crate::json::{FromAndToJSON, JSONProperty, JSONValue, parse_json_property};
use crate::symbol::SYMBOL;

#[test]
fn parse() {
    struct SomeObject {
        prop_a: String,
        prop_b: bool
    }

    impl FromAndToJSON for SomeObject {
        fn list_properties() -> Vec<JSONProperty> {
            let mut list = vec![];

            let property = JSONProperty { property_name: "prop_a".to_string(), property_type: "String".to_string() };
            list.push(property);

            let property = JSONProperty { property_name: "prop_b".to_string(), property_type: "bool".to_string() };
            list.push(property);

            list
        }

        fn get_property(&self, property_name: String) -> JSONValue {
            let mut value = JSONValue {
                i8: None,
                u8: None,
                i16: None,
                u16: None,
                i32: None,
                u32: None,
                i64: None,
                u64: None,
                i128: None,
                u128: None,
                usize: None,
                isize: None,
                string: None,
                boolean: None,
                null: None,
            };

            if property_name == "prop_a".to_string() {
                let string : String = self.prop_a.to_owned();
                value.string = Some(string);
            }

            if property_name == "prop_b".to_string() {
                let boolean : bool = self.prop_b;
                value.boolean = Some(boolean);
            }

            value
        }

        fn to_json_string(&self) -> String {
            let mut json_list = vec![];
            json_list.push(SYMBOL.opening_curly_bracket.to_string());

            let properties = SomeObject::list_properties();
            for property in properties {
                let value = self.get_property(property.property_name.to_string());

                if &property.property_type == "String" {
                    let raw_value = value.string.unwrap();
                    let formatted_property = format!("  \"{}\": \"{}\"", &property.property_name, raw_value);
                    json_list.push(formatted_property.to_string());
                }

                if &property.property_type == "bool" {
                    let raw_value = value.boolean.unwrap();
                    let formatted_property = format!("  \"{}\": {}", &property.property_name, raw_value);
                    json_list.push(formatted_property.to_string());
                }
            }
            json_list.push(SYMBOL.closing_curly_bracket.to_string());


            let json = json_list.join(SYMBOL.new_line_carriage_return);
            json
        }

        fn from_json_string(json_string: String) -> Self {
            todo!()
        }
    }

    let obj = SomeObject { prop_a: "123abc".to_string(), prop_b: true };

    let json_string = obj.to_json_string();
    let expected_json_string = "{\r\n  \"prop_a\": \"123abc\"\r\n  \"prop_b\": true\r\n}";

    assert_eq!(expected_json_string, json_string)
}

#[test]
fn parse_raw_property() {
    let property_key = "key";
    let property_value = "some data";
    let property_type = "String";

    let raw_string = format!("\"{}\": \"{}\"", property_key, property_value);
    let (key, value) = parse_json_property(&raw_string).unwrap();

    assert_eq!(key.property_name, property_key);
    assert_eq!(key.property_type, property_type);
    assert_eq!(value.string.unwrap(), property_value);
}