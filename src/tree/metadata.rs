use std::rc::{Rc, Weak};
use std::path::Path;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use super::*;

use kg_io::FileType;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FileFormat {
    Binary,
    Text,
    Json,
    Yaml,
    Toml,
}

impl FileFormat {
    pub fn from(f: &str) -> FileFormat {
        if f.eq_ignore_ascii_case("text") || f.eq_ignore_ascii_case("txt") {
            FileFormat::Text
        } else if f.eq_ignore_ascii_case("json") {
            FileFormat::Json
        } else if f.eq_ignore_ascii_case("yaml") || f.eq_ignore_ascii_case("yml") {
            FileFormat::Yaml
        } else if f.eq_ignore_ascii_case("toml") {
            FileFormat::Toml
        } else {
            FileFormat::Binary
        }
    }
}

impl<'a> std::convert::From<&'a str> for FileFormat {
    fn from(s: &'a str) -> Self {
        FileFormat::from(s)
    }
}

impl std::str::FromStr for FileFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FileFormat::from(s))
    }
}

impl std::fmt::Display for FileFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            FileFormat::Binary => write!(f, "binary"),
            FileFormat::Text => write!(f, "text"),
            FileFormat::Json => write!(f, "json"),
            FileFormat::Yaml => write!(f, "yaml"),
            FileFormat::Toml => write!(f, "toml"),
        }
    }
}


#[derive(Debug)]
pub struct FileInfo {
    file_path: PathBuf,
    file_type: FileType,
    file_format: FileFormat,
}

impl FileInfo {
    pub fn new<P: Into<PathBuf> + AsRef<Path>>(file_path: P, file_type: FileType, file_format: FileFormat) -> Rc<FileInfo> {
        debug_assert!(file_path.as_ref().is_absolute());

        Rc::new(FileInfo {
            file_path: file_path.into(),
            file_type,
            file_format,
        })
    }

    pub fn file_path_abs(&self) -> &Path {
        &self.file_path
    }

    pub fn file_path(&self) -> &Path {
        crate::relative_path(&self.file_path)
    }

    pub fn file_type(&self) -> FileType {
        self.file_type
    }

    pub fn file_format(&self) -> FileFormat {
        self.file_format
    }

    pub fn deep_copy(&self) -> Rc<FileInfo> {
        FileInfo::new(&self.file_path, self.file_type, self.file_format)
    }
}

impl std::fmt::Display for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            match self.file_type {
                FileType::Dir => write!(f, "{}:{}", self.file_type, self.file_path.display()),
                FileType::File => write!(f, "{}<{}>:{}", self.file_type, self.file_format, self.file_path.display()),
                _ => unreachable!(),
            }
        } else {
            match self.file_type {
                FileType::Dir => write!(f, "{}:{}", self.file_type, crate::relative_path(&self.file_path).display()),
                FileType::File => write!(f, "{}<{}>:{}", self.file_type, self.file_format, crate::relative_path(&self.file_path).display()),
                _ => unreachable!(),
            }
        }
    }
}

impl PartialEq<FileInfo> for FileInfo {
    fn eq(&self, other: &FileInfo) -> bool {
        self.file_path == other.file_path
    }
}

impl Eq for FileInfo { }

impl PartialOrd<FileInfo> for FileInfo {
    fn partial_cmp(&self, other: &FileInfo) -> Option<Ordering> {
        self.file_path.partial_cmp(&other.file_path)
    }
}

impl Ord for FileInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file_path.cmp(&other.file_path)
    }
}

impl Hash for FileInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file_path.hash(state);
    }
}

impl HeapSizeOf for FileInfo {
    fn heap_size_of_children(&self) -> usize {
        PathBufHeapSize(&self.file_path).heap_size_of_children()
    }
}


#[derive(Debug)]
pub struct Metadata {
    parent: Option<Weak<RefCell<Node>>>,
    index: usize,
    key: Symbol,
    file: Option<Rc<FileInfo>>,
}

impl Metadata {
    pub (super) fn new() -> Metadata {
        Metadata {
            parent: None,
            index: 0,
            key: Symbol::default(),
            file: None,
        }
    }

    pub fn parent(&self) -> Option<NodeRef> {
        match self.parent {
            Some(ref p) => Some(NodeRef::wrap(p.upgrade().unwrap())),
            None => None,
        }
    }

    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    pub fn set_parent(&mut self, p: Option<&NodeRef>) {
        self.parent = p.map(|p| Rc::downgrade(p.unwrap()));
    }

    pub fn key(&self) -> &str {
        self.key.as_ref()
    }

    pub fn set_key(&mut self, key: Cow<str>) {
        self.key = Symbol::from(key);
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn file(&self) -> Option<&Rc<FileInfo>> {
        self.file.as_ref()
    }

    pub fn set_file(&mut self, file: Option<Rc<FileInfo>>) {
        self.file = file;
    }

    pub (super) fn detach(&mut self) {
        self.parent = None;
        self.index = 0;
        self.key = Symbol::default();
    }

    pub (super) fn deep_copy(&self) -> Metadata {
        Metadata {
            parent: None,
            index: 0,
            key: Symbol::default(),
            file: self.file.as_ref().map(|f| f.deep_copy()),
        }
    }
}

impl HeapSizeOf for Metadata {
    fn heap_size_of_children(&self) -> usize {
        0
    }
}
