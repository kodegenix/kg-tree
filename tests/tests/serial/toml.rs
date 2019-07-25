use kg_diag::{MemCharReader, Span};
use crate::serial::TomlParser as Parser;
use kg_tree::NodeRef;
use kg_utils::collections::LinkedHashMap;
use kg_diag::io::fs::FileType::Link;
use crate::tests::serial::NodeRefExt;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = LinkedHashMap::new();
            $(
                m.insert($key.into(), $value);
            )+
            m
        }
     };
);

macro_rules! assert_node {
            ($input: expr, $expected: expr) => {
                let mut r = MemCharReader::new($input.as_bytes());
                let mut parser = Parser::new();

                for t in $expected {
                    let token = parser.lex(&mut r).expect("Cannot get token!");
                    assert_eq!(token.term(), t);
                }
            }
}

#[test]
fn integer() {
    let input= r#"
        int1 = +99
        int2 = 42
        int3 = 0
        int4 = -17
    "#;
    let mut r = MemCharReader::new(input.as_bytes());
    let mut parser = Parser::new();
    let node = parser.parse(&mut r).unwrap();

    assert_eq!(99, node.get_key("int1").to_int());
    assert_eq!(42, node.get_key("int2").to_int());
    assert_eq!(0, node.get_key("int3").to_int());
    assert_eq!(-17, node.get_key("int4").to_int());
}

#[test]
fn integer_underscore() {
    let input= r#"
        int1 = 1_000
        int2 = 5_349_221
        int3 = 1_2_3_4_5
    "#;
    let mut r = MemCharReader::new(input.as_bytes());
    let mut parser = Parser::new();
    let node = parser.parse(&mut r).unwrap();

    assert_eq!(1_000, node.get_key("int1").to_int());
    assert_eq!(5_349_221, node.get_key("int2").to_int());
    assert_eq!(1_2_3_4_5, node.get_key("int3").to_int());
}

#[test]
fn integer_prefix() {
    let input= r#"
        hex1 = 0xDEADBEEF
        hex2 = 0xdeadbeef
        hex3 = 0xdead_beef


        oct1 = 0o0123_4567
        oct2 = 0o755

        bin1 = 0b11010110
        bin2 = 0b1101_0110
    "#;
    let mut r = MemCharReader::new(input.as_bytes());
    let mut parser = Parser::new();
    let node = parser.parse(&mut r).unwrap();

    assert_eq!(0xdeadbeef, node.get_key("hex1").to_int());
    assert_eq!(0xdeadbeef, node.get_key("hex2").to_int());
    assert_eq!(0xdead_beef, node.get_key("hex3").to_int());

    assert_eq!(0o01234567, node.get_key("oct1").to_int());
    assert_eq!(0o755, node.get_key("oct2").to_int());

    assert_eq!(0b11010110, node.get_key("bin1").to_int());
    assert_eq!(0b11010110, node.get_key("bin2").to_int());
}

#[test]
fn floats() {
    let input= r#"
        flt1 = +1.0
        flt2 = 3.1415
        flt3 = -0.01

        flt4 = 5e+22
        flt5 = 1e6
        flt6 = -2E-2

        flt7 = 6.626e-34
    "#;
    let mut r = MemCharReader::new(input.as_bytes());
    let mut parser = Parser::new();
    let node = parser.parse(&mut r).unwrap_or_else(|err|{
        println!("{}", err); panic!()
    });

    assert_eq!(1.0, node.get_key("flt1").to_float());
    assert_eq!(3.1415, node.get_key("flt2").to_float());
    assert_eq!(-0.01, node.get_key("flt3").to_float());

    assert_eq!(5e+22, node.get_key("flt4").to_float());
    assert_eq!(1e6, node.get_key("flt5").to_float());
    assert_eq!(-2E-2, node.get_key("flt6").to_float());

    assert_eq!(6.626e-34, node.get_key("flt7").to_float());
}

#[test]
fn comments() {
    let input= r#"
    # comment
    # is discarded
    "#;
    let mut r = MemCharReader::new(input.as_bytes());
    let mut parser = Parser::new();

    let node = parser.parse(&mut r).unwrap();

    let expected  = NodeRef::object(LinkedHashMap::new());

    assert!(node.is_identical_deep(&expected));
}
