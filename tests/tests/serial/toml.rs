use kg_diag::{MemCharReader, Span, Diag};
use crate::serial::TomlParser as Parser;
use crate::serial::TomlParseErrDetail;
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

macro_rules! parse_node_err {
            ($input: expr) => {
                {
                    let mut r = kg_diag::MemCharReader::new($input.as_bytes());
                    let mut parser = crate::serial::TomlParser::new();
                    let err = parser.parse(&mut r).expect_err("Error expected");
                    err
                }
            }
}

use kg_diag::ParseDiag;
macro_rules! assert_err {
            ($err: expr, $variant: pat) => {
               let detail = $err.detail().downcast_ref::<TomlParseErrDetail>()
                    .expect("cannot downcast to TomlParseErrorDetail");

                match detail {
                    $variant => {}
                    err => {
                        panic!("Expected error {} got {:?}", stringify!($variant), err)
                    }
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

    assert_eq!(99, node.get_key("int1").as_int_ext());
    assert_eq!(42, node.get_key("int2").as_int_ext());
    assert_eq!(0, node.get_key("int3").as_int_ext());
    assert_eq!(-17, node.get_key("int4").as_int_ext());
}

#[test]
fn integer_underscore() {
    let input = r#"
        int1 = 1_000
        int2 = 5_349_221
        int3 = 1_2_3_4_5
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(1_000, node.get_key("int1").as_int_ext());
    assert_eq!(5_349_221, node.get_key("int2").as_int_ext());
    assert_eq!(1_2_3_4_5, node.get_key("int3").as_int_ext());
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

    assert_eq!(0xdeadbeef, node.get_key("hex1").as_int_ext());
    assert_eq!(0xdeadbeef, node.get_key("hex2").as_int_ext());
    assert_eq!(0xdead_beef, node.get_key("hex3").as_int_ext());

    assert_eq!(0o01234567, node.get_key("oct1").as_int_ext());
    assert_eq!(0o755, node.get_key("oct2").as_int_ext());

    assert_eq!(0b11010110, node.get_key("bin1").as_int_ext());
    assert_eq!(0b11010110, node.get_key("bin2").as_int_ext());
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

    assert_eq!(1.0, node.get_key("flt1").as_float_ext());
    assert_eq!(3.1415, node.get_key("flt2").as_float_ext());
    assert_eq!(-0.01, node.get_key("flt3").as_float_ext());

    assert_eq!(5e+22, node.get_key("flt4").as_float_ext());
    assert_eq!(1e6, node.get_key("flt5").as_float_ext());
    assert_eq!(-2E-2, node.get_key("flt6").as_float_ext());

    assert_eq!(6.626e-34, node.get_key("flt7").as_float_ext());
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

    assert_eq!(1.0, node.get_key("flt1").as_float_ext());
    assert_eq!(3.1415, node.get_key("flt2").as_float_ext());
    assert_eq!(-0.01, node.get_key("flt3").as_float_ext());

    assert_eq!(5e+22, node.get_key("flt4").as_float_ext());
    assert_eq!(1e6, node.get_key("flt5").as_float_ext());
    assert_eq!(-2E-2, node.get_key("flt6").as_float_ext());

    assert_eq!(66.626e-34, node.get_key("flt7").as_float_ext());
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

    assert_eq!(std::f64::INFINITY, node.get_key("sf1").as_float_ext());
    assert_eq!(std::f64::INFINITY, node.get_key("sf2").as_float_ext());
    assert_eq!(std::f64::NEG_INFINITY, node.get_key("sf3").as_float_ext());

    assert!(node.get_key("sf4").as_float_ext().is_nan());
    assert!(node.get_key("sf5").as_float_ext().is_nan());
    assert!(node.get_key("sf6").as_float_ext().is_nan());
}

#[test]
fn booleans() {
    let input = r#"
        bool1 = true
        bool2 = false
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(true, node.get_key("bool1").as_bool_ext());
    assert_eq!(false, node.get_key("bool2").as_bool_ext());
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

    assert_eq!("some basic\nmultiline\nstring\n \t \"", node.get_key("str1").as_string_ext());
}

#[test]
fn bare_keys() {
    let input = r#"
        key = "value1"
        bare_key = "value2"
        bare-key = "value3"
        1234 = "value4"
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("value1", node.get_key("key").as_string_ext());
    assert_eq!("value2", node.get_key("bare_key").as_string_ext());
    assert_eq!("value3", node.get_key("bare-key").as_string_ext());
    assert_eq!("value4", node.get_key("1234").as_string_ext());
}

#[test]
fn quoted_keys() {
    let input = r#"
        "127.0.0.1" = "value1"
        "character encoding" = "value2"
        "ʎǝʞ" = "value3"
        'key2' = "value4"
        'quoted "value"' = "value5"
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("value1", node.get_key("127.0.0.1").as_string_ext());
    assert_eq!("value2", node.get_key("character encoding").as_string_ext());
    assert_eq!("value3", node.get_key("ʎǝʞ").as_string_ext());
    assert_eq!("value4", node.get_key("key2").as_string_ext());
    assert_eq!("value5", node.get_key("quoted \"value\"").as_string_ext());
}

#[test]
fn dotted_keys_bare() {
    let input = r#"
        physical.color = "orange"
        physical.shape = "round"
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("orange", node.get_key("physical").get_key("color").as_string_ext());
    assert_eq!("round", node.get_key("physical").get_key("shape").as_string_ext());
}

#[test]
fn dotted_keys() {
    let input = r#"
        name = "Orange"
        physical.color = "orange"
        physical.shape = "round"
        site."google.com" = true
        "quoted part".value = true
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("Orange", node.get_key("name").as_string_ext());
    assert_eq!("orange", node.get_key("physical").get_key("color").as_string_ext());
    assert_eq!("round", node.get_key("physical").get_key("shape").as_string_ext());
    assert_eq!(true, node.get_key("site").get_key("google.com").as_bool_ext());
    assert_eq!(true, node.get_key("quoted part").get_key("value").as_bool_ext());
}

#[test]
fn dotted_keys_nested() {
    let input = r#"
        a.b.c = 1
        a.d = 2
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(1, node.get_key("a").get_key("b").get_key("c").as_int_ext());
    assert_eq!(2, node.get_key("a").get_key("d").as_int_ext());
}

#[test]
fn redefined_key() {
    let input = r#"
            name = "Tom"
            name = "Pradyun"
            "#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::RedefinedKey {..});
}

#[test]
fn redefined_key_nested() {
    let input = r#"
        a.b = 1
        a.b.c = 2
    "#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::RedefinedKey {..});
}

#[test]
fn redefined_key_nested_2() {
    let input = r#"
        a.b.c = 4
        a.b.c = 2
    "#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::RedefinedKey {..});
}

#[test]
fn arrays() {
    let input = r#"
        arr1 = [ 1, 2, 3 ]
        arr2 = [ "red", "yellow", "green" ]
        arr3 = [ "all", 'strings', """are the same""", '''type''']
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(1, node.get_key("arr1").as_array_ext()[0].as_int_ext());
    assert_eq!(2, node.get_key("arr1").as_array_ext()[1].as_int_ext());
    assert_eq!(3, node.get_key("arr1").as_array_ext()[2].as_int_ext());

    assert_eq!("red", node.get_key("arr2").as_array_ext()[0].as_string_ext());
    assert_eq!("yellow", node.get_key("arr2").as_array_ext()[1].as_string_ext());
    assert_eq!("green", node.get_key("arr2").as_array_ext()[2].as_string_ext());

    assert_eq!("all", node.get_key("arr3").as_array_ext()[0].as_string_ext());
    assert_eq!("strings", node.get_key("arr3").as_array_ext()[1].as_string_ext());
    assert_eq!("are the same", node.get_key("arr3").as_array_ext()[2].as_string_ext());
    assert_eq!("type", node.get_key("arr3").as_array_ext()[3].as_string_ext());

}

#[test]
fn arrays_of_arrays() {
    let input = r#"
        arr1 = [ [ 1, 2 ], [3, 4, 5] ]
        arr2 = [ [ 1, 2 ], ["a", "b", "c"] ]
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(1, node.get_key("arr1").as_array_ext()[0].as_array_ext()[0].as_int_ext());
    assert_eq!(2, node.get_key("arr1").as_array_ext()[0].as_array_ext()[1].as_int_ext());
    assert_eq!(3, node.get_key("arr1").as_array_ext()[1].as_array_ext()[0].as_int_ext());
    assert_eq!(4, node.get_key("arr1").as_array_ext()[1].as_array_ext()[1].as_int_ext());
    assert_eq!(5, node.get_key("arr1").as_array_ext()[1].as_array_ext()[2].as_int_ext());

    assert_eq!(1, node.get_key("arr2").as_array_ext()[0].as_array_ext()[0].as_int_ext());
    assert_eq!(2, node.get_key("arr2").as_array_ext()[0].as_array_ext()[1].as_int_ext());
    assert_eq!("a", node.get_key("arr2").as_array_ext()[1].as_array_ext()[0].as_string_ext());
    assert_eq!("b", node.get_key("arr2").as_array_ext()[1].as_array_ext()[1].as_string_ext());
    assert_eq!("c", node.get_key("arr2").as_array_ext()[1].as_array_ext()[2].as_string_ext());
}

#[test]
fn array_mixed_types() {
    let input = r#"
        arr1 = [ 1, 2.0 ]
    "#;

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::MixedArrayType {..});
}

#[test]
fn array_mixed_types_2() {
    let input = r#"
        arr1 = [ 1, "string" ]
    "#;

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::MixedArrayType {..});
}

