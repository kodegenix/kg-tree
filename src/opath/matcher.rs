use super::*;

use std::collections::HashSet;


#[derive(Debug)]
pub struct NodePathMatcher {
    paths: HashSet<Opath>,
}

impl NodePathMatcher {
    pub fn new() -> NodePathMatcher {
        NodePathMatcher {
            paths: HashSet::new(),
        }
    }

    pub fn insert(&mut self, n: &NodeRef) {
        let path = Opath::from(n);
        self.paths.insert(path);
    }

    pub fn insert_cache(&mut self, n: &NodeRef, cache: &mut dyn OpathCache) {
        let path = cache.get(n).clone();
        self.paths.insert(path);
    }

    pub fn resolve(&mut self, expr: &Opath, root: &NodeRef, current: &NodeRef) {
        let res = expr.apply(root, current);
        for ref n in res {
            self.insert(n);
        }
    }

    pub fn resolve_cache(&mut self, expr: &Opath, root: &NodeRef, current: &NodeRef, cache: &mut dyn OpathCache) {
        let res = expr.apply(root, current);
        for ref n in res {
            self.insert_cache(n, cache);
        }
    }

    pub fn resolve_ext(&mut self, expr: &Opath, root: &NodeRef, current: &NodeRef, scope: &Scope) {
        let res = expr.apply_ext(root, current, scope);
        for ref n in res {
            self.insert(n);
        }
    }

    pub fn resolve_ext_cache(&mut self, expr: &Opath, root: &NodeRef, current: &NodeRef, scope: &Scope, cache: &mut dyn OpathCache) {
        let res = expr.apply_ext(root, current, scope);
        for ref n in res {
            self.insert_cache(n, cache);
        }
    }

    pub fn matches(&self, path: &Opath) -> bool {
        self.paths.contains(path)
    }

    pub fn clear(&mut self) {
        self.paths.clear();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_node<'a>() -> NodeRef {
        let jsona = r#"
        {
            "pa": "papa",
            "star": "*",
            "propa1": {
                "aa": {
                    "bb": "aaaa",
                    "dd": [12,13, 14,20,34],
                    "cc": false
                }
            }
        }"#;

        NodeRef::from_json(jsona).unwrap()
    }

    #[test]
    fn without_cache() {
        let n = test_node();

        let mut m = NodePathMatcher::new();

        let expr = Opath::parse("$.*").unwrap();
        m.resolve(&expr, &n, &n);

        assert!(m.matches(&Opath::parse("$.pa").unwrap()));
    }

    #[test]
    fn with_cache() {
        let n = test_node();

        let mut m = NodePathMatcher::new();
        let mut cache = NodePathLruCache::with_size(128);

        let expr = Opath::parse("$.*").unwrap();
        m.resolve_cache(&expr, &n, &n, &mut cache);

        assert!(m.matches(&Opath::parse("$.pa").unwrap()));
        assert_eq!(cache.len(), 3);
    }
}
