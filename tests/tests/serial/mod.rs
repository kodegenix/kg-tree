use kg_tree::NodeRef;

mod toml;

/// Helper trait for serialization testing
pub trait NodeRefExt {
    fn into_int(&self) -> i64;
    fn into_float(&self) -> f64;
    fn into_bool(&self) -> bool;
    fn into_string(&self) -> String;
    fn get_key(&self, key: &str) -> NodeRef;
    fn get_idx(&self, idx: usize) -> NodeRef;
}

impl NodeRefExt for NodeRef {
    fn into_int(&self) -> i64 {
        assert!(self.is_integer());
        self.as_integer().unwrap()
    }

    fn into_float(&self) -> f64 {
        assert!(self.is_float());
        self.as_float()
    }

    fn into_bool(&self) -> bool {
        assert!(self.is_boolean());
        self.as_boolean()
    }

    fn into_string(&self) -> String {
        assert!(self.is_string());
        self.as_string()
    }

    fn get_key(&self, key: &str) -> NodeRef {
        self.get_child_key(key).unwrap()
    }

    fn get_idx(&self, idx: usize) -> NodeRef {
        self.get_child_index(idx).unwrap()
    }
}