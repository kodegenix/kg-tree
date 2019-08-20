use super::*;
use kg_diag::Diag;
use kg_tree::opath::FuncCallErrorDetail;
use kg_tree::opath::NodeSet;
macro_rules! eval_opath {
    ($opath:expr) => {{
        let opath = match kg_tree::opath::Opath::parse($opath) {
            Ok(op) => op,
            Err(err) => panic!("Error parsing opath!: {}", err),
        };
        let root = NodeRef::object(kg_utils::collections::LinkedHashMap::new());

        opath.apply(&root, &root)
    }};
}

macro_rules! assert_one {
    ($node_set:expr) => {{
        match $node_set {
            NodeSet::One(node) => node,
            got => panic!("Expected single node, got: {:?}", got),
        }
    }};
}

#[test]
fn array() {
    let opath = r#"array("hello", "world")"#;

    let res = eval_opath!(opath).unwrap();

    let node = assert_one!(res);
    assert_eq!("hello", node.as_array_ext()[0].as_string_ext());
    assert_eq!("world", node.as_array_ext()[1].as_string_ext());
}

#[test]
fn array_opath_err() {
    let opath = r#"array(array(nonExistingFunc()), "world")"#;

    let res = eval_opath!(opath);

    assert_detail!(
        res,
        FuncCallErrorDetail,
        FuncCallErrorDetail::UnknownFunc { name },
        assert_eq!("nonExistingFunc", name)
    );
}

#[test]
fn read_file_json() {
    let (_tmp, dir) = get_tmp_dir();
    set_base_path(&dir);

    write_file!(dir.join("example_file.json"), r#"{"key": "value"}"#);

    let opath = r#"readFile("example_file.json")"#;

    let res = eval_opath!(opath).unwrap();

    let node = assert_one!(res);
    assert_eq!("value", node.get_key("key").as_string_ext())
}

#[test]
fn read_file_malformed() {
    let (_tmp, dir) = get_tmp_dir();
    set_base_path(&dir);

    write_file!(dir.join("example_file.json"), r#"{"key": "value}"#);

    let opath = r#"readFile("example_file.json")"#;

    let res = eval_opath!(opath);

    let (err, _) =
        assert_detail!(res, FuncCallErrorDetail, FuncCallErrorDetail::FuncCallCustom{..});

    assert!(err.cause().is_some())
    // FIXME is this error message ok?

    //    println!("{}", err);
}
