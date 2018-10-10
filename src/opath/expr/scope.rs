use super::*;

use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use std::convert::From;
use std::ops::Deref;



#[derive(Debug)]
struct Inner {
    func_map: HashMap<Symbol, Box<FuncCallable>>,
    method_map: HashMap<Symbol, Box<MethodCallable>>,
    var_map: HashMap<Symbol, NodeSet>,
    parent: Option<Scope>,
}

impl Inner {
    fn new() -> Self {
        Inner {
            func_map: HashMap::new(),
            method_map: HashMap::new(),
            var_map: HashMap::new(),
            parent: None,
        }
    }

    fn child(parent: Scope) -> Self {
        Inner {
            func_map: HashMap::new(),
            method_map: HashMap::new(),
            var_map: HashMap::new(),
            parent: Some(parent),
        }
    }
}

impl std::fmt::Display for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if !self.func_map.is_empty() {
            let mut keys: Vec<_> = self.func_map.keys().collect();
            keys.sort();
            for k in keys {
                write!(f, "{}()\n", k)?;
            }
        }
        if !self.method_map.is_empty() {
            let mut keys: Vec<_> = self.method_map.keys().collect();
            keys.sort();
            for k in keys {
                write!(f, ".{}()\n", k)?;
            }
        }
        if !self.var_map.is_empty() {
            let mut keys: Vec<_> = self.var_map.keys().collect();
            keys.sort();
            for k in keys {
                write!(f, "${} = {}\n", k, self.var_map[k])?;
            }
        }
        if let Some(ref parent) = self.parent {
            write!(f, "---\n")?;
            std::fmt::Display::fmt(parent, f)?;
        }
        Ok(())
    }
}


trait ScopeImpl: Sized {
    fn borrow(&self) -> Ref<Inner>;

    fn get_func(&self, func: &'_ str) -> Option<Ref<Box<FuncCallable>>> {
        if let Some(f) = self.borrow().func_map.get(func) {
            Some(Ref::map(self.borrow(), |_| unsafe { std::mem::transmute(f) }))
        } else {
            if let Some(ref p) = self.borrow().parent {
                unsafe { std::mem::transmute(p.get_func(func)) }
            } else {
                None
            }
        }
    }

    fn get_method(&self, method: &'_ str) -> Option<Ref<Box<MethodCallable>>> {
        if let Some(m) = self.borrow().method_map.get(method) {
            Some(Ref::map(self.borrow(), |_| unsafe { std::mem::transmute(m) }))
        } else {
            if let Some(ref p) = self.borrow().parent {
                unsafe { std::mem::transmute(p.get_method(method)) }
            } else {
                None
            }
        }
    }

    fn get_var(&self, var: &'_ str) -> Option<Ref<NodeSet>> {
        if let Some(v) = self.borrow().var_map.get(var) {
            Some(Ref::map(self.borrow(), |_| unsafe { std::mem::transmute(v) }))
        } else {
            if let Some(ref p) = self.borrow().parent {
                unsafe { std::mem::transmute(p.get_var(var)) }
            } else {
                None
            }
        }
    }

    fn get_var_value<T: Primitive>(&self, var: &str) -> Result<T, Error> {
        match self.get_var(var) {
            Some(v) => match *v {
                NodeSet::One(ref n) => Ok(T::get(n)),
                _ => Err(Error::Undef(line!())), //FIXME (jc): expected single result
            }
            None => Err(Error::Undef(line!())), //FIXME (jc): variable not found
        }
    }

    fn get_var_value_opt<T: Primitive>(&self, var: &str) -> Option<T> {
        match self.get_var(var) {
            Some(v) => match *v {
                NodeSet::One(ref n) => Some(T::get(n)),
                _ => None,
            }
            None => None,
        }
    }

    fn get_var_value_or_default<T: Primitive>(&self, var: &str, def: &T) -> T {
        self.get_var_value(var).unwrap_or_else(|_| def.clone())
    }

    fn get_var_value_or_empty<T: Primitive>(&self, var: &str) -> T {
        self.get_var_value(var).unwrap_or_else(|_| T::empty())
    }

    fn parent(&self) -> Option<Ref<Scope>> {
        if let Some(ref p) = self.borrow().parent {
            Some(Ref::map(self.borrow(), |_| unsafe { std::mem::transmute(p) }))
        } else {
            None
        }
    }

