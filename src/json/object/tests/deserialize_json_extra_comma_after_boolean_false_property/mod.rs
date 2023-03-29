use file_ext::FileExt;
use crate::json::example_object::ExampleObject;
use crate::json::object::{FromJSON};

#[test]
fn deserialize_json_extra_comma_after_boolean_false_property() {
    let path = FileExt::build_path(&["src", "json", "object", "tests", "deserialize_json_extra_comma_after_boolean_false_property", "some-object.json"]);
    let pwd = FileExt::working_directory().unwrap();

    let absolute_file_path = FileExt::build_path(&[pwd.as_str(), path.as_str()]);
    let file_as_bytes = FileExt::read_file(absolute_file_path.as_str()).unwrap();
    let json_missing_comma = String::from_utf8(file_as_bytes).unwrap();

    let mut parsed_json = ExampleObject {
        prop_a: "qwerty".to_string(),
        prop_b: false,
        prop_c: false,
        prop_d: 0,
        prop_e: 0.0,
        prop_f: None,
        prop_g: None,
    };
    let json_without_comma = parsed_json.parse(json_missing_comma);
    assert!(json_without_comma.is_err());

    let message = json_without_comma.err().unwrap();
    assert_eq!("provided json is not valid. error near byte 24 of 24 `{\r\n  \"prop_b\": false,\r\n}`", message);


}