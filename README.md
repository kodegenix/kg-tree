# kg-tree

[![Latest Version](https://img.shields.io/crates/v/kg-tree.svg)](https://crates.io/crates/kg-tree)
[![Documentation](https://docs.rs/kg-tree/badge.svg)](https://docs.rs/kg-tree)
[![Build Status](https://travis-ci.org/Kodegenix/kg-tree.svg?branch=master)](https://travis-ci.org/Kodegenix/kg-tree)

Generic object tree with Opath query language, similar to XPath.

## Opath
Simple language for object tree lookup and transformation, similar to XPath in function.

### Data types
All data types transferable through `json`, `yaml` and `toml` formats are supported.
* *null* - empty value
* *number* - internally stored as either 64-bit integer or 64-bit float
* *boolean* - `true` or `false`
* *string* - strings are stored as UTF-8 encoded data.
* *binary* - binary data.
* *object* - object or map, can contain string-keyed properties
* *array* - array or sequence of elements
* *date* - **to be implemented!!**

### Literals
* `123`, `-2` - 64-bit integer values
* `1.13`, `.e10`, `-1E-2`, `.3` - 64-bit float values
* `'id'`, `"id"` - string values
* `true`, `false` - boolean values
* `null` - null value

### Type conversions
Same as ECMAScript, integers promoted to floats when mixed operands (do rozwiniecia)

### Mathematical operators
Typical mathematical operators and parentheses are supported.
* `2 + 3`, `@.count + 1` - numerical addition
* `2 - 3`, `@.count - 1` - numerical subtraction
* `2 * 3`, `@.count * 2` - numerical multiplication
* `2 / 3`, `@.count / 2` - numerical division
* `2 + 6 / 2` - yields value `5`, as expected
* `(2 + 6) / 2` - yields value `4`

Internally, type conversion is avoided as long as possible, i.e. adding two integer values will yield integer sum.

### String concatenation
If any of the addition operands has a string value, addition will become string concatenation
* `2 + "3"`, `"2" + 3` - both expressions yield string value `"23"`
* `"John" + " " + 'Doe'` - yields `"John Doe"`

### Comparison operators
* `2 > 3`
* `2 >= 3`
* `2 < 3`
* `2 <= 3`
* `2 == 3`
* `2 != 3`
* `'aaabbb' ^= 'aa'` - `true` if left string operand starts with right string operand
* `'aaabbb' *= 'aa'` - `true` if left string operand contains right string operand
* `'aaabbb' $= 'bb'` - `true` if left string operand ends with right string operand

### Logical operators
* `not true`, `!true`
* `true and true`, `true && true`
* `true or true`, `true || true`

### Number ranges
* `:10` - range from `0` (inclusive) to `10` (inclusive)
* `1:10` - range from `1` (inclusive) to `10` (inclusive)
* `0:2:10` - range from `0` (inclusive) to `10` (inclusive) with `2` increments
* `5:-0.1:-1.4` - floats in ranges are also supported
* `1..10` - range from `1` (inclusive) to `10` (inclusive)
* `..10` - range from `0` (inclusive) to `10` (inclusive)

### Context
Every `Opath` expression is executed in the context of **root** (denoted `$`) and **current** (denoted `@`) elements. To access any element in the object tree, it's relation to the **current** (`@`) or **root** (`$`) element needs to be defined, much like for paths in the filesystem are relative to the current directory, or filesystem root. For expressions based at the **current** element, explicit denotion of `@` can usually be omitted.
* `@.name` - returns the value of property "name" from the **current** element
* `name` - same as above
* `$.name` - returns the value of property "name" from the **root** element

### Indexing for arrays
Array elements can be accessed with `[]` notation. Arrays are indexed starting from `0`.
* `@[0]` - returns the first element of the **current** array
* `@[0, 1..3, 5]` - arrays can be indexed by multiple comma-separated indices as well as ranges of indices
* `@[-1,-2]` - negative indices are calculated from the end of an array, `-1` being the last element of an array
* `@[3..]` - when using ranges in array indexing expressions (inside `[]`), range ending value can be omitted, and it will be equal to the array length (number of array elements)

Accessing array with out-of-bounds index values yields empty result. Accessing array element on a non-array and non-object type yields empty result.

### Property access for objects
Properties in objects can be accessed with typical `.` or `[]` notations.
* `name` - returns the value of property "name" from the **current** element
* `@[name]` - same as above, with `[]` notation
* `[name]` - this is illegal!
* `@."name"`, `@["name"]` - property names can be quoted, and if so, can contain spaces and special characters
* `"name"` - this is string literal, not property access!
* `@.(first_name, last_name, age)` - one can select a few properties with a single expression using parentheses

Accessing an nonexistent property yields empty result. Accessing a property on a non-object type also yields empty result.

### Property indexing for objects
Every object can also be indexed as an array, where index value will correspond with property position within the object.
For example if **current** object will be:
```json
{
   "first_name": "John",
   "last_name": "Doe"
}
```
expression `@[1]` will yield string value `"Doe"` (value of the secod property). Objects have strictly defined and stable insertion order of properties.

### Property / element filtering
Properties in objects or elements in arrays can also be filtered with logical expressions inside `[]` notation.
* `@[@.@key $= "name"]` - yields **current** element property values for which property name ends with `"name"`.
* `@[@.@index >= 3]` - yields **current** element properties / elements with index greater or equal `3`

Note that inside the `[]` expression the **current** element (`@`) becomes the child of the outer element.

### Property / element access wildcard operator `*`
* `@.*`, `@[*]` - yields all properties of the **current** object or all elements of the **current** array, or empty result, depending on the **current** type
* `@.(@.star)` - if **current** has a `"star"` property with value `"*"` this will proto.work the same as above (FIXME byc moze to dzialanie trzeba bedzie zmienic)

### Property / element access recursive descent operator `**`
* `@.**`, `@[**]` - yields all properties of the **current** object, and recursively all of their properties in depth-first descending order.
* `@."**"`, `@['**']` - this will also proto.work as above.
* `@.(@.starstar)` - if **current** has a `"starstar"` property with value `"**"` this will proto.work the same as above (FIXME byc moze to dzialanie trzeba bedzie zmienic)
* `@.**{1,4}`, `@.**{,4}`, `@.**{2}`, `@.**{0,2}`- optionally depth level range can be specified. The depth level is specified relative from the element being accessed (**current** in those examples). 
If minimal depth level value is omitted, `1` is assumed.
If maximal depth level is omitted, descend operator will be unbound from the top, i.e. will continue for all descendants. 
If minimal depth level value is `0`, the result will also include accessed element itself.

### Parent access operator `^`
* `@^` - this yields parent element of the **current** element. 
* `@.name^` - if **current** element is an object and contains "name" property, this expression will yield **current** element.

### Ascendant access recursive operator `^**`
* `@^**` - yields all ascendants of the **current** element, in order of decreasing depth. The last element will be **root**.
* `@^**{1,4}`, `@^**{,4}`, `@^**{2}`- optionally recursive distance range can be specified, analogically like for `**`. The distance is specified relative from the element being accessed.
* `@^(@.starstar)` - if **current** has a `"starstar"` property with value `"**"` this will proto.work the same as above (FIXME byc moze to dzialanie trzeba bedzie zmienic)

### Metadata (attributes)
All elements contain readable metadata (attributes). Those attributes are accessed like regular properties, but with name prefixed with `@` character.
* `@.@index` - index of **current** element in its parent (if the parent is an object, this will be the property position)
* `@.@key` - property name of **current** element in its parent (for arrays this will be string value of index)
* `@.@level` - distance from the **root** element for **current** element,
* `@.@kind` - string value of **current** element's kind, either one of `"null"`, `"boolean"`, `"number"`, `"string"`, `"object"`, `"array"` FIXME (date, binary)
* `@.@file` - string describing the file or file structure, **current** element was read from (if any), for instance `"file<yaml>:./data.yml"`.
* `@.@file_type`- string with file type (if any), either `"file"` or `"dir"`
* `@.@file_format`- string with file format (if any), supported values are: `"json"`, `"yaml"`, `"toml"`, `"text"`, `"binary"` 
* `@.@file_path`- string with file path (if any), for instance `"./data.yml"`
* `@.@file_name`- string with file name (if any), for instance `"data.yml"`
* `@.@file_stem`- string with file stem (if any), for instance `"data"`. For file names starting with `"."`, like `".data.yml"` stem will be `".data"` 
* `@.@file_ext`- string with file extension (if any), for instance `"yml"`
* `@.@path` - path to the **current** element from the **root**, for instance `"$.nested.array[3]"`

## License

Licensed under either of
* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

`SPDX-License-Identifier: Apache-2.0 OR MIT`

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Copyright

Copyright (c) 2018 Kodegenix Sp. z o.o. [http://www.kodegenix.pl](http://www.kodegenix.pl)
