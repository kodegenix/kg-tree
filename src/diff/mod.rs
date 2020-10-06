use std::cmp::Ordering;
use std::ops::{BitAnd, BitOr};

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use super::*;
use super::opath::{NodePathCache, Opath, OpathCache};

mod distance;
mod opts;

use self::distance::distance;

pub use self::opts::NodeDiffOptions;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ChangeKind {
    Added = 1,
    Removed = 2,
    Updated = 4,
    Moved = 8,
}

impl ChangeKind {
    pub fn mark(&self) -> char {
        match *self {
            ChangeKind::Added => '+',
            ChangeKind::Removed => '-',
            ChangeKind::Updated => '*',
            ChangeKind::Moved => '~',
        }
    }

    pub fn mark_str(&self) -> &str {
        match *self {
            ChangeKind::Added => "+",
            ChangeKind::Removed => "-",
            ChangeKind::Updated => "*",
            ChangeKind::Moved => "~",
        }
    }

    pub fn from_mark(m: char) -> Option<ChangeKind> {
        match m {
            '+' => Some(ChangeKind::Added),
            '-' => Some(ChangeKind::Removed),
            '*' => Some(ChangeKind::Updated),
            '~' => Some(ChangeKind::Moved),
            _ => None,
        }
    }

    pub fn from_mark_str(m: &str) -> Option<ChangeKind> {
        match m {
            "+" | "add" | "added" => Some(ChangeKind::Added),
            "-" | "remove" | "removed" => Some(ChangeKind::Removed),
            "*" | "update" | "updated" => Some(ChangeKind::Updated),
            "~" | "move" | "moved" => Some(ChangeKind::Moved),
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
    where
        S: Serializer,
    {
        serializer.serialize_str(self.mark_str())
    }
}

impl<'de> Deserialize<'de> for ChangeKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, Unexpected};

