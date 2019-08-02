use crate::serial::JsonParseErrDetail;
use crate::serial::JsonParser as Parser;
use crate::tests::serial::NodeRefExt;
use kg_diag::{Diag};
use kg_tree::NodeRef;
use kg_diag::ParseDiag;

macro_rules! parse_node {
    ($input: expr) => {{
        let mut r = kg_diag::MemCharReader::new($input.as_bytes());
        let mut parser = crate::serial::JsonParser::new();
        parser.parse(&mut r).unwrap_or_else(|err| {
            eprintln!("{}", err);
            panic!("Error parsing node!")
        })
    }};
}

macro_rules! parse_node_err {
    ($input: expr) => {{
        let mut r = kg_diag::MemCharReader::new($input.as_bytes());
        let mut parser = crate::serial::JsonParser::new();
        let err = parser.parse(&mut r).map(|node|{
            panic!("Error expected! got node: {}", node.to_json_pretty())
        })
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
fn strings() { //TODO MC Add \b, \f to parser
    let input = r#"{
        "str1": " literal string \n \t \r \t \" \\ '"
    }"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(
        " literal string \n \t \r \t \" \\ '",
        node.get_key("str1").as_string_ext()
    );
}

#[test]
fn symbol_in_key() {
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

    assert_eq!("red", node.get_key("arr2").as_array_ext()[0].as_string_ext());
    assert_eq!("yellow", node.get_key("arr2").as_array_ext()[1].as_string_ext());
    assert_eq!("green", node.get_key("arr2").as_array_ext()[2].as_string_ext());

    assert_eq!(true, node.get_key("arr3").as_array_ext()[0].as_bool_ext());
    assert_eq!(false, node.get_key("arr3").as_array_ext()[1].as_bool_ext());
}

#[test]
fn array_of_arrays() {
    let input = r#"{
        "arr1": [ [ 1, 2 ], [3, 4, 5] ]
    }"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(1, node.get_key("arr1").as_array_ext()[0].as_array_ext()[0].as_int_ext());
    assert_eq!(2, node.get_key("arr1").as_array_ext()[0].as_array_ext()[1].as_int_ext());
    assert_eq!(3, node.get_key("arr1").as_array_ext()[1].as_array_ext()[0].as_int_ext());
    assert_eq!(4, node.get_key("arr1").as_array_ext()[1].as_array_ext()[1].as_int_ext());
    assert_eq!(5, node.get_key("arr1").as_array_ext()[1].as_array_ext()[2].as_int_ext());
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

    assert_eq!(1, node.get_key("arr1").as_array_ext()[0].as_array_ext()[0].as_int_ext());
    assert_eq!(2, node.get_key("arr1").as_array_ext()[0].as_array_ext()[1].as_int_ext());
    assert_eq!("a", node.get_key("arr1").as_array_ext()[1].as_array_ext()[0].as_string_ext());
    assert_eq!("b", node.get_key("arr1").as_array_ext()[1].as_array_ext()[1].as_string_ext());
    assert_eq!("c", node.get_key("arr1").as_array_ext()[1].as_array_ext()[2].as_string_ext());

    assert_eq!(1, node.get_key("arr2").as_array_ext()[0].as_int_ext());
    assert_eq!(2.0, node.get_key("arr2").as_array_ext()[1].as_float_ext());

    assert_eq!(1, node.get_key("arr3").as_array_ext()[0].as_int_ext());
    assert_eq!("string", node.get_key("arr3").as_array_ext()[1].as_string_ext());

    assert_eq!(1, node.get_key("arr4").as_array_ext()[0].as_int_ext());
    assert!(node.get_key("arr4").as_array_ext()[1].is_null());

    assert_eq!(1, node.get_key("arr5").as_array_ext()[0].as_int_ext());
    assert!(node.get_key("arr5").as_array_ext()[1].is_empty_ext());
}

//#########################################

#[test]
fn brace_right_after_comma() {
    let input = r#"{"key": 1,}"#;

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, JsonParseErrDetail::UnexpectedTokenOne {..});
}

#[test] //FIXME MC Error should be expected and parser should be fixed.
fn square_bracket_right_after_comma() {
    let input = r#"{"arr1": [1,]}"#;

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, JsonParseErrDetail::UnexpectedTokenMany {..});
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
let input = r#"{"int":1}"#;
let input = r#"{"int": 1 , "int2": 2}"#;
let input = r#"{int: 1 int2: 2}"#;
    let input = r#"{
        "str1": " literal string \n \t \u1234"
    }"#;
Test with UTF-8 BOM
Test with duplicated keys
Test with whitespaces
*/
