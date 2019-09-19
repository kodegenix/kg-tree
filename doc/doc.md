# Opath query language

Opath is a simple language for object tree lookup and transformation, similar to 
[XPath](https://www.w3.org/TR/xpath/) in function.

## Data types

All data types transferable through `json`, `yaml` and `toml` formats are supported.

* *null* - empty value
* *number* - internally stored as either 64-bit integer or 64-bit float
* *boolean* - `true` or `false`
* *string* - strings are stored as UTF-8 encoded data.
* *binary* - binary data.
* *object* - object or map, can contain string-keyed properties
* *array* - array or sequence of elements

[comment]: <> (TODO Issue "Fix serialization of binary data to json" and add binary data in example below)

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "null": null,
  "integer": 1,
  "float": 0.1,
  "boolean": true,
  "string": "apple",
  "object": { "key": "value" },
  "array": [ 1, 2, 3, 4 ]
}"#;

let node = NodeRef::from_json(model).unwrap();
assert!(node.is_object());
assert!(node.get_child_key("null").unwrap().is_null());
assert!(node.get_child_key("integer").unwrap().is_integer());
assert!(node.get_child_key("float").unwrap().is_float());
assert!(node.get_child_key("boolean").unwrap().is_boolean());
assert!(node.get_child_key("string").unwrap().is_string());
assert!(node.get_child_key("object").unwrap().is_object());
assert!(node.get_child_key("array").unwrap().is_array());
```

## Literals

Function `array()` is used in examples below. Functions are described in subsection **Functions** below.

`123`, `0` `-2` - 64-bit integer values:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let node = NodeRef::null();

let query = r#"array(123, 0, -2)[*]"#;

let expr = Opath::parse(query).unwrap();
let res = expr.apply(&node, &node).unwrap();
let nodes = res.into_vec();
assert!(nodes[0].is_integer());
assert!(nodes[1].is_integer());
assert!(nodes[2].is_integer());
```

`0.78`, `-0.12`, `-2.54e+3`, `4.74E-4`, `7.34e2`, `-3.1E6` - 64-bit float values:

[comment]: <> (TODO Examples with dot at the begining don't work, examples with dot at the end don't work)

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let node = NodeRef::null();

let query = r#"array(0.78, -0.12, -2.54e+3, 4.74E-4, 7.34e2, -3.1E6)[*]"#;

let expr = Opath::parse(query).unwrap();
let res = expr.apply(&node, &node).unwrap();
let nodes = res.into_vec();
assert!(nodes[0].is_float());
assert!(nodes[1].is_float());
assert!(nodes[2].is_float());
assert!(nodes[3].is_float());
assert!(nodes[4].is_float());
assert!(nodes[5].is_float());
```

`'id'`, `"id"` - string values:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let node = NodeRef::null();

let query = r#"array('string', "string")[*]"#;

let expr = Opath::parse(query).unwrap();
let res = expr.apply(&node, &node).unwrap();
let nodes = res.into_vec();
assert!(nodes[0].is_string());
assert!(nodes[1].is_string());
```

`true`, `false` - boolean values:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let node = NodeRef::null();

let query = r#"array(true, false)[*]"#;

let expr = Opath::parse(query).unwrap();
let res = expr.apply(&node, &node).unwrap();
let nodes = res.into_vec();
assert!(nodes[0].is_boolean());
assert!(nodes[1].is_boolean());
```

`null` - null value:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let node = NodeRef::null();

let query = r#"null"#;

let expr = Opath::parse(query).unwrap();
let res = expr.apply(&node, &node).unwrap();
let nodes = res.into_vec();
assert!(nodes[0].is_null());
```

## Type conversions

[comment]: <> (TODO MC Add all of the conversions. Information about conversions is in functions as_float, as_integer...)

Similar to ECMAScript, with small differences. For example integers promoted to floats when mixed operands.

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "integer": 1
}"#;

let query = r#"integer + 1.1"#;