        let s = <&str>::deserialize(deserializer)?;
        match ChangeKind::from_mark_str(s) {
            Some(k) => Ok(k),
            None => Err(D::Error::invalid_value(
                Unexpected::Str(s),
                &"either '-', '+', '~' or '*'",
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ChangeKindMask(u32);

impl ChangeKindMask {
    pub fn parse(mask: &str) -> ChangeKindMask {
        let mut m: u32 = 0;
        let mut mask_it = mask.char_indices();
        while let Some((pos, c)) = mask_it.next() {
            if let Some(kind) = ChangeKind::from_mark(c) {
                m |= kind as u32;
            } else {
                if c.is_ascii_alphabetic() {
                    let r = &mask[pos..];
                    let i = pos + r.find(|c: char| !c.is_ascii_alphabetic()).unwrap_or(r.len());
                    let s = &mask[pos..i];
                    for _ in 0..s.len() {
                        mask_it.next();
                    }
                    if s.eq_ignore_ascii_case("all") {
                        m = ChangeKind::Added | ChangeKind::Removed | ChangeKind::Updated | ChangeKind::Moved;
                    } else if let Some(kind) = ChangeKind::from_mark_str(s) {
                        m |= kind as u32;
                    }
                }
            }
        }

        ChangeKindMask(m)
    }

    pub fn all() -> ChangeKindMask {
        ChangeKindMask(
            ChangeKind::Added
                | ChangeKind::Removed
                | ChangeKind::Removed
                | ChangeKind::Moved
        )
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

    pub fn has_moved(&self) -> bool {
        self.has(ChangeKind::Moved)
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
        if self.has_moved() {
            write!(f, "{}", ChangeKind::Moved.mark())?;
        }
        Ok(())
    }
}

impl Serialize for ChangeKindMask {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ChangeKindMask {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        Ok(ChangeKindMask::parse(s))
    }
}

/// Represents single logical model change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeChange {
    kind: ChangeKind,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    old_path: Option<Opath>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    new_path: Option<Opath>,
}

impl NodeChange {
    fn new(kind: ChangeKind, old_path: Option<Opath>, new_path: Option<Opath>) -> NodeChange {
        debug_assert!(old_path.is_some() || new_path.is_some());
        NodeChange {
            kind,
            old_path,
            new_path,
        }
    }

    pub fn kind(&self) -> ChangeKind {
        self.kind
    }

    pub fn old_path(&self) -> Option<&Opath> {
        self.old_path.as_ref()
    }

    pub fn new_path(&self) -> Option<&Opath> {
        self.new_path.as_ref()
    }
}

impl std::fmt::Display for NodeChange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: ", self.kind)?;
        if self.old_path.is_some() {
            write!(f, "{}", self.old_path().unwrap())?;
        } else {
            write!(f, ".")?;
        }
        write!(f, " => ")?;
        if self.new_path.is_some() {
            write!(f, "{}", self.new_path().unwrap())?;
        } else {
            write!(f, ".")?;
        }
        Ok(())
    }
}

impl PartialEq for NodeChange {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.old_path == other.old_path && self.new_path == other.old_path
    }
}

impl Eq for NodeChange {}


fn diff_node(a: &NodeRef, b: &NodeRef, changes: &mut Vec<NodeChange>, cache: &mut dyn OpathCache) {
    if !a.is_ref_eq(b) {
        match (a.data().value(), b.data().value()) {
            (&Value::Null, &Value::Null) => {}
            (&Value::Boolean(ba), &Value::Boolean(bb)) => {
                if ba != bb {
                    changes.push(NodeChange::new(
                        ChangeKind::Updated,
                        Some(cache.get(a).clone()),
                        Some(cache.get(b).clone())));
                }
            }
            (&Value::Integer(na), &Value::Integer(nb)) => {
                if na != nb {
                    changes.push(NodeChange::new(
                        ChangeKind::Updated,
                        Some(cache.get(a).clone()),
                        Some(cache.get(b).clone())));
                }
            }
            (&Value::Float(na), &Value::Float(nb)) => {
                if na.to_bits() != nb.to_bits() {
                    changes.push(NodeChange::new(
                        ChangeKind::Updated,
                        Some(cache.get(a).clone()),
                        Some(cache.get(b).clone())));
                }
            }
            (&Value::String(ref sa), &Value::String(ref sb)) => {
                if sa != sb {
                    changes.push(NodeChange::new(
                        ChangeKind::Updated,
                        Some(cache.get(a).clone()),
                        Some(cache.get(b).clone())));
                }
            }
            (&Value::Binary(ref sa), &Value::Binary(ref sb)) => {
                if sa != sb {
                    changes.push(NodeChange::new(
                        ChangeKind::Updated,
                        Some(cache.get(a).clone()),
                        Some(cache.get(b).clone())));
                }
            }
            (&Value::Object(ref propsa), &Value::Object(ref propsb)) => {
                let mut keys: SymbolMap<()> = SymbolMap::with_capacity(propsa.len());
                for k in propsa.keys() {
                    keys.insert(k.clone(), ());
                }
                for k in propsb.keys() {
                    keys.insert(k.clone(), ());
                }
                for k in keys.keys() {
                    match (propsa.get(k), propsb.get(k)) {
                        (Some(a), Some(b)) => diff_node(a, b, changes, cache),
                        (Some(a), None) => {
                            changes.push(NodeChange::new(
                                ChangeKind::Removed,
                                Some(cache.get(a).clone()),
                                None));
                        },
                        (None, Some(b)) => {
                            changes.push(NodeChange::new(
                                ChangeKind::Added,
                                None,
                                Some(cache.get(b).clone())));
                        }
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
                            changes.push(NodeChange::new(
                                ChangeKind::Added,
                                None,
                                Some(cache.get(b).clone())));
                        }
                    }
                    Ordering::Greater => {
                        for a in elemsa[elemsb.len()..].iter() {
                            changes.push(NodeChange::new(
                                ChangeKind::Removed,
                                Some(cache.get(a).clone()),
                                None));
                        }
                    }
                }
            }
            (&Value::Array(_), _) | (&Value::Object(_), _) | (_, &Value::Array(_)) | (_, &Value::Object(_)) => {
                changes.push(NodeChange::new(
                    ChangeKind::Removed,
                    Some(cache.get(a).clone()),
                    None));
                changes.push(NodeChange::new(
                    ChangeKind::Added,
                    None,
                    Some(cache.get(b).clone())));
            }
            (_, _) => {
                changes.push(NodeChange::new(
                    ChangeKind::Updated,
                    Some(cache.get(a).clone()),
                    Some(cache.get(b).clone())));
            }
        }
    }
}


