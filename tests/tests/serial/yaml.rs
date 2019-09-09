use crate::serial::yaml::YamlParseErrDetail;
use crate::serial::yaml::YamlParser as Parser;
use crate::tests::NodeRefExt;
use kg_tree::NodeRef;

macro_rules! assert_err {
    ($err: expr, $variant: pat) => {
        let detail = $err
            .detail()
            .downcast_ref::<YamlParseErrDetail>()
            .expect("cannot downcast to YamlParseErrDetail");

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
fn boolean_false() {
    let input = r#"false"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(false, node.as_bool_ext());
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
    let input = r#"string"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("string", node.as_string_ext());
}

#[test]
fn string_with_apostrophes() {
    let input = r#"'string with apostrophes'"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("string with apostrophes", node.as_string_ext());
}

#[test]
fn string_with_apostrophes_and_newline() {
    let input = r#"'string with apostrophes and
newline'"#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("string with apostrophes and\nnewline", node.as_string_ext());
}

#[test]
fn string_with_quotation_marks() {
    let input = r#""string with quotation marks""#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("string with quotation marks", node.as_string_ext());
}

#[test]
fn string_with_quotation_marks_and_newline() {
    let input = r#""string with quotation marks and
newline""#;
    let node: NodeRef = parse_node!(input);

    assert_eq!("string with quotation marks and\nnewline", node.as_string_ext());
}

#[test]
fn string_with_escape_codes() { //TODO MC Add handling \u1234 escapes
    let input = r#"" string \n \t""#;
    let node: NodeRef = parse_node!(input);

    assert_eq!(" string \n \t", node.as_string_ext());
}