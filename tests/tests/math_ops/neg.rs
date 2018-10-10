use super::*;

pub static NEG_OPERATOR: &str = "!";

fn assert_negation(value: &str, expected: bool) {
    let results = query(&format!("{}{}", NEG_OPERATOR, value), EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.is_boolean());
    assert_eq!(res.as_boolean(), expected);
}

#[test]
fn bool() {
    assert_negation("true", false);
}

#[test]
fn null() {
    assert_negation("null", true)
}

#[test]
fn integer_zero() {
    assert_negation("0", true);
}

#[test]
fn integer_non_zero() {
    assert_negation("1", false);
}

#[test]
fn float_zero() {
    assert_negation("0.0", true);
}

#[test]
fn float_non_zero() {
    assert_negation("0.1", false);
}

#[test]
fn empty_string() {
    assert_negation("''", true);
}

#[test]
fn non_empty_string() {
    assert_negation("'aaa'", false);
}

#[test]
fn empty_array() {
    assert_negation("@.empty_array", false);
}

#[test]
fn non_empty_array() {
    assert_negation("@.array", false);
}

#[test]
fn object() {
    assert_negation("@.empty_object", false);
}

#[test]
fn undefined() {
    assert_negation("undefined", true);
}

