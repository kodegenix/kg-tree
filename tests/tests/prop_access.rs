use super::*;
use kg_tree::diff::Diff;

mod simple {
    use super::*;

    #[test]
    fn current() {
        let results = query("one", EXAMPLE_JSON);

        let res = results.get(0).unwrap();

        assert!(res.data().is_integer());
        assert_eq!(res.as_integer().unwrap(), 1);
    }

    #[test]
    fn current_at() {
        let results = query("@.one", EXAMPLE_JSON);

        let res = results.get(0).unwrap();

        assert!(res.data().is_integer());
        assert_eq!(res.as_integer().unwrap(), 1);
    }


    #[test]
    fn current_bracket_at() {
        let results = query("@[one]", EXAMPLE_JSON);

        let res = results.get(0).unwrap();

        assert!(res.data().is_integer());
        assert_eq!(res.as_integer().unwrap(), 1);
    }

    #[test]
    fn current_quot_at() {
        let results = query("@.'one'", EXAMPLE_JSON);

        let res = results.get(0).unwrap();

        assert!(res.data().is_integer());
        assert_eq!(res.as_integer().unwrap(), 1);
    }

    #[test]
    fn current_double_quot_at() {
        let results = query("@.\"one\"", EXAMPLE_JSON);

        let res = results.get(0).unwrap();

        assert!(res.data().is_integer());
        assert_eq!(res.as_integer().unwrap(), 1);
    }

    #[test]
    fn current_bracket_double_quot_at() {
        let results = query("@[\"one\"]", EXAMPLE_JSON);

        let res = results.get(0).unwrap();

        assert!(res.data().is_integer());
        assert_eq!(res.as_integer().unwrap(), 1);
    }

    #[test]
    fn current_prop_index() {
        let results = query("@[0]", EXAMPLE_JSON);

        let res = results.get(0).unwrap();

        assert!(res.data().is_integer());
        assert_eq!(res.as_integer().unwrap(), 1);
    }


    #[test]
    fn root() {
        let results = query("$.one", EXAMPLE_JSON);

        let res = results.get(0).unwrap();

        assert!(res.data().is_integer());
        assert_eq!(res.as_integer().unwrap(), 1);
    }


    #[test]
    fn root_bracket() {
        let results = query("$[one]", EXAMPLE_JSON);

        let res = results.get(0).unwrap();

        assert!(res.data().is_integer());
        assert_eq!(res.as_integer().unwrap(), 1);
    }

    #[test]
    fn root_quot() {
        let results = query("$.'one'", EXAMPLE_JSON);

        let res = results.get(0).unwrap();

        assert!(res.data().is_integer());
        assert_eq!(res.as_integer().unwrap(), 1);
    }

    #[test]
    fn root_quot_whitespace() {
        let results = query("$.'whitespace key'", EXAMPLE_JSON);

        let res = results.get(0).unwrap();

        assert!(res.data().is_string());
        assert_eq!(res.as_string(), String::from("value"));
    }

    #[test]
    fn root_double_quot() {
        let results = query("$.\"one\"", EXAMPLE_JSON);

        let res = results.get(0).unwrap();

        assert!(res.data().is_integer());
        assert_eq!(res.as_integer().unwrap(), 1);
    }

    #[test]
    fn root_bracket_double_quot() {
        let results = query("$[\"one\"]", EXAMPLE_JSON);

        let res = results.get(0).unwrap();

        assert!(res.data().is_integer());
        assert_eq!(res.as_integer().unwrap(), 1);
    }

    #[test]
    fn root_prop_index() {
        let results = query("$[0]", EXAMPLE_JSON);

        let res = results.get(0).unwrap();

        assert!(res.data().is_integer());
        assert_eq!(res.as_integer().unwrap(), 1);
    }

    #[test]
    fn root_multiple() {
        let results = query("$.(one, empty_array, nested)", EXAMPLE_JSON);

        assert_eq!(results.len(), 3);

        assert!(results.get(0).unwrap().is_number());

        assert!(results.get(2).unwrap().is_object());
    }

    #[test]
    fn current_multiple() {
        let results = query("(one, empty_array, nested)", EXAMPLE_JSON);

        assert_eq!(results.len(), 3);

        assert!(results.get(0).unwrap().is_number());

        assert!(results.get(2).unwrap().is_object());
    }

    #[test]
    fn current_multiple_at() {
        let results = query("@.(one, empty_array, nested)", EXAMPLE_JSON);

        assert_eq!(results.len(), 3);

        assert!(results.get(0).unwrap().is_number());

        assert!(results.get(2).unwrap().is_object());
    }

    #[test]
    fn array_multiple() {
        let json: &str = r#"["a","b","c"]"#;

        let results = query("@.(0, 1, 2)", json);

        assert_eq!(results.len(), 3);

        assert!(results.get(2).unwrap().is_string());
    }
}

mod wildcards {
    use super::*;

