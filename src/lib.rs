#![feature(specialization, integer_atomics, box_syntax)]

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
