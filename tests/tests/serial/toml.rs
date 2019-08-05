use crate::serial::TomlParseErrDetail;
use crate::tests::serial::NodeRefExt;
use kg_diag::Diag;
use kg_tree::NodeRef;

macro_rules! parse_node {
    ($input: expr) => {{
        let mut r = kg_diag::MemCharReader::new($input.as_bytes());
        let mut parser = crate::serial::TomlParser::new();
        parser.parse(&mut r).unwrap_or_else(|err| {
            eprintln!("{}", err);
            panic!("Error parsing node!")
        })
    }};
}

macro_rules! parse_node_err {
    ($input: expr) => {{
        let mut r = kg_diag::MemCharReader::new($input.as_bytes());
        let mut parser = crate::serial::TomlParser::new();
        let err = parser
            .parse(&mut r)
            .map(|node| panic!("Error expected! got node: {}", node.to_json_pretty()))
            .unwrap_err();
        err
    }};
}

use kg_diag::ParseDiag;
macro_rules! assert_err {
    ($err: expr, $variant: pat) => {
        let detail = $err
            .detail()
            .downcast_ref::<TomlParseErrDetail>()
            .expect("cannot downcast to TomlParseErrorDetail");

        match detail {
            $variant => {}
            err => panic!("Expected error {} got {:?}", stringify!($variant), err),
        }
    };
}

#[test]
fn invalid_char() {
    let input = "key=\"value\"\n&key2=\"value2\"";

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::InvalidChar {..});
}

#[test]
fn multiple_kv_without_newline() {
    let input = r#"
        first = "Tom" last = "Preston-Werner"
    "#;

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::UnexpectedTokenMany {..});
}

#[test]
fn integers() {
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
fn integers_underscore() {
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
fn integers_prefix() {
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
fn integers_underscore_after_prefix() {
    let input = r#"
        hex1 = 0x_DEADBEEF
    "#;

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::InvalidChar {..});
}

#[test]
fn integers_invalid_integer() {
    let input = r#"
        num = 99999999999999999999999999999999999999999
    "#;

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::InvalidIntegerLiteral {..});
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
fn scientific_notation_invalid_char() {
    let input = r#"
        num = 1.2e-s
    "#;

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::InvalidCharMany {..});
}

#[test]
fn scientific_notation_invalid_char_2() {
    let input = r#"
        num = 1ee23
    "#;

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::InvalidCharMany {..});
}

#[test]
fn comment_invalid_eol() {
    let input = "num = 1 #comment\raf";

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::InvalidChar {..});
}

#[test]
fn comments() {
    let input = r#"
        # This is a full-line comment
        key = "value"  # This is a comment at the end of a line
        another = '# This is not a comment'
        [table] # comment
        #comment 2
        [[array]]# comment3
        # comment 4
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("value", node.get_key("key").as_string_ext());
    assert_eq!(
        "# This is not a comment",
        node.get_key("another").as_string_ext()
    );

    assert!(node.get_key("table").is_empty_ext());
    assert!(node.get_key("array").as_array_ext()[0].is_empty_ext());
}

#[test]
fn parse_float_err() {
    let input = r#"
        num = -e1
    "#;

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::InvalidFloatLiteral {..});
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
fn literal_strings() {
    let input = r#"
        winpath  = 'C:\Users\nodejs\templates'
        winpath2 = '\\ServerX\admin$\system32\'
        quoted   = 'Tom "Dubs" Preston-Werner'
        regex    = '<\i\c*\s*>'
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(
        "C:\\Users\\nodejs\\templates",
        node.get_key("winpath").as_string_ext()
    );
    assert_eq!(
        "\\\\ServerX\\admin$\\system32\\",
        node.get_key("winpath2").as_string_ext()
    );
    assert_eq!(
        "Tom \"Dubs\" Preston-Werner",
        node.get_key("quoted").as_string_ext()
    );
    assert_eq!("<\\i\\c*\\s*>", node.get_key("regex").as_string_ext());
}

#[test]
fn literal_string2() {
    let input = r#"
        str1 = ' literal string \n \t \u1234'
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(
        " literal string \\n \\t \\u1234",
        node.get_key("str1").as_string_ext()
    );
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

    assert_eq!(
        "multiline\nliteral string\n",
        node.get_key("str1").as_string_ext()
    );
}

#[test]
fn basic_string() {
    let input = r#"
        str1 = "some basic string\n \t \" '"
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(
        "some basic string\n \t \" '",
        node.get_key("str1").as_string_ext()
    );
}

