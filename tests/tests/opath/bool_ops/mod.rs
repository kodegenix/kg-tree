use super::*;

mod comparison;
mod logical;

#[test]
fn true_and_many() {
    let json: &str = r#"
                    {
                        "tr": true,
                        "fa": false
                    }"#;

    let results = query("true and @.(tr, fa)", json);

    assert_eq!(results.len(), 2);

    assert_eq!(results.get(0).unwrap().as_boolean(), true);
    assert_eq!(results.get(1).unwrap().as_boolean(), false);
}

#[test]
fn many_and_true() {
    let json: &str = r#"
                    {
                        "tr": true,
                        "fa": false
                    }"#;

    let results = query("@.(tr, fa) and true", json);

    assert_eq!(results.len(), 2);

    assert_eq!(results.get(0).unwrap().as_boolean(), true);
    assert_eq!(results.get(1).unwrap().as_boolean(), false);
}

#[test]
fn many_and_many() {
    let json: &str = r#"
                    {
                        "tr": true,
                        "fa": false,
                        "xx": false
                    }"#;

    let results = query("@.(tr, fa, xx) and @.(tr, fa)", json);

    assert_eq!(results.len(), 2);

    assert_eq!(results.get(0).unwrap().as_boolean(), true);
    assert_eq!(results.get(1).unwrap().as_boolean(), false);
}
