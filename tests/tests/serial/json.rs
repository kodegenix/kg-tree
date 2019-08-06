use crate::serial::json::JsonParseErrDetail;
use crate::tests::serial::NodeRefExt;
use kg_diag::Diag;
use kg_diag::ParseDiag;
use kg_tree::NodeRef;
use kg_tree::serial::json::*;

macro_rules! parse_node {
    ($input: expr) => {{
        let mut r = kg_diag::MemCharReader::new($input.as_bytes());
        let mut parser = crate::serial::json::JsonParser::new();
        parser.parse(&mut r).unwrap_or_else(|err| {
            eprintln!("{}", err);
            panic!("Error parsing node!")
        })
    }};
}

macro_rules! parse_node_err {
    ($input: expr) => {{
        let mut r = kg_diag::MemCharReader::new($input.as_bytes());
        let mut parser = crate::serial::json::JsonParser::new();
        let err = parser
            .parse(&mut r)
            .map(|node| panic!("Error expected! got node: {}", node.to_json_pretty()))
            .unwrap_err();
        err
    }};
}

macro_rules! assert_err {
    ($err: expr, $variant: pat) => {
        let detail = $err
            .detail()
            .downcast_ref::<JsonParseErrDetail>()
            .expect("cannot downcast to JsonParseErrDetail");

        match detail {
            $variant => {}
            err => panic!("Expected error {} got {:?}", stringify!($variant), err),
        }
    };
}

#[test]
fn unexpected_end_of_input() {
    let input = r#"{
        "key": "\"#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, JsonParseErrDetail::UnexpectedEoiOne {..});
}

#[test]
fn invalid_char() {
    let input = r#"{=}"#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, JsonParseErrDetail::InvalidChar {..});
}

#[test]
fn null() {
    let input = r#"null"#;
    let node: NodeRef = parse_node!(input);

    assert!(node.is_null());
}

#[test]
fn integer() {
    let input = r#"1"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(1, node.as_int_ext());
}

#[test]
fn float() {
    let input = r#"15.21"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(15.21, node.as_float_ext());
}

#[test]
fn string() {
    let input = r#""string""#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("string", node.as_string_ext());
}

#[test]
fn boolean_true() {
    let input = r#"true"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(true, node.as_bool_ext());
}

#[test]
fn boolean_false() {
    let input = r#"false"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(false, node.as_bool_ext());
}

#[test]
fn array() {
    let input = r#"[]"#;
    let node: NodeRef = parse_node!(input);

    assert!(node.as_array_ext().is_empty());
}

#[test]
fn object() {
    let input = r#"{}"#;
    let node: NodeRef = parse_node!(input);

    assert!(node.is_empty_ext());
}

#[test]
fn nulls() {
    let input = r#"{
        "nth1": null
    }"#;
    let node: NodeRef = parse_node!(input);
    assert!(node.get_key("nth1").is_null());
}

#[test]
fn integers() {
    let input = r#"{
        "int1": 421,
        "int2": -452,
        "int3": 0
    }"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(421, node.get_key("int1").as_int_ext());
    assert_eq!(-452, node.get_key("int2").as_int_ext());
    assert_eq!(0, node.get_key("int3").as_int_ext());
}

#[test]
fn floats() {
    let input = r#"{
        "flt1": 42.552,
        "flt2": -4.24,
        "flt3": 0.01,

        "flt4": 42e34,
        "flt5": 28e+26,
        "flt6": 84e-16,
        "flt7": -85e+86,
        "flt8": -49e-36,

        "flt9": 4.2e34,
        "flt10": 2.8e+26,
        "flt11": 8.4e-16,
        "flt12": -8.5e+86,
        "flt13": -4.9e-36
    }"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(42.552, node.get_key("flt1").as_float_ext());
    assert_eq!(-4.24, node.get_key("flt2").as_float_ext());
    assert_eq!(0.01, node.get_key("flt3").as_float_ext());

    assert_eq!(42e34, node.get_key("flt4").as_float_ext());
    assert_eq!(28e+26, node.get_key("flt5").as_float_ext());
    assert_eq!(84e-16, node.get_key("flt6").as_float_ext());
    assert_eq!(-85e+86, node.get_key("flt7").as_float_ext());
    assert_eq!(-49e-36, node.get_key("flt8").as_float_ext());

    assert_eq!(4.2e34, node.get_key("flt9").as_float_ext());
    assert_eq!(2.8e+26, node.get_key("flt10").as_float_ext());
    assert_eq!(8.4e-16, node.get_key("flt11").as_float_ext());
    assert_eq!(-8.5e+86, node.get_key("flt12").as_float_ext());
    assert_eq!(-4.9e-36, node.get_key("flt13").as_float_ext());
}

