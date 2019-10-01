use std::hash::{Hash, Hasher};

use serde::{de, ser};

use super::*;

#[derive(Debug)]
pub struct Opath {
    expr: Expr,
}


impl Opath {
    pub(super) fn new(expr: Expr) -> Opath {
        Opath { expr }
    }

    pub(super) fn expr(&self) -> &Expr {
        &self.expr
    }

    pub(super) fn into_expr(self) -> Expr {
        self.expr
    }

    pub fn parse(expr: &str) -> Result<Opath, OpathParseError> {
        let mut r = MemCharReader::new(expr.as_bytes());
        super::expr::parse::Parser::new().parse(&mut r)
    }

    pub fn parse_opt_delims(
        expr: &str,
        open_delim: &str,
        close_delim: &str,
    ) -> Result<Opath, OpathParseError> {
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
        let mut seg = Vec::new();
        while !n.is_ref_eq(from) {
            let p = n.data().parent();
            if let Some(p) = p {
                match *p.data().value() {
                    Value::Array(_) => seg.push(PathSegment::Index(n.data().index())),
                    Value::Object(_) => seg.push(PathSegment::Key(Id::new(n.data().key()))),
                    _ => unreachable!(),
                }
                n = p;
            } else {
                return Opath::new(Expr::Sequence(Vec::new()));
            }
        }
        seg.reverse();
        Opath::new(Expr::Path(seg))
    }

    pub fn from<'a>(node: &NodeRef) -> Opath {
        let mut seg = Vec::new();
        let mut n = node.clone();
        loop {
            let p = n.data().parent();
            if let Some(p) = p {
                match *p.data().value() {
                    Value::Array(_) => seg.push(PathSegment::Index(n.data().index())),
                    Value::Object(_) => seg.push(PathSegment::Key(Id::new(n.data().key()))),
                    _ => unreachable!(),
                }
                n = p;
            } else {
                break;
            }
        }
        seg.reverse();
        Opath::new(Expr::Path(seg))
    }

    pub fn string(value: String) -> Opath {
        Opath::new(Expr::String(value))
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
        Opath::new(Expr::FuncCall(Box::new(FuncCall::new(
            FuncId::Json,
            vec![Expr::String(json)],
        ))))
    }

    fn apply_env(&self, env: Env) -> ExprResult<NodeSet> {
        let _r = env.root().clone(); //(jc) additional reference to mark root as non-consumable
        self.expr.apply(env, Context::Expr)
    }

    pub fn apply(&self, root: &NodeRef, current: &NodeRef) -> ExprResult<NodeSet> {
        self.apply_env(Env::new(root, current, None))
    }

    pub fn apply_ext(&self, root: &NodeRef, current: &NodeRef, scope: &Scope) -> ExprResult<NodeSet> {
        self.apply_env(Env::new(root, current, Some(scope)))
    }

    pub fn apply_ext_diff(
        &self,
        root: &NodeRef,
        current: &NodeRef,
        scope: &Scope,
        old_root: &NodeRef,
        diff: &NodeDiff,
    ) -> ExprResult<NodeSet> {
        self.apply_env(Env::new(root, current, Some(scope)).with_diff(old_root, diff))
    }

    pub fn apply_one(&self, root: &NodeRef, current: &NodeRef) -> ExprResult<NodeRef> {
        let ns = self.apply_env(Env::new(root, current, None))?;
        let res = match ns {
            NodeSet::Empty => unimplemented!(), //FIXME (jc) report error here
            NodeSet::One(a) => a,
            NodeSet::Many(_) => unimplemented!(), //FIXME (jc) report error here
        };
        Ok(res)
    }

    pub fn apply_one_ext(
        &self,
        root: &NodeRef,
        current: &NodeRef,
        scope: &Scope,
    ) -> ExprResult<NodeRef> {
        let ns = self.apply_env(Env::new(root, current, Some(scope)))?;
        let res = match ns {
            NodeSet::Empty => unimplemented!(), //FIXME (jc)
            NodeSet::One(a) => a,
            NodeSet::Many(_) => unimplemented!(), //FIXME (jc)
        };
        Ok(res)
    }

    pub fn is_path(&self) -> bool {
        match self.expr {
            Expr::Root | Expr::Path(_) => true,
            _ => false,
        }
    }

    pub fn path_len(&self) -> usize {
        match self.expr {
            Expr::Root => 0,
            Expr::Path(ref segments) => segments.len(),
            _ => panic!("not a path"),
        }
    }

    pub fn parent_path(&self) -> Option<Opath> {
        match self.expr {
            Expr::Path(ref seg) => {
                if seg.len() > 1 {
                    Some(Opath::new(Expr::Path(seg.iter().cloned().take(seg.len() - 1).collect())))
                } else {
                    Some(Opath::new(Expr::Root))
                }
            },
            _ => None,
        }
    }

    pub fn is_ancestor_path(&self, other: &Opath) -> bool {
        match (&self.expr, &other.expr) {
            (&Expr::Root, &Expr::Path(_)) => true,
            (&Expr::Path(ref a), &Expr::Path(ref b)) => {
                if a.len() >= b.len() {
                    false
                } else {
                    for (a, b) in a.iter().zip(b.iter()) {
                        if a != b {
                            return false;
                        }
                    }
                    true
                }
            }
            _ => false,
        }
    }
}

impl Clone for Opath {
    fn clone(&self) -> Self {
        Opath {
            expr: self.expr.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.expr = source.expr.clone();
    }
}

impl Default for Opath {
    fn default() -> Self {
        Opath::new(Expr::Null)
    }
}

impl std::fmt::Display for Opath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.expr, f)
    }
}

impl Hash for Opath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.expr.hash(state)
    }
}

impl PartialEq for Opath {
    fn eq(&self, other: &Opath) -> bool {
        self.expr == other.expr
    }
}

impl Eq for Opath {}

impl From<Vec<PathSegment>> for Opath {
    fn from(segments: Vec<PathSegment>) -> Self {
        if segments.is_empty() {
            Opath::new(Expr::Root)
        } else {
            Opath::new(Expr::Path(segments))
        }
    }
}

impl Into<Vec<PathSegment>> for Opath {
    fn into(self) -> Vec<PathSegment> {
        match self.expr {
            Expr::Root => Vec::new(),
            Expr::Path(segments) => segments,
            _ => panic!("not a path"),
        }
    }
}

impl ser::Serialize for Opath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
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

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match Opath::parse_opt_delims(v, "${", "}") {
            Ok(expr) => Ok(expr),
            Err(err) => Err(de::Error::custom(err.detail())),
        }
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(v)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(&v)
    }
}

impl<'de> de::Deserialize<'de> for Opath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
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
        fn paths_can_only_contain_simple_property_and_index_accessors() {
            let o = Opath::parse("$.*.arr[3]").unwrap();
            let p = o.parent_path();
            assert!(p.is_none());

            let o = Opath::parse("$[\"prop1\"]^.arr[3]").unwrap();
            let p = o.parent_path();
            assert!(p.is_none());

            let o = Opath::parse("$[\"prop1\"].arr[3]").unwrap();
            let p = o.parent_path();
            assert!(p.is_some());
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
