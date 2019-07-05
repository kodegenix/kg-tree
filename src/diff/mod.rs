use std::cmp::Ordering;

use serde::ser::{Serializer, Serialize};
use serde::de::{Deserializer, Deserialize};
use std::ops::{BitOr, BitAnd};

use super::*;
use super::opath::{Opath, OpathCache, NodePathCache};


#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ChangeKind {
    Added = 1,
    Removed = 2,
    Updated = 4,
    Renamed = 8,
}

impl ChangeKind {
    pub fn mark(&self) -> char {
        match *self {
            ChangeKind::Added => '+',
            ChangeKind::Removed => '-',
            ChangeKind::Updated => '*',
            ChangeKind::Renamed => '~',
        }
    }

    pub fn mark_str(&self) -> &str {
        match *self {
            ChangeKind::Added => "+",
            ChangeKind::Removed => "-",
            ChangeKind::Updated => "*",
            ChangeKind::Renamed => "~",
        }
    }

    pub fn from_mark(m: char) -> Option<ChangeKind> {
        match m {
            '+' => Some(ChangeKind::Added),
            '-' => Some(ChangeKind::Removed),
            '*' => Some(ChangeKind::Updated),
            '~' => Some(ChangeKind::Renamed),
            _ => None,
        }
    }

    pub fn from_mark_str(m: &str) -> Option<ChangeKind> {
        match m {
            "+" | "add" | "added" => Some(ChangeKind::Added),
            "-" | "remove" | "removed" => Some(ChangeKind::Removed),
            "*" | "update" | "updated" => Some(ChangeKind::Updated),
            "~" | "rename" | "renamed" => Some(ChangeKind::Renamed),
            _ => None,
        }
    }
}

impl std::fmt::Display for ChangeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.mark())
    }
}

impl Into<ChangeKindMask> for ChangeKind {
    fn into(self) -> ChangeKindMask {
        ChangeKindMask(self as u32)
    }
}

impl BitOr<ChangeKind> for ChangeKind {
    type Output = u32;

    fn bitor(self, rhs: ChangeKind) -> Self::Output {
        self as u32 | rhs as u32
    }
}

impl BitOr<ChangeKind> for u32 {
    type Output = u32;

    fn bitor(self, rhs: ChangeKind) -> Self::Output {
        self | rhs as u32
    }
}

impl BitAnd<ChangeKind> for ChangeKind {
    type Output = u32;

    fn bitand(self, rhs: ChangeKind) -> Self::Output {
        self as u32 & rhs as u32
    }
}

impl BitAnd<ChangeKind> for u32 {
    type Output = u32;

    fn bitand(self, rhs: ChangeKind) -> Self::Output {
        self & rhs as u32
    }
}


impl Serialize for ChangeKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(self.mark_str())
    }
}

impl<'de> Deserialize<'de> for ChangeKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::{Error, Unexpected};

        let s = <(&str)>::deserialize(deserializer)?;
        match ChangeKind::from_mark_str(s) {
            Some(k) => Ok(k),
            None => Err(D::Error::invalid_value(Unexpected::Str(s), &"either '-', '+', or '*'")),
        }
    }
}


#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ChangeKindMask(u32);

impl ChangeKindMask {
    pub fn parse(mask: &str) -> ChangeKindMask {
        let mut m: u32 = 0;
        if mask.contains("all") {
            m = ChangeKind::Added as u32 + ChangeKind::Removed as u32 + ChangeKind::Updated as u32;
        } else {
            if mask.contains("+") || mask.contains("add") {
                m = m + ChangeKind::Added as u32;
            }
            if mask.contains("-") || mask.contains("remove") {
                m = m + ChangeKind::Removed as u32;
            }
            if mask.contains("*") || mask.contains("update") {
                m = m + ChangeKind::Updated as u32;
            }
            if mask.contains("~") || mask.contains("rename") {
                m = m + ChangeKind::Renamed as u32;
            }
            if m == 0 {
                m = ChangeKind::Added as u32 + ChangeKind::Removed as u32 + ChangeKind::Renamed as u32 + ChangeKind::Updated as u32;
            }
        }
        ChangeKindMask(m)
    }