let result = r#"{
  "type": "one",
  "data": 2.1
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
assert!(expected.into_one().unwrap().is_float());
```

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

In the next example first `@` means **root** element, second `@` means **property** in **root** element:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "key0": "value0",
  "key1": "value1"
}"#;

let query = r#"@[@.@key == "key0"]"#;

let result = r#"{
  "type": "one",
  "data": "value0"
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

## Indexing for arrays

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

`$[0]` - returns the first element of the **root** array:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"["one", "two", "three"]"#;

let query = r#"$[0]"#;

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

`@[1][2]` - returns the third element of the second array from **current** array:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"[
  ["one", "two", "three"],
  ["four", "five", "six"],
  ["seven", "eight", "nine"]
]"#;

let query = r#"@[1][2]"#;

let result = r#"{
  "type": "one",
  "data": "six"
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

## Property access for objects

Properties in objects can be accessed with typical `.` or `[]` notations.

`name` - returns the value of property "name" from the **current** element:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "name": "value"
}"#;

let query = r#"name"#;

let result = r#"{
  "type": "one",
  "data": "value"
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`@["name"]`, `@['name']`, `@."name"` - property names can be quoted, and if so, can contain
spaces and special characters:

[comment]: <> (TODO MC What with @[name]?)

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "name": "value"
}"#;

let query = r#"@["name"]"#;

let result = r#"{
  "type": "one",
  "data": "value"
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
  "name": "value"
}"#;

let query = r#"@['name']"#;

let result = r#"{
  "type": "one",
  "data": "value"
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
  "name": "value"
}"#;

let query = r#"@."name""#;

let result = r#"{
  "type": "one",
  "data": "value"
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`@.(first_name, last_name)` - one can select a few properties with a single expression using parentheses:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "first_name": "John",
  "last_name": "Doe"
}"#;

let query = r#"@.(first_name, last_name)"#;

let result = r#"{
  "type": "many",
  "data": ["John", "Doe"]
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`[name]`, `@[name]` - this is illegal!

`"name"` - this is string literal, not property access!

## Property indexing for objects

Every object can also be indexed as an array, where index value will correspond with property position within 
the object. For example if **current** object will be:
```json
{
   "first_name": "John",
   "last_name": "Doe"
}
```
expression `@[1]` will yield string value `"Doe"` (value of the second property). Objects have strictly defined 
and stable insertion order of properties:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "first_name": "John",
  "last_name": "Doe"
}"#;

let query = r#"@[1]"#;

let result = r#"{
  "type": "one",
  "data": "Doe"
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

## Metadata (attributes)

All elements contain readable metadata (attributes). Those attributes are accessed like regular properties, but with 
name prefixed with `@` character.

`@.@index` - index of **current** element in its parent (if the parent is an object, this will be the property
position):

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "key0": "value0",
  "key1": "value1"
}"#;

let query = r#"key1.@index"#;

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

`@.@key` - property name of **current** element in its parent (for arrays this will be string value of index):

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "key0": "value0"
}"#;

let query = r#"@[0].@key"#;

let result = r#"{
  "type": "one",
  "data": "key0"
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`@.@level` - distance from the **root** element for **current** element:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "key0": {
    "key00": "value00"
  }
}"#;

let query = r#"key0.key00.@level"#;

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

`@.@kind` - string value of **current** element's kind, either one of `"null"`, `"boolean"`, `"integer"`, `"float"`,
`"string"`, `"binary"`, `"object"`, `"array"`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "key0": 0.1
}"#;

let query = r#"key0.@kind"#;

let result = r#"{
  "type": "one",
  "data": "float"
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`@.@type` - string value of **current** element's type, either one of `"null"`, `"boolean"`, `"number"`, `"string"`, 
`"binary"`, `"object"`, `"array"`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "key0": 0.1
}"#;

let query = r#"key0.@type"#;

let result = r#"{
  "type": "one",
  "data": "number"
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`@.@file` - string describing the file or file structure with relative path, **current** element was read from (if any), for instance 
`"file<yaml>:./data.yml"`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::{NodeRef, FileInfo, FileFormat, set_base_path};
use kg_diag::FileType;

let node = NodeRef::null();
node.data_mut().set_file(Some(&FileInfo::new("/abs/path/directive/data.yml", FileType::File, FileFormat::Yaml)));
set_base_path("/abs/path");

let query = r#"@.@file"#;

let result = r#"{
  "type": "one",
  "data": "file<yaml>:directive/data.yml"
}"#;

let expr = Opath::parse(query).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`@.@file_abs` - string describing the file or file structure with absolute path, **current** element was read from (if any), for instance 
`"file<yaml>:./data.yml"`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::{NodeRef, FileInfo, FileFormat, set_base_path};
use kg_diag::FileType;

let node = NodeRef::null();
node.data_mut().set_file(Some(&FileInfo::new("/abs/path/directive/data.yml", FileType::File, FileFormat::Yaml)));
set_base_path("/abs/path");

let query = r#"@.@file_abs"#;

let result = r#"{
  "type": "one",
  "data": "file<yaml>:/abs/path/directive/data.yml"
}"#;

let expr = Opath::parse(query).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`@.@file_type`- string with file type (if any), either `"file"` or `"dir"`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::{NodeRef, FileInfo, FileFormat};
use kg_diag::FileType;

let node = NodeRef::null();
node.data_mut().set_file(Some(&FileInfo::new("/directive/data.yml", FileType::File, FileFormat::Yaml)));

let query = r#"@.@file_type"#;

let result = r#"{
  "type": "one",
  "data": "file"
}"#;

let expr = Opath::parse(query).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`@.@file_format`- string with file format (if any), supported values are: `"json"`, `"yaml"`, `"toml"`, `"text"`, 
`"binary"`: 

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::{NodeRef, FileInfo, FileFormat};
use kg_diag::FileType;

let node = NodeRef::null();
node.data_mut().set_file(Some(&FileInfo::new("/directive/data.yml", FileType::File, FileFormat::Yaml)));

let query = r#"@.@file_format"#;

let result = r#"{
  "type": "one",
  "data": "yaml"
}"#;

let expr = Opath::parse(query).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`@.@file_path`- string with relative file path (if any), for instance `"./data.yml"`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::{NodeRef, FileInfo, FileFormat, set_base_path};
use kg_diag::FileType;

let node = NodeRef::null();
node.data_mut().set_file(Some(&FileInfo::new("/abs/path/directive/data.yml", FileType::File, FileFormat::Yaml)));
set_base_path("/abs/path");

let query = r#"@.@file_path"#;

let result = r#"{
  "type": "one",
  "data": "directive/data.yml"
}"#;

let expr = Opath::parse(query).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`@.@file_path_abs`- string with absolute file path (if any), for instance `"./data.yml"`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::{NodeRef, FileInfo, FileFormat, set_base_path};
use kg_diag::FileType;

let node = NodeRef::null();
node.data_mut().set_file(Some(&FileInfo::new("/abs/path/directive/data.yml", FileType::File, FileFormat::Yaml)));
set_base_path("/abs/path");

let query = r#"@.@file_path_abs"#;

let result = r#"{
  "type": "one",
  "data": "/abs/path/directive/data.yml"
}"#;

let expr = Opath::parse(query).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`@.@file_name`- string with file name (if any), for instance `"data.yml"`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::{NodeRef, FileInfo, FileFormat};
use kg_diag::FileType;

let node = NodeRef::null();
node.data_mut().set_file(Some(&FileInfo::new("/directive/data.yml", FileType::File, FileFormat::Yaml)));

let query = r#"@.@file_name"#;

let result = r#"{
  "type": "one",
  "data": "data.yml"
}"#;

let expr = Opath::parse(query).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`@.@file_stem`- string with file stem (if any), for instance `"data"`. For file names starting with `"."`, 
like `".data.yml"` stem will be `".data"`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::{NodeRef, FileInfo, FileFormat};
use kg_diag::FileType;

let node = NodeRef::null();
node.data_mut().set_file(Some(&FileInfo::new("/directive/data.yml", FileType::File, FileFormat::Yaml)));

let query = r#"@.@file_stem"#;

let result = r#"{
  "type": "one",
  "data": "data"
}"#;

let expr = Opath::parse(query).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`@.@file_ext`- string with file extension (if any), for instance `"yml"`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::{NodeRef, FileInfo, FileFormat};
use kg_diag::FileType;

let node = NodeRef::null();
node.data_mut().set_file(Some(&FileInfo::new("/directive/data.yml", FileType::File, FileFormat::Yaml)));

let query = r#"@.@file_ext"#;

let result = r#"{
  "type": "one",
  "data": "yml"
}"#;

let expr = Opath::parse(query).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`@file_path_components` - array with file path components. This array contains names of directories.
Last element of the array is the name of the file. If there is a prefix, then the first element
of the array is prefix:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::{NodeRef, FileInfo, FileFormat, set_base_path};
use kg_diag::FileType;

let node = NodeRef::null();
node.data_mut().set_file(Some(&FileInfo::new("/abs/path/directive/data.yml", FileType::File, FileFormat::Yaml)));
set_base_path("/abs/path");

let query = r#"@.@file_path_components"#;

let result = r#"{
  "type": "one",
  "data": ["directive", "data.yml"]
}"#;

let expr = Opath::parse(query).unwrap();
let res = expr.apply(&node, &node).unwrap().into_one().unwrap();
let expected = NodeSet::from_json(result).unwrap().into_one().unwrap();
assert!(res.is_identical_deep(&expected));
```