struct Move {
    distance: f64,
    add_index: usize,
    del_index: usize,
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.distance.to_bits() == other.distance.to_bits()
            && self.add_index == other.add_index
            && self.del_index == other.del_index
    }
}

impl Eq for Move { }

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        let o = other.distance.partial_cmp(&self.distance).unwrap();
        if o != Ordering::Equal {
            return o;
        }
        let o = self.add_index.cmp(&other.add_index);
        if o != Ordering::Equal {
            return o;
        }
        self.del_index.cmp(&other.del_index)
    }
}

fn diff(a: &NodeRef, b: &NodeRef, opts: &NodeDiffOptions, cache: &mut dyn OpathCache, out: &mut Vec<NodeChange>) {
    use std::collections::BinaryHeap;

    fn resolve(node: &NodeRef, path: &Opath) -> NodeRef {
        let n = node.root();
        path.apply_one(&n, &n).unwrap()
    }

    fn add_change(c: NodeChange, a: &NodeRef, b: &NodeRef, cache: &mut dyn OpathCache, out: &mut Vec<NodeChange>) {
        match c.kind {
            ChangeKind::Removed => {
                let a = resolve(a, c.old_path().unwrap());
                out.push(c);
                a.visit_recursive(|_r, _p, n| {
                    if !a.is_ref_eq(n) {
                        out.push(NodeChange::new(ChangeKind::Removed, Some(cache.get(n).clone()), None));
                    }
                    return true;
                });
            }
            ChangeKind::Added => {
                let b = resolve(b, c.new_path().unwrap());
                out.push(c);
                b.visit_recursive(|_r, _p, n| {
                    if !b.is_ref_eq(n) {
                        out.push(NodeChange::new(ChangeKind::Added, None, Some(cache.get(n).clone())));
                    }
                    return true;
                });
            }
            _ => {
                out.push(c);
            }
        }
    }

    let mut changes = Vec::new();
    diff_node(a, b, &mut changes, cache);

    if changes.is_empty() {
        return;
    }

    if opts.detect_move() {
        let mut adds = 0;
        let mut dels = 0;
        for c in changes.iter() {
            match c.kind {
                ChangeKind::Added => adds += 1,
                ChangeKind::Removed => dels += 1,
                _ => {}
            }
        }

        if adds > 0 && dels > 0 {
            let max_distance = opts.max_distance().unwrap_or(0.1);

            let mut moves = BinaryHeap::with_capacity(adds * dels);
            for add in changes.iter().enumerate().filter(|c| c.1.kind == ChangeKind::Added) {
                let b = resolve(b, add.1.new_path().unwrap());
                for del in changes.iter().enumerate().filter(|c| c.1.kind == ChangeKind::Removed) {
                    let a = resolve(a, del.1.old_path().unwrap());
                    let d = distance(&a, &b, opts.min_count());
                    if d <= max_distance {
                        moves.push(Move {
                            distance: d,
                            add_index: add.0,
                            del_index: del.0,
                        });
                    }
                }
            }

            let mut idx = vec![false; changes.len()];
            for m in moves {
                if !idx[m.add_index] && !idx[m.del_index] {
                    idx[m.add_index] = true;
                    idx[m.del_index] = true;
                    let index = usize::min(m.add_index, m.del_index);
                    let old_path = changes[m.del_index].old_path.take();
                    let new_path = changes[m.add_index].new_path.take();
                    changes[index] = NodeChange::new(ChangeKind::Moved, old_path, new_path);
                }
            }

            for (i, c) in changes.into_iter().enumerate() {
                if idx[i] {
                    if c.kind == ChangeKind::Moved {
                        let na = resolve(a, c.old_path().unwrap());
                        let nb = resolve(b, c.new_path().unwrap());
                        if !na.is_ref_eq(a) || !nb.is_ref_eq(b) {
                            out.push(c);
                            diff(&na, &nb, opts, cache, out);
                        }
                    }
                } else {
                    add_change(c, a, b, cache, out);
                }
            }

            return;
        }
    }

    for c in changes {
        add_change(c, a, b, cache, out);
    }
}