#[test]
fn basic_multiline_string() {
    let input = "str1 = \"\"\"\nsome basic\nmultiline\nstring\\n \\t \\\"\"\"\"";

    let node: NodeRef = parse_node!(input);

    assert_eq!(
        "some basic\nmultiline\nstring\n \t \"",
        node.get_key("str1").as_string_ext()
    );
}

#[test]
fn basic_multiline_invalid_open() {
    let input = r#"
    val = ""aaa"""
    "#;

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::InvalidCharOne {..});
}

#[test]
fn basic_multiline_string_eoi() {
    let input = r#"
    val = """aaa""
    "#;

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::UnexpectedEoiOne {..});
}

#[test]
fn literal_multiline_invalid_open() {
    let input = r#"
    val = ''aaa'''
    "#;

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::InvalidCharOne {..});
}

#[test]
fn literal_multiline_string_eoi() {
    let input = r#"
    val = '''
    aaa''
    "#;

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::UnexpectedEoiOne {..});
}

#[test]
fn basic_multiline_string_first_crlf() {
    let input = "str1 = \"\"\"\r\nsecond line\"\"\"";

    let node: NodeRef = parse_node!(input);

    assert_eq!("second line", node.get_key("str1").as_string_ext());
}

#[test]
fn basic_string_utf8() {
    let input = r#"
        str1 = "✅ ❄ ❤ 💖"
    "#;

    let node: NodeRef = parse_node!(input);

    assert_eq!("✅ ❄ ❤ 💖", node.get_key("str1").as_string_ext());
}

#[test]
fn basic_string_escapes() {
    let input = r#"
        str1 = "\b \t \n \f \r \" \\"
    "#;

    let node: NodeRef = parse_node!(input);

    assert_eq!(
        "\u{0008} \t \n \u{000c} \r \" \\",
        node.get_key("str1").as_string_ext()
    );
}

#[test]
fn custom_escapes() {
    let input = r#"
        escape1 = "\u0022"
    "#;

    let node: NodeRef = parse_node!(input);

    assert_eq!("\"", node.get_key("escape1").as_string_ext());
}

#[test]
fn bad_custom_escape() {
    let input = r#"
        escape1 = "\uD800"
    "#;

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::InvalidEscape {..});
}

#[test]
fn too_short_custom_escape() {
    let input = r#"
        escape1 = "\u002"
    "#;

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::InvalidEscape {..});
}

#[test]
fn bad_escape() {
    let input = r#"
        str1 = "\h"
    "#;

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::InvalidEscape {..});
}

#[test]
fn basic_string_unexpected_eol() {
    let input = "key=\"val\nue\"";

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::UnexpectedEol {..});
}

