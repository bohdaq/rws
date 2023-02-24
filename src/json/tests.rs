use crate::json::{ToJSON, JSONProperty, JSONValue};
use crate::symbol::SYMBOL;

#[test]
fn parse() {
    struct SomeObject {
        prop_a: String,
        prop_b: bool
    }

    impl SomeObject {
        fn from_json_string(&self, json_string: String) -> Result<SomeObject, String> {
            let result= SomeObject{ prop_a: "".to_string(), prop_b: false };


            Ok(result)
        }
    }

    impl ToJSON for SomeObject {
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
                f64: None,
                i128: None,
                String: None,
                bool: None,
                null: None,
            };

            if property_name == "prop_a".to_string() {
                let string : String = self.prop_a.to_owned();
                value.String = Some(string);
            }

            if property_name == "prop_b".to_string() {
                let boolean : bool = self.prop_b;
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

    let obj = SomeObject { prop_a: "123abc".to_string(), prop_b: true };

    let json_string = obj.to_json_string();
    let expected_json_string = "{\r\n  \"prop_a\": \"123abc\",\r\n  \"prop_b\": true\r\n}";

    assert_eq!(expected_json_string, json_string);

    let parsed_json_object : SomeObject = obj.from_json_string(json_string.to_string()).unwrap();
}

