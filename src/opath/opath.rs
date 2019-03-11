use super::*;

use std::sync::atomic::{AtomicU64, Ordering};
use std::hash::{Hash, Hasher};

use serde::{ser, de};


#[derive(Debug)]
pub struct Opath {
    expr: Expr,
    hash: AtomicU64,
}

impl Opath {
    pub (super) fn new(e: Expr) -> Opath {
        Opath {
            expr: e,
            hash: AtomicU64::new(0),
        }
    }

    pub (super) fn expr(&self) -> &Expr {
        &self.expr
    }

    pub (super) fn into_expr(self) -> Expr {
        self.expr
    }

    pub fn parse(expr: &str) -> Result<Opath, OpathParseError> {
        use kg_io::*;

        let mut r = MemCharReader::new(expr.as_bytes());
        super::expr::parse::Parser::new().parse(&mut r)
    }

    pub fn parse_opt_delims(expr: &str, open_delim: &str, close_delim: &str) -> Result<Opath, OpathParseError> {
        let expr = expr.trim();
        let expr = if expr.starts_with(open_delim) && expr.ends_with(close_delim) {
            &expr[open_delim.len()..expr.len() - close_delim.len()]
        } else {
            expr
        };
        Self::parse(expr)
    }

    pub fn between<'a>(from: &NodeRef, to: &NodeRef) -> Opath {
        let mut n = to.clone();
        let mut seq = Vec::new();
        while !n.is_ref_eq(from) {
            let p = n.data().parent();
            if let Some(p) = p {
                match *p.data().value() {
                    Value::Array(_) => seq.push(Expr::Index(Box::new(Expr::Integer(n.data().index() as i64)))),
                    Value::Object(_) => seq.push(Expr::Property(Box::new(Expr::String(n.data().key().to_string())))),
                    _ => unreachable!(),
                }
                n = p;
            } else {
                return Opath::new(Expr::Sequence(Vec::new()));
            }
        }
        seq.reverse();
        Opath::new(Expr::Sequence(seq))
    }

    pub fn from<'a>(node: &NodeRef) -> Opath {
        let mut seq = Vec::new();
        let mut n = node.clone();
        loop {
            let p = n.data().parent();
            if let Some(p) = p {
                match *p.data().value() {
                    Value::Array(_) => seq.push(Expr::Index(Box::new(Expr::Integer(n.data().index() as i64)))),
                    Value::Object(_) => seq.push(Expr::Property(Box::new(Expr::String(n.data().key().to_string())))),
                    _ => unreachable!(),
                }
                n = p;
            } else {
                break;
            }
        }
        seq.push(Expr::Root);
        seq.reverse();
        Opath::new(Expr::Sequence(seq))
    }

    pub fn string(value: String) -> Opath {
        Opath::new(Expr::StringEnc(value))
    }

    pub fn boolean(value: bool) -> Opath {
        Opath::new(Expr::Boolean(value))
    }

    pub fn null() -> Opath {
        Opath::new(Expr::Null)
    }

    pub fn root() -> Opath {
        Opath::new(Expr::Root)
    }

    pub fn current() -> Opath {
        Opath::new(Expr::Current)
    }

    pub fn json(json: String) -> Opath {
        Opath::new(Expr::FuncCall(Box::new(FuncCall::new(FuncId::Json, vec![Expr::StringEnc(json)]))))
    }

    pub fn apply(&self, root: &NodeRef, current: &NodeRef) -> NodeSet {
        let _r = root.clone(); //(jc) additional reference to mark root as non-consumable
        self.expr.apply(Env::new(root, current, None), Context::Expr)
    }

    pub fn apply_ext(&self, root: &NodeRef, current: &NodeRef, scope: &Scope) -> NodeSet {
        let _r = root.clone(); //(jc) additional reference to mark root as non-consumable
        self.expr.apply(Env::new(root, current, Some(scope)), Context::Expr)
    }

    pub fn apply_one(&self, root: &NodeRef, current: &NodeRef) -> NodeRef {
        let _r = root.clone(); //(jc) additional reference to mark root as non-consumable
        let res = self.expr.apply(Env::new(root, current, None), Context::Expr);
        match res {
            NodeSet::Empty => NodeRef::null(),
            NodeSet::One(a) => a,
            NodeSet::Many(_) => unimplemented!(),
        }
    }

    pub fn apply_one_ext(&self, root: &NodeRef, current: &NodeRef, scope: &Scope) -> NodeRef {
        let _r = root.clone(); //(jc) additional reference to mark root as non-consumable
        let res = self.expr.apply(Env::new(root, current, Some(scope)), Context::Expr);
        match res {
            NodeSet::Empty => NodeRef::null(),
            NodeSet::One(a) => a,
            NodeSet::Many(_) => unimplemented!(),
        }
    }

    pub fn parent_path(&self) -> Option<Opath> {
        match self.expr {
            Expr::Sequence(ref seq) => {
                if seq.len() < 2 {
                    None
                } else {
                    let mut pseq = Vec::with_capacity(seq.len() - 1);
                    let mut it = seq.iter().take(seq.len() - 1);
                    if let Some(&Expr::Root) = it.next() {
                        pseq.push(Expr::Root);
                    } else {
                        return None;
                    }
                    while let Some(e) = it.next() {
                        match *e {
                            Expr::Property(ref expr) => match **expr {
                                Expr::String(_) | Expr::StringEnc(_) => pseq.push(e.clone()),
                                _ => return None,
                            }
                            Expr::Index(ref expr) => match **expr {
                                Expr::Integer(_) => pseq.push(e.clone()),
                                _ => return None,
                            }
                            _ => return None,
                        }
                    }
                    Some(Opath::new(Expr::Sequence(pseq)))
                }
            }
            _ => None,
        }
    }

    #[inline(always)]
    fn _hash_get(&self) -> u64 {
        self.hash.load(Ordering::SeqCst)
    }

    #[inline(always)]
    fn _hash_set(&self, h: u64) {
        self.hash.store(h, Ordering::SeqCst)
    }
}

