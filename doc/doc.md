# Opath query language

Opath is a simple language for object tree lookup and transformation, similar to 
[XPath](https://www.w3.org/TR/xpath/) in function.

# Data types

All data types transferable through `json`, `yaml` and `toml` formats are supported.

* *null* - empty value
* *number* - internally stored as either 64-bit integer or 64-bit float
* *boolean* - `true` or `false`
* *string* - strings are stored as UTF-8 encoded data.
* *binary* - binary data.
* *object* - object or map, can contain string-keyed properties
* *array* - array or sequence of elements

## Context

Every `Opath` expression is executed in the context of **root** (denoted `$`) and **current** 
(denoted `@`) elements. To access any element in the object tree, it's relation to 
the **current** (`@`) or **root** (`$`) element needs to be defined, much like for 
paths in the filesystem are relative to the current directory, or filesystem root. 
For expressions based at the **current** element, explicit denotion of `@` can usually be omitted.

* `@.name` - returns the value of property "name" from the **current** element
* `name` - same as above
* `$.name` - returns the value of property "name" from the **root** element

`@.name`:

```
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

`name`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "foo": "bar"
}"#;

let query = "foo";

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

`$.name`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "foo": "bar"
}"#;

let query = "$.foo";

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

## Mathematical operators
Numerical addition

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "foo": 1
}"#;

let query = "@.foo + 1";

let result = r#"{
  "type": "one",
  "data": "2"
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```