`@dir`- string with relative file path without file in it (if any), for instance `"./data.yml"`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::{NodeRef, FileInfo, FileFormat, set_base_path};
use kg_diag::FileType;

let node = NodeRef::null();
node.data_mut().set_file(Some(&FileInfo::new("/abs/path/directive/data.yml", FileType::File, FileFormat::Yaml)));
set_base_path("/abs/path");

let query = r#"@.@dir"#;

let result = r#"{
  "type": "one",
  "data": "directive"
}"#;

let expr = Opath::parse(query).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`@dir_abs`- string with absolute file path without file in it (if any), for instance `"./data.yml"`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::{NodeRef, FileInfo, FileFormat, set_base_path};
use kg_diag::FileType;

let node = NodeRef::null();
node.data_mut().set_file(Some(&FileInfo::new("/abs/path/directive/data.yml", FileType::File, FileFormat::Yaml)));
set_base_path("/abs/path");

let query = r#"@.@dir_abs"#;

let result = r#"{
  "type": "one",
  "data": "/abs/path/directive"
}"#;

let expr = Opath::parse(query).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`@.@path` - path to the **current** element from the **root**, for instance `"$.nested.array[3]"`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "key0": {
    "key00": "value00"
  }
}"#;

let query = r#"@[0][0].@path"#;