#[test]
fn empty_table() {
    let input = r#"
        [table]
    "#;
    let node: NodeRef = parse_node!(input);
    assert!(node.get_key("table").is_object())
}

#[test]
fn single_table() {
    let input = r#"
        [table-1]
        key1 = "some string"
        key2 = 123
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("some string", node.get_key("table-1").get_key("key1").as_string_ext());
    assert_eq!(123, node.get_key("table-1").get_key("key2").as_int_ext());
}

#[test]
fn multiple_tables() {
    let input = r#"
        [table-1]
        key1 = "some string"
        key2 = 123
        [table-2]
        key1 = "another string"
        key2 = 456
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("some string", node.get_key("table-1").get_key("key1").as_string_ext());
    assert_eq!(123, node.get_key("table-1").get_key("key2").as_int_ext());

    assert_eq!("another string", node.get_key("table-2").get_key("key1").as_string_ext());
    assert_eq!(456, node.get_key("table-2").get_key("key2").as_int_ext());
}


#[test]
fn table_dotted_key() {
    let input = r#"
        [dog."tater.man"]
        type.name = "pug"
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("pug", node.get_key("dog")
                                .get_key("tater.man")
                                .get_key("type")
                                .get_key("name").as_string_ext());
}

#[test]
fn redefined_table() {
    let input = r#"
        [a]
        b = 1

        [a]
        c = 2
    "#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::RedefinedKey {..});
}

#[test]
fn redefined_table_nested() {
    let input = r#"
        [a]
        b = 1

        [a.b]
        c = 2
    "#;
    let err: ParseDiag = parse_node_err!(input);

    println!("{}", err);

    // TODO fix quote
    panic!();
    assert_err!(err, TomlParseErrDetail::RedefinedKey {..});
}