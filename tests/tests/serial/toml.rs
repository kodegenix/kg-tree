use kg_diag::{MemCharReader, Span};
use crate::serial::TomlParser as Parser;
use kg_tree::NodeRef;
use kg_utils::collections::LinkedHashMap;
use crate::tests::serial::NodeRefExt;

macro_rules! parse_node {
            ($input: expr) => {
                {
                    let mut r = kg_diag::MemCharReader::new($input.as_bytes());
                    let mut parser = crate::serial::TomlParser::new();
                    parser.parse(&mut r).unwrap_or_else(|err|{
                        println!("{}", err); panic!()
                    })
                }
            }
}

#[test]
fn integer() {
    let input = r#"
        int1 = +99
        int2 = 42
        int3 = 0
        int4 = -17
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(99, node.get_key("int1").into_int());
    assert_eq!(42, node.get_key("int2").into_int());
    assert_eq!(0, node.get_key("int3").into_int());
    assert_eq!(-17, node.get_key("int4").into_int());
}

#[test]
fn integer_underscore() {
    let input = r#"
        int1 = 1_000
        int2 = 5_349_221
        int3 = 1_2_3_4_5
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(1_000, node.get_key("int1").into_int());
    assert_eq!(5_349_221, node.get_key("int2").into_int());
    assert_eq!(1_2_3_4_5, node.get_key("int3").into_int());
}

#[test]
fn integer_prefix() {
    let input = r#"
        hex1 = 0xDEADBEEF
        hex2 = 0xdeadbeef
        hex3 = 0xdead_beef


        oct1 = 0o0123_4567
        oct2 = 0o755

        bin1 = 0b11010110
        bin2 = 0b1101_0110
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(0xdeadbeef, node.get_key("hex1").into_int());
    assert_eq!(0xdeadbeef, node.get_key("hex2").into_int());
    assert_eq!(0xdead_beef, node.get_key("hex3").into_int());

    assert_eq!(0o01234567, node.get_key("oct1").into_int());
    assert_eq!(0o755, node.get_key("oct2").into_int());

    assert_eq!(0b11010110, node.get_key("bin1").into_int());
    assert_eq!(0b11010110, node.get_key("bin2").into_int());
}

#[test]
fn floats() {
    let input = r#"
        flt1 = +1.0
        flt2 = 3.1415
        flt3 = -0.01

        flt4 = 5e+22
        flt5 = 1e6
        flt6 = -2E-2

        flt7 = 6.626e-34
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(1.0, node.get_key("flt1").into_float());
    assert_eq!(3.1415, node.get_key("flt2").into_float());
    assert_eq!(-0.01, node.get_key("flt3").into_float());

    assert_eq!(5e+22, node.get_key("flt4").into_float());
    assert_eq!(1e6, node.get_key("flt5").into_float());
    assert_eq!(-2E-2, node.get_key("flt6").into_float());

    assert_eq!(6.626e-34, node.get_key("flt7").into_float());
}

#[test]
fn floats_underscore() {
    let input = r#"
        flt1 = +1.0
        flt2 = 3.14_15
        flt3 = -0.0_1

        flt4 = 5e+22
        flt5 = 1e6
        flt6 = -2E-2

        flt7 = 6_6.6_26e-34
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(1.0, node.get_key("flt1").into_float());
    assert_eq!(3.1415, node.get_key("flt2").into_float());
    assert_eq!(-0.01, node.get_key("flt3").into_float());

    assert_eq!(5e+22, node.get_key("flt4").into_float());
    assert_eq!(1e6, node.get_key("flt5").into_float());
    assert_eq!(-2E-2, node.get_key("flt6").into_float());

    assert_eq!(66.626e-34, node.get_key("flt7").into_float());
}

#[test]
fn floats_special() {
    let input = r#"
        sf1 = inf
        sf2 = +inf
        sf3 = -inf

        sf4 = nan
        sf5 = +nan
        sf6 = -nan
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(std::f64::INFINITY, node.get_key("sf1").into_float());
    assert_eq!(std::f64::INFINITY, node.get_key("sf2").into_float());
    assert_eq!(std::f64::NEG_INFINITY, node.get_key("sf3").into_float());

    assert!(node.get_key("sf4").into_float().is_nan());
    assert!(node.get_key("sf5").into_float().is_nan());
    assert!(node.get_key("sf6").into_float().is_nan());
}

#[test]
fn booleans() {
    let input = r#"
        bool1 = true
        bool2 = false
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(true, node.get_key("bool1").into_bool());
    assert_eq!(false, node.get_key("bool2").into_bool());
}

#[test]
fn literal_string() {
    let input = r#"
        str1 = ' literal string \n \t \u1234'
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(" literal string \\n \\t \\u1234", node.get_key("str1").into_string());
}

#[test]
fn literal_multiline_string() {
    let input = r#"
        str1 = '''
multiline
literal string
'''
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("multiline\nliteral string\n", node.get_key("str1").into_string());
}

#[test]
fn basic_string() {
    let input = r#"
        str1 = "some basic string\n \t \" '"
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("some basic string\n \t \" '", node.get_key("str1").into_string());
}

#[test]
fn basic_multiline_string() {
    let input = "str1 = \"\"\"\nsome basic\nmultiline\nstring\\n \\t \\\"\"\"\"";

    let node: NodeRef = parse_node!(input);

    assert_eq!("some basic\nmultiline\nstring\n \t \"", node.get_key("str1").into_string());
}




#[test]
fn comments() {
    let input = r#"
    # comment
    # is discarded
    "#;
    let node: NodeRef = parse_node!(input);

    let expected = NodeRef::object(LinkedHashMap::new());

    assert!(node.is_identical_deep(&expected));
}