let result = r#"{
  "type": "one",
  "data": "$.key0.key00"
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

## Property / element filtering

Properties in objects or elements in arrays can also be filtered with logical expressions inside `[]` notation.

Note that inside the `[]` expression the **current** element (`@`) becomes the child of the outer element.

`@[@.@key $= "name"]` - yields **current** element property values for which property name ends with `"name"`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "first_name": "John",
  "last_name": "Doe",
  "age": 30
}"#;

let query = r#"@[@.@key $= "name"]"#;

let result = r#"{
  "type": "many",
  "data": ["John", "Doe"]
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`@[@.@index >= 2]` - yields **current** element properties / elements with index greater or equal `2`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "first_name": "John",
  "last_name": "Doe",
  "age": 30
}"#;

let query = r#"@[@.@index >= 2]"#;

let result = r#"{
  "type": "one",
  "data": 30
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

## Property / element access wildcard operator `*`

`@.*`, `@[*]` - yields all properties of the **current** object or all elements of the **current** array, or 
empty result, depending on the **current** type:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "first_name": "John",
  "last_name": "Doe",
  "age": 30
}"#;

let query = r#"@.*"#;

let result = r#"{
  "type": "many",
  "data": ["John", "Doe", 30]
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

## Property / element access recursive descent operator `**`

`@.**`, `@[**]`, `@."**"`, `@['**']` - yields all properties of the **current** object, and recursively all of their properties in 
depth-first descending order:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "key0": "value0",
  "key1": "value1",
  "key2": {
    "key20": "value20",
    "key21": "value21"
  }
}"#;