    #[test]
    fn dot_star() {
        let results = query("@.*", EXAMPLE_JSON);
        assert_eq!(results.len(), 7);
    }

    #[test]
    fn bracket_star() {
        let results = query("@[*]", EXAMPLE_JSON);
        assert_eq!(results.len(), 7);
    }

    #[test]
    fn dot_double_star() {
        let results = query("@.**", EXAMPLE_JSON);

        assert_eq!(results.len(), 12);

        //check depth-first
        let res = results.get(4).unwrap();
        assert!(res.is_string());
        assert_eq!(res.as_string(), String::from("a"));

        let res = results.get(11).unwrap();
        assert!(res.is_integer());
        assert_eq!(res.as_integer().unwrap(), 4)
    }

    #[test]
    fn dot_quot_double_star() {
        let results = query("@.'**'", EXAMPLE_JSON);

        assert_eq!(results.len(), 12);

        //check depth-first
        let res = results.get(4).unwrap();
        assert!(res.is_string());
        assert_eq!(res.as_string(), String::from("a"));

        let res = results.get(11).unwrap();
        assert!(res.is_integer());
        assert_eq!(res.as_integer().unwrap(), 4)
    }

    #[test]
    fn bracket_double_star_at() {
        let results = query("@[**]", EXAMPLE_JSON);

        assert_eq!(results.len(), 12);

        //check depth-first
        let res = results.get(4).unwrap();
        assert!(res.is_string());
        assert_eq!(res.as_string(), String::from("a"));

        let res = results.get(11).unwrap();
        assert!(res.is_integer());
        assert_eq!(res.as_integer().unwrap(), 4)
    }

    #[test]
    fn bracket_double_star_at_lev_range() {
        let results = query("@[**{1,1}]", EXAMPLE_JSON);

        assert_eq!(results.len(), 7);
    }

    #[test]
    fn bracket_double_star() {
        let results = query("[**]", EXAMPLE_JSON);

        assert_eq!(results.len(), 12);

        //check depth-first
        let res = results.get(4).unwrap();
        assert!(res.is_string());
        assert_eq!(res.as_string(), String::from("a"));

        let res = results.get(11).unwrap();
        assert!(res.is_integer());
        assert_eq!(res.as_integer().unwrap(), 4)
    }

    #[test]
    fn bracket_quot_double_star() {
        let results = query("@['**']", EXAMPLE_JSON);

        assert_eq!(results.len(), 12);

        //check depth-first
        let res = results.get(4).unwrap();
        assert!(res.is_string());
        assert_eq!(res.as_string(), String::from("a"));

        let res = results.get(11).unwrap();
        assert!(res.is_integer());
        assert_eq!(res.as_integer().unwrap(), 4)
    }

    #[test]
    fn double_star_depth_min_max() {
        let json: &str = r#"
                            {
                                "nested1": {
                                    "nested1prop": 1,
                                    "nested2": {
                                        "nested2prop":2,
                                        "nested3": {
                                            "nested3prop" : 3,
                                            "nested4" : {
                                                "nested4prop": 4
                                            }
                                        }
                                    }

                                }
                            }"#;

        let results = query("@.**{0,2}", json);

        assert_eq!(results.len(), 4)
    }

    #[test]
    fn double_star_depth_max() {
        let json: &str = r#"
                            {
                                "nested1": {
                                    "nested1prop": 1,
                                    "nested2": {
                                        "nested2prop":2,
                                        "nested3": {
                                            "nested3prop" : 3,
                                            "nested4" : {
                                                "nested4prop": 4
                                            }
                                        }
                                    }

                                }
                            }"#;

        let results = query("@.**{,3}", json);

        assert_eq!(results.len(), 5);
    }

    #[test]
    fn double_star_depth() {
        let json: &str = r#"
                            {
                                "nested1": {
                                    "nested1prop": 1,
                                    "nested2": {
                                        "nested2prop":2,
                                        "nested3": {
                                            "nested3prop" : 3,
                                            "nested4" : {
                                                "nested4prop": 4
                                            }
                                        }
                                    }

                                }
                            }"#;


        let results = query("@.**{3}", json);

        assert_eq!(results.len(), 5)
    }

    #[test]
    fn parent() {
        let json: &str = r#"
                                {
                                    "nested1": {
                                        "nested1prop": 1,
                                        "nested2": {
                                            "nested2prop":2,
                                            "nested3": {
                                                "nested3prop" : 3,
                                                "nested4" : {
                                                    "nested4prop": 4
                                                }
                                            }
                                        }

                                    }
                                }"#;

        let results = query("nested1^", json);

        let res = results.get(0).unwrap();

        let diffs = Diff::minimal(res, &NodeRef::from_json(json).unwrap());

        assert!(diffs.is_empty());
    }


