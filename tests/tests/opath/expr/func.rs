use super::*;
use kg_tree::opath::NodeSet;

macro_rules! eval_opath {
    ($opath:expr) => {{
        let opath = match kg_tree::opath::Opath::parse($opath) {
            Ok(op) => op,
            Err(err) => {
                panic!("Error parsing opath!: {}", err)
            }
        };
        let root = NodeRef::object(kg_utils::collections::LinkedHashMap::new());

        match opath.apply(&root, &root) {
            Ok(res) => res,
            Err(err) => {
                panic!("Error evaluating opath expression!: {}", err)
            }
        }
    }};
}

macro_rules! assert_one {
    ($node_set:expr) => {{
        match $node_set {
            NodeSet::One(node) => node,
            got => panic!("Expected single node, got: {:?}", got)
        }
    }};
}


#[test]
fn array () {
    let opath = r#"array("hello", "world")"#;

    let res = eval_opath!(opath);

    let node = assert_one!(res);
    assert_eq!("hello", node.as_array_ext()[0].as_string_ext());
    assert_eq!("world", node.as_array_ext()[1].as_string_ext());
}