let query = r#"@.**"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
assert_eq!(res.len(), 5);
let nodes = res.into_vec();
assert_eq!(nodes[0].as_string(), "value0");
assert_eq!(nodes[1].as_string(), "value1");
assert!(nodes[2].is_object());
assert_eq!(nodes[2].get_child_key("key20").unwrap().as_string(), "value20");
assert_eq!(nodes[2].get_child_key("key21").unwrap().as_string(), "value21");
assert_eq!(nodes[3].as_string(), "value20");
assert_eq!(nodes[4].as_string(), "value21");
```

`@.**{1,4}`, `@.**{,4}`, `@.**{2}`, `@.**{0,2}`- optionally depth level range can be specified. The depth level 
is specified relative from the element being accessed (**current** in those examples):

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "key0": "value0",
  "key1": "value1",
  "key2": {
    "key20": "value20",
    "key21": "value21"
  }
}"#;

let query = r#"@.**{2,2}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
assert_eq!(res.len(), 2);
let nodes = res.into_vec();
assert_eq!(nodes[0].as_string(), "value20");
assert_eq!(nodes[1].as_string(), "value21");
```

if minimal depth level value is omitted, `1` is assumed:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "key0": "value0",
  "key1": "value1",
  "key2": {
    "key20": "value20",
    "key21": "value21"
  }
}"#;

let query = r#"@.**{,2}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
assert_eq!(res.len(), 5);
let nodes = res.into_vec();
assert_eq!(nodes[0].as_string(), "value0");
assert_eq!(nodes[1].as_string(), "value1");
assert!(nodes[2].is_object());
assert_eq!(nodes[2].get_child_key("key20").unwrap().as_string(), "value20");
assert_eq!(nodes[2].get_child_key("key21").unwrap().as_string(), "value21");
assert_eq!(nodes[3].as_string(), "value20");
assert_eq!(nodes[4].as_string(), "value21");
```

if maximal depth level is omitted, descend operator will be unbound from the top, i.e. will
continue for all descendants:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "key0": "value0",
  "key1": "value1",
  "key2": {
    "key20": "value20",
    "key21": "value21"
  }
}"#;

let query = r#"@.**{1}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
assert_eq!(res.len(), 5);
let nodes = res.into_vec();
assert_eq!(nodes[0].as_string(), "value0");
assert_eq!(nodes[1].as_string(), "value1");
assert!(nodes[2].is_object());
assert_eq!(nodes[2].get_child_key("key20").unwrap().as_string(), "value20");
assert_eq!(nodes[2].get_child_key("key21").unwrap().as_string(), "value21");
assert_eq!(nodes[3].as_string(), "value20");
assert_eq!(nodes[4].as_string(), "value21");
```

if minimal depth level value is `0`, the result will also include accessed element itself:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "key0": "value0",
  "key1": "value1",
  "key2": {
    "key20": "value20",
    "key21": "value21"
  }
}"#;

let query = r#"@.**{0,2}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
assert_eq!(res.len(), 6);
let nodes = res.into_vec();
assert!(nodes[0].is_object());
assert_eq!(nodes[0].get_child_key("key0").unwrap().as_string(), "value0");
assert_eq!(nodes[0].get_child_key("key1").unwrap().as_string(), "value1");
assert!(nodes[0].get_child_key("key2").unwrap().is_object());
assert_eq!(nodes[1].as_string(), "value0");
assert_eq!(nodes[2].as_string(), "value1");
assert!(nodes[3].is_object());
assert_eq!(nodes[3].get_child_key("key20").unwrap().as_string(), "value20");
assert_eq!(nodes[3].get_child_key("key21").unwrap().as_string(), "value21");
assert_eq!(nodes[4].as_string(), "value20");
assert_eq!(nodes[5].as_string(), "value21");
```

## Parent access operator `^`

`@^` - this yields parent element of the **current** element:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "name": "value",
  "name1": "value1"
}"#;

let query = r#"@[0]^.name1"#;

let result = r#"{
  "type": "one",
  "data": "value1"
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`@.name^` - if **current** element is an object and contains "name" property, this expression will yield 
**current** element:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "name": "value",
  "name1": "value1"
}"#;

let query = r#"@.name^.name1"#;

let result = r#"{
  "type": "one",
  "data": "value1"
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

## Ascendant access recursive operator `^**`

`@^**` - yields all ascendants of the **current** element, in order of decreasing depth. The last element will 
be **root**:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "key0": "value0",
  "key1": "value1",
  "key2": {
    "key20": "value20",
    "key21": "value21"
  }
}"#;

let query = r#"key2.key20^**"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
assert_eq!(res.len(), 2);
let nodes = res.into_vec();
assert!(nodes[0].is_object());
assert_eq!(nodes[0].get_child_key("key20").unwrap().as_string(), "value20");
assert_eq!(nodes[0].get_child_key("key21").unwrap().as_string(), "value21");
assert!(nodes[1].is_object());
assert_eq!(nodes[1].get_child_key("key0").unwrap().as_string(), "value0");
assert_eq!(nodes[1].get_child_key("key1").unwrap().as_string(), "value1");
```

