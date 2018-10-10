use std::rc::Rc;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Null    = 0x01,
    Boolean = 0x02,
    Integer = 0x04,
    Float   = 0x08,
    String  = 0x10,
    Binary  = 0x20,
    Array   = 0x40,
    Object  = 0x80,
}

impl Kind {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Kind::Null => "null",
            Kind::Boolean => "boolean",
            Kind::Integer => "integer",
            Kind::Float => "float",
            Kind::String => "string",
            Kind::Binary => "binary",
            Kind::Array => "array",
            Kind::Object => "object",
        }
    }

    pub fn as_type_str(&self) -> &'static str {
        match *self {
            Kind::Null => "null",
            Kind::Boolean => "boolean",
            Kind::Integer => "number",
            Kind::Float => "number",
            Kind::String => "string",
            Kind::Binary => "binary",
            Kind::Array => "array",
            Kind::Object => "object",
        }
    }
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_type_str())
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KindMask(u8);

impl KindMask {
    pub fn none() -> KindMask {
        KindMask(0)
    }

    pub fn all() -> KindMask {
        KindMask(0xFF)
    }

    pub fn null() -> KindMask {
        Self::none().with(Kind::Null)
    }
    pub fn boolean() -> KindMask {
        Self::none().with(Kind::Boolean)
    }

    pub fn integer() -> KindMask {
        Self::none().with(Kind::Integer)
    }

    pub fn float() -> KindMask {
        Self::none().with(Kind::Float)
    }

    pub fn number() -> KindMask {
        Self::none().with(Kind::Integer).with(Kind::Float)
    }

    pub fn string() -> KindMask {
        Self::none().with(Kind::String)
    }

    pub fn binary() -> KindMask {
        Self::none().with(Kind::Binary)
    }

    pub fn array() -> KindMask {
        Self::none().with(Kind::Array)
    }

    pub fn object() -> KindMask {
        Self::none().with(Kind::Object)
    }

    pub fn container() -> KindMask {
        Self::none().with(Kind::Array).with(Kind::Object)
    }

    pub fn with(self, kind: Kind) -> KindMask {
        KindMask(self.0 | kind as u8)
    }

    pub fn without(self, kind: Kind) -> KindMask {
        KindMask(self.0 & !(kind as u8))
    }

    pub fn has(&self, kind: Kind) -> bool {
        self.0 & (kind as u8) > 0
    }
}


#[derive(Debug)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Binary(Vec<u8>),
    Array(Elements),
    Object(Properties),
}

impl Value {
    fn deep_copy(&self) -> Value {
        match *self {
            Value::Null => Value::Null,
            Value::Boolean(b) => Value::Boolean(b),
            Value::Integer(n) => Value::Integer(n),
            Value::Float(n) => Value::Float(n),
            Value::String(ref s) => Value::String(s.clone()),
            Value::Binary(ref b) => Value::Binary(b.clone()),
            Value::Array(ref elems) => Value::Array(elems.iter().map(|n| n.deep_copy()).collect()),
            Value::Object(ref props) => {
                let mut p = Properties::with_capacity(props.len());
                for (k, v) in props.iter() {
                    p.insert(k.to_string().into(), v.deep_copy());
                }
                Value::Object(p)
            }
        }
    }

    fn shrink_to_fit(&mut self) {
        match *self {
            Value::String(ref mut s) => {
                s.shrink_to_fit();
            },
            Value::Binary(ref mut b) => {
                b.shrink_to_fit();
            },
            Value::Array(ref mut elems) => {
                elems.shrink_to_fit();
                for e in elems.iter_mut() {
                    e.data_mut().shrink_to_fit();
                }
            }
            Value::Object(ref mut props) => {
                props.shrink_to_fit();
                for (_, v) in props.iter_mut() {
                    v.data_mut().shrink_to_fit();
                }
            }
            _ => {}
        }
    }
}

impl HeapSizeOf for Value {
    fn heap_size_of_children(&self) -> usize {
        match *self {
            Value::Null => 0,
            Value::Boolean(_) => 0,
            Value::Integer(_) => 0,
            Value::Float(_) => 0,
            Value::String(ref s) => s.heap_size_of_children(),
            Value::Binary(ref b) => b.heap_size_of_children(),
            Value::Array(ref elems) => (*elems).heap_size_of_children(),
            Value::Object(ref props) => (*props).heap_size_of_children(),
        }
    }
}


#[derive(Debug)]
pub struct Node {
    metadata: Metadata,
    value: Value,
}

