use super::*;

#[test]
fn index() {
    let json: &str = r#"
                        {
                            "child0": {
                                "child0_0":"child0_0 string val",
                                "child0_1":"child0_1 string val",
                                "child0_2":"child9_2 string val"
                            },
                            "child1": {
                                "child1_0":"child1_0 string val"
                            }
                        }"#;


    let results = query("child0.child0_2.@index", json);

    let res = results.get(0).unwrap();
    assert!(res.is_integer());

    assert_eq!(res.as_integer().unwrap(), 2)
}

#[test]
fn array_index() {
    let json: &str = r#"[0,1,2]"#;

    let results = query("@[1].@index", json);

    let res = results.get(0).unwrap();
    assert!(res.is_integer());

    assert_eq!(res.as_integer().unwrap(), 1)
}


#[test]
fn key() {
    let json: &str = r#"
                        {
                            "child0": {
                                "child0_0":"child0_0 string val",
                                "child0_1":"child0_1 string val",
                                "child0_2":"child9_2 string val"
                            },
                            "child1": {
                                "child1_0":"child1_0 string val"
                            }
                        }"#;


    let results = query("child0.child0_2.@key", json);

    let res = results.get(0).unwrap();
    assert!(res.is_string());

    assert_eq!(res.as_string(), "child0_2")
}

#[test]
fn array_key() {
    let json: &str = r#"[0,1,2]"#;

    let results = query("@[1].@key", json);

    let res = results.get(0).unwrap();
    assert!(res.is_string());

    assert_eq!(res.as_string(), "1")
}

#[test]
fn level() {
    let json: &str = r#"
                        {
                            "child0": {
                                "child0_0":"child0_0 string val",
                                "child0_1":"child0_1 string val",
                                "child0_2":"child9_2 string val"
                            },
                            "child1": {
                                "child1_0":"child1_0 string val"
                            }
                        }"#;


    let results = query("child0.child0_0.@level", json);

    let res = results.get(0).unwrap();
    assert!(res.is_integer());

    assert_eq!(res.as_integer().unwrap(), 2)
}

#[test]
fn level_root() {
    let json: &str = r#"
                        {
                            "child0": {
                                "child0_0":"child0_0 string val",
                                "child0_1":"child0_1 string val",
                                "child0_2":"child9_2 string val"
                            },
                            "child1": {
                                "child1_0":"child1_0 string val"
                            }
                        }"#;


    let results = query("$.@level", json);

    let res = results.get(0).unwrap();

    assert!(res.is_integer());
    assert_eq!(res.as_integer().unwrap(), 0)
}

#[test]
fn type_null() {
    let results = query("null_value.@type", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_string());
    assert_eq!(res.as_string(), "null")
}

#[test]
fn kind_bool() {
    let json: &str = r#"{"bool_val":true}"#;

    let results = query("bool_val.@type", json);

    let res = results.get(0).unwrap();

    assert!(res.is_string());
    assert_eq!(res.as_string(), "boolean")
}

#[test]
fn type_number() {
    let results = query("one.@type", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_string());
    assert_eq!(res.as_string(), "number")
}

#[test]
fn type_string() {
    let results = query("nested.three_string.@type", EXAMPLE_JSON);

    let res = results.get(0).unwrap();
    assert!(res.is_string());

    assert_eq!(res.as_string(), "string")
}

#[test]
fn type_object() {
    let results = query("empty_object.@type", EXAMPLE_JSON);

    let res = results.get(0).unwrap();
    assert!(res.is_string());

    assert_eq!(res.as_string(), "object")
}

#[test]
fn type_array() {
    let results = query("array.@type", EXAMPLE_JSON);

    let res = results.get(0).unwrap();
    assert!(res.is_string());

    assert_eq!(res.as_string(), "array")
}
// TODO ws pozostałe atrybuty