impl Clone for Opath {
    fn clone(&self) -> Self {
        Opath {
            expr: self.expr.clone(),
            hash: AtomicU64::new(self._hash_get())
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.expr = source.expr.clone();
        self.hash = AtomicU64::new(source._hash_get());
    }
}

impl Default for Opath {
    fn default() -> Self {
        Opath::new(Expr::Null)
    }
}

impl std::fmt::Display for Opath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#}", self.expr)
        } else {
            write!(f, "{}", self.expr)
        }
    }
}

impl Hash for Opath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self._hash_get() {
            0 => {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                self.expr.hash(&mut hasher);
                let h = hasher.finish();
                let h = if h == 0 { std::u64::MAX } else { h };
                state.write_u64(h);
                self._hash_set(h);
            },
            h => state.write_u64(h),
        }
    }
}

impl PartialEq for Opath {
    fn eq(&self, other: &Opath) -> bool {
        self.expr == other.expr
    }
}

impl Eq for Opath {}

impl ser::Serialize for Opath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ser::Serializer {
        serializer.collect_str(&format_args!("${{{}}}", self.expr))
    }
}


struct OpathVisitor();

impl OpathVisitor {
    fn new() -> OpathVisitor {
        OpathVisitor()
    }
}

impl<'de> de::Visitor<'de> for OpathVisitor {
    type Value = Opath;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: de::Error {
        match Opath::parse_opt_delims(v, "${", "}") {
            Ok(expr) => Ok(expr),
            Err(err) => Err(de::Error::custom(err.detail())),
        }
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> where E: de::Error {
        self.visit_str(v)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: de::Error {
        self.visit_str(&v)
    }
}

impl<'de> de::Deserialize<'de> for Opath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: de::Deserializer<'de> {
        deserializer.deserialize_str(OpathVisitor::new())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod parent_path {
        use super::*;

        #[test]
        fn paths_must_start_with_root() {
            let o = Opath::parse("@.prop1.arr[3]").unwrap();
            let p = o.parent_path();
            assert!(p.is_none());

            let o = Opath::parse("$.prop1.arr[3]").unwrap();
            let p = o.parent_path();
            assert!(p.is_some());
        }

        #[test]
        fn paths_can_only_contain_property_and_integer_index_accessors() {
            let o = Opath::parse("$.*.arr[3]").unwrap();
            let p = o.parent_path();
            assert!(p.is_none());

            let o = Opath::parse("$[\"prop1\"].arr[3]").unwrap();
            let p = o.parent_path();
            assert!(p.is_none());
        }

        #[test]
        fn array_element_parent() {
            let o = Opath::parse("$.prop1.arr[3]").unwrap();
            let p = o.parent_path().unwrap();

            assert_eq!(p.to_string(), "$.prop1.arr");
        }

        #[test]
        fn property_parent() {
            let o = Opath::parse("$.prop1.prop2").unwrap();
            let p = o.parent_path().unwrap();

            assert_eq!(p.to_string(), "$.prop1");
        }
    }
}
