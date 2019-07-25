use kg_tree::NodeRef;

mod toml;

pub trait NodeRefExt {
    fn to_int(&self) -> i64;
    fn to_float(&self) -> f64;
    fn get_key(&self, key: &str) -> NodeRef;
    fn get_idx(&self, idx: usize) -> NodeRef;
}

impl NodeRefExt for NodeRef {
    fn to_int(&self) -> i64 {
        self.as_integer().unwrap()
    }

    fn to_float(&self) -> f64 {
        self.as_float()
    }

    fn get_key(&self, key: &str) -> NodeRef {
        self.get_child_key(key).unwrap()
    }

    fn get_idx(&self, idx: usize) -> NodeRef {
        self.get_child_index(idx).unwrap()
    }
}