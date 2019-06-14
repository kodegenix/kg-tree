use super::*;

use super::opath::Opath;

pub mod node;
pub mod metadata;
pub mod convert;

use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use std::cmp::{PartialEq, PartialOrd, Ordering};
use std::path::Path;

use serde::ser;
use serde::ser::{SerializeSeq, SerializeMap};
use serde::de;



//FIXME (jc) error handling
#[derive(Debug)]
pub enum ErrorKind {
    Io,
    Undef(u32),
}

impl From<kg_io::error::IoError> for ErrorKind {
    fn from(err: kg_io::error::IoError) -> Self {
        eprintln!("{}", err);
        ErrorKind::Io
    }
}


#[derive(Debug)]
pub struct NodeRef(Rc<RefCell<Node>>);

impl NodeRef {
    pub fn null() -> NodeRef {
        NodeRef::new(Node::new(
            Metadata::new(),
            Value::Null,
        ))
    }

    pub fn boolean(b: bool) -> NodeRef {
        NodeRef::new(Node::new(
            Metadata::new(),
            Value::Boolean(b),
        ))
    }

    pub fn integer(n: i64) -> NodeRef {
        NodeRef::new(Node::new(
            Metadata::new(),
            Value::Integer(n),
        ))
    }

    pub fn float(n: f64) -> NodeRef {
        NodeRef::new(Node::new(
            Metadata::new(),
            Value::Float(n),
        ))
    }

    pub fn string<S: Into<String>>(s: S) -> NodeRef {
        NodeRef::new(Node::new(
            Metadata::new(),
            Value::String(s.into()),
        ))
    }

    pub fn binary<B: Into<Vec<u8>>>(b: B) -> NodeRef {
        NodeRef::new(Node::new(
            Metadata::new(),
            Value::Binary(b.into()),
        ))
    }

    pub fn array(elems: Elements) -> NodeRef {
        NodeRef::new(Node::new(
            Metadata::new(),
            Value::Array(elems),
        ))
    }

    pub fn object(props: Properties) -> NodeRef {
        NodeRef::new(Node::new(
            Metadata::new(),
            Value::Object(props),
        ))
    }

    fn new(n: Node) -> NodeRef {
        let n = NodeRef(Rc::new(RefCell::new(n)));
        n.update_children_metadata();
        n
    }

    pub (crate) fn wrap(n: Rc<RefCell<Node>>) -> NodeRef {
        NodeRef(n)
    }

    pub (crate) fn unwrap(&self) -> &Rc<RefCell<Node>> {
        &self.0
    }

    pub fn is_consumable(&self) -> bool {
        self.data().is_root() && Rc::strong_count(&self.0) == 1
    }

    pub fn into_consumable(self) -> NodeRef {
        if self.is_consumable() {
            self
        } else {
            self.deep_copy()
        }
    }

    pub (crate) fn update_children_metadata(&self) {
        match *self.data_mut().value_mut() {
            Value::Array(ref mut elems) => {
                for (i, n) in elems.iter_mut().enumerate() {
                    let mut nd = n.data_mut();
                    let m = nd.metadata_mut();
                    m.set_key(i.to_string().into());
                    m.set_index(i);
                    m.set_parent(Some(&self));
                }
            }
            Value::Object(ref mut props) => {
                for (i, (k, n)) in props.iter_mut().enumerate() {
                    let mut nd = n.data_mut();
                    let m = nd.metadata_mut();
                    m.set_key(k.as_ref().into());
                    m.set_index(i);
                    m.set_parent(Some(&self));
                }
            }
            _ => {}
        }
    }

    pub fn from_type<T>(value: T) -> Result<NodeRef, serial::Error> where T: serde::Serialize {
        serial::to_tree(&value)
    }

    pub fn from_json(s: &str) -> Result<NodeRef, serde_json::Error> {
        serde_json::from_str(s)
    }

    pub fn from_yaml(s: &str) -> Result<NodeRef, serde_yaml::Error> {
        serde_yaml::from_str(s)
    }

    pub fn from_toml(s: &str) -> Result<NodeRef, toml::de::Error> {
        toml::from_str(s)
    }