`@^**{2,2}`, `@^**{,2}`, `@^**{1}`- optionally recursive distance range can be specified, analogically like 
for `**`. The distance is specified relative from the element being accessed:

`@^**{2,2}`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "key0": "value0",
  "key1": "value1",
  "key2": {
    "key20": "value20",
    "key21": "value21"
  }
}"#;

let query = r#"key2.key20^**{2,2}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
assert_eq!(res.len(), 1);
let nodes = res.into_vec();
assert!(nodes[0].is_object());
assert_eq!(nodes[0].get_child_key("key0").unwrap().as_string(), "value0");
assert_eq!(nodes[0].get_child_key("key1").unwrap().as_string(), "value1");
```

`@^**{,2}`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "key0": "value0",
  "key1": "value1",
  "key2": {
    "key20": "value20",
    "key21": "value21"
  }
}"#;

let query = r#"key2.key20^**{,2}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
assert_eq!(res.len(), 2);
let nodes = res.into_vec();
assert!(nodes[0].is_object());
assert_eq!(nodes[0].get_child_key("key20").unwrap().as_string(), "value20");
assert_eq!(nodes[0].get_child_key("key21").unwrap().as_string(), "value21");
assert!(nodes[1].is_object());
assert_eq!(nodes[1].get_child_key("key0").unwrap().as_string(), "value0");
assert_eq!(nodes[1].get_child_key("key1").unwrap().as_string(), "value1");
```

`@^**{1}`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"{
  "key0": "value0",
  "key1": "value1",
  "key2": {
    "key20": "value20",
    "key21": "value21"
  }
}"#;

let query = r#"key2.key20^**{1}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
assert_eq!(res.len(), 2);
let nodes = res.into_vec();
assert!(nodes[0].is_object());
assert_eq!(nodes[0].get_child_key("key20").unwrap().as_string(), "value20");
assert_eq!(nodes[0].get_child_key("key21").unwrap().as_string(), "value21");
assert!(nodes[1].is_object());
assert_eq!(nodes[1].get_child_key("key0").unwrap().as_string(), "value0");
assert_eq!(nodes[1].get_child_key("key1").unwrap().as_string(), "value1");
```

## Functions

`array()`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let node = NodeRef::null();

let query = r#"array("one", "two", "three")"#;

let result = r#"{
  "type": "one",
  "data": ["one", "two", "three"]
}"#;

let expr = Opath::parse(query).unwrap();
let res = expr.apply(&node, &node).unwrap().into_one().unwrap();
let expected = NodeSet::from_json(result).unwrap().into_one().unwrap();
assert!(res.is_identical_deep(&expected));
```

`map()`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let node = NodeRef::null();

let query = r#"map()"#;

let result = r#"{
  "type": "one",
  "data": {}
}"#;

let expr = Opath::parse(query).unwrap();
let res = expr.apply(&node, &node).unwrap().into_one().unwrap();
let expected = NodeSet::from_json(result).unwrap().into_one().unwrap();
assert!(res.is_identical_deep(&expected));
```

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::{NodeRef};

let model = r#"{
  "obj": {
    "key1": "value1",
    "key2": "value2"
  },
  "obj2": {
    "key3": "value3",
    "key4": "value4"
  }
}"#;

let query = r#"map($.*)"#;

let result = r#"{
  "type": "one",
  "data": {
    "key1": "value1",
    "key2": "value2",
    "key3": "value3",
    "key4": "value4"
  }
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap().into_one().unwrap();
let expected = NodeSet::from_json(result).unwrap().into_one().unwrap();
assert!(res.is_identical_deep(&expected));
```

`parse()`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::{NodeRef};

let model = r#"null"#;

