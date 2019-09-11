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

[comment]: <> (TODO MC Assertion?)

```
let model = r#"{
  "null": null,
  "number": 1,
  "boolean": true,
  "string": "apple",
  "object": { "key": "value" },
  "array": [ 1, 2, 3, 4 ],
  "binary": "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
  Z"
}"#;
```

# Literals

[comment]: <> (TODO MC Code with example?)

* `123`, `-2` - 64-bit integer values
* `1.13`, `.e10`, `-1E-2`, `.3` - 64-bit float values
* `'id'`, `"id"` - string values
* `true`, `false` - boolean values
* `null` - null value

# Type conversions

[comment]: <> (TODO MC Code with example?)

Same as ECMAScript, integers promoted to floats when mixed operands (do rozwiniecia)

## Context

[comment]: <> (TODO MC Better code with example?)

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

let query = r#"@.foo"#;

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

let query = r#"foo"#;

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

let query = r#"$.foo"#;

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

# Indexing for arrays

Array elements can be accessed with `[]` notation. Arrays are indexed starting from `0`.

`@[0]` - returns the first element of the **current** array:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"["one", "two", "three"]"#;

let query = r#"@[0]"#;

let result = r#"{
  "type": "one",
  "data": "one"
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`@[0, 1..3, 5]` - arrays can be indexed by multiple comma-separated indices as well as ranges of indices:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"["one", "two", "three", "four", "five", "six"]"#;

let query = r#"@[0, 1..3, 5]"#;

let result = r#"{
  "type": "many",
  "data": ["one", "two", "three", "four", "six"]
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`@[-1,-2]` - negative indices are calculated from the end of an array, `-1` being the last element of an array:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"["one", "two", "three"]"#;

let query = r#"@[-1, -2]"#;

let result = r#"{
  "type": "many",
  "data": ["three", "two"]
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`@[3..]` - when using ranges in array indexing expressions (inside `[]`), range ending value can be omitted, 
and it will be equal to the array length (number of array elements):

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"["one", "two", "three", "four", "five", "six"]"#;

let query = r#"@[3..]"#;

let result = r#"{
  "type": "many",
  "data": ["four", "five", "six"]
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

Accessing array with out-of-bounds index values yields empty result:

[comment]: <> (TODO MC How to present result?)

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"["one", "two", "three"]"#;

let query = r#"@[4]"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::Empty;
assert_eq!(res, expected);
```

Accessing array element on a non-array and non-object type yields empty result:

[comment]: <> (TODO MC How to present result?)

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#""string""#;

let query = r#"@[0]"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::Empty;
assert_eq!(res, expected);
```

## Mathematical operators

Typical mathematical operators and parentheses are supported.

Internally, type conversion is avoided as long as possible, i.e. adding two integer values will yield integer sum.

Numerical addition:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "count": 1
}"#;

let query = r#"count + 1"#;

let result = r#"{
  "type": "one",
  "data": 2
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

Numerical subtraction:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "count": 2
}"#;

let query = r#"count - 1"#;

let result = r#"{
  "type": "one",
  "data": 1
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

Numerical multiplication:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "count": 2
}"#;

let query = r#"count * 3"#;

let result = r#"{
  "type": "one",
  "data": 6
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

Numerical division:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "count": 6
}"#;

let query = r#"count / 3"#;

let result = r#"{
  "type": "one",
  "data": 2
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

Mathematical order of performing actions.

This expression yields value 5, as expected:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "count": 2
}"#;

let query = r#"count + 6 / 2"#;

let result = r#"{
  "type": "one",
  "data": 5
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

This expression yields value 4, as expected:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "count": 2
}"#;

let query = r#"(count + 6) / 2"#;

let result = r#"{
  "type": "one",
  "data": 4
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

# String concatenation

If any of the addition operands has a string value, addition will become string concatenation

`2 + "3"`, `"2" + 3` - both expressions yield string value `"23"`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "number": 2
}"#;

let query = r#"number + "3""#;

let result = r#"{
  "type": "one",
  "data": "23"
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "string": "2"
}"#;

let query = r#"string + 3"#;

let result = r#"{
  "type": "one",
  "data": "23"
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`"John" + " " + 'Doe'` - yields `"John Doe"`

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "first_name": "John"
}"#;

let query = r#"first_name + " " + 'Doe'"#;

let result = r#"{
  "type": "one",
  "data": "John Doe"
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

# Comparison operators

Greater than `>`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "number": 2
}"#;

let query = r#"number > 1"#;

let result = r#"{
  "type": "one",
  "data": true
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

Greater than or equal to `>=`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "number": 1
}"#;

let query = r#"number >= 1"#;

let result = r#"{
  "type": "one",
  "data": true
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

Less than `<`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "number": 1
}"#;

let query = r#"number < 2"#;

let result = r#"{
  "type": "one",
  "data": true
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

Less than or equal to `<=`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "number": 1
}"#;

let query = r#"number <= 1"#;

let result = r#"{
  "type": "one",
  "data": true
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

Equal to `==`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "number": 3
}"#;

let query = r#"number == 3"#;

let result = r#"{
  "type": "one",
  "data": true
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

Not equal to `!=`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "number": 1
}"#;

let query = r#"number != 3"#;

let result = r#"{
  "type": "one",
  "data": true
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`'aaabbb' ^= 'aa'` - `true` if left string operand starts with right string operand:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "string": "aaabbb"
}"#;

let query = r#"string ^= 'aa'"#;

let result = r#"{
  "type": "one",
  "data": true
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`'aaabbb' *= 'ab'` - `true` if left string operand contains right string operand:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "string": "aaabbb"
}"#;

let query = r#"string *= 'ab'"#;

let result = r#"{
  "type": "one",
  "data": true
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`'aaabbb' $= 'bb'` - `true` if left string operand ends with right string operand:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "string": "aaabbb"
}"#;

let query = r#"string $= 'bb'"#;

let result = r#"{
  "type": "one",
  "data": true
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

# Logical operators

`not true` and `!true`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "bool": true
}"#;

let query = r#"not bool"#;

let result = r#"{
  "type": "one",
  "data": false
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "bool": true
}"#;

let query = r#"!bool"#;

let result = r#"{
  "type": "one",
  "data": false
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`true and true` and `true && true`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "bool": true
}"#;

let query = r#"bool and true"#;

let result = r#"{
  "type": "one",
  "data": true
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "bool": true
}"#;

let query = r#"bool && true"#;

let result = r#"{
  "type": "one",
  "data": true
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`true or false` and `true || false`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "bool": true
}"#;

let query = r#"bool or false"#;

let result = r#"{
  "type": "one",
  "data": true
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "bool": true
}"#;

let query = r#"bool || false"#;

let result = r#"{
  "type": "one",
  "data": true
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```