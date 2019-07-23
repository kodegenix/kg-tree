use super::*;

pub trait Primitive: Clone {
    fn get(node: &NodeRef) -> Self;

    fn empty() -> Self;
}

impl Primitive for String {
    fn get(node: &NodeRef) -> Self {
        node.as_string()
    }

    fn empty() -> Self {
        String::new()
    }
}

impl Primitive for PathBuf {
    fn get(node: &NodeRef) -> Self {
        PathBuf::from(node.as_string())
    }

    fn empty() -> Self {
        PathBuf::new()
    }
}

impl Primitive for f64 {
    fn get(node: &NodeRef) -> Self {
        node.as_float()
    }

    fn empty() -> Self {
        std::f64::NAN
    }
}

impl Primitive for bool {
    fn get(node: &NodeRef) -> Self {
        node.as_boolean()
    }

    fn empty() -> Self {
        false
    }
}
