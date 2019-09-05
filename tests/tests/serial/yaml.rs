use crate::serial::yaml::YamlParseErrDetail;
use crate::serial::yaml::YamlParser as Parser;
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
