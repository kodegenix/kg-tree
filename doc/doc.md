 Some text
 
 text
 
 text
 
 text
 ```rust
 use kg_tree::opath::{Opath, NodeSet};
 use kg_tree::NodeRef;
 let model = r#"{
   "foo": "bar"
 }"#;
 let query = "@.foo";
 let result = r#"{
   "type": "one",
   "data": "bar"
 }"#;
 let expr = Opath::parse(query).unwrap();
 let node = NodeRef::from_json(model).unwrap();
 let res = expr.apply(&node, &node).unwrap();
 let expected = NodeSet::from_json(result).unwrap();
 assert_eq!(res, expected);
 ```
