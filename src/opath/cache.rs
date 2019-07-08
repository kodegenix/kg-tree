use std::collections::HashMap;

use kg_utils::collections::LruCache;

use super::*;

pub trait OpathCache {
    fn get(&mut self, n: &NodeRef) -> &Opath;

    fn contains(&mut self, n: &NodeRef) -> bool;

    fn len(&self) -> usize;
}


#[derive(Debug)]
pub struct NodePathLruCache {
    cache: LruCache<*const Node, Opath>,
}

impl NodePathLruCache {
    pub fn with_size(size: usize) -> NodePathLruCache {
        NodePathLruCache {
            cache: LruCache::new(size),
        }
    }
}

impl OpathCache for NodePathLruCache {
    fn get(&mut self, n: &NodeRef) -> &Opath {
        let p = n.data_ptr();
        if !self.cache.contains_key(&p) {
            self.cache.insert(p, Opath::from(n));
        }
        self.cache.get_mut(&p).unwrap()
    }

    fn contains(&mut self, n: &NodeRef) -> bool {
        let p = n.data_ptr();
        self.cache.contains_key(&p)
    }

    fn len(&self) -> usize {
        self.cache.len()
    }
}


#[derive(Debug)]
pub struct NodePathCache {
    cache: HashMap<*const Node, Opath>,
}

impl NodePathCache {
    pub fn new() -> NodePathCache {
        NodePathCache {
            cache: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> NodePathCache {
        NodePathCache {
            cache: HashMap::with_capacity(capacity),
        }
    }
}

impl OpathCache for NodePathCache {
    fn get(&mut self, n: &NodeRef) -> &Opath {
        let p = n.data_ptr();
        if !self.cache.contains_key(&p) {
            self.cache.insert(p, Opath::from(n));
        }
        self.cache.get(&p).unwrap()
    }

    fn contains(&mut self, n: &NodeRef) -> bool {
        let p = n.data_ptr();
        self.cache.contains_key(&p)
    }

    fn len(&self) -> usize {
        self.cache.len()
    }
}