    //FIXME (jc) error handling
    pub fn from_str(s: Cow<'_, str>, format: FileFormat) -> Result<NodeRef, ErrorKind> {
        match format {
            FileFormat::Json => match NodeRef::from_json(&s) {
                Ok(n) => Ok(n),
                Err(err) => {
                    println!("{}", err);
                    Err(ErrorKind::Undef(line!()))
                }
            }
            FileFormat::Yaml => match NodeRef::from_yaml(&s) {
                Ok(n) => Ok(n),
                Err(err) => {
                    println!("{}", err);
                    Err(ErrorKind::Undef(line!()))
                }
            }
            FileFormat::Toml => match NodeRef::from_toml(&s) {
                Ok(n) => Ok(n),
                Err(err) => {
                    println!("{}", err);
                    Err(ErrorKind::Undef(line!()))
                }
            }
            FileFormat::Text => Ok(NodeRef::string(s)),
            FileFormat::Binary => Ok(NodeRef::binary(s.as_bytes())),
        }
    }

    //FIXME (jc) error handling
    pub fn from_bytes(s: &[u8], format: FileFormat) -> Result<NodeRef, ErrorKind> {
        fn to_str(s: &[u8]) -> Result<&str, ErrorKind> {
            match std::str::from_utf8(s) {
                Ok(s) => Ok(s),
                Err(err) => {
                    println!("{}", err);
                    Err(ErrorKind::Undef(line!()))
                }
            }
        }
        match format {
            FileFormat::Json => match NodeRef::from_json(to_str(s)?) {
                Ok(n) => Ok(n),
                Err(err) => {
                    println!("{}", err);
                    Err(ErrorKind::Undef(line!()))
                }
            }
            FileFormat::Yaml => match NodeRef::from_yaml(to_str(s)?) {
                Ok(n) => Ok(n),
                Err(err) => {
                    println!("{}", err);
                    Err(ErrorKind::Undef(line!()))
                }
            }
            FileFormat::Toml => match NodeRef::from_toml(to_str(s)?) {
                Ok(n) => Ok(n),
                Err(err) => {
                    println!("{}", err);
                    Err(ErrorKind::Undef(line!()))
                }
            }
            FileFormat::Text => Ok(NodeRef::string(to_str(s)?)),
            FileFormat::Binary => Ok(NodeRef::binary(s)),
        }
    }

    //FIXME (jc) error handling
    pub fn from_file(file_path: &Path, format: Option<FileFormat>) -> Result<NodeRef, ErrorKind> {
        use kg_io::*;

        let file_path_ = if file_path.is_absolute() {
            fs::canonicalize(file_path)?
        } else {
            fs::canonicalize(fs::current_dir()?.join(file_path))?
        };

        let format = match format {
            Some(f) => f,
            None => file_path_.extension().map_or(FileFormat::Text, |ext| FileFormat::from(ext.to_str().unwrap())),
        };

        let mut s = String::new();
        fs::read_to_string(&file_path, &mut s)?;
        let n = NodeRef::from_str(s.into(), format)?;
        n.data_mut().set_file(Some(&FileInfo::new(&file_path_,kg_io::FileType::File, format)));
        Ok(n)
    }

