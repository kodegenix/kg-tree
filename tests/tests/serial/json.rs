use crate::serial::JsonParser as Parser;
use crate::tests::serial::NodeRefExt;
use kg_diag::{MemCharReader};
use kg_tree::NodeRef;

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

/*
let input = r#""#;
let input = r#"{}"#;
let input = r#"{int: 1}"#;
let input = r#"{"int": 1,}"#; //przecinek przed }, nie ma zabezpieczenia w parserze
let input = r#"{"int":1}"#;
let input = r#"{"int": 1 , "int2": 2}"#;
let input = r#"{
int: 1
int2: 2}"#;
*/
