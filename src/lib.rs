#![feature(specialization, integer_atomics, box_syntax, clamp, external_doc)]

#![doc(include = "../doc/doc.md")]

//!```rust
//!use kg_tree::opath::{Opath, NodeSet};
//!use kg_tree::NodeRef;
//!
//!let model = r#"{
//!  "key0": "value0",
//!  "key1": "value1",
//!  "key2": {
//!    "key20": "value20",
//!    "key21": "value21"
//!  }
//!}"#;
//!
//!let query = r#"@.**"#;
//!
//!let expr = Opath::parse(query).unwrap();
//!let node = NodeRef::from_json(model).unwrap();
//!let res = expr.apply(&node, &node).unwrap();
//!assert_eq!(res.len(), 5);
//!let nodes = res.into_vec();
//!assert_eq!(nodes[0].as_string(), "value0");
//!assert_eq!(nodes[1].as_string(), "value1");
//!assert!(nodes[2].is_object());
//!assert_eq!(nodes[2].get_child_key("key20").unwrap().as_string(), "value20");
//!assert_eq!(nodes[2].get_child_key("key21").unwrap().as_string(), "value21");
//!assert_eq!(nodes[3].as_string(), "value20");
//!assert_eq!(nodes[4].as_string(), "value21");
//!```

#[macro_use]
extern crate kg_diag_derive;
#[macro_use]
extern crate kg_display_derive;
#[macro_use]
extern crate serde_derive;

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use heapsize::HeapSizeOf;
use kg_diag::io::fs;
use kg_diag::*;
use kg_symbol::Symbol;
use kg_utils::collections::LinkedHashMap;

pub use tree::convert::Primitive;
use tree::metadata::Metadata;
pub use tree::metadata::{FileFormat, FileInfo};
pub use tree::node::{Kind, KindMask, Node, Value};
pub use tree::{NodeRef, TreeErrorDetail};

mod tree;

pub type Properties = LinkedHashMap<Symbol, NodeRef>;
pub type Elements = Vec<NodeRef>;

pub mod diff;
pub mod opath;

pub mod serial;

thread_local! {
    static BASE_PATH: RefCell<PathBuf> = RefCell::new(std::env::current_dir().unwrap());
    static BASE_PATH_STACK: RefCell<Vec<PathBuf>> = RefCell::new(Vec::new());
}

pub fn set_base_path<P: AsRef<Path> + Into<PathBuf>>(base_path: P) {
    debug_assert!(base_path.as_ref().is_absolute());
    BASE_PATH_STACK.with(|s| s.borrow_mut().clear());
    BASE_PATH.with(|b| b.replace(base_path.into()));
}

pub fn push_base_path<P: AsRef<Path> + Into<PathBuf>>(base_path: P) {
    debug_assert!(base_path.as_ref().is_absolute());
    let current_path = BASE_PATH.with(|b| b.replace(base_path.into()));
    BASE_PATH_STACK.with(|s| s.borrow_mut().push(current_path));
}

pub fn pop_base_path() -> PathBuf {
    let path = BASE_PATH_STACK.with(|s| {
        s.borrow_mut()
            .pop()
            .expect("kg_tree::pop_base_path() called on an empty stack")
    });
    BASE_PATH.with(|b| b.replace(path))
}

pub fn relative_path(path: &Path) -> &Path {
    debug_assert!(path.is_absolute());
    BASE_PATH.with(|b| unsafe {
        std::mem::transmute(path.strip_prefix(b.borrow().as_path()).unwrap_or(path))
    })
}

pub fn resolve_path(path: &Path) -> Cow<Path> {
    if path.is_absolute() {
        path.into()
    } else {
        BASE_PATH.with(|b| b.borrow().as_path().join(path).into())
    }
}

pub fn resolve_path_str(path: &str) -> Cow<Path> {
    resolve_path(Path::new(path))
}

struct PathBufHeapSize<'a>(&'a PathBuf);

impl<'a> HeapSizeOf for PathBufHeapSize<'a> {
    fn heap_size_of_children(&self) -> usize {
        unsafe { std::mem::transmute::<&PathBuf, &Vec<u8>>(self.0).heap_size_of_children() }
    }
}

pub type NodeMap = HashMap<*const Node, NodeRef>;

pub trait Remappable {
    fn remap(&mut self, node_map: &NodeMap);
}