    fn func_names(&self) -> Vec<Symbol> {
        let mut keys: Vec<Symbol> = self.borrow().func_map.keys().cloned().collect();
        keys.sort();
        keys
    }

    fn method_names(&self) -> Vec<Symbol> {
        let mut keys: Vec<Symbol> = self.borrow().method_map.keys().cloned().collect();
        keys.sort();
        keys
    }

    fn var_names(&self) -> Vec<Symbol> {
        let mut keys: Vec<Symbol> = self.borrow().var_map.keys().cloned().collect();
        keys.sort();
        keys
    }
}


trait ScopeMutImpl: ScopeImpl + Sized {
    fn borrow_mut(&self) -> RefMut<Inner>;

    fn with_func(self, name: Symbol, func: Box<FuncCallable>) -> Self {
        self.set_func(name, func);
        self
    }

    fn with_method(self, name: Symbol, method: Box<MethodCallable>) -> Self {
        self.set_method(name, method);
        self
    }

    fn with_var(self, name: Symbol, var: NodeSet) -> Self {
        self.set_var(name, var);
        self
    }


    fn set_func(&self, name: Symbol, func: Box<FuncCallable>) {
        self.borrow_mut().func_map.insert(name, func);
    }

    fn set_method(&self, name: Symbol, method: Box<MethodCallable>) {
        self.borrow_mut().method_map.insert(name, method);
    }

    fn set_var(&self, name: Symbol, var: NodeSet) {
        self.borrow_mut().var_map.insert(name, var);
    }

    fn set_parent(&self, parent: Option<Scope>) {
        self.borrow_mut().parent = parent;
    }

    fn clear_funcs(&self) {
        self.borrow_mut().func_map.clear();
    }

    fn clear_methods(&self) {
        self.borrow_mut().method_map.clear();
    }

    fn clear_vars(&self) {
        self.borrow_mut().var_map.clear();
    }
}


#[derive(Debug, Clone)]
pub struct Scope(Rc<RefCell<Inner>>);

impl Scope {
    pub fn get_func(&self, func: &'_ str) -> Option<Ref<Box<FuncCallable>>> {
        ScopeImpl::get_func(self, func)
    }

    pub fn get_method(&self, method: &'_ str) -> Option<Ref<Box<MethodCallable>>> {
        ScopeImpl::get_method(self, method)
    }

    pub fn get_var(&self, var: &'_ str) -> Option<Ref<NodeSet>> {
        ScopeImpl::get_var(self, var)
    }

    pub fn get_var_value<T: Primitive>(&self, var: &str) -> Result<T, Error> {
        ScopeImpl::get_var_value(self, var)
    }

    pub fn get_var_value_opt<T: Primitive>(&self, var: &str) -> Option<T> {
        ScopeImpl::get_var_value_opt(self, var)
    }

    pub fn get_var_value_or_default<T: Primitive>(&self, var: &str, def: &T) -> T {
        ScopeImpl::get_var_value_or_default(self, var, def)
    }

    pub fn get_var_value_or_empty<T: Primitive>(&self, var: &str) -> T {
        ScopeImpl::get_var_value_or_empty(self, var)
    }

    pub fn func_names(&self) -> Vec<Symbol> {
        ScopeImpl::func_names(self)
    }

    pub fn method_names(&self) -> Vec<Symbol> {
        ScopeImpl::method_names(self)
    }

    pub fn var_names(&self) -> Vec<Symbol> {
        ScopeImpl::var_names(self)
    }
}

impl ScopeImpl for Scope {
    fn borrow(&self) -> Ref<Inner> {
        self.0.borrow()
    }
}

impl From<ScopeMut> for Scope {
    fn from(scope: ScopeMut) -> Self {
        Scope(scope.0)
    }
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.borrow().fmt(f)
    }
}


#[derive(Debug, Clone)]
pub struct ScopeMut(Rc<RefCell<Inner>>);

impl ScopeMut {
    pub fn new() -> Self {
        Self::wrap(Inner::new())
    }

    pub fn child(parent: Scope) -> Self {
        Self::wrap(Inner::child(parent))
    }

    fn wrap(scope: Inner) -> Self {
        ScopeMut(Rc::new(RefCell::new(scope)))
    }

    pub fn get_func(&self, func: &'_ str) -> Option<Ref<Box<FuncCallable>>> {
        ScopeImpl::get_func(self, func)
    }

