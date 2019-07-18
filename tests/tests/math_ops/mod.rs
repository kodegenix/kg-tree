use super::*;

mod add;
mod div;
mod mul;
mod neg;
mod sub;

#[test]
fn negative() {
    let results = query("-(2+3)", EXAMPLE_JSON);

    let res = results.get(0).unwrap();

    assert!(res.data().is_float());
    assert_eq!(res.as_float(), -5.0);
}

#[test]
fn int_plus_many() {
    let json: &str = r#"
                    {
                        "one": 1,
                        "two" :2
                    }"#;

    let results = query("2 + @.(one, two)", json);

    assert_eq!(results.len(), 2);

    assert_eq!(results.get(0).unwrap().as_integer().unwrap(), 3);
    assert_eq!(results.get(1).unwrap().as_integer().unwrap(), 4);
}

#[test]
fn many_plus_int() {
    let json: &str = r#"
                    {
                        "one": 1,
                        "two" :2
                    }"#;

    let results = query("@.(one, two) + 2 ", json);

    assert_eq!(results.len(), 2);

    assert_eq!(results.get(0).unwrap().as_integer().unwrap(), 3);
    assert_eq!(results.get(1).unwrap().as_integer().unwrap(), 4);
}

#[test]
fn many_plus_many() {
    let json: &str = r#"
                    {
                        "one": 1,
                        "two":2,
                        "xx": "aa"
                    }"#;

    let results = query("@.(one, two) + @.(one, two, xx) ", json);

    assert_eq!(results.len(), 2);

    assert_eq!(results.get(0).unwrap().as_integer().unwrap(), 2);
    assert_eq!(results.get(1).unwrap().as_integer().unwrap(), 4);
}