    pub fn to_type<'de, T>(&self) -> Result<T, serial::Error> where T: serde::Deserialize<'de> {
        serial::from_tree(self)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Node should be always serializable")
    }

    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).expect("Node should be always serializable")
    }

    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).expect("Node should be always serializable")
    }

    pub fn to_toml(&self) -> String {
        toml::to_string(self).expect("Node should be always serializable")
    }

    pub fn to_format(&self, format: FileFormat, pretty: bool) -> String {
        match format {
            FileFormat::Binary | FileFormat::Text => self.as_string(),
            FileFormat::Json => if pretty {
                self.to_json_pretty()
            } else {
                self.to_json()
            }
            FileFormat::Toml => self.to_toml(),
            FileFormat::Yaml => self.to_yaml()
        }
    }

    pub fn data(&self) -> Ref<Node> {
        self.0.borrow()
    }

    pub fn data_mut(&self) -> RefMut<Node> {
        self.0.borrow_mut()
    }

    pub fn data_ptr(&self) -> *const Node {
        self.0.as_ptr() as *const Node
    }

    pub fn path(&self) -> Opath {
        Opath::from(self)
    }

    pub fn into_string(self) -> String {
        match Rc::try_unwrap(self.0) {
            Ok(data) => data.into_inner().into_string(),
            Err(n) => n.borrow().as_string().to_string(),
        }
    }

    pub fn as_string(&self) -> String {
        self.data().as_string().to_string()
    }

    pub fn as_integer(&self) -> Option<i64> {
        self.data().as_integer()
    }

    pub fn as_float(&self) -> f64 {
        self.data().as_float()
    }

    pub fn as_binary(&self) -> Option<Vec<u8>> {
        self.data().as_binary().map(|b| b.to_vec())
    }

    pub fn as_boolean(&self) -> bool {
        self.data().as_boolean()
    }

    pub fn is_integer(&self) -> bool {
        self.data().is_integer()
    }

    pub fn is_float(&self) -> bool {
        self.data().is_float()
    }

    pub fn is_number(&self) -> bool {
        self.data().is_number()
    }

    pub fn is_object(&self) -> bool {
        self.data().is_object()
    }

    pub fn is_string(&self) -> bool {
        self.data().is_string()
    }

    pub fn is_array(&self) -> bool {
        self.data().is_array()
    }

    pub fn is_binary(&self) -> bool {
        self.data().is_binary()
    }

    pub fn is_boolean(&self) -> bool {
        self.data().is_boolean()
    }

    pub fn is_null(&self) -> bool {
        self.data().is_null()
    }

    pub fn is_parent(&self) -> bool {
        self.data().is_parent()
    }

    pub fn get_child_index(&self, index: usize) -> Option<NodeRef> {
        match *self.data().value() {
            Value::Array(ref elems) => {
                elems.get(index).cloned()
            }
            Value::Object(ref props) => {
                props.values().nth(index).cloned()
            }
            _ => None,
        }
    }

    pub fn get_child_key(&self, key: &str) -> Option<NodeRef> {
        use std::str::FromStr;

        match *self.data().value() {
            Value::Array(ref elems) => {
                match usize::from_str(key) {
                    Ok(index) => elems.get(index).cloned(),
                    Err(_) => None,
                }
            }
            Value::Object(ref props) => {
                props.get(key).cloned()
            }
            _ => None,
        }
    }

    pub fn add_child(&self, index: Option<usize>, key: Option<Symbol>, value: NodeRef) -> Result<Option<NodeRef>, ErrorKind> {
        let n = match *self.data_mut().value_mut() {
            Value::Array(ref mut elems) => {
                match index {
                    Some(i) => elems.insert(i, value),
                    None => elems.push(value),
                }
                None
            }
            Value::Object(ref mut props) => {
                if let Some(k) = key {
                    match index {
                        Some(i) => props.insert_at(i, k, value),
                        None => props.insert(k, value),
                    }
                } else {
                    None
                }
            }
            _ => return Err(ErrorKind::Undef(line!())), // invalid node type
        };

        self.update_children_metadata();

        if let Some(ref n) = n {
            n.data_mut().metadata_mut().detach();
        }
        Ok(n)
    }

    pub fn add_children<'a, I>(&self, drop: bool, mut items: I) -> Result<Vec<NodeRef>, ErrorKind>
        where I: Iterator<Item = (Option<usize>, Option<Symbol>, NodeRef)> {
        let mut res = Vec::new();

        match *self.data_mut().value_mut() {
            Value::Array(ref mut elems) => {
                while let Some((index, _, value)) = items.next() {
                    match index {
                        Some(i) => elems.insert(i, value),
                        None => elems.push(value),
                    }
                }
            }
            Value::Object(ref mut props) => {
                while let Some((index, key, value)) = items.next() {
                    let n = if let Some(k) = key {
                        match index {
                            Some(i) => props.insert_at(i, k, value),
                            None => props.insert(k, value),
                        }
                    } else {
                        None
                    };

                    if !drop {
                        if let Some(n) = n {
                            n.data_mut().metadata_mut().detach();
                            res.push(n);
                        }
                    }
                }
            }
            _ => return Err(ErrorKind::Undef(line!())), // invalid node type
        }

        self.update_children_metadata();

        Ok(res)
    }


    pub fn remove_child(&self, index: Option<usize>, key: Option<Cow<'_, str>>) -> Result<Option<NodeRef>, ErrorKind> {
        let n = match *self.data_mut().value_mut() {
            Value::Array(ref mut elems) => {
                match index {
                    Some(i) => {
                        if i < elems.len() {
                            Some(elems.remove(i))
                        } else {
                            None
                        }
                    }
                    None => elems.pop(),
                }
            }
            Value::Object(ref mut props) => {
                if let Some(k) = key {
                    props.remove(k.as_ref())
                } else if let Some(i) = index {
                    props.remove_at(i)
                } else {
                    None
                }
            }
            _ => return Err(ErrorKind::Undef(line!())), // invalid node type
        };

        if let Some(ref n) = n {
            n.data_mut().metadata_mut().detach();
            self.update_children_metadata();
        }
        Ok(n)
    }

    pub fn remove_children<'a, I>(&self, drop: bool, mut items: I) -> Result<Vec<NodeRef>, ErrorKind>
        where I: Iterator<Item = (Option<usize>, Option<Cow<'a, str>>)> {
        let mut res = Vec::new();

        match *self.data_mut().value_mut() {
            Value::Array(ref mut elems) => {
                while let Some((index, _)) = items.next() {
                    let n = match index {
                        Some(i) => {
                            if i < elems.len() {
                                Some(elems.remove(i))
                            } else {
                                None
                            }
                        }
                        None => elems.pop(),
                    };

                    if !drop {
                        if let Some(n) = n {
                            n.data_mut().metadata_mut().detach();
                            res.push(n);
                        }
                    }
                }
            }
            Value::Object(ref mut props) => {
                while let Some((index, key)) = items.next() {
                    let n = if let Some(k) = key {
                        props.remove(k.as_ref())
                    } else if let Some(i) = index {
                        props.remove_at(i)
                    } else {
                        None
                    };

                    if !drop {
                        if let Some(n) = n {
                            n.data_mut().metadata_mut().detach();
                            res.push(n);
                        }
                    }
                }
            }
            _ => return Err(ErrorKind::Undef(line!())), // invalid node type
        };

        self.update_children_metadata();

        Ok(res)
    }

    #[inline]
    fn extend_internal(&self, o: NodeRef, index: Option<usize>) -> Result<bool, ErrorKind> {
        if !self.is_ref_eq(&o) {
            let mut n = self.data_mut();
            let mut o = o.data_mut();
            match (n.value_mut(), o.value_mut()) {
                (&mut Value::Array(ref mut nelems), &mut Value::Array(ref mut oelems)) => {
                    match index {
                        Some(i) if i < nelems.len() => {
                            nelems.reserve(oelems.len());
                            let mut rest = nelems.split_off(i);
                            nelems.append(oelems);
                            nelems.append(&mut rest);
                        }
                        _ => nelems.append(oelems),
                    }
                }
                (&mut Value::Object(ref mut nprops), &mut Value::Object(ref mut oprops)) => {
                    nprops.reserve(oprops.len());
                    match index {
                        Some(mut i) if i < nprops.len() => {
                            while let Some((k, v)) = oprops.pop_front() {
                                nprops.insert_at(i, k, v);
                                i += 1;
                            }
                        }
                        _ => {
                            while let Some((k, v)) = oprops.pop_front() {
                                nprops.insert(k, v);
                            }
                        }
                    }
                }
                _ => return Err(ErrorKind::Undef(line!())), // incompatible types
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn extend(&self, o: NodeRef, index: Option<usize>) -> Result<(), ErrorKind> {
        if self.extend_internal(o, index)? {
            self.update_children_metadata();
        }
        Ok(())
    }

    pub fn extend_multiple<I>(&self, mut extends: I) -> Result<(), ErrorKind>
        where I: Iterator<Item = (NodeRef, Option<usize>)> {
        let mut updated = false;

        while let Some((o, index)) = extends.next() {
            updated |= self.extend_internal(o, index)?;
        }

        if updated {
            self.update_children_metadata();
        }

        Ok(())
    }


    pub fn is_ref_eq(&self, other: &NodeRef) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }

    pub fn deep_copy(&self) -> NodeRef {
        NodeRef::new(self.data().deep_copy())
    }

    pub fn visit_recursive<F>(&self, mut visitor: F) where F: FnMut(&NodeRef, &NodeRef, &NodeRef) -> bool {
        fn visit<'a, F>(r: &NodeRef, p: &NodeRef, n: &NodeRef, visitor: &mut F) -> bool
            where F: FnMut(&NodeRef, &NodeRef, &NodeRef) -> bool {
            if visitor(r, p, n) {
                match *n.data().value() {
                    Value::Array(ref elems) => {
                        for e in elems.iter() {
                            if !visit(r, n,e, visitor) {
                                return false;
                            }
                        }
                    },
                    Value::Object(ref props) => {
                        for e in props.values() {
                            if !visit(r, n,e, visitor) {
                                return false;
                            }
                        }
                    },
                    _ => {},
                }
                true
            } else {
                false
            }
        }

        visit(self, self, self, &mut visitor);
    }

    pub fn visit_children<F>(&self, mut visitor: F) -> bool
        where F: FnMut(&NodeRef, &NodeRef) -> bool
    {
        match *self.data().value() {
            Value::Array(ref elems) => {
                for e in elems.iter() {
                    if !visitor(self, e) {
                        break;
                    }
                }
                true
            }
            Value::Object(ref props) => {
                for e in props.values() {
                    if !visitor(self, e) {
                        break;
                    }
                }
                true
            }
            _ => false,
        }
    }

    pub fn heap_size(&self) -> usize {
        use std::collections::HashSet;

        let mut size = self.heap_size_of_children();
        let mut fmap: HashSet<*const FileInfo> = HashSet::new();
        self.visit_recursive(|_r, _p, n| {
            let nd = n.data();
            if let Some(f) = nd.metadata().file() {
                fmap.insert(Rc::into_raw(f.clone()));
            }
            true
        });

        for f in fmap.into_iter() {
            size += unsafe { heapsize::heap_size_of(f) };
            let f = unsafe { Rc::from_raw(f) };
            size += f.heap_size_of_children()
        }
        size
    }
}

impl<'a> Clone for NodeRef {
    fn clone(&self) -> Self {
        NodeRef(self.0.clone())
    }
}