#[test]
fn scientific_notation_invalid_char() {
    let input = r#"{
        "num": 1.2e-s
    }"#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, JsonParseErrDetail::InvalidCharMany {..});
}

#[test]
fn scientific_notation_invalid_char_2() {
    let input = r#"{
        "num": 1.2ee23
    }"#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, JsonParseErrDetail::InvalidCharMany {..});
}

#[test]
fn parse_float_err() {
    let input = r#"{
        "num": -e1
    }"#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, JsonParseErrDetail::InvalidFloatLiteral {..});
}

#[test]
fn scientific_notation_unexpected_end_of_input() {
    let input = r#"{
        "num": 1.2e+"#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, JsonParseErrDetail::UnexpectedEoiMany {..});
}

#[test]
fn booleans() {
    let input = r#"{
        "bool1": true,
        "bool2": false
    }"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(true, node.get_key("bool1").as_bool_ext());
    assert_eq!(false, node.get_key("bool2").as_bool_ext());
}

#[test]
fn strings() {
    let input = r#"{
        "str1": " literal string \n \t \r \t \" \\ \b \f'"
    }"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(
        " literal string \n \t \r \t \" \\ \u{0008} \u{000c}'",
        node.get_key("str1").as_string_ext()
    );
}

#[test]
fn string_utf8() {
    let input = r#"{
        "str1": "✅ ❄ ❤ 💖"
    }"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("✅ ❄ ❤ 💖", node.get_key("str1").as_string_ext());
}

//FIXME MC UTF-8 as \u0000 should work in json parser
//#[test]
fn custom_escapes() {
    let input = r#"{
        "str1": "\u00f8C"
    }"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("\\u00f8C", node.get_key("str1").as_string_ext());
}

//FIXME MC UTF-8 as \u0000 should work in json parser
//#[test]
fn bad_custom_escape() {
    let input = r#"{
        "str1": "\uD800"
    }"#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, JsonParseErrDetail::InvalidEscape {..});
}

//FIXME MC UTF-8 as \u0000 should work in json parser
//#[test]
fn too_short_custom_escape() {
    let input = r#"{
        "str1": "\u002"
    }"#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, JsonParseErrDetail::InvalidEscape {..});
}

#[test]
fn string_bad_escape() {
    let input = r#"{
        "str1": "\h"
    }"#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, JsonParseErrDetail::InvalidEscape {..});
}

//#[test] //FIXME MC Fix parser, error is expected.
fn control_char_in_string() {
    let input = "{\"key\": \"val\nue\"}";
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, JsonParseErrDetail::InvalidChar {..});
}

#[test]
fn invalid_input_unexpected_end_of_input() {
    let input = r#"{
        "key": n"#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, JsonParseErrDetail::UnexpectedEoi {..});
}

#[test]
fn empty_key() {
    let input = r#"{
        "": "no key name"
    }"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("no key name", node.get_key("").as_string_ext());
}

#[test]
fn different_keys() {
    let input = r#"{
        "⌨": "value1",
        "127.0.0.1": "value2",
        "character encoding": "value3",
        "ʎǝʞ": "value4",
        "'key'": "value5"
    }"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("value1", node.get_key("⌨").as_string_ext());
    assert_eq!("value2", node.get_key("127.0.0.1").as_string_ext());
    assert_eq!("value3", node.get_key("character encoding").as_string_ext());
    assert_eq!("value4", node.get_key("ʎǝʞ").as_string_ext());
    assert_eq!("value5", node.get_key("'key'").as_string_ext());
}

#[test]
fn no_colon() {
    let input = r#"{
        "key" "no colon"
    }"#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, JsonParseErrDetail::UnexpectedTokenOne {..});
}

#[test]
fn unexpected_token_after_key() {
    let input = r#"{
        "key": ,
    }"#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, JsonParseErrDetail::UnexpectedTokenMany {..});
}

//#[test] //FIXME MC Fix parser, error is expected.
fn control_char_in_key() {
    let input = r#"{
        "ke
        y": "value"
    }"#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, JsonParseErrDetail::UnexpectedTokenMany {..});
}

#[test]
fn two_words_in_key() {
    let input = r#"{
        "two words": "value"
    }"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("value", node.get_key("two words").as_string_ext());
}