    pub fn get_method(&self, method: &'_ str) -> Option<Ref<Box<MethodCallable>>> {
        ScopeImpl::get_method(self, method)
    }

    pub fn get_var(&self, var: &'_ str) -> Option<Ref<NodeSet>> {
        ScopeImpl::get_var(self, var)
    }

    pub fn get_var_value<T: Primitive>(&self, var: &str) -> Result<T, Error> {
        ScopeImpl::get_var_value(self, var)
    }

    pub fn get_var_value_opt<T: Primitive>(&self, var: &str) -> Option<T> {
        ScopeImpl::get_var_value_opt(self, var)
    }

    pub fn get_var_value_or_default<T: Primitive>(&self, var: &str, def: &T) -> T {
        ScopeImpl::get_var_value_or_default(self, var, def)
    }

    pub fn get_var_value_or_empty<T: Primitive>(&self, var: &str) -> T {
        ScopeImpl::get_var_value_or_empty(self, var)
    }

    pub fn func_names(&self) -> Vec<Symbol> {
        ScopeImpl::func_names(self)
    }

    pub fn method_names(&self) -> Vec<Symbol> {
        ScopeImpl::method_names(self)
    }

    pub fn var_names(&self) -> Vec<Symbol> {
        ScopeImpl::var_names(self)
    }

    pub fn with_func(self, name: Symbol, func: Box<FuncCallable>) -> Self {
        ScopeMutImpl::with_func(self, name, func)
    }

    pub fn with_method(self, name: Symbol, method: Box<MethodCallable>) -> Self {
        ScopeMutImpl::with_method(self, name, method)
    }

    pub fn with_var(self, name: Symbol, var: NodeSet) -> Self {
        ScopeMutImpl::with_var(self, name, var)
    }


    pub fn set_func(&self, name: Symbol, func: Box<FuncCallable>) {
        ScopeMutImpl::set_func(self, name, func)
    }

    pub fn set_method(&self, name: Symbol, method: Box<MethodCallable>) {
        ScopeMutImpl::set_method(self, name, method)
    }

    pub fn set_var(&self, name: Symbol, var: NodeSet) {
        ScopeMutImpl::set_var(self, name, var);
    }

    pub fn clear_funcs(&self) {
        ScopeMutImpl::clear_funcs(self)
    }

    pub fn clear_methods(&self) {
        ScopeMutImpl::clear_methods(self)
    }

    pub fn clear_vars(&self) {
        ScopeMutImpl::clear_vars(self)
    }

    pub fn set_parent(&self, parent: Option<Scope>) {
        ScopeMutImpl::set_parent(self, parent);
    }
}

impl ScopeImpl for ScopeMut {
    fn borrow(&self) -> Ref<Inner> {
        self.0.borrow()
    }
}

impl ScopeMutImpl for ScopeMut {
    fn borrow_mut(&self) -> RefMut<Inner> {
        self.0.borrow_mut()
    }
}

impl AsRef<Scope> for ScopeMut {
    fn as_ref(&self) -> &Scope {
        unsafe { std::mem::transmute(self) }
    }
}

impl Deref for ScopeMut {
    type Target = Scope;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl std::fmt::Display for ScopeMut {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.borrow().fmt(f)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_parent_var() {
        let s1 = ScopeMut::new();
        s1.set_var("var1".into(), NodeSet::One(NodeRef::string("value1")));

        let s2 = ScopeMut::child(s1.clone().into());

        let v = s2.get_var("var1");
        assert!(v.is_some());
        assert_eq!(v.unwrap().clone().into_one().unwrap().as_string(), "value1");
    }

    #[test]
    #[should_panic(expected = "already borrowed: BorrowMutError")]
    fn mutate_parent_while_borrowing_must_panic() {
        let parent = ScopeMut::new();
        parent.set_var("var1".into(), NodeSet::One(NodeRef::string("value1")));

        let child = ScopeMut::child(parent.clone().into());

        let v = child.get_var("var1");
        assert!(v.is_some());

        //this must panic
        parent.borrow_mut().var_map.clear();
    }

    #[test]
    fn mutate_parent_while_not_borrowing_cannot_panic() {
        let parent = ScopeMut::new();
        parent.set_var("var1".into(), NodeSet::One(NodeRef::string("value1")));

        let child = ScopeMut::child(parent.clone().into());

        {
            let v = child.get_var("var1");
            assert!(v.is_some());
        }

        child.borrow_mut().var_map.clear();
    }
}