impl<'a> PartialEq for NodeRef {
    fn eq(&self, other: &NodeRef) -> bool {
        let a = self.data();
        let b = other.data();
        match (a.value(), b.value()) {
            (&Value::Null, &Value::Null) => true,
            (&Value::Null, _) => false,
            (_, &Value::Null) => false,
            (&Value::Object(_), &Value::Object(_)) => self.is_ref_eq(other),
            (&Value::Array(_), &Value::Array(_)) => self.is_ref_eq(other),
            (&Value::String(ref a), &Value::String(ref b)) => a == b,
            (&Value::String(ref a), _) => a == b.as_string().as_ref(),
            (_, &Value::String(ref b)) => a.as_string().as_ref() == b,
            (&Value::Boolean(a), &Value::Boolean(b)) => a == b,
            (&Value::Boolean(a), _) => a == b.as_boolean(),
            (_, &Value::Boolean(b)) => a.as_boolean() == b,
            (&Value::Float(a), &Value::Float(b)) => a == b,
            (&Value::Float(a), _) => a == b.as_float(),
            (_, &Value::Float(b)) => a.as_float() == b,
            (&Value::Integer(a), &Value::Integer(b)) => a == b,
            (_, _) => false,
        }
    }
}

impl<'a> PartialOrd for NodeRef {
    fn partial_cmp(&self, other: &NodeRef) -> Option<Ordering> {
        let a = self.data();
        let b = other.data();
        match (a.value(), b.value()) {
            (&Value::Float(a), &Value::Float(b)) => a.partial_cmp(&b),
            (&Value::Float(a), _) => a.partial_cmp(&b.as_float()),
            (_, &Value::Float(b)) => a.as_float().partial_cmp(&b),
            (&Value::Integer(a), &Value::Integer(b)) => a.partial_cmp(&b),
            (&Value::String(ref a), &Value::String(ref b)) => a.partial_cmp(b),
            (_, _) => a.as_float().partial_cmp(&b.as_float()),
        }
    }
}