//#[test]
fn test() {
    //FIXME MC Fix parser: add "duplicated keys" error, fix test: error should be expected
    let input = r#"{
        "key1": "value1",
        "key1": "value2"
    }"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("value1", node.get_key("key1").as_string_ext());
}

#[test]
fn arrays() {
    let input = r#"{
        "arr1": [1, 2, 3],
        "arr2": ["red", "yellow", "green"],
        "arr3": [true, false]
    }"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(1, node.get_key("arr1").as_array_ext()[0].as_int_ext());
    assert_eq!(2, node.get_key("arr1").as_array_ext()[1].as_int_ext());
    assert_eq!(3, node.get_key("arr1").as_array_ext()[2].as_int_ext());

    assert_eq!(
        "red",
        node.get_key("arr2").as_array_ext()[0].as_string_ext()
    );
    assert_eq!(
        "yellow",
        node.get_key("arr2").as_array_ext()[1].as_string_ext()
    );
    assert_eq!(
        "green",
        node.get_key("arr2").as_array_ext()[2].as_string_ext()
    );

    assert_eq!(true, node.get_key("arr3").as_array_ext()[0].as_bool_ext());
    assert_eq!(false, node.get_key("arr3").as_array_ext()[1].as_bool_ext());
}

#[test]
fn array_of_arrays() {
    let input = r#"{
        "arr1": [ [ 1, 2 ], [3, 4, 5] ]
    }"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(
        1,
        node.get_key("arr1").as_array_ext()[0].as_array_ext()[0].as_int_ext()
    );
    assert_eq!(
        2,
        node.get_key("arr1").as_array_ext()[0].as_array_ext()[1].as_int_ext()
    );
    assert_eq!(
        3,
        node.get_key("arr1").as_array_ext()[1].as_array_ext()[0].as_int_ext()
    );
    assert_eq!(
        4,
        node.get_key("arr1").as_array_ext()[1].as_array_ext()[1].as_int_ext()
    );
    assert_eq!(
        5,
        node.get_key("arr1").as_array_ext()[1].as_array_ext()[2].as_int_ext()
    );
}

#[test]
fn array_mixed_types() {
    let input = r#"{
        "arr1": [ [ 1, 2 ], ["a", "b", "c"] ],
        "arr2": [ 1, 2.0 ],
        "arr3": [ 1, "string" ],
        "arr4": [ 1, null ],
        "arr5": [ 1, {} ]
    }"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(
        1,
        node.get_key("arr1").as_array_ext()[0].as_array_ext()[0].as_int_ext()
    );
    assert_eq!(
        2,
        node.get_key("arr1").as_array_ext()[0].as_array_ext()[1].as_int_ext()
    );
    assert_eq!(
        "a",
        node.get_key("arr1").as_array_ext()[1].as_array_ext()[0].as_string_ext()
    );
    assert_eq!(
        "b",
        node.get_key("arr1").as_array_ext()[1].as_array_ext()[1].as_string_ext()
    );
    assert_eq!(
        "c",
        node.get_key("arr1").as_array_ext()[1].as_array_ext()[2].as_string_ext()
    );

    assert_eq!(1, node.get_key("arr2").as_array_ext()[0].as_int_ext());
    assert_eq!(2.0, node.get_key("arr2").as_array_ext()[1].as_float_ext());

    assert_eq!(1, node.get_key("arr3").as_array_ext()[0].as_int_ext());
    assert_eq!(
        "string",
        node.get_key("arr3").as_array_ext()[1].as_string_ext()
    );

    assert_eq!(1, node.get_key("arr4").as_array_ext()[0].as_int_ext());
    assert!(node.get_key("arr4").as_array_ext()[1].is_null());

    assert_eq!(1, node.get_key("arr5").as_array_ext()[0].as_int_ext());
    assert!(node.get_key("arr5").as_array_ext()[1].is_empty_ext());
}

#[test]
fn array_unexpected_token() {
    let input = r#"{
        "arr1": [ 1 { ]
    }"#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, JsonParseErrDetail::UnexpectedToken {..});
}

#[test]
fn array_newline() {
    let input = r#"{
        "arr1": [
        1,
        2,
        3 ]
    }"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(1, node.get_key("arr1").as_array_ext()[0].as_int_ext());
    assert_eq!(2, node.get_key("arr1").as_array_ext()[1].as_int_ext());
    assert_eq!(3, node.get_key("arr1").as_array_ext()[2].as_int_ext());
}

