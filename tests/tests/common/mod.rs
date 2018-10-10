use kg_tree::*;

// Do not change this string!
pub static EXAMPLE_JSON: &str = r#"
                    {
                        "one": 1,
                        "empty_object" :{},
                        "empty_array":[],
                        "array":["a","b"],
                        "whitespace key": "value",
                        "null_value":null,
                        "nested": {
                            "two": 2,
                            "three_string": "3",
                            "four": 4
                        }
                    }"#;

pub fn query(query: &str, json: &str) -> Vec<NodeRef> {
    let opath = opath::Opath::parse(query).unwrap();
    let n = NodeRef::from_json(json).unwrap();
    opath.apply(&n, &n).into_vec()
}

pub fn assert_bool_op(query_str: &str, expected: bool) {
    let json = r#"{}"#;

    let results = query(query_str, json);
    let res = results.get(0).unwrap();

    assert!(res.is_boolean());
    assert_eq!(res.as_boolean(), expected);
}