impl Node {
    pub (super) fn new(metadata: Metadata, value: Value) -> Node {
        Node {
            metadata,
            value,
        }
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut Value {
        &mut self.value
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }

    pub fn into_string(self) -> String {
        match self.value {
            Value::Null => "null".to_string(),
            Value::Boolean(b) => if b { "true" } else { "false" }.to_string(),
            Value::Integer(n) => n.to_string(),
            Value::Float(n) => n.to_string(),
            Value::String(s) => s,
            Value::Binary(_) => "[binary]".into(),
            Value::Array(arr) => {
                let mut s = String::new();
                let mut iter = arr.iter().peekable();
                while let Some(el) = iter.next() {
                    s.push_str(&el.data().as_string());
                    if iter.peek().is_some() {
                        s.push(',');
                    }
                }
                s
            }
            Value::Object(_) => "[object]".to_string(),
        }
    }

    pub fn as_string(&self) -> Cow<str> {
        use std::borrow::Borrow;

        match self.value {
            Value::Null => "null".into(),
            Value::Boolean(b) => if b { "true" } else { "false" }.into(),
            Value::Integer(n) => n.to_string().into(),
            Value::Float(n) => n.to_string().into(),
            Value::String(ref s) => Cow::Borrowed(s.borrow()),
            Value::Binary(_) => "[binary]".into(),
            Value::Array(ref arr) => {
                let mut s = String::new();
                let mut iter = arr.iter().peekable();
                while let Some(el) = iter.next() {
                    s.push_str(&el.data().as_string());
                    if iter.peek().is_some() {
                        s.push(',');
                    }
                }
                s.into()
            }
            Value::Object(_) => "[object]".into(),
        }
    }

    pub fn as_boolean(&self) -> bool {
        match self.value {
            Value::Null => false,
            Value::Boolean(b) => b,
            Value::Integer(n) => n != 0,
            Value::Float(n) => n.is_normal(),
            Value::String(ref s) => s.len() > 0,
            Value::Binary(ref b) => b.len() > 0,
            Value::Array(_) => true,
            Value::Object(_) => true,
        }
    }

    pub fn as_float(&self) -> f64 {
        use std::f64;
        use std::str::FromStr;

        match self.value {
            Value::Null => 0f64,
            Value::Boolean(b) => b as i32 as f64,
            Value::Integer(n) => n as f64,
            Value::Float(n) => n,
            Value::String(ref s) => match f64::from_str(s) {
                Ok(n) => n,
                Err(_) => f64::NAN,
            },
            Value::Binary(_) | Value::Object(_) | Value::Array(_) => f64::NAN,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        use std::i64;
        use std::str::FromStr;

        match self.value {
            Value::Null => Some(0),
            Value::Boolean(b) => Some(b as i64),
            Value::Integer(n) => Some(n),
            Value::Float(n) => if n.is_finite() {
                Some(n as i64)
            } else {
                None
            },
            Value::String(ref s) => i64::from_str(s).ok(),
            Value::Binary(_) => None,
            Value::Array(_) => None,
            Value::Object(_) => None,
        }
    }

    pub fn is_null(&self) -> bool {
        match self.value {
            Value::Null => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self.value {
            Value::String(_) => true,
            _ => false,
        }
    }

    pub fn is_binary(&self) -> bool {
        match self.value {
            Value::Binary(_) => true,
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self.value {
            Value::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self.value {
            Value::Integer(_) | Value::Float(_) => true,
            _ => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self.value {
            Value::Integer(_) => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self.value {
            Value::Integer(_) | Value::Float(_) => true,
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match self.value {
            Value::Object(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self.value {
            Value::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_parent(&self) -> bool {
        match self.value {
            Value::Array(_) | Value::Object(_) => true,
            _ => false,
        }
    }

    pub fn level(&self) -> usize {
        if let Some(p) = self.metadata.parent() {
            p.data().level() + 1
        } else {
            0
        }
    }

    pub fn index(&self) -> usize {
        self.metadata.index()
    }

    pub fn key(&self) -> &str {
        self.metadata.key()
    }

    pub fn kind(&self) -> Kind {
        match self.value {
            Value::Null => Kind::Null,
            Value::Boolean(_) => Kind::Boolean,
            Value::Integer(_) => Kind::Integer,
            Value::Float(_) => Kind::Float,
            Value::String(_) => Kind::String,
            Value::Binary(_) => Kind::Binary,
            Value::Array(_) => Kind::Array,
            Value::Object(_) => Kind::Object,
        }
    }

    pub fn parent(&self) -> Option<NodeRef> {
        self.metadata.parent()
    }

    pub fn file(&self) -> Option<&FileInfo> {
        self.metadata.file().map(|f| &**f)
    }

    pub fn set_file(&mut self, file: Option<&Rc<FileInfo>>) {
        fn update_file(d: &mut Node, file: &Rc<FileInfo>) {
            d.metadata_mut().set_file(Some(file.clone()));
            match d.value {
                Value::Object(ref props) => {
                    for p in props.values() {
                        if p.data().metadata().file().is_none() {
                            update_file(&mut p.data_mut(), file);
                        }
                    }
                }
                Value::Array(ref elems) => {
                    for e in elems.iter() {
                        if e.data().metadata().file().is_none() {
                            update_file(&mut e.data_mut(), file);
                        }
                    }
                }
                _ => {}
            }
        }
        match file {
            Some(file) => update_file(self, file),
            None => self.metadata.set_file(None),
        }
    }

    pub fn file_string(&self) -> String {
        match self.file() {
            Some(f) => format!("{}", f),
            None => String::new(),
        }
    }

    pub fn file_string_abs(&self) -> String {
        match self.file() {
            Some(f) => format!("{:#}", f),
            None => String::new(),
        }
    }

    pub fn file_type(&self) -> String {
        match self.file() {
            Some(f) => format!("{}", f.file_type()),
            None => String::new(),
        }
    }

    pub fn file_format(&self) -> String {
        match self.file() {
            Some(f) => match f.file_type() {
                FileType::File => format!("{}", f.file_format()),
                _ => String::new(),
            },
            None => String::new(),
        }
    }

    pub fn file_path(&self) -> String {
        match self.file() {
            Some(f) => f.file_path().to_str().unwrap().to_string(),
            None => String::new(),
        }
    }

    pub fn file_path_abs(&self) -> String {
        match self.file() {
            Some(f) => f.file_path_abs().to_str().unwrap().to_string(),
            None => String::new(),
        }
    }

    pub fn dir(&self) -> String {
        match self.file() {
            Some(f) => if f.file_type() == FileType::Dir {
                f.file_path().to_str().unwrap().to_string()
            } else {
                f.file_path().parent().map_or(String::new(), |p| p.to_str().unwrap().to_string())
            },
            None => String::new(),
        }
    }

    pub fn dir_abs(&self) -> String {
        match self.file() {
            Some(f) => if f.file_type() == FileType::Dir {
                f.file_path_abs().to_str().unwrap().to_string()
            } else {
                f.file_path_abs().parent().map_or(String::new(), |p| p.to_str().unwrap().to_string())
            },
            None => String::new(),
        }
    }

    pub fn file_path_components<'b>(&'b self) -> impl Iterator<Item = String> + 'b {
        match self.file() {
            Some(f) => box f.file_path().components().map(|c| c.as_os_str().to_str().unwrap().to_string()) as Box<Iterator<Item = String>>,
            None => box std::iter::empty() as Box<Iterator<Item = String>>,
        }
    }

    pub fn file_name(&self) -> String {
        match self.file() {
            Some(f) => f.file_path().file_name().map_or(String::new(), |e| e.to_str().unwrap().to_string()),
            None => String::new(),
        }
    }

    pub fn file_stem(&self) -> String {
        match self.file() {
            Some(f) => f.file_path().file_stem().map_or(String::new(), |e| e.to_str().unwrap().to_string()),
            None => String::new(),
        }
    }

    pub fn file_ext(&self) -> String {
        match self.file() {
            Some(f) => f.file_path().extension().map_or(String::new(), |e| e.to_str().unwrap().to_string()),
            None => String::new(),
        }
    }

    pub fn is_root(&self) -> bool {
        !self.metadata.has_parent()
    }

    pub fn children_count(&self) -> Option<usize> {
        match self.value {
            Value::Array(ref elems) => Some(elems.len()),
            Value::Object(ref props) => Some(props.len()),
            _ => None,
        }
    }

    pub fn deep_copy(&self) -> Node {
        Node::new(self.metadata.deep_copy(), self.value.deep_copy())
    }

    pub fn shrink_to_fit(&mut self) {
        self.value.shrink_to_fit();
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::fmt::Write;
        use kg_display::PrettyPrinter;

        #[inline]
        fn padding<'a>(f: &mut std::fmt::Formatter) -> (usize, Cow<'a, str>) {
            match f.width() {
                Some(0) => (0, "".into()),
                Some(1) => (1, " ".into()),
                Some(2) => (2, "  ".into()),
                Some(3) => (3, "   ".into()),
                Some(4) => (4, "    ".into()),
                _ => (2, "  ".into()),
            }
        }

        if f.alternate() {
            let ref m = self.metadata;
            write!(f, "<{:p},", m.parent().map_or(std::ptr::null(), |p| p.data_ptr()))?;
            write!(f, "{},", m.index())?;
            write!(f, "{:?},", m.key())?;
            match m.file() {
                Some(file) => write!(f, "\"{}\"> ", file)?,
                None => write!(f, "-> ")?,
            }
        }

        match *self.value() {
            Value::Null => write!(f, "null"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Integer(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::String(ref s) => write!(f, "{:?}", s),
            Value::Binary(ref b) => write!(f, "{:?}", b),
            Value::Array(ref elems) => {
                if f.alternate() {
                    {
                        let (width, pad) = padding(f);
                        let mut p = PrettyPrinter::new(f, &pad);
                        write!(p, "[\n")?;
                        let mut iter = elems.iter().peekable();
                        while let Some(e) = iter.next() {
                            if iter.peek().is_some() {
                                write!(p, "{:#1$},\n", e, width)?;
                            } else {
                                write!(p, "{:#1$}\n", e, width)?;
                            }
                        }
                    }
                    write!(f, "]")
                } else {
                    write!(f, "[")?;
                    let mut iter = elems.iter().peekable();
                    while let Some(e) = iter.next() {
                        if iter.peek().is_some() {
                            write!(f, "{},", e)?;
                        } else {
                            write!(f, "{}", e)?;
                        }
                    }
                    write!(f, "]")
                }
            }
            Value::Object(ref props) => {
                if f.alternate() {
                    {
                        let (width, pad) = padding(f);
                        let mut p = PrettyPrinter::new(f, &pad);
                        write!(p, "{{\n")?;
                        let mut iter = props.iter().peekable();
                        while let Some((k, e)) = iter.next() {
                            if iter.peek().is_some() {
                                write!(p, "\"{}\": {:#2$},\n", k, e, width)?;
                            } else {
                                write!(p, "\"{}\": {:#2$}\n", k, e, width)?;
                            }
                        }
                    }
                    write!(f, "}}")
                } else {
                    write!(f, "{{")?;
                    let mut iter = props.iter().peekable();
                    while let Some((k, e)) = iter.next() {
                        if iter.peek().is_some() {
                            write!(f, "\"{}\":{},", k, e)?;
                        } else {
                            write!(f, "\"{}\":{}", k, e)?;
                        }
                    }
                    write!(f, "}}")
                }
            }
        }
    }
}

impl HeapSizeOf for Node {
    fn heap_size_of_children(&self) -> usize {
        self.metadata.heap_size_of_children() + self.value.heap_size_of_children()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_mask_without() {
        let m = KindMask::all();
        assert!(m.has(Kind::Null));
        assert!(m.has(Kind::Boolean));
        assert!(m.has(Kind::Integer));
        assert!(m.has(Kind::Float));
        assert!(m.has(Kind::String));
        assert!(m.has(Kind::Binary));
        assert!(m.has(Kind::Array));
        assert!(m.has(Kind::Object));

        let m = m.without(Kind::Integer);
        assert!(m.has(Kind::Null));
        assert!(m.has(Kind::Boolean));
        assert!(!m.has(Kind::Integer));
        assert!(m.has(Kind::Float));
        assert!(m.has(Kind::String));
        assert!(m.has(Kind::Binary));
        assert!(m.has(Kind::Array));
        assert!(m.has(Kind::Object));
    }

    #[test]
    fn kind_mask_with() {
        let m = KindMask::none();
        assert!(!m.has(Kind::Null));
        assert!(!m.has(Kind::Boolean));
        assert!(!m.has(Kind::Integer));
        assert!(!m.has(Kind::Float));
        assert!(!m.has(Kind::String));
        assert!(!m.has(Kind::Binary));
        assert!(!m.has(Kind::Array));
        assert!(!m.has(Kind::Object));

        let m = m.with(Kind::Integer);
        assert!(!m.has(Kind::Null));
        assert!(!m.has(Kind::Boolean));
        assert!(m.has(Kind::Integer));
        assert!(!m.has(Kind::Float));
        assert!(!m.has(Kind::String));
        assert!(!m.has(Kind::Binary));
        assert!(!m.has(Kind::Array));
        assert!(!m.has(Kind::Object));
    }
}