#[test]
fn values_in_object() {
    let input = r#"{
        "ob1": {
            "key1": "string",
            "key2": 74,
            "key3": true,
            "key4": null,
            "key5": 3.37e+12,
            "key6": [],
            "key7": {}
        }
    }"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(
        "string",
        node.get_key("ob1").get_key("key1").as_string_ext()
    );
    assert_eq!(74, node.get_key("ob1").get_key("key2").as_int_ext());
    assert_eq!(true, node.get_key("ob1").get_key("key3").as_bool_ext());
    assert!(node.get_key("ob1").get_key("key4").is_null());
    assert_eq!(3.37e12, node.get_key("ob1").get_key("key5").as_float_ext());
    assert!(node
        .get_key("ob1")
        .get_key("key6")
        .as_array_ext()
        .is_empty());
    assert!(node.get_key("ob1").get_key("key7").is_empty_ext());
}

#[test]
fn multiple_objects() {
    let input = r#"{
        "ob1": {
            "key1": "first string",
            "key2": 936
        },
        "ob2": {
            "key1": "second string",
            "key2": 375
        }
    }"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(
        "first string",
        node.get_key("ob1").get_key("key1").as_string_ext()
    );
    assert_eq!(936, node.get_key("ob1").get_key("key2").as_int_ext());

    assert_eq!(
        "second string",
        node.get_key("ob2").get_key("key1").as_string_ext()
    );
    assert_eq!(375, node.get_key("ob2").get_key("key2").as_int_ext());
}

#[test]
fn object_in_object() {
    let input = r#"{
        "ob1": {
            "ob2": {
                "key": "value"
            }
        }

    }"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(
        "value",
        node.get_key("ob1")
            .get_key("ob2")
            .get_key("key")
            .as_string_ext()
    );
}

#[test]
fn objects_in_array() {
    let input = r#"{
        "arr": [
            {
                "key": "value1"
            },
            {
                "key": "value2"
            }
        ]
    }"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(
        "value1",
        node.get_key("arr").as_array_ext()[0]
            .get_key("key")
            .as_string_ext()
    );
    assert_eq!(
        "value2",
        node.get_key("arr").as_array_ext()[1]
            .get_key("key")
            .as_string_ext()
    );
}

#[test]
fn object_with_unexpected_token() {
    let input = r#"{
        "ob": {"key": "value":}
    }"#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, JsonParseErrDetail::UnexpectedTokenMany {..});

    let detail = err
        .detail()
        .downcast_ref::<JsonParseErrDetail>()
        .expect("cannot downcast to JsonParseErrDetail");

    match detail {
        JsonParseErrDetail::UnexpectedTokenMany { expected, ..} => {
            assert_eq!(Terminal::Comma, expected[0]);
            assert_eq!(Terminal::BraceRight, expected[1]);
        }
        err => panic!("Unexpected error type {:?}", err),
    }
}

//#########################################

#[test]
fn brace_right_after_comma() {
    let input = r#"{"key": 1,}"#;

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, JsonParseErrDetail::UnexpectedTokenOne {..});
}

#[test]
fn square_bracket_right_after_comma() {
    let input = r#"{"arr1": [1,]}"#;

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, JsonParseErrDetail::UnexpectedTokenMany {..});
}

#[test]
fn unexpected_eoi_one() {
    let input = r#"{
        "string": "cos
    }"#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, JsonParseErrDetail::UnexpectedEoiOne {..});
}
//#########################################

#[test]
fn no_whitespace_after_curly_bracket() {
    let input = r#"{"int1": 421}"#;
    let node: NodeRef = parse_node!(input);
    assert_eq!(421, node.get_key("int1").as_int_ext());
}

#[test]
fn no_whitespace_after_comma() {
    let input = r#"{"int1": 421,"int2": -452}"#;
    let node: NodeRef = parse_node!(input);
    assert_eq!(421, node.get_key("int1").as_int_ext());
    assert_eq!(-452, node.get_key("int2").as_int_ext());
}

/* TODO MC Tests:
let input = r#""#;
let input = r#"{int: 1}"#;
let input = r#"{"int": 1(}"#;
let input = r#"{"int":1}"#;
let input = r#"{"int": 1 , "int2": 2}"#;
let input = r#"{int: 1 int2: 2}"#;
    let input = r#"{
        "str1": " literal string \n \t \u1234"
    }"#;
Test with UTF-8 BOM
Test with duplicated keys
Test with whitespaces
Test with InvalidCharOne
*/