/// Struct representing logical model changes. Operates on in-memory model representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeDiff {
    changes: Vec<NodeChange>,
}

impl NodeDiff {
    pub fn diff(a: &NodeRef, b: &NodeRef, opts: &NodeDiffOptions) -> NodeDiff {
        let mut cache = NodePathCache::new();
        NodeDiff::diff_cache(a, b, opts, &mut cache)
    }

    pub fn diff_cache(a: &NodeRef, b: &NodeRef, opts: &NodeDiffOptions, cache: &mut dyn OpathCache) -> NodeDiff {
        let mut changes = Vec::new();
        diff(a, b, opts, cache, &mut changes);
        changes.shrink_to_fit();
        NodeDiff { changes }
    }

    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    pub fn changes(&self) -> &Vec<NodeChange> {
        &self.changes
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Diff should be always serializable")
    }

    pub fn find_old(&self, n: &NodeRef, old_root: &NodeRef, new_root: &NodeRef) -> Option<NodeRef> {
        if !n.root().is_ref_eq(new_root) {
            None
        } else {
            let path = n.path();
            for c in self.changes.iter().rev() {
                if let Some(new_path) = c.new_path() {
                    if let Some(old_path) = c.old_path() {
                        if new_path == &path {
                            return old_path.apply_one(old_root, old_root).ok();
                        } else if new_path.is_ancestor_path(&path) {
                            let mut segments: Vec<_> = old_path.clone().into();
                            let seg: Vec<_> = path.into();
                            segments.extend_from_slice(&seg[new_path.path_len()..]);
                            let p: Opath = segments.into();
                            return p.apply_one(old_root, old_root).ok();
                        }
                    }
                }
            }
            path.apply_one(old_root, old_root).ok()
        }
    }

    pub fn find_new(&self, n: &NodeRef, old_root: &NodeRef, new_root: &NodeRef) -> Option<NodeRef> {
        if !n.root().is_ref_eq(old_root) {
            None
        } else {
            let path = n.path();
            for c in self.changes.iter().rev() {
                if let Some(new_path) = c.new_path() {
                    if let Some(old_path) = c.old_path() {
                        if old_path == &path {
                            return new_path.apply_one(new_root, new_root).ok();
                        } else if old_path.is_ancestor_path(&path) {
                            let mut segments: Vec<_> = new_path.clone().into();
                            let seg: Vec<_> = path.into();
                            segments.extend_from_slice(&seg[old_path.path_len()..]);
                            let p: Opath = segments.into();
                            return p.apply_one(new_root, new_root).ok();
                        }
                    }
                }
            }
            path.apply_one(new_root, new_root).ok()
        }
    }
}

impl std::fmt::Display for NodeDiff {
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
    fn change_kind_mask_parse_chars() {
        let mask = ChangeKindMask::parse("+");
        assert_eq!(mask, ChangeKindMask(ChangeKind::Added as u32));

        let mask = ChangeKindMask::parse("-");
        assert_eq!(mask, ChangeKindMask(ChangeKind::Removed as u32));

        let mask = ChangeKindMask::parse("*");
        assert_eq!(mask, ChangeKindMask(ChangeKind::Updated as u32));

        let mask = ChangeKindMask::parse("~");
        assert_eq!(mask, ChangeKindMask(ChangeKind::Moved as u32));

        let mask = ChangeKindMask::parse("+++");
        assert_eq!(mask, ChangeKindMask(ChangeKind::Added as u32));

        let mask = ChangeKindMask::parse("~~~");
        assert_eq!(mask, ChangeKindMask(ChangeKind::Moved as u32));

        let mask = ChangeKindMask::parse("++--~~");
        assert_eq!(mask, ChangeKindMask(ChangeKind::Added | ChangeKind::Removed | ChangeKind::Moved));
    }

