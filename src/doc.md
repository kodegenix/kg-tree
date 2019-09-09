# Test
Test test.

```
use std::path::PathBuf;
use op_model::resolve_model_path;

// path relative to current dir
let p = resolve_model_path("./some_file.yaml", "current_dir", "/abs/path/model_dir");
assert_eq!(PathBuf::from("/abs/path/model_dir/current_dir/some_file.yaml"), p);

// path relative to model_dir
let p = resolve_model_path("some_file.yaml", "current_dir", "/abs/path/model_dir");
assert_eq!(PathBuf::from("/abs/path/model_dir/some_file.yaml"), p);

// absolute path
let p = resolve_model_path("/some/abs/path/some_file.yaml", "current_dir", "/abs/path/model_dir");
assert_eq!(PathBuf::from("/some/abs/path/some_file.yaml"), p);
```
