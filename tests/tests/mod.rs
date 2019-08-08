use super::*;
use std::path::PathBuf;
use std::ffi::OsStr;
use tempfile::TempDir;


// some of these helpers should probably be extracted to external crate.
macro_rules! write_file {
    ($path: expr, $content:expr) => {{
        use std::io::Write;
        let mut f = std::fs::File::create(&$path)
            .expect(&format!("Cannot create file: '{}'", $path.display()));
        f.write_all($content.as_bytes())
            .expect(&format!("Cannot write file: '{}'", $path.display()))
    }};
}

#[macro_export]
macro_rules! assert_detail {
    ($res: expr, $detail:ident, $variant: pat) => {
        assert_detail!($res, $detail, $variant, {})
    };
    ($res: expr, $detail:ident, $variant: pat, $block:expr) => {{
        use kg_diag::Diag;
        let err = match $res {
            Ok(ref val) => panic!("Error expected, got {:?}", val),
            Err(ref err) => err,
        };
        let det = err
            .detail()
            .downcast_ref::<$detail>()
            .expect(&format!("Cannot downcast to '{}'", stringify!($detail)));

        match det {
            $variant => {
                $block;
                (err, det)
            }
            err => panic!("Expected error {} got {:?}", stringify!($variant), err),
        }
    }};
}

/// Get absolute path to the "target" directory ("build" dir)
pub fn get_target_dir() -> PathBuf {
    let bin = std::env::current_exe().expect("exe path");
    let mut target_dir = PathBuf::from(bin.parent().expect("bin parent"));
    while target_dir.file_name() != Some(OsStr::new("target")) {
        target_dir.pop();
    }
    target_dir
}

/// Get temporary directory located in "target".
pub fn get_tmp_dir() -> (TempDir, PathBuf) {
    let target = get_target_dir();
    let resources_dir = target.join("test_resources");

    if let Err(err) = std::fs::create_dir(&resources_dir) {
        if err.kind() != std::io::ErrorKind::AlreadyExists {
            panic!("Cannot create test resources dir: {:?}", err)
        }
    }
    let dir = tempfile::tempdir_in(resources_dir).expect("Cannot create temporary dir!");
    let path = dir.path().to_path_buf();
    (dir, path)
}

/// Helper trait for testing
pub trait NodeRefExt {
    fn as_int_ext(&self) -> i64;
    fn as_float_ext(&self) -> f64;
    fn as_bool_ext(&self) -> bool;
    fn as_string_ext(&self) -> String;
    fn as_array_ext(&self) -> Vec<NodeRef>;
    fn is_empty_ext(&self) -> bool;
    fn get_key(&self, key: &str) -> NodeRef;
    fn get_idx(&self, idx: usize) -> NodeRef;
}

impl NodeRefExt for NodeRef {
    fn as_int_ext(&self) -> i64 {
        assert!(self.is_integer());
        self.as_integer().unwrap()
    }

    fn as_float_ext(&self) -> f64 {
        assert!(self.is_float());
        self.as_float()
    }

    fn as_bool_ext(&self) -> bool {
        assert!(self.is_boolean());
        self.as_boolean()
    }

    fn as_string_ext(&self) -> String {
        assert!(self.is_string());
        self.as_string()
    }

    fn as_array_ext(&self) -> Vec<NodeRef> {
        assert!(self.is_array());
        match self.data().value() {
            Value::Array(elems) => elems.clone(),
            _ => unreachable!(),
        }
    }

    fn is_empty_ext(&self) -> bool {
        self.data()
            .children_count()
            .expect("cannot get children count")
            == 0
    }

    fn get_key(&self, key: &str) -> NodeRef {
        self.get_child_key(key)
            .expect(&format!("key not found: '{}'", key))
    }

    fn get_idx(&self, idx: usize) -> NodeRef {
        self.get_child_index(idx)
            .expect(&format!("index not found: '{}'", idx))
    }
}

mod opath;
mod serial;