    pub fn all() -> ChangeKindMask {
        ChangeKindMask(ChangeKind::Added as u32 | ChangeKind::Removed as u32 | ChangeKind::Removed as u32 | ChangeKind::Renamed as u32)
    }

    pub fn has(&self, kind: ChangeKind) -> bool {
        self.0 & kind as u32 == kind as u32
    }

    pub fn has_added(&self) -> bool {
        self.has(ChangeKind::Added)
    }

    pub fn has_removed(&self) -> bool {
        self.has(ChangeKind::Removed)
    }

    pub fn has_updated(&self) -> bool {
        self.has(ChangeKind::Updated)
    }

    pub fn has_renamed(&self) -> bool {
        self.has(ChangeKind::Renamed)
    }

    pub fn has_all(&self) -> bool {
        let all = Self::all().0;
        self.0 & all == all
    }
}

impl BitOr<ChangeKind> for ChangeKindMask {
    type Output = ChangeKindMask;

    fn bitor(self, rhs: ChangeKind) -> Self::Output {
        ChangeKindMask(self.0 | rhs as u32)
    }
}

impl std::fmt::Display for ChangeKindMask {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.has_added() {
            write!(f, "{}", ChangeKind::Added.mark())?;
        }
        if self.has_removed() {
            write!(f, "{}", ChangeKind::Removed.mark())?;
        }
        if self.has_updated() {
            write!(f, "{}", ChangeKind::Updated.mark())?;
        }
        if self.has_renamed() {
            write!(f, "{}", ChangeKind::Renamed.mark())?;
        }
        Ok(())
    }
}

impl Serialize for ChangeKindMask {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ChangeKindMask {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = <(&str)>::deserialize(deserializer)?;
        Ok(ChangeKindMask::parse(s))
    }
}

/// Represents single logical model change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelChange {
    path: Opath,
    kind: ChangeKind,
}

impl ModelChange {
    fn new(path: Opath, kind: ChangeKind) -> ModelChange {
        ModelChange {
            path,
            kind,
        }
    }

    pub fn path(&self) -> &Opath {
        &self.path
    }

    pub fn kind(&self) -> ChangeKind {
        self.kind
    }
}

impl std::fmt::Display for ModelChange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.path, self.kind)
    }
}

impl PartialEq for ModelChange {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.path == other.path
    }
}

impl Eq for ModelChange {}

impl PartialOrd for ModelChange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.path.to_string().partial_cmp(&other.path.to_string()) {
            Some(Ordering::Equal) => self.kind.partial_cmp(&other.kind),
            o => o,
        }
    }
}

impl Ord for ModelChange {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.path.to_string().cmp(&other.path.to_string()) {
            Ordering::Equal => self.kind.cmp(&other.kind),
            o => o,
        }
    }
}