    #[test]
    fn change_kind_mask_parse_words() {
        let mask = ChangeKindMask::parse("add");
        assert_eq!(mask, ChangeKindMask(ChangeKind::Added as u32));

        let mask = ChangeKindMask::parse("remove");
        assert_eq!(mask, ChangeKindMask(ChangeKind::Removed as u32));

        let mask = ChangeKindMask::parse("update");
        assert_eq!(mask, ChangeKindMask(ChangeKind::Updated as u32));

        let mask = ChangeKindMask::parse("move");
        assert_eq!(mask, ChangeKindMask(ChangeKind::Moved as u32));

        let mask = ChangeKindMask::parse("add add added");
        assert_eq!(mask, ChangeKindMask(ChangeKind::Added as u32));

        let mask = ChangeKindMask::parse("moved moved");
        assert_eq!(mask, ChangeKindMask(ChangeKind::Moved as u32));

        let mask = ChangeKindMask::parse("add,removed,moved");
        assert_eq!(mask, ChangeKindMask(ChangeKind::Added | ChangeKind::Removed | ChangeKind::Moved));

        let mask = ChangeKindMask::parse("update,removed,move");
        assert_eq!(mask, ChangeKindMask(ChangeKind::Updated | ChangeKind::Removed | ChangeKind::Moved));

        let mask = ChangeKindMask::parse("all");
        assert_eq!(mask, ChangeKindMask(ChangeKind::Added | ChangeKind::Removed | ChangeKind::Updated | ChangeKind::Moved));
    }

    #[test]
    fn simple_diff() {
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

        let opts = NodeDiffOptions::default();
        let d = NodeDiff::diff(&a, &b, &opts);

        assert_eq!(d.changes().len(), 11);

        assert_eq!(d.changes()[0].old_path().unwrap().to_string(), "$.pa");
        assert_eq!(d.changes()[0].new_path(), None);
        assert_eq!(d.changes()[0].kind(), ChangeKind::Removed);

        assert_eq!(d.changes()[1].old_path().unwrap().to_string(), "$.pa.test1");
        assert_eq!(d.changes()[1].new_path(), None);
        assert_eq!(d.changes()[1].kind(), ChangeKind::Removed);

        assert_eq!(d.changes()[2].old_path().unwrap().to_string(), "$.pa.bb");
        assert_eq!(d.changes()[2].new_path(), None);
        assert_eq!(d.changes()[2].kind(), ChangeKind::Removed);

        assert_eq!(d.changes()[3].old_path().unwrap().to_string(), "$.star");
        assert_eq!(d.changes()[3].new_path().unwrap().to_string(), "$.star");
        assert_eq!(d.changes()[3].kind(), ChangeKind::Updated);

        assert_eq!(d.changes()[4].old_path(), None);
        assert_eq!(d.changes()[4].new_path().unwrap().to_string(), "$.pb");
        assert_eq!(d.changes()[4].kind(), ChangeKind::Added);

        assert_eq!(d.changes()[5].old_path().unwrap().to_string(), "$.p1.aa.dd[0]");
        assert_eq!(d.changes()[5].new_path().unwrap().to_string(), "$.p1.aa.dd[0]");
        assert_eq!(d.changes()[5].kind(), ChangeKind::Updated);

        assert_eq!(d.changes()[6].old_path().unwrap().to_string(), "$.p1.aa.dd[1]");
        assert_eq!(d.changes()[6].new_path().unwrap().to_string(), "$.p1.aa.dd[1]");
        assert_eq!(d.changes()[6].kind(), ChangeKind::Updated);

        assert_eq!(d.changes()[7].old_path().unwrap().to_string(), "$.p1.aa.dd[4]");
        assert_eq!(d.changes()[7].new_path(), None);
        assert_eq!(d.changes()[7].kind(), ChangeKind::Removed);

        assert_eq!(d.changes()[8].old_path().unwrap().to_string(), "$.p1.aa.cc");
        assert_eq!(d.changes()[8].new_path(), None);
        assert_eq!(d.changes()[8].kind(), ChangeKind::Removed);

        assert_eq!(d.changes()[9].old_path(), None);
        assert_eq!(d.changes()[9].new_path().unwrap().to_string(), "$.p1.aa.cc");
        assert_eq!(d.changes()[9].kind(), ChangeKind::Added);

        assert_eq!(d.changes()[10].old_path(), None);
        assert_eq!(d.changes()[10].new_path().unwrap().to_string(), "$.p1.aa.cc.prop");
        assert_eq!(d.changes()[10].kind(), ChangeKind::Added);
    }