impl<'a> std::fmt::Display for NodeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "<{:p},{},{}> {:#}", self.data_ptr(), Rc::strong_count(&self.0), Rc::weak_count(&self.0), self.data())
        } else {
            write!(f, "{}", self.data())
        }
    }
}

impl<'a> ser::Serialize for NodeRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ser::Serializer {
        match *self.data().value() {
            Value::Null => serializer.serialize_none(),
            Value::Boolean(b) => serializer.serialize_bool(b),
            Value::Integer(n) => serializer.serialize_i64(n),
            Value::Float(n) => serializer.serialize_f64(n),
            Value::String(ref s) => serializer.serialize_str(s),
            Value::Binary(ref b) => serializer.serialize_bytes(b),
            Value::Array(ref elems) => {
                let mut seq = serializer.serialize_seq(Some(elems.len()))?;
                for e in elems.iter() {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
            Value::Object(ref props) => {
                let mut map = serializer.serialize_map(Some(props.len()))?;
                for (k, e) in props.iter() {
                    map.serialize_entry(k, e)?;
                }
                map.end()
            }
        }
    }
}


struct NodeVisitor;

impl NodeVisitor {
    fn new() -> NodeVisitor {
        NodeVisitor
    }
}

impl<'de> de::Visitor<'de> for NodeVisitor {
    type Value = NodeRef;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "any data")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> where E: de::Error {
        Ok(NodeRef::boolean(v))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E> where E: de::Error {
        Ok(NodeRef::integer(v as i64))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E> where E: de::Error {
        Ok(NodeRef::integer(v as i64))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where E: de::Error {
        Ok(NodeRef::integer(v as i64))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: de::Error {
        Ok(NodeRef::integer(v as i64))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E> where E: de::Error {
        Ok(NodeRef::integer(v as i64))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E> where E: de::Error {
        Ok(NodeRef::integer(v as i64))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> where E: de::Error {
        Ok(NodeRef::integer(v as i64))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: de::Error {
        Ok(NodeRef::integer(v as i64))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E> where E: de::Error {
        Ok(NodeRef::float(v as f64))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> where E: de::Error {
        Ok(NodeRef::float(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: de::Error {
        Ok(NodeRef::string(v))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> where E: de::Error {
        Ok(NodeRef::string(v))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: de::Error {
        Ok(NodeRef::string(v))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> where E: de::Error {
        Ok(NodeRef::null())
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> where E: de::Error {
        Ok(NodeRef::null())
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: de::SeqAccess<'de> {
        let mut elems = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(value) = seq.next_element()? {
            elems.push(value);
        }
        Ok(NodeRef::array(elems))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: de::MapAccess<'de> {
        let mut props = Properties::with_capacity(map.size_hint().unwrap_or(0));
        while let Some((key, value)) = map.next_entry()? {
            props.insert(key, value);
        }
        Ok(NodeRef::object(props))
    }
}

impl<'de> de::Deserialize<'de> for NodeRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: de::Deserializer<'de> {
        deserializer.deserialize_any(NodeVisitor::new())
    }
}

impl<'a> HeapSizeOf for NodeRef {
    fn heap_size_of_children(&self) -> usize {
        self.data().heap_size_of_children()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_visit_recursive() {
        let n = NodeRef::from_json(r#"{
            "nested_object": {
                "int": 12,
                "float": 1.6,
                "string": "string value",
                "boolean": true,
                "null_value": null
            },
            "nested_array": [12, 10, "aaa", true, null],
            "prop_string": "string property"
        }"#).unwrap();

        let mut string_count = 0;

        n.visit_recursive(|_, _, n| {
            if n.is_string() {
                string_count += 1;
            }
            true
        });

        assert_eq!(string_count, 3);
    }
}