let query = r#"parse("true", "json")"#;

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

`parseInt()`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::{NodeRef};

let model = r#"{
    "key": ["1", "2"]
}"#;

let query = r#"parseInt(key.*)"#;

let result = r#"{
  "type": "many",
  "data": [1, 2]
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::{NodeRef};

let model = r#"{
    "key": ["010", "01"]
}"#;

let query = r#"parseInt(key.*, 2)"#;

let result = r#"{
  "type": "many",
  "data": [2, 1]
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`parseFloat()`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::{NodeRef};

let model = r#"{
    "key": ["0.12", "-1.3", "+4.214", "-2.43e+21", "+4.3E3"]
}"#;

let query = r#"parseFloat(key.*)"#;

let result = r#"{
  "type": "many",
  "data": [0.12, -1.3, 4.214, -2.43e+21, 4.3E3]
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`isNaN()`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::{NodeRef};

let model = r#"null"#;

let query = r#"isNaN("string")"#;

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

`sqrt()`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::{NodeRef};

let model = r#"null"#;

let query = r#"sqrt(4)"#;

let result = r#"{
  "type": "one",
  "data": 2.0
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`json()`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::{NodeRef};

let model = r#"null"#;

let query = r#"json("1")"#;

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

`stringify()`:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::{NodeRef};

let model = r#"{
  "key0": {
    "key01": "value01",
    "key02": "value02"
  }
}"#;

let query = r#"stringify(key0)"#;

let result = r#"{
  "type": "one",
  "data": "{\"key01\":\"value01\",\"key02\":\"value02\"}"
}"#;

let expr = Opath::parse(query).expect("1");
let node = NodeRef::from_json(model).expect("2");
let res = expr.apply(&node, &node).expect("3");
let expected = NodeSet::from_json(result).expect("3");
assert_eq!(res, expected);
```

[comment]: <> (TODO MC)

ReadFile,
ParseBinary,
Stringify,
Custom(String),

## Method

[comment]: <> (TODO MC)

Length,
ToString,
Find,
Set,
Delete,
Extend,
Push,
Pop,
Shift,
Unshift,
Join,
Replace,
Split,
Custom(String),
    
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

## String concatenation

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

## Comparison operators

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

## Logical operators

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

## Number ranges

`:4` - range from `0` (inclusive) to `4` (inclusive):

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"["one", "two", "three", "four", "five", "six"]"#;

let query = r#"@[:4]"#;

let result = r#"{
  "type": "many",
  "data": ["one", "two", "three", "four", "five"]
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`1:4` - range from `1` (inclusive) to `4` (inclusive):

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"["one", "two", "three", "four", "five", "six"]"#;

let query = r#"@[1:4]"#;

let result = r#"{
  "type": "many",
  "data": ["two", "three", "four", "five"]
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`0:2:4` - range from `0` (inclusive) to `10` (inclusive) with `2` increments:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"["one", "two", "three", "four", "five", "six"]"#;

let query = r#"@[0:2:4]"#;

let result = r#"{
  "type": "many",
  "data": ["one", "three", "five"]
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`0:1.5:5` - floats in ranges are also supported:

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"["one", "two", "three", "four", "five", "six"]"#;

let query = r#"@[0:1.5:5]"#;

let result = r#"{
  "type": "many",
  "data": ["one", "two", "four", "five"]
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`1..3` - range from `1` (inclusive) to `3` (inclusive):

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"["one", "two", "three", "four", "five", "six"]"#;

let query = r#"@[1..3]"#;

let result = r#"{
  "type": "many",
  "data": ["two", "three", "four"]
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```

`..3` - range from `0` (inclusive) to `10` (inclusive):

```
use kg_tree::opath::{Opath, NodeSet};
use kg_tree::NodeRef;

let model = r#"["one", "two", "three", "four", "five", "six"]"#;

let query = r#"@[..3]"#;

let result = r#"{
  "type": "many",
  "data": ["one", "two", "three", "four"]
}"#;

let expr = Opath::parse(query).unwrap();
let node = NodeRef::from_json(model).unwrap();
let res = expr.apply(&node, &node).unwrap();
let expected = NodeSet::from_json(result).unwrap();
assert_eq!(res, expected);
```