    #[test]
    fn diff_should_detect_move() {
        let jsona = r#"
        {
            "star": "*",
            "pb": { "aa": "test2", "b": false },
            "aaa": [11, {"aa": 1}]
        }"#;
        let jsonb = r#"
        {
            "star": "*",
            "aaa": [12, 11],
            "bb": 12,
            "pc": { "aa": "test2", "b": false, "dd":[12,11] }
        }"#;

        let a = NodeRef::from_json(jsona).unwrap();
        let b = NodeRef::from_json(jsonb).unwrap();

        let opts = NodeDiffOptions::new(true, Some(1), Some(0.5));
        let d = NodeDiff::diff(&a, &b, &opts);

        let new_n = b.get_child_key("pc").unwrap().get_child_key("aa").unwrap();
        let old_n = d.find_old(&new_n, &a, &b).unwrap();
        println!("new: {} {}", new_n.path(), new_n);
        println!("old: {} {}", old_n.path(), old_n);

        assert_eq!(d.changes().len(), 9);

        assert_eq!(d.changes()[0].old_path().unwrap().to_string(), "$.pb");
        assert_eq!(d.changes()[0].new_path().unwrap().to_string(), "$.pc");
        assert_eq!(d.changes()[0].kind(), ChangeKind::Moved);

        assert_eq!(d.changes()[1].old_path(), None);
        assert_eq!(d.changes()[1].new_path().unwrap().to_string(), "$.pc.dd");
        assert_eq!(d.changes()[1].kind(), ChangeKind::Added);

        assert_eq!(d.changes()[2].old_path(), None);
        assert_eq!(d.changes()[2].new_path().unwrap().to_string(), "$.pc.dd[0]");
        assert_eq!(d.changes()[2].kind(), ChangeKind::Added);

        assert_eq!(d.changes()[3].old_path(), None);
        assert_eq!(d.changes()[3].new_path().unwrap().to_string(), "$.pc.dd[1]");
        assert_eq!(d.changes()[3].kind(), ChangeKind::Added);

        assert_eq!(d.changes()[4].old_path().unwrap().to_string(), "$.aaa[0]");
        assert_eq!(d.changes()[4].new_path().unwrap().to_string(), "$.aaa[0]");
        assert_eq!(d.changes()[4].kind(), ChangeKind::Updated);

        assert_eq!(d.changes()[5].old_path().unwrap().to_string(), "$.aaa[1]");
        assert_eq!(d.changes()[5].new_path(), None);
        assert_eq!(d.changes()[5].kind(), ChangeKind::Removed);

        assert_eq!(d.changes()[6].old_path().unwrap().to_string(), "$.aaa[1].aa");
        assert_eq!(d.changes()[6].new_path(), None);
        assert_eq!(d.changes()[6].kind(), ChangeKind::Removed);

        assert_eq!(d.changes()[7].old_path(), None);
        assert_eq!(d.changes()[7].new_path().unwrap().to_string(), "$.aaa[1]");
        assert_eq!(d.changes()[7].kind(), ChangeKind::Added);

        assert_eq!(d.changes()[8].old_path(), None);
        assert_eq!(d.changes()[8].new_path().unwrap().to_string(), "$.bb");
        assert_eq!(d.changes()[8].kind(), ChangeKind::Added);
    }
}