    #[test]
    fn nested_property_parent() {
        let json: &str = r#"
                                {
                                    "nested1": {
                                        "nested1prop": 1,
                                        "nested2": {
                                            "nested2prop":2,
                                            "nested3": {
                                                "nested3prop" : 3,
                                                "nested4" : {
                                                    "nested4prop": 4
                                                }
                                            }
                                        }

                                    }
                                }"#;

        let nested1: &str = r#"
                                {
                                    "nested1prop": 1,
                                    "nested2": {
                                        "nested2prop":2,
                                        "nested3": {
                                            "nested3prop" : 3,
                                            "nested4" : {
                                                "nested4prop": 4
                                            }
                                        }
                                    }
                                 }"#;

        let results = query("@.nested1.nested1prop^", json);

        let res = results.get(0).unwrap();

        let diffs = Diff::minimal(res, &NodeRef::from_json(nested1).unwrap());

        assert!(diffs.is_empty());
    }

    #[test]
    fn ancestors() {
        let json: &str = r#"
                                {
                                    "nested1": {
                                        "nested1prop": 1,
                                        "nested2": {
                                            "nested2prop":2,
                                            "nested3": {
                                                "nested3prop" : 3,
                                                "nested4" : {
                                                    "nested4prop": 4
                                                }
                                            }
                                        }

                                    }
                                }"#;

        let results = query("@.nested1.nested2.nested3^**", json);
        assert_eq!(results.len(), 3);

        //check that root is last element
        let root = results.get(results.len() - 1).unwrap();
        let diffs = Diff::minimal(root, &NodeRef::from_json(json).unwrap());
        assert!(diffs.is_empty());
    }

    #[test]
    fn ancestors_depth_min_max() {
        let json: &str = r#"
                                {
                                    "nested1": {
                                        "nested1prop": 1,
                                        "nested2": {
                                            "nested2prop":2,
                                            "nested3": {
                                                "nested3prop" : 3,
                                                "nested4" : {
                                                    "nested4prop": 4
                                                }
                                            }
                                        }

                                    }
                                }"#;

        let nested2: &str = r#"{
                                    "nested2prop":2,
                                    "nested3": {
                                        "nested3prop" : 3,
                                        "nested4" : {
                                            "nested4prop": 4
                                        }
                                    }
                                 }"#;


        let results = query("nested1.nested2.nested3.nested4^**{2,3}", json);
        assert_eq!(results.len(), 2);

        let n2 = results.get(0).unwrap();
        let diffs = Diff::minimal(n2, &NodeRef::from_json(nested2).unwrap());
        assert!(diffs.is_empty());
    }

    #[test]
    fn ancestors_depth_max() {
        let json: &str = r#"
                                {
                                    "nested1": {
                                        "nested1prop": 1,
                                        "nested2": {
                                            "nested2prop":2,
                                            "nested3": {
                                                "nested3prop" : 3,
                                                "nested4" : {
                                                    "nested4prop": 4
                                                }
                                            }
                                        }

                                    }
                                }"#;

        let nested2: &str = r#"{
                                    "nested2prop":2,
                                    "nested3": {
                                        "nested3prop" : 3,
                                        "nested4" : {
                                            "nested4prop": 4
                                        }
                                    }
                                 }"#;

        let nested3: &str = r#"{
                                        "nested3prop" : 3,
                                        "nested4" : {
                                            "nested4prop": 4
                                        }
                                    }"#;

        let results = query("nested1.nested2.nested3.nested4^**{,2}", json);
        assert_eq!(results.len(), 2);

        let n2 = results.get(1).unwrap();
        let diffs = Diff::minimal(n2, &NodeRef::from_json(nested2).unwrap());
        assert!(diffs.is_empty());

        let n3 = results.get(0).unwrap();
        let diffs = Diff::minimal(n3, &NodeRef::from_json(nested3).unwrap());
        assert!(diffs.is_empty());
    }

    #[test]
    fn ancestors_depth_min() {
        let json: &str = r#"
                                {
                                    "nested1": {
                                        "nested1prop": 1,
                                        "nested2": {
                                            "nested2prop":2,
                                            "nested3": {
                                                "nested3prop" : 3,
                                                "nested4" : {
                                                    "nested4prop": 4
                                                }
                                            }
                                        }

                                    }
                                }"#;

        let nested1: &str = r#"
                                {
                                    "nested1prop": 1,
                                    "nested2": {
                                        "nested2prop":2,
                                        "nested3": {
                                            "nested3prop" : 3,
                                            "nested4" : {
                                                "nested4prop": 4
                                            }
                                        }
                                    }
                                 }"#;

        let results = query("nested1.nested2.nested3.nested4^**{2}", json);
        assert_eq!(results.len(), 3);

        let n1 = results.get(1).unwrap();
        let diffs = Diff::minimal(n1, &NodeRef::from_json(nested1).unwrap());
        assert!(diffs.is_empty());

        let root = results.get(2).unwrap();
        let diffs = Diff::minimal(root, &NodeRef::from_json(json).unwrap());
        assert!(diffs.is_empty());
    }
}