fn diff_node(a: &NodeRef, b: &NodeRef, changes: &mut Vec<ModelChange>, cache: &mut dyn OpathCache) {
    if !a.is_ref_eq(b) {
        match (a.data().value(), b.data().value()) {
            (&Value::Null, &Value::Null) => {}
            (&Value::Boolean(ba), &Value::Boolean(bb)) => if ba != bb {
                changes.push(ModelChange::new(cache.get(b).clone(), ChangeKind::Updated));
            }
            (&Value::Integer(na), &Value::Integer(nb)) => if na != nb {
                changes.push(ModelChange::new(cache.get(b).clone(), ChangeKind::Updated));
            }
            (&Value::Float(na), &Value::Float(nb)) => if na.to_bits() != nb.to_bits() {
                changes.push(ModelChange::new(cache.get(b).clone(), ChangeKind::Updated));
            }
            (&Value::Integer(na), &Value::Float(nb)) => if na as f64 != nb {
                changes.push(ModelChange::new(cache.get(b).clone(), ChangeKind::Updated));
            }
            (&Value::Float(na), &Value::Integer(nb)) => if na != nb as f64 {
                changes.push(ModelChange::new(cache.get(b).clone(), ChangeKind::Updated));
            }
            (&Value::String(ref sa), &Value::String(ref sb)) => if sa != sb {
                changes.push(ModelChange::new(cache.get(b).clone(), ChangeKind::Updated));
            }
            (&Value::Object(ref propsa), &Value::Object(ref propsb)) => {
                let mut keys: LinkedHashMap<&str, ()> = LinkedHashMap::with_capacity(propsa.len());
                for k in propsa.keys() {
                    keys.insert(k.as_ref(), ());
                }
                for k in propsb.keys() {
                    keys.insert(k.as_ref(), ());
                }
                for &k in keys.keys() {
                    match (propsa.get(k), propsb.get(k)) {
                        (Some(a), Some(b)) => diff_node(a, b, changes, cache),
                        (Some(a), None) => changes.push(ModelChange::new(cache.get(a).clone(), ChangeKind::Removed)),
                        (None, Some(b)) => changes.push(ModelChange::new(cache.get(b).clone(), ChangeKind::Added)),
                        (None, None) => unreachable!(),
                    }
                }
            }
            (&Value::Array(ref elemsa), &Value::Array(ref elemsb)) => {
                for (a, b) in elemsa.iter().zip(elemsb.iter()) {
                    diff_node(a, b, changes, cache);
                }
                match elemsa.len().cmp(&elemsb.len()) {
                    Ordering::Equal => {}
                    Ordering::Less => {
                        for b in elemsb[elemsa.len()..].iter() {
                            changes.push(ModelChange::new(cache.get(b).clone(), ChangeKind::Added));
                        }
                    }
                    Ordering::Greater => {
                        for a in elemsa[elemsb.len()..].iter() {
                            changes.push(ModelChange::new(cache.get(a).clone(), ChangeKind::Removed));
                        }
                    }
                }
            }
            (&Value::Object(ref propsa), _) => {
                changes.push(ModelChange::new(cache.get(b).clone(), ChangeKind::Updated));
                for e in propsa.values() {
                    changes.push(ModelChange::new(cache.get(e).clone(), ChangeKind::Removed));
                }
            }
            (_, &Value::Object(ref propsb)) => {
                changes.push(ModelChange::new(cache.get(b).clone(), ChangeKind::Updated));
                for e in propsb.values() {
                    changes.push(ModelChange::new(cache.get(e).clone(), ChangeKind::Added));
                }
            }
            (&Value::Array(ref elemsa), _) => {
                changes.push(ModelChange::new(cache.get(b).clone(), ChangeKind::Updated));
                for e in elemsa.iter() {
                    changes.push(ModelChange::new(cache.get(e).clone(), ChangeKind::Removed));
                }
            }
            (_, &Value::Array(ref elemsb)) => {
                changes.push(ModelChange::new(cache.get(b).clone(), ChangeKind::Updated));
                for e in elemsb.iter() {
                    changes.push(ModelChange::new(cache.get(e).clone(), ChangeKind::Added));
                }
            }
            (_, _) => {
                changes.push(ModelChange::new(cache.get(b).clone(), ChangeKind::Updated));
            }
        }
    }
}


/// Struct representing logical model changes. Operates on in-memory model representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelDiff {
    changes: Vec<ModelChange>,
}

impl ModelDiff {
    pub fn minimal(a: &NodeRef, b: &NodeRef) -> ModelDiff {
        let mut cache = NodePathCache::new();
        ModelDiff::minimal_cache(a, b, &mut cache)
    }

    pub fn minimal_cache(a: &NodeRef, b: &NodeRef, cache: &mut dyn OpathCache) -> ModelDiff {
        let mut changes = Vec::new();
        diff_node(a, b, &mut changes, cache);
        ModelDiff {
            changes,
        }
    }

    pub fn full(a: &NodeRef, b: &NodeRef) -> ModelDiff {
        let mut cache = NodePathCache::new();
        ModelDiff::full_cache(a, b, &mut cache)
    }