#[test]
fn line_ending_backslash() {
    let input = r#"
        str1 = """
The quick brown \


  fox jumps over \
    the lazy dog."""

str2 = """\
       The quick brown \
       fox jumps over \
       the lazy dog.\
       """
    "#;
    let phrase = "The quick brown fox jumps over the lazy dog.";

    let node: NodeRef = parse_node!(input);

    assert_eq!(phrase, node.get_key("str1").as_string_ext());
    assert_eq!(phrase, node.get_key("str2").as_string_ext());
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
fn empty_bare_key() {
    let input = r#"
        = "no key name"
            "#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::UnexpectedTokenMany {..});
}

#[test]
fn empty_quoted_key() {
    let input = r#"
        '' = "no key name"
            "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("no key name", node.get_key("").as_string_ext());
}

#[test]
fn empty_d_quoted_key() {
    let input = r#"
        "" = "no key name"
            "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("no key name", node.get_key("").as_string_ext());
}

#[test]
fn key_starts_with_boolean() {
    let input = r#"
        true1a = "starts with bool"
        true_1a = "starts with bool"
        true-1a = "starts with bool"

        false1a = "starts with bool"
        false_1a = "starts with bool"
        false-1a = "starts with bool"
            "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("starts with bool", node.get_key("true1a").as_string_ext());
    assert_eq!("starts with bool", node.get_key("true_1a").as_string_ext());
    assert_eq!("starts with bool", node.get_key("true-1a").as_string_ext());

    assert_eq!("starts with bool", node.get_key("false1a").as_string_ext());
    assert_eq!("starts with bool", node.get_key("false_1a").as_string_ext());
    assert_eq!("starts with bool", node.get_key("false-1a").as_string_ext());
}

#[test]
fn key_starts_with_special_float() {
    let input = r#"
        nan1a = "starts with nan"
        nan_1a = "starts with nan"
        nan-1a = "starts with nan"

        inf1a = "starts with inf"
        inf_1a = "starts with inf"
        inf-1a = "starts with inf"
            "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("starts with nan", node.get_key("nan1a").as_string_ext());
    assert_eq!("starts with nan", node.get_key("nan_1a").as_string_ext());
    assert_eq!("starts with nan", node.get_key("nan-1a").as_string_ext());

    assert_eq!("starts with inf", node.get_key("inf1a").as_string_ext());
    assert_eq!("starts with inf", node.get_key("inf_1a").as_string_ext());
    assert_eq!("starts with inf", node.get_key("inf-1a").as_string_ext());
}

#[test]
fn no_equals_after_key() {
    let input = r#"
        some.key no_equals
            "#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::UnexpectedTokenOne {..});
}

#[test]
fn unexpected_key_value() {
    let input = r#"
        key= # comment"
            "#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::UnexpectedTokenMany {..});
}

#[test]
fn multiline_string_as_key() {
    let input = r#"
"""mutliline
key""" = "value"
            "#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::MultilineKey);
}

#[test]
fn keys_unexpected_token() {
    let input = r#"
        dotted.key.=unexpected = "value1"
            "#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::UnexpectedTokenMany {..});
}

#[test]
fn quoted_keys() {
    let input = r#"
        "127.0.0.1" = "value1"
        "character encoding" = "value2"
        "ʎǝʞ" = "value3"
        'key2ʞ' = "value4"
        'quoted "value"' = "value5"
        "💖" = "heart"
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("value1", node.get_key("127.0.0.1").as_string_ext());
    assert_eq!("value2", node.get_key("character encoding").as_string_ext());
    assert_eq!("value3", node.get_key("ʎǝʞ").as_string_ext());
    assert_eq!("value4", node.get_key("key2ʞ").as_string_ext());
    assert_eq!("value5", node.get_key("quoted \"value\"").as_string_ext());
    assert_eq!("heart", node.get_key("💖").as_string_ext());
}

#[test]
fn dotted_keys_bare() {
    let input = r#"
        physical.color = "orange"
        physical.shape = "round"
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(
        "orange",
        node.get_key("physical").get_key("color").as_string_ext()
    );
    assert_eq!(
        "round",
        node.get_key("physical").get_key("shape").as_string_ext()
    );
}

#[test]
fn ignore_keys_whitespaces() {
    let input = r#"
        [ j . "ʞ" . 'l' ]
    "#;
    let node: NodeRef = parse_node!(input);

    assert!(node.get_key("j").get_key("ʞ").get_key("l").is_empty_ext())
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
    assert_eq!(
        "orange",
        node.get_key("physical").get_key("color").as_string_ext()
    );
    assert_eq!(
        "round",
        node.get_key("physical").get_key("shape").as_string_ext()
    );
    assert_eq!(
        true,
        node.get_key("site").get_key("google.com").as_bool_ext()
    );
    assert_eq!(
        true,
        node.get_key("quoted part").get_key("value").as_bool_ext()
    );
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
fn redefined_key_nested_inline_tables() {
    let input = r#"
        a.b.c = {val = 3}
        a.b.c = {val = 2}
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

    assert_eq!(
        "all",
        node.get_key("arr3").as_array_ext()[0].as_string_ext()
    );
    assert_eq!(
        "strings",
        node.get_key("arr3").as_array_ext()[1].as_string_ext()
    );
    assert_eq!(
        "are the same",
        node.get_key("arr3").as_array_ext()[2].as_string_ext()
    );
    assert_eq!(
        "type",
        node.get_key("arr3").as_array_ext()[3].as_string_ext()
    );
}

#[test]
fn arrays_of_arrays() {
    let input = r#"
        arr1 = [ [ 1, 2 ], [3, 4, 5] ]
        arr2 = [ [ 1, 2 ], ["a", "b", "c"] ]
    "#;
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

    assert_eq!(
        1,
        node.get_key("arr2").as_array_ext()[0].as_array_ext()[0].as_int_ext()
    );
    assert_eq!(
        2,
        node.get_key("arr2").as_array_ext()[0].as_array_ext()[1].as_int_ext()
    );
    assert_eq!(
        "a",
        node.get_key("arr2").as_array_ext()[1].as_array_ext()[0].as_string_ext()
    );
    assert_eq!(
        "b",
        node.get_key("arr2").as_array_ext()[1].as_array_ext()[1].as_string_ext()
    );
    assert_eq!(
        "c",
        node.get_key("arr2").as_array_ext()[1].as_array_ext()[2].as_string_ext()
    );
}

macro_rules! test_mixed(
    ($name:ident, $input:expr) => (
        #[test]
        fn $name() {
            let err: ParseDiag = parse_node_err!($input);
            assert_err!(err, TomlParseErrDetail::MixedArrayType {..});
        }
    )
);
test_mixed! {mixed_array_types_bool, "arr1 = [ true, 2.0 ]"}
test_mixed! {mixed_array_types_int, "arr1 = [ 1, 2.0 ]"}
test_mixed! {mixed_array_types_string, "arr1 = [ \"1\", 2.0 ]"}
test_mixed! {mixed_array_types_float, "arr1 = [ 1.0, 2 ]"}
test_mixed! {mixed_array_types_array, "arr1 = [ [1], 2 ]"}
test_mixed! {mixed_array_types_table, "arr1 = [ {k=\"v\"}, 2 ]"}

#[test]
fn array_unexpected_token() {
    let input = r#"
        arr1 = [ 1 = ]
    "#;

    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::UnexpectedToken {..});
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
fn array_newline() {
    let input = r#"
        arr1 = [
            1,
         2,
         3 ]
    "#;
    let node: NodeRef = parse_node!(input);
    assert_eq!(1, node.get_key("arr1").as_array_ext()[0].as_int_ext());
    assert_eq!(2, node.get_key("arr1").as_array_ext()[1].as_int_ext());
    assert_eq!(3, node.get_key("arr1").as_array_ext()[2].as_int_ext());
}

#[test]
fn empty_table() {
    let input = r#"
        [table]
    "#;
    let node: NodeRef = parse_node!(input);
    assert!(node.get_key("table").is_empty_ext())
}

#[test]
fn single_table() {
    let input = r#"
        [table-1]
        key1 = "some string"
        key2 = 123
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(
        "some string",
        node.get_key("table-1").get_key("key1").as_string_ext()
    );
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

    assert_eq!(
        "some string",
        node.get_key("table-1").get_key("key1").as_string_ext()
    );
    assert_eq!(123, node.get_key("table-1").get_key("key2").as_int_ext());

    assert_eq!(
        "another string",
        node.get_key("table-2").get_key("key1").as_string_ext()
    );
    assert_eq!(456, node.get_key("table-2").get_key("key2").as_int_ext());
}

#[test]
fn table_dotted_key() {
    let input = r#"
        [dog."tater.man"]
        type.name = "pug"
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(
        "pug",
        node.get_key("dog")
            .get_key("tater.man")
            .get_key("type")
            .get_key("name")
            .as_string_ext()
    );
}

#[test]
fn define_super_table_afterwards() {
    let input = r#"
        [x.y.z.w]

        [x]
        key = "val"

        [x.y]
        key2 = "val2"
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("val", node.get_key("x").get_key("key").as_string_ext());

    assert_eq!(
        "val2",
        node.get_key("x")
            .get_key("y")
            .get_key("key2")
            .as_string_ext()
    );

    assert!(node
        .get_key("x")
        .get_key("y")
        .get_key("z")
        .get_key("w")
        .is_empty_ext());
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

    assert_err!(err, TomlParseErrDetail::RedefinedKey {..});
}

#[test]
fn inline_tables() {
    let input = r#"
        name = { first = "Tom", last = "Preston-Werner" }
        point = { x = 1, y = 2 }
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("Tom", node.get_key("name").get_key("first").as_string_ext());

    assert_eq!(
        "Preston-Werner",
        node.get_key("name").get_key("last").as_string_ext()
    );

    assert_eq!(1, node.get_key("point").get_key("x").as_int_ext());
    assert_eq!(2, node.get_key("point").get_key("y").as_int_ext());
}

#[test]
fn inline_tables_dotted_key() {
    let input = r#"
        animal = { type.name = "pug" }
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(
        "pug",
        node.get_key("animal")
            .get_key("type")
            .get_key("name")
            .as_string_ext()
    );
}

#[test]
fn inline_tables_nested() {
    let input = r#"
        table = { nested = {key = "value"} }
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(
        "value",
        node.get_key("table")
            .get_key("nested")
            .get_key("key")
            .as_string_ext()
    );
}

#[test]
fn array_of_inline_tables() {
    let input = r#"
        points = [ { x = 1, y = 2},
                   { x = 7, y = 8} ]
    "#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(
        1,
        node.get_key("points").as_array_ext()[0]
            .get_key("x")
            .as_int_ext()
    );
    assert_eq!(
        2,
        node.get_key("points").as_array_ext()[0]
            .get_key("y")
            .as_int_ext()
    );

    assert_eq!(
        7,
        node.get_key("points").as_array_ext()[1]
            .get_key("x")
            .as_int_ext()
    );
    assert_eq!(
        8,
        node.get_key("points").as_array_ext()[1]
            .get_key("y")
            .as_int_ext()
    );
}

#[test]
fn inline_table_newline() {
    let input = r#"
table = { key = "value",
key2="value2}
    "#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::UnexpectedTokenMany {..});
}

#[test]
fn inline_table_unexpected_token() {
    let input = r#"
table = { key = "value" =}
    "#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::UnexpectedTokenMany {..});
}

#[test]
fn array_of_tables() {
    let input = r#"
        [[products]]
        name = "Hammer"
        sku = 738594937

        [[products]]

        [[products]]
        name = "Nail"
        sku = 284758393
        color = "gray"
    "#;
    let node: NodeRef = parse_node!(input);

    let products = node.get_key("products").as_array_ext();

    assert_eq!("Hammer", products[0].get_key("name").as_string_ext());
    assert_eq!(738594937, products[0].get_key("sku").as_int_ext());

    assert!(products[1].is_empty_ext());

    assert_eq!("Nail", products[2].get_key("name").as_string_ext());
    assert_eq!(284758393, products[2].get_key("sku").as_int_ext());
    assert_eq!("gray", products[2].get_key("color").as_string_ext());
}

#[test]
fn nested_array_of_tables() {
    let input = r#"
        [[fruit]]
          name = "apple"

          [fruit.physical]
            color = "red"
            shape = "round"

          [[fruit.variety]]
            name = "red delicious"

          [[fruit.variety]]
            name = "granny smith"

        [[fruit]]
          name = "banana"

          [[fruit.variety]]
            name = "plantain"
    "#;
    let node: NodeRef = parse_node!(input);

    let fruit = node.get_key("fruit").as_array_ext();

    assert_eq!("apple", fruit[0].get_key("name").as_string_ext());
    assert_eq!(
        "red",
        fruit[0]
            .get_key("physical")
            .get_key("color")
            .as_string_ext()
    );
    assert_eq!(
        "round",
        fruit[0]
            .get_key("physical")
            .get_key("shape")
            .as_string_ext()
    );
    assert_eq!(
        "red delicious",
        fruit[0].get_key("variety").as_array_ext()[0]
            .get_key("name")
            .as_string_ext()
    );
    assert_eq!(
        "granny smith",
        fruit[0].get_key("variety").as_array_ext()[1]
            .get_key("name")
            .as_string_ext()
    );

    assert_eq!("banana", fruit[1].get_key("name").as_string_ext());
    assert_eq!(
        "plantain",
        fruit[1].get_key("variety").as_array_ext()[0]
            .get_key("name")
            .as_string_ext()
    );
}

#[test]
fn multiple_array_of_tables() {
    let input = r#"
        [[fruit]]
          name = "apple"
        [[fruit]]
          name = "banana"
        [[animal]]
          name = "bob"
        [[animal]]
          name = "spike"
    "#;
    let node: NodeRef = parse_node!(input);

    let fruit = node.get_key("fruit").as_array_ext();
    assert_eq!("apple", fruit[0].get_key("name").as_string_ext());
    assert_eq!("banana", fruit[1].get_key("name").as_string_ext());

    let animal = node.get_key("animal").as_array_ext();
    assert_eq!("bob", animal[0].get_key("name").as_string_ext());
    assert_eq!("spike", animal[1].get_key("name").as_string_ext());
}

#[test]
fn redefined_table_as_array() {
    let input = r#"
    [[fruit]]
      name = "apple"

      [fruit.variety]
        name = "granny smith"

      [[fruit.variety]]
        name = "red delicious"
    "#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::RedefinedKey {..});
}

#[test]
fn redefined_array_as_table() {
    let input = r#"
    [[fruit]]
      name = "apple"

      [[fruit.variety]]
        name = "red delicious"

      [fruit.variety]
        name = "granny smith"
    "#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::RedefinedKey {..});
}

#[test]
fn redefined_static_array_as_array_of_tables() {
    let input = r#"
        fruit = []

        [[fruit]]
    "#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::RedefinedKey {..});
}

#[test]
fn redefined_static_array_as_array_of_tables_nested() {
    let input = r#"
        fruit.desc = ["aa"]

        [[fruit.desc.val]]
    "#;
    let err: ParseDiag = parse_node_err!(input);

    assert_err!(err, TomlParseErrDetail::RedefinedKey {..});
}