    pub fn full_cache(a: &NodeRef, b: &NodeRef, cache: &mut dyn OpathCache) -> ModelDiff {
        let mut changes = Vec::new();

        diff_node(a, b, &mut changes, cache);

        let mut res = Vec::with_capacity(2 * changes.len());

        for c in changes {
            let mut ppath = c.path.parent_path().unwrap();
            let i = res.len();
            loop {
                if let Some(pb) = ppath.apply(b, b).into_one() {
                    if !cache.contains(&pb) {
                        res.insert(i, ModelChange::new(cache.get(&pb).clone(), ChangeKind::Updated));
                    } else {
                        break;
                    }
                } else {
                    unreachable!();
                }
                if let Some(p) = ppath.parent_path() {
                    ppath = p;
                } else {
                    break;
                }
            }
            res.push(c.clone());
            let kind = c.kind;
            if kind == ChangeKind::Removed {
                if let Some(a) = c.path.apply(a, a).into_one() {
                    a.visit_recursive(|_r, _p, n| {
                        if !a.is_ref_eq(n) {
                            res.push(ModelChange::new(cache.get(n).clone(), ChangeKind::Removed));
                        }
                        return true;
                    });
                } else {
                    unreachable!()
                }
            } else if kind == ChangeKind::Added {
                if let Some(b) = c.path.apply(b, b).into_one() {
                    b.visit_recursive(|_r, _p, n| {
                        if !b.is_ref_eq(n) {
                            res.push(ModelChange::new(cache.get(n).clone(), ChangeKind::Added));
                        }
                        return true;
                    });
                } else {
                    unreachable!()
                }
            }
        }

        ModelDiff {
            changes: res,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    pub fn changes(&self) -> &Vec<ModelChange> {
        &self.changes
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Diff should be always serializable")
    }
}

impl std::fmt::Display for ModelDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "changes: {}", self.changes.len())?;
        for c in self.changes.iter() {
            writeln!(f, "{}", c)?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_diff() {
        let jsona = r#"
        {
            "pa": {"test1":12,"bb":"aaaa"},
            "star": "*",
            "p1": {
                "aa": {
                    "bb": "aaaa",
                    "dd": [12,13,14,20,34],
                    "cc": false
                }
            }
        }"#;
        let jsonb = r#"
        {
            "star": "**",
            "pb": "test2",
            "p1": {
                "aa": {
                    "bb": "aaaa",
                    "dd": [13,12,14,20],
                    "cc": {"prop":12}
                }
            }
        }"#;

        let a = NodeRef::from_json(jsona).unwrap();
        let b = NodeRef::from_json(jsonb).unwrap();

        let d = ModelDiff::minimal(&a, &b);

        assert_eq!(d.changes().len(), 8);

        assert_eq!(d.changes()[0].path().to_string(), "$.pa");
        assert_eq!(d.changes()[0].kind(), ChangeKind::Removed);

        assert_eq!(d.changes()[1].path().to_string(), "$.star");
        assert_eq!(d.changes()[1].kind(), ChangeKind::Updated);

        assert_eq!(d.changes()[2].path().to_string(), "$.pb");
        assert_eq!(d.changes()[2].kind(), ChangeKind::Added);

        assert_eq!(d.changes()[3].path().to_string(), "$.p1.aa.dd[0]");
        assert_eq!(d.changes()[3].kind(), ChangeKind::Updated);

        assert_eq!(d.changes()[4].path().to_string(), "$.p1.aa.dd[1]");
        assert_eq!(d.changes()[4].kind(), ChangeKind::Updated);

        assert_eq!(d.changes()[5].path().to_string(), "$.p1.aa.dd[4]");
        assert_eq!(d.changes()[5].kind(), ChangeKind::Removed);

        assert_eq!(d.changes()[6].path().to_string(), "$.p1.aa.cc");
        assert_eq!(d.changes()[6].kind(), ChangeKind::Updated);

        assert_eq!(d.changes()[7].path().to_string(), "$.p1.aa.cc.prop");
        assert_eq!(d.changes()[7].kind(), ChangeKind::Added);
    }

    #[test]
    fn full_diff() {
        let jsona = r#"
        {
            "pa": {"test1":12,"bb":"aaaa"},
            "star": "*",
            "p1": {
                "aa": {
                    "bb": "aaaa",
                    "dd": [12,13,14,20,34],
                    "cc": false
                }
            }
        }"#;
        let jsonb = r#"
        {
            "star": "**",
            "pb": "test2",
            "p1": {
                "aa": {
                    "bb": "aaaa",
                    "dd": [13,12,14,20],
                    "cc": {"prop":12}
                }
            }
        }"#;

        let a = NodeRef::from_json(jsona).unwrap();
        let b = NodeRef::from_json(jsonb).unwrap();

        let d = ModelDiff::full(&a, &b);

        assert_eq!(d.changes().len(), 14);
    }
}
