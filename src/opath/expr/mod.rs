use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use super::*;

pub use self::scope::{Scope, ScopeMut};

pub(super) mod func;
pub(super) mod parse;

pub use func::FuncCallErrorDetail;

mod scope;

pub type ExprError = BasicDiag;

pub type ExprResult<T> = Result<T, ExprError>;
pub type ApplyResult = ExprResult<()>;

#[derive(Debug, Display, Detail)]
#[diag(code_offset = 600)]
pub enum ExprErrorDetail {
    #[display(fmt = "expected single value in variable: '{var_name}'")]
    MultipleVarValues { var_name: String },

    #[display(fmt = "variable not found: '{var_name}'")]
    VariableNotFound { var_name: String },

    // This variant should probably be placed in resolve.rs module
    #[display(fmt = "too many iterations while resolving interpolations: '{depth}'")]
    InterpolationDepthReached { depth: usize },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum NodeSet {
    Empty,
    One(NodeRef),
    Many(Vec<NodeRef>),
}

impl NodeSet {
    pub fn len(&self) -> usize {
        match *self {
            NodeSet::Empty => 0,
            NodeSet::One(..) => 1,
            NodeSet::Many(ref e) => e.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match *self {
            NodeSet::Empty => true,
            _ => false,
        }
    }

    pub fn is_one(&self) -> bool {
        match *self {
            NodeSet::One(..) => true,
            _ => false,
        }
    }

    pub fn is_many(&self) -> bool {
        match *self {
            NodeSet::Many(..) => true,
            _ => false,
        }
    }

    pub fn is_lvalue(&self) -> bool {
        match *self {
            NodeSet::One(ref e) => !e.is_consumable(),
            _ => false,
        }
    }

    pub fn into_vec(self) -> Vec<NodeRef> {
        match self {
            NodeSet::Empty => Vec::new(),
            NodeSet::One(a) => vec![a],
            NodeSet::Many(e) => e,
        }
    }

    pub fn into_one(self) -> Option<NodeRef> {
        match self {
            NodeSet::Empty => None,
            NodeSet::One(a) => Some(a),
            NodeSet::Many(_) => None,
        }
    }

    pub fn as_slice(&self) -> &[NodeRef] {
        match *self {
            NodeSet::Empty => unsafe { std::slice::from_raw_parts(0x1 as *const NodeRef, 0) },
            NodeSet::One(ref a) => std::slice::from_ref(a),
            NodeSet::Many(ref e) => e.as_ref(),
        }
    }

    pub fn iter(&self) -> std::slice::Iter<NodeRef> {
        self.as_slice().iter()
    }

    pub fn into_consumable(self) -> NodeSet {
        match self {
            NodeSet::Empty => NodeSet::Empty,
            NodeSet::One(n) => NodeSet::One(n.into_consumable()),
            NodeSet::Many(elems) => {
                NodeSet::Many(elems.into_iter().map(|n| n.into_consumable()).collect())
            }
        }
    }
}

impl Remappable for NodeSet {
    fn remap(&mut self, node_map: &NodeMap) {
        fn remap(node: &mut NodeRef, node_map: &NodeMap) {
            if let Some(n) = node_map.get(&node.data_ptr()) {
                *node = n.clone();
            } else {
                *node = node.deep_copy();
            }
        }

        match *self {
            NodeSet::Empty => {}
            NodeSet::One(ref mut n) => remap(n, node_map),
            NodeSet::Many(ref mut elems) => {
                for n in elems.iter_mut() {
                    remap(n, node_map)
                }
            }
        }
    }
}

impl IntoIterator for NodeSet {
    type Item = NodeRef;
    type IntoIter = std::vec::IntoIter<NodeRef>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            NodeSet::Empty => Vec::new().into_iter(),
            NodeSet::One(a) => vec![a].into_iter(),
            NodeSet::Many(e) => e.into_iter(),
        }
    }
}

impl std::fmt::Display for NodeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            NodeSet::Empty => write!(f, "<~>"),
            NodeSet::One(ref a) => {
                write!(f, "<1>:")?;
                a.fmt(f)
            }
            NodeSet::Many(ref e) => {
                write!(f, "<+>:[")?;
                let mut n_it = e.iter().peekable();
                while let Some(n) = n_it.next() {
                    n.fmt(f)?;
                    if n_it.peek().is_some() {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

impl From<NodeRef> for NodeSet {
    fn from(node: NodeRef) -> Self {
        NodeSet::One(node)
    }
}

impl From<Vec<NodeRef>> for NodeSet {
    fn from(mut nodes: Vec<NodeRef>) -> Self {
        match nodes.len() {
            0 => NodeSet::Empty,
            1 => NodeSet::One(nodes.pop().unwrap()),
            _ => NodeSet::Many(nodes),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct LevelRange {
    min: Expr,
    max: Expr,
}

impl LevelRange {
    pub(super) fn min(&self) -> &Expr {
        &self.min
    }

    pub(super) fn set_min(&mut self, min: Expr) {
        self.min = min;
    }

    pub(super) fn max(&self) -> &Expr {
        &self.max
    }

    pub(super) fn set_max(&mut self, max: Expr) {
        self.max = max;
    }
}

impl Default for LevelRange {
    fn default() -> LevelRange {
        use std::i64;
        LevelRange {
            min: Expr::Integer(1),
            max: Expr::Integer(i64::MAX),
        }
    }
}

impl std::fmt::Display for LevelRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::i64;
        match (&self.min, &self.max) {
            (&Expr::Integer(1), &Expr::Integer(i64::MAX)) => Ok(()),
            (&Expr::Integer(1), _) => write!(f, "{{,{}}}", self.max),
            (_, &Expr::Integer(i64::MAX)) => write!(f, "{{{}}}", self.min),
            (_, _) => write!(f, "{{{},{}}}", self.min, self.max),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct NumberRange {
    start: Option<Expr>,
    step: Option<Expr>,
    stop: Option<Expr>,
}

impl NumberRange {
    pub(super) fn start(&self) -> Option<&Expr> {
        self.start.as_ref()
    }

    pub(super) fn set_start(&mut self, start: Option<Expr>) {
        self.start = start;
    }

    pub(super) fn step(&self) -> Option<&Expr> {
        self.step.as_ref()
    }

    pub(super) fn set_step(&mut self, step: Option<Expr>) {
        self.step = step;
    }

    pub(super) fn stop(&self) -> Option<&Expr> {
        self.stop.as_ref()
    }

    pub(super) fn set_stop(&mut self, stop: Option<Expr>) {
        self.stop = stop;
    }
}

impl Default for NumberRange {
    fn default() -> NumberRange {
        NumberRange {
            start: None,
            step: None,
            stop: None,
        }
    }
}

impl std::fmt::Display for NumberRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match (&self.start, &self.step, &self.stop) {
            (&None, &None, &None) => write!(f, ".."),
            (&Some(ref start), &None, &None) => write!(f, "{}..", start),
            (&None, &None, &Some(ref stop)) => write!(f, "..{}", stop),
            (&Some(ref start), &None, &Some(ref stop)) => write!(f, "{}..{}", start, stop),
            (&Some(ref start), &Some(ref step), &None) => write!(f, "{}:{}:", start, step),
            (&None, &Some(ref step), &Some(ref stop)) => write!(f, ":{}:{}", step, stop),
            (&None, &Some(ref step), &None) => write!(f, ":{}:", step),
            (&Some(ref start), &Some(ref step), &Some(ref stop)) => {
                write!(f, "{}:{}:{}", start, step, stop)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct MethodCall {
    id: MethodId,
    args: Vec<Expr>,
}

impl MethodCall {
    pub(super) fn new(id: MethodId, args: Vec<Expr>) -> MethodCall {
        MethodCall { id, args }
    }

    pub(super) fn id(&self) -> &MethodId {
        &self.id
    }

    pub(super) fn args(&self) -> Args {
        Args::new(&self.args)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct FuncCall {
    id: FuncId,
    args: Vec<Expr>,
}

impl FuncCall {
    pub(super) fn new(id: FuncId, args: Vec<Expr>) -> FuncCall {
        FuncCall { id, args }
    }

    pub(super) fn id(&self) -> &FuncId {
        &self.id
    }

    pub(super) fn args(&self) -> Args {
        Args::new(&self.args)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Env<'a> {
    current: &'a NodeRef,
    root: &'a NodeRef,
    scope: Option<&'a Scope>,
}

impl<'a> Env<'a> {
    pub fn new(root: &'a NodeRef, current: &'a NodeRef, scope: Option<&'a Scope>) -> Env<'a> {
        Env {
            current,
            root,
            scope,
        }
    }

    pub fn with_current(&'a self, current: &'a NodeRef) -> Env<'a> {
        Env { current, ..*self }
    }

    pub fn with_root(&'a self, root: &'a NodeRef) -> Env<'a> {
        Env { root, ..*self }
    }

    pub fn with_ext(&'a self, scope: Option<&'a Scope>) -> Env<'a> {
        Env { scope, ..*self }
    }

    pub fn current(&self) -> &NodeRef {
        self.current
    }

    pub fn root(&self) -> &NodeRef {
        self.root
    }

    pub fn scope(&self) -> Option<&Scope> {
        self.scope
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Context {
    Expr,
    Property,
    Index,
}

#[derive(Debug)]
pub struct NodeBuf {
    multiple: bool,
    elems: Vec<NodeRef>,
}

impl NodeBuf {
    fn new() -> NodeBuf {
        NodeBuf {
            multiple: false,
            elems: Vec::new(),
        }
    }

    pub fn add(&mut self, n: NodeRef) {
        self.elems.push(n);
    }

    pub fn add_all(&mut self, n: &NodeSet) {
        match *n {
            NodeSet::Empty => {}
            NodeSet::One(ref n) => {
                self.elems.push(n.clone());
            }
            NodeSet::Many(ref elems) => {
                self.append(elems.iter());
            }
        }
    }

    pub fn append<'a, I>(&mut self, iter: I)
    where
        I: Iterator<Item = &'a NodeRef> + ExactSizeIterator,
    {
        self.multiple = true;
        self.elems.reserve(iter.len());
        for n in iter {
            self.elems.push(n.clone());
        }
    }

    fn merge_multiple(&mut self, multiple: bool) {
        self.multiple = self.multiple || multiple;
    }

    fn merge(&mut self, mut o: NodeBuf) {
        self.merge_multiple(o.multiple);
        self.elems.reserve(o.elems.len());
        self.elems.append(&mut o.elems);
    }

    fn into_node_set(mut self) -> NodeSet {
        match self.elems.len() {
            0 => NodeSet::Empty,
            1 => {
                if self.multiple {
                    NodeSet::Many(self.elems)
                } else {
                    NodeSet::One(self.elems.pop().unwrap())
                }
            }
            _ => NodeSet::Many(self.elems),
        }
    }

    fn make_consumable(&mut self, consumable: bool) {
        if consumable {
            for v in self.elems.iter_mut() {
                if !v.is_consumable() {
                    *v = v.deep_copy();
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.elems.clear();
        self.multiple = false;
    }
}

#[derive(Debug, Clone)]
pub(super) enum Expr {
    String(String),
    StringEnc(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
    Concat(Vec<Expr>),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    StartsWith(Box<Expr>, Box<Expr>),
    EndsWith(Box<Expr>, Box<Expr>),
    Contains(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Ne(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Ge(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Le(Box<Expr>, Box<Expr>),
    Root,
    Current,
    Parent,
    All,
    Ancestors(Box<LevelRange>),
    Descendants(Box<LevelRange>),
    Property(Box<Expr>),
    Index(Box<Expr>),
    Range(Box<NumberRange>),
    Group(Vec<Expr>),
    Sequence(Vec<Expr>),
    MethodCall(Box<MethodCall>),
    FuncCall(Box<FuncCall>),
    Var(Box<Expr>),
    Env(Box<Expr>),
}

impl Expr {
    fn tag(&self) -> u8 {
        unsafe { *std::mem::transmute::<&Expr, &u8>(self) }
    }

    fn apply_to(&self, env: Env<'_>, ctx: Context, out: &mut NodeBuf) -> ExprResult<()> {
        use std::{f64, i64};

        #[inline]
        fn to_abs_index(index: i64, len: usize) -> usize {
            if index < 0 {
                let index = len as i64 + index;
                if index >= 0 {
                    index as usize
                } else {
                    len
                }
            } else {
                index as usize
            }
        }

        fn get_child_all(current: &NodeRef, out: &mut NodeBuf) {
            match *current.data().value() {
                Value::Array(ref elems) => out.append(elems.iter()),
                Value::Object(ref props) => out.append(props.values()),
                _ => {}
            }
        }

        fn get_child_index(current: &NodeRef, index: i64, out: &mut NodeBuf) {
            match *current.data().value() {
                Value::Array(ref elems) => {
                    let index = to_abs_index(index, elems.len());
                    if let Some(e) = elems.get(index) {
                        out.add(e.clone());
                    }
                }
                Value::Object(ref props) => {
                    let index = to_abs_index(index, props.len());
                    if let Some(e) = props.values().nth(index) {
                        out.add(e.clone());
                    }
                }
                _ => {}
            }
        }

        fn get_child_key(current: &NodeRef, key: &str, out: &mut NodeBuf) {
            match key {
                "@key" => out.add(NodeRef::string(current.data().key())),
                "@index" => out.add(NodeRef::integer(current.data().index() as i64)),
                "@level" => out.add(NodeRef::integer(current.data().level() as i64)),
                "@type" => out.add(NodeRef::string(current.data().kind().as_type_str())),
                "@kind" => out.add(NodeRef::string(current.data().kind().as_str())),
                "@file" => out.add(NodeRef::string(current.data().file_string())),
                "@file_abs" => out.add(NodeRef::string(current.data().file_string_abs())),
                "@file_type" => out.add(NodeRef::string(current.data().file_type())),
                "@file_format" => out.add(NodeRef::string(current.data().file_format())),
                "@file_path" => out.add(NodeRef::string(current.data().file_path())),
                "@file_path_abs" => out.add(NodeRef::string(current.data().file_path_abs())),
                "@file_name" => out.add(NodeRef::string(current.data().file_name())),
                "@file_stem" => out.add(NodeRef::string(current.data().file_stem())),
                "@file_ext" => out.add(NodeRef::string(current.data().file_ext())),
                "@file_path_components" => {
                    let d = current.data();
                    let array: Vec<NodeRef> = d
                        .file_path_components()
                        .map(|c| NodeRef::string(c))
                        .collect();
                    out.add(NodeRef::array(array));
                }
                "@dir" => out.add(NodeRef::string(current.data().dir())),
                "@dir_abs" => out.add(NodeRef::string(current.data().dir_abs())),
                "@path" => out.add(NodeRef::string(Opath::from(current).to_string())),
                _ => match *current.data().value() {
                    Value::Array(ref elems) => {
                        if let Ok(index) = key.parse::<f64>() {
                            let index = to_abs_index(index as i64, elems.len());
                            if let Some(e) = elems.get(index) {
                                out.add(e.clone());
                            }
                        }
                    }
                    Value::Object(ref props) => {
                        if let Some(e) = props.get(key) {
                            out.add(e.clone());
                        } else if let Ok(index) = key.parse::<f64>() {
                            let index = to_abs_index(index as i64, props.len());
                            if let Some(e) = props.values().nth(index) {
                                out.add(e.clone());
                            }
                        }
                    }
                    _ => {}
                },
            }
        }

        fn add_descendants(
            current: &NodeRef,
            level: i64,
            level_min: i64,
            level_max: i64,
            out: &mut NodeBuf,
        ) {
            if level <= level_max {
                if level >= level_min {
                    out.add(current.clone());
                }
                if level < level_max {
                    match *current.data().value() {
                        Value::Array(ref elems) => {
                            for e in elems.iter() {
                                add_descendants(e, level + 1, level_min, level_max, out);
                            }
                        }
                        Value::Object(ref props) => {
                            for e in props.values() {
                                add_descendants(e, level + 1, level_min, level_max, out);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        #[inline]
        fn math_binary_op<F>(
            env: Env<'_>,
            ctx: Context,
            a: &Expr,
            b: &Expr,
            op: F,
            out: &mut NodeBuf,
        ) -> ApplyResult
        where
            F: Fn(&NodeRef, Context, &NodeRef, &NodeRef, &mut NodeBuf) -> ApplyResult,
        {
            let a = a.apply(env, Context::Expr)?;
            let b = b.apply(env, Context::Expr)?;
            let current = env.current();
            match (a, b) {
                (NodeSet::Empty, NodeSet::Empty) | (NodeSet::Empty, _) | (_, NodeSet::Empty) => {
                    apply_float(current, ctx, f64::NAN, out)
                }
                (NodeSet::One(a), NodeSet::One(b)) => op(current, ctx, &a, &b, out),
                (NodeSet::One(a), NodeSet::Many(b)) => {
                    for b in b {
                        op(current, ctx, &a, &b, out)?;
                    }
                    Ok(())
                }
                (NodeSet::Many(a), NodeSet::One(b)) => {
                    for a in a {
                        op(current, ctx, &a, &b, out)?;
                    }
                    Ok(())
                }
                (NodeSet::Many(a), NodeSet::Many(b)) => {
                    for (a, b) in a.into_iter().zip(b.into_iter()) {
                        op(current, ctx, &a, &b, out)?;
                    }
                    Ok(())
                }
            }
        }

        #[inline]
        fn bool_binary_op<F>(
            env: Env<'_>,
            ctx: Context,
            a: &Expr,
            b: &Expr,
            op: F,
            out: &mut NodeBuf,
        ) -> ApplyResult
        where
            F: Fn(&NodeRef, &NodeRef) -> bool,
        {
            #[inline]
            fn bool_op<F>(
                env: Env<'_>,
                ctx: Context,
                a: &Expr,
                b: &Expr,
                op: &F,
                out: &mut NodeBuf,
            ) -> ApplyResult
            where
                F: Fn(&NodeRef, &NodeRef) -> bool,
            {
                let a = a.apply(env, Context::Expr)?;
                let b = b.apply(env, Context::Expr)?;
                let current = env.current();
                match (a, b) {
                    (NodeSet::Empty, NodeSet::Empty)
                    | (NodeSet::Empty, _)
                    | (_, NodeSet::Empty) => apply_boolean(current, ctx, false, out),
                    (NodeSet::One(a), NodeSet::One(b)) => {
                        apply_boolean(current, ctx, op(&a, &b), out)
                    }
                    (NodeSet::One(a), NodeSet::Many(b)) => {
                        for b in b {
                            apply_boolean(current, ctx, op(&a, &b), out)?;
                        }
                        Ok(())
                    }
                    (NodeSet::Many(a), NodeSet::One(b)) => {
                        for a in a {
                            apply_boolean(current, ctx, op(&a, &b), out)?;
                        }
                        Ok(())
                    }
                    (NodeSet::Many(a), NodeSet::Many(b)) => {
                        for (a, b) in a.into_iter().zip(b.into_iter()) {
                            apply_boolean(current, ctx, op(&a, &b), out)?;
                        }
                        Ok(())
                    }
                }
            }

            if !out.multiple && (ctx == Context::Property || ctx == Context::Index) {
                match *env.current().data().value() {
                    Value::Array(ref elems) => {
                        for e in elems.iter() {
                            bool_op(env.with_current(e), ctx, a, b, &op, out)?;
                        }
                        Ok(())
                    }
                    Value::Object(ref props) => {
                        for e in props.values() {
                            bool_op(env.with_current(e), ctx, a, b, &op, out)?;
                        }
                        Ok(())
                    }
                    _ => Ok(()),
                }
            } else {
                bool_op(env, ctx, a, b, &op, out)
            }
        }

        #[inline]
        fn bool_or_op(
            env: Env<'_>,
            ctx: Context,
            a: &Expr,
            b: &Expr,
            out: &mut NodeBuf,
        ) -> ApplyResult {
            #[inline]
            fn bool_or(
                env: Env<'_>,
                ctx: Context,
                a: &Expr,
                b: &Expr,
                out: &mut NodeBuf,
            ) -> ApplyResult {
                let na = a.apply(env, Context::Expr)?;
                match na {
                    NodeSet::Empty => {
                        let nb = b.apply(env, Context::Expr)?;
                        for b in nb.into_iter() {
                            apply_node(env.current(), ctx, b, out)?;
                        }
                        Ok(())
                    }
                    NodeSet::One(a) => {
                        if a.as_boolean() {
                            apply_node(env.current(), ctx, a, out)
                        } else {
                            let nb = b.apply(env, Context::Expr)?;
                            for b in nb.into_iter() {
                                apply_node(env.current(), ctx, b, out)?;
                            }
                            Ok(())
                        }
                    }
                    NodeSet::Many(a) => {
                        let nb = b.apply(env, Context::Expr)?;
                        match nb {
                            NodeSet::Empty => {
                                for a in a.into_iter() {
                                    apply_node(env.current(), ctx, a, out)?;
                                }
                                Ok(())
                            }
                            NodeSet::One(b) => {
                                for a in a.into_iter() {
                                    if a.as_boolean() {
                                        apply_node(env.current(), ctx, a, out)?;
                                    } else {
                                        apply_node(env.current(), ctx, b.clone(), out)?;
                                    }
                                }
                                Ok(())
                            }
                            NodeSet::Many(b) => {
                                for (a, b) in a.into_iter().zip(b.into_iter()) {
                                    if a.as_boolean() {
                                        apply_node(env.current(), ctx, a, out)?;
                                    } else {
                                        apply_node(env.current(), ctx, b, out)?;
                                    }
                                }
                                Ok(())
                            }
                        }
                    }
                }
            }

            if !out.multiple && (ctx == Context::Property || ctx == Context::Index) {
                match *env.current().data().value() {
                    Value::Array(ref elems) => {
                        for e in elems.iter() {
                            bool_or(env.with_current(e), ctx, a, b, out)?;
                        }
                        Ok(())
                    }
                    Value::Object(ref props) => {
                        for e in props.values() {
                            bool_or(env.with_current(e), ctx, a, b, out)?;
                        }
                        Ok(())
                    }
                    _ => Ok(()),
                }
            } else {
                bool_or(env, ctx, a, b, out)
            }
        }

        #[inline]
        fn bool_not_op(env: Env<'_>, ctx: Context, a: &Expr, out: &mut NodeBuf) -> ApplyResult {
            #[inline]
            fn not_op(env: Env<'_>, ctx: Context, a: &Expr, out: &mut NodeBuf) -> ApplyResult {
                let a = a.apply(env, Context::Expr)?;
                match a {
                    NodeSet::Empty => apply_boolean(env.current(), ctx, true, out),
                    NodeSet::One(a) => apply_boolean(env.current(), ctx, !a.as_boolean(), out),
                    NodeSet::Many(a) => {
                        for a in a {
                            apply_boolean(env.current(), ctx, !a.as_boolean(), out)?
                        }
                        Ok(())
                    }
                }
            }

            if ctx == Context::Property || ctx == Context::Index {
                match *env.current().data().value() {
                    Value::Array(ref elems) => {
                        for e in elems.iter() {
                            not_op(env.with_current(e), ctx, a, out)?;
                        }
                        Ok(())
                    }
                    Value::Object(ref props) => {
                        for e in props.values() {
                            not_op(env.with_current(e), ctx, a, out)?;
                        }
                        Ok(())
                    }
                    _ => Ok(()),
                }
            } else {
                not_op(env, ctx, a, out)
            }
        }

        #[inline]
        fn apply_string(
            current: &NodeRef,
            ctx: Context,
            s: Cow<str>,
            out: &mut NodeBuf,
        ) -> ApplyResult {
            match ctx {
                Context::Property | Context::Index => get_child_key(current, &s, out),
                _ => out.add(NodeRef::string(s)),
            };
            Ok(())
        }

        #[inline]
        fn apply_integer(
            current: &NodeRef,
            ctx: Context,
            n: i64,
            out: &mut NodeBuf,
        ) -> ApplyResult {
            match ctx {
                Context::Property | Context::Index => get_child_index(current, n, out),
                _ => out.add(NodeRef::integer(n)),
            };
            Ok(())
        }

        #[inline]
        fn apply_float(current: &NodeRef, ctx: Context, n: f64, out: &mut NodeBuf) -> ApplyResult {
            match ctx {
                Context::Property | Context::Index => get_child_index(current, n as i64, out),
                _ => out.add(NodeRef::float(n)),
            }
            Ok(())
        }

        #[inline]
        fn apply_boolean(
            current: &NodeRef,
            ctx: Context,
            b: bool,
            out: &mut NodeBuf,
        ) -> ApplyResult {
            match ctx {
                Context::Property | Context::Index => {
                    if b {
                        out.add(current.clone());
                    }
                }
                _ => out.add(NodeRef::boolean(b)),
            }
            Ok(())
        }

        #[inline]
        fn apply_null(_: &NodeRef, ctx: Context, out: &mut NodeBuf) -> ApplyResult {
            match ctx {
                Context::Property | Context::Index => {}
                _ => out.add(NodeRef::null()),
            }
            Ok(())
        }

        #[inline]
        fn apply_node(
            current: &NodeRef,
            ctx: Context,
            n: NodeRef,
            out: &mut NodeBuf,
        ) -> ApplyResult {
            match ctx {
                Context::Property | Context::Index => match *n.data().value() {
                    Value::Null => {}
                    Value::Boolean(b) => {
                        if b {
                            out.add(current.clone());
                        }
                    }
                    Value::Integer(n) => get_child_index(current, n, out),
                    Value::Float(n) => get_child_index(current, n as i64, out),
                    Value::String(ref s) => get_child_key(current, s, out),
                    Value::Binary(_) | Value::Array(_) | Value::Object(_) => {
                        if n.as_boolean() {
                            out.add(current.clone());
                        }
                    }
                },
                _ => out.add(n),
            }
            Ok(())
        }

        #[inline]
        fn add(
            current: &NodeRef,
            ctx: Context,
            a: &NodeRef,
            b: &NodeRef,
            out: &mut NodeBuf,
        ) -> ApplyResult {
            let a = a.data();
            let b = b.data();
            match (a.value(), b.value()) {
                (&Value::Object(_), &Value::Array(ref elems)) => {
                    if elems.len() == 0 {
                        apply_float(current, ctx, 0f64, out)
                    } else {
                        apply_float(current, ctx, f64::NAN, out)
                    }
                }
                (&Value::Array(_), _) | (_, &Value::Array(_)) | (_, &Value::Object(_)) => {
                    let a = a.as_string();
                    let b = b.as_string();
                    let mut s = String::with_capacity(a.len() + b.len());
                    s.push_str(a.as_ref());
                    s.push_str(b.as_ref());
                    apply_string(current, ctx, s.into(), out)
                }
                (&Value::Object(_), _) => apply_float(current, ctx, 0f64 + b.as_float(), out),
                (&Value::String(ref a), &Value::String(ref b)) => {
                    let mut s = String::with_capacity(a.len() + b.len());
                    s.push_str(a);
                    s.push_str(b);
                    apply_string(current, ctx, s.into(), out)
                }
                (&Value::String(ref a), _) => {
                    let b = &b.as_string();
                    let mut s = String::with_capacity(a.len() + b.len());
                    s.push_str(a);
                    s.push_str(b);
                    apply_string(current, ctx, s.into(), out)
                }
                (_, &Value::String(ref b)) => {
                    let a = &a.as_string();
                    let mut s = String::with_capacity(a.len() + b.len());
                    s.push_str(a);
                    s.push_str(b);
                    apply_string(current, ctx, s.into(), out)
                }
                (&Value::Integer(a), &Value::Integer(b)) => match a.checked_add(b) {
                    Some(res) => apply_integer(current, ctx, res, out),
                    None => apply_float(current, ctx, a as f64 + b as f64, out),
                },
                (&Value::Float(a), &Value::Float(b)) => apply_float(current, ctx, a + b, out),
                (&Value::Float(a), _) => apply_float(current, ctx, a + b.as_float(), out),
                (_, &Value::Float(b)) => apply_float(current, ctx, a.as_float() + b, out),
                (_, _) => apply_float(current, ctx, a.as_float() + b.as_float(), out),
            }
        }

        #[inline]
        fn sub(
            current: &NodeRef,
            ctx: Context,
            a: &NodeRef,
            b: &NodeRef,
            out: &mut NodeBuf,
        ) -> ApplyResult {
            let a = a.data();
            let b = b.data();
            match (a.value(), b.value()) {
                (&Value::Integer(a), &Value::Integer(b)) => match a.checked_sub(b) {
                    Some(res) => apply_integer(current, ctx, res, out),
                    None => apply_float(current, ctx, a as f64 - b as f64, out),
                },
                (&Value::Object(_), &Value::Array(ref elems)) => {
                    if elems.len() == 0 {
                        apply_float(current, ctx, 0f64, out)
                    } else {
                        apply_float(current, ctx, f64::NAN, out)
                    }
                }
                (&Value::Array(ref elems1), &Value::Array(ref elems2)) => {
                    if elems1.len() == 0 && elems2.len() == 0 {
                        apply_float(current, ctx, 0f64, out)
                    } else {
                        apply_float(current, ctx, f64::NAN, out)
                    }
                }
                (&Value::Array(ref elems), _) => {
                    if elems.len() == 0 {
                        apply_float(current, ctx, -b.as_float(), out)
                    } else {
                        apply_float(current, ctx, f64::NAN, out)
                    }
                }
                (_, &Value::Array(ref elems)) => {
                    if elems.len() == 0 {
                        apply_float(current, ctx, a.as_float(), out)
                    } else {
                        apply_float(current, ctx, f64::NAN, out)
                    }
                }
                (&Value::Object(ref obj), _) => {
                    if obj.len() == 0 {
                        apply_float(current, ctx, -b.as_float(), out)
                    } else {
                        apply_float(current, ctx, f64::NAN, out)
                    }
                }
                (&Value::Float(a), &Value::Float(b)) => apply_float(current, ctx, a - b, out),
                (&Value::Float(a), _) => apply_float(current, ctx, a - b.as_float(), out),
                (_, &Value::Float(b)) => apply_float(current, ctx, a.as_float() - b, out),
                (_, _) => apply_float(current, ctx, a.as_float() - b.as_float(), out),
            }
        }

        #[inline]
        fn mul(
            current: &NodeRef,
            ctx: Context,
            a: &NodeRef,
            b: &NodeRef,
            out: &mut NodeBuf,
        ) -> ApplyResult {
            let a = a.data();
            let b = b.data();
            match (a.value(), b.value()) {
                (&Value::Integer(a), &Value::Integer(b)) => match a.checked_mul(b) {
                    Some(res) => apply_integer(current, ctx, res, out),
                    None => apply_float(current, ctx, a as f64 * b as f64, out),
                },
                (&Value::Array(ref elems1), &Value::Array(ref elems2)) => {
                    if elems1.len() == 0 && elems2.len() == 0 {
                        apply_float(current, ctx, 0f64, out)
                    } else {
                        apply_float(current, ctx, f64::NAN, out)
                    }
                }
                (&Value::Array(ref elems), _) => {
                    if elems.len() == 0 {
                        apply_float(current, ctx, 0f64 * b.as_float(), out)
                    } else {
                        apply_float(current, ctx, f64::NAN, out)
                    }
                }
                (_, &Value::Array(ref elems)) => {
                    if elems.len() == 0 {
                        apply_float(current, ctx, a.as_float() * 0f64, out)
                    } else {
                        apply_float(current, ctx, f64::NAN, out)
                    }
                }
                (&Value::Float(a), &Value::Float(b)) => apply_float(current, ctx, a * b, out),
                (&Value::Float(a), _) => apply_float(current, ctx, a * b.as_float(), out),
                (_, &Value::Float(b)) => apply_float(current, ctx, a.as_float() * b, out),
                (_, _) => apply_float(current, ctx, a.as_float() * b.as_float(), out),
            }
        }

        #[inline]
        fn div(
            current: &NodeRef,
            ctx: Context,
            a: &NodeRef,
            b: &NodeRef,
            out: &mut NodeBuf,
        ) -> ApplyResult {
            let a = a.data();
            let b = b.data();
            match (a.value(), b.value()) {
                (&Value::Array(ref elems), _) => {
                    if elems.len() == 0 {
                        apply_float(current, ctx, 0f64 / b.as_float(), out)
                    } else {
                        apply_float(current, ctx, f64::NAN, out)
                    }
                }
                (_, &Value::Array(ref elems)) => {
                    if elems.len() == 0 {
                        apply_float(current, ctx, a.as_float() / 0f64, out)
                    } else {
                        apply_float(current, ctx, f64::NAN, out)
                    }
                }
                (&Value::Float(a), &Value::Float(b)) => apply_float(current, ctx, a / b, out),
                (&Value::Float(a), _) => apply_float(current, ctx, a / b.as_float(), out),
                (_, &Value::Float(b)) => apply_float(current, ctx, a.as_float() / b, out),
                (_, _) => apply_float(current, ctx, a.as_float() / b.as_float(), out),
            }
        }

        match *self {
            Expr::String(ref s) => apply_string(env.current(), ctx, s.as_str().into(), out),
            Expr::StringEnc(ref s) => apply_string(env.current(), ctx, s.as_str().into(), out),
            Expr::Integer(n) => apply_integer(env.current(), ctx, n, out),
            Expr::Float(n) => apply_float(env.current(), ctx, n, out),
            Expr::Boolean(b) => apply_boolean(env.current(), ctx, b, out),
            Expr::Null => apply_null(env.current(), ctx, out),
            Expr::Concat(ref elems) => {
                let mut res = NodeBuf::new();
                for e in elems.iter() {
                    e.apply_to(env, ctx, &mut res)?;
                }
                let mut buf = String::new();
                for n in res.elems.drain(..) {
                    buf.push_str(&n.data().as_string());
                }
                out.add(NodeRef::string(buf));
                Ok(())
            }
            Expr::Neg(ref a) => {
                let mut res = NodeBuf::new();
                a.apply_to(env, Context::Expr, &mut res)?;
                out.merge_multiple(res.multiple);
                for n in res.elems.drain(..) {
                    apply_float(env.current(), ctx, -n.data().as_float(), out)?;
                }
                Ok(())
            }
            Expr::Add(ref a, ref b) => math_binary_op(env, ctx, a, b, add, out),
            Expr::Sub(ref a, ref b) => math_binary_op(env, ctx, a, b, sub, out),
            Expr::Mul(ref a, ref b) => math_binary_op(env, ctx, a, b, mul, out),
            Expr::Div(ref a, ref b) => math_binary_op(env, ctx, a, b, div, out),
            Expr::Not(ref a) => bool_not_op(env, ctx, a, out),
            Expr::And(ref a, ref b) => {
                bool_binary_op(env, ctx, a, b, |a, b| a.as_boolean() && b.as_boolean(), out)
            }
            Expr::Or(ref a, ref b) => bool_or_op(env, ctx, a, b, out),
            Expr::Eq(ref a, ref b) => bool_binary_op(env, ctx, a, b, |a, b| a == b, out),
            Expr::Ne(ref a, ref b) => bool_binary_op(env, ctx, a, b, |a, b| a != b, out),
            Expr::Lt(ref a, ref b) => bool_binary_op(env, ctx, a, b, |a, b| a < b, out),
            Expr::Le(ref a, ref b) => bool_binary_op(env, ctx, a, b, |a, b| a <= b, out),
            Expr::Gt(ref a, ref b) => bool_binary_op(env, ctx, a, b, |a, b| a > b, out),
            Expr::Ge(ref a, ref b) => bool_binary_op(env, ctx, a, b, |a, b| a >= b, out),
            Expr::StartsWith(ref a, ref b) => bool_binary_op(
                env,
                ctx,
                a,
                b,
                |a, b| {
                    a.data()
                        .as_string()
                        .as_ref()
                        .starts_with(b.data().as_string().as_ref())
                },
                out,
            ),
            Expr::EndsWith(ref a, ref b) => bool_binary_op(
                env,
                ctx,
                a,
                b,
                |a, b| {
                    a.data()
                        .as_string()
                        .as_ref()
                        .ends_with(b.data().as_string().as_ref())
                },
                out,
            ),
            Expr::Contains(ref a, ref b) => bool_binary_op(
                env,
                ctx,
                a,
                b,
                |a, b| {
                    a.data()
                        .as_string()
                        .as_ref()
                        .contains(b.data().as_string().as_ref())
                },
                out,
            ),
            Expr::Root => {
                out.add(env.root().clone());
                Ok(())
            }
            Expr::Current => {
                out.add(env.current().clone());
                Ok(())
            }
            Expr::Parent => {
                if let Some(p) = env.current().data().parent() {
                    out.add(p);
                }
                Ok(())
            }
            Expr::Ancestors(ref r) => {
                out.multiple = true;
                let nmin = r.min.apply_one(env, Context::Expr)?;
                let nmax = r.max.apply_one(env, Context::Expr)?;
                let min = nmin.data().as_integer().unwrap_or(1);
                let max = nmax.data().as_integer().unwrap_or(i64::MAX);
                if min >= 0 && max >= min {
                    let mut curr = env.current().clone();
                    let mut parent;
                    let mut level = 0;
                    loop {
                        if level > max {
                            break;
                        }
                        if level >= min {
                            out.add(curr.clone());
                        }
                        level += 1;
                        if let Some(p) = curr.data().parent() {
                            parent = p;
                        } else {
                            break;
                        }
                        curr = parent;
                    }
                }
                Ok(())
            }
            Expr::Descendants(ref r) => {
                out.multiple = true;
                let nmin = r.min().apply_one(env, Context::Expr)?;
                let nmax = r.max().apply_one(env, Context::Expr)?;
                let min = nmin.data().as_integer().unwrap_or(1);
                let max = nmax.data().as_integer().unwrap_or(i64::MAX);
                if min >= 0 && max >= min {
                    add_descendants(env.current(), 0, min, max, out);
                }
                Ok(())
            }
            Expr::All => match ctx {
                Context::Property | Context::Index => {
                    out.multiple = true;
                    get_child_all(env.current(), out);
                    Ok(())
                }
                _ => unreachable!(),
            },
            Expr::Property(ref e) => e.apply_to(env, Context::Property, out),
            Expr::Index(ref e) => e.apply_to(env, Context::Index, out),
            Expr::Range(ref r) => {
                fn get_opt_float(env: Env<'_>, e: Option<&Expr>) -> ExprResult<Option<f64>> {
                    match e {
                        None => Ok(None),
                        Some(e) => Ok(Some(e.apply_one(env, Context::Expr)?.as_float())),
                    }
                }

                let mut start;
                let mut stop;

                if ctx == Context::Index || ctx == Context::Property {
                    let len = env.current().data().children_count().unwrap_or(0);
                    if len == 0 {
                        return Ok(());
                    }
                    let len = len as f64;
                    let last = len - 1.;
                    start = get_opt_float(env, r.start())?.unwrap_or(0.);
                    stop = get_opt_float(env, r.stop())?.unwrap_or(last);
                    if start < 0. {
                        start += len;
                    }
                    if stop < 0. {
                        stop += len;
                    }
                    if stop < 0. && start == 0. {
                        return Ok(());
                    }
                    if start > last && stop > last {
                        return Ok(());
                    }
                    start = start.clamp(0., last);
                    stop = stop.clamp(0., last);
                } else {
                    start = get_opt_float(env, r.start())?.unwrap_or(0.);
                    stop = get_opt_float(env, r.stop())?.unwrap_or(0.);
                }

                if (start - stop).abs() < std::f64::EPSILON {
                    apply_float(env.current(), ctx, start, out)
                } else if start < stop {
                    let step = get_opt_float(env, r.step())?.unwrap_or(1f64);
                    if step > 0f64 {
                        loop {
                            apply_float(env.current(), ctx, start, out)?;
                            start += step;
                            if start > stop {
                                break;
                            }
                        }
                    }
                    Ok(())
                } else {
                    let step = get_opt_float(env, r.step())?.unwrap_or(-1f64);
                    if step < 0f64 {
                        loop {
                            apply_float(env.current(), ctx, start, out)?;
                            start += step;
                            if start < stop {
                                break;
                            }
                        }
                    }
                    Ok(())
                }
            }
            Expr::Group(ref elems) => {
                out.multiple = true;
                for e in elems.iter() {
                    e.apply_to(env, ctx, out)?;
                }
                Ok(())
            }
            Expr::Sequence(ref elems) => {
                let mut out1 = NodeBuf::new();
                let mut out2 = NodeBuf::new();
                out1.add(env.current().clone());

                for e in elems.iter() {
                    out2.clear();
                    out2.merge_multiple(out1.multiple);
                    for n in out1.elems.iter() {
                        e.apply_to(env.with_current(n), Context::Expr, &mut out2)?;
                    }
                    std::mem::swap(&mut out1, &mut out2);
                }
                if ctx == Context::Index {
                    for n in out1.elems {
                        apply_node(env.current(), ctx, n, out)?;
                    }
                    Ok(())
                } else {
                    out.merge(out1);
                    Ok(())
                }
            }
            Expr::MethodCall(ref call) => {
                func::apply_method_to(call.id(), call.args(), env, ctx, out)
            }
            Expr::FuncCall(ref call) => func::apply_func_to(call.id(), call.args(), env, ctx, out),
            Expr::Var(ref e) => {
                if let Some(scope) = env.scope() {
                    let res = e.apply(env, Context::Expr)?;
                    match res {
                        NodeSet::Empty => Ok(()),
                        NodeSet::One(n) => {
                            if let Some(var) = scope.get_var(&n.data().as_string()) {
                                out.add_all(&var);
                            }
                            Ok(())
                        }
                        _ => unimplemented!(), //FIXME (jc) probably report error?
                    }
                } else {
                    Ok(())
                }
            }
            Expr::Env(ref e) => {
                let res = e.apply(env, Context::Expr)?;
                match res {
                    NodeSet::Empty => unimplemented!(), //FIXME (jc) probably report error?
                    NodeSet::One(node) => {
                        let var_name = node.as_string();
                        let res = std::env::var(var_name).unwrap_or(String::new());
                        out.add(NodeRef::string(res));
                        Ok(())
                    }
                    NodeSet::Many(nodes) => {
                        for node in nodes {
                            let var_name = node.as_string();
                            let res = std::env::var(var_name).unwrap_or(String::new());
                            out.add(NodeRef::string(res));
                        }
                        Ok(())
                    }
                }
            }
        }
    }

    pub(super) fn apply(&self, env: Env<'_>, ctx: Context) -> ExprResult<NodeSet> {
        let mut out = NodeBuf::new();
        self.apply_to(env, ctx, &mut out)?;
        Ok(out.into_node_set())
    }

    pub(super) fn apply_one(&self, env: Env<'_>, ctx: Context) -> ExprResult<NodeRef> {
        let n = self.apply(env, ctx)?;
        let res = match n {
            NodeSet::Empty => NodeRef::null(),
            NodeSet::One(n) => n,
            NodeSet::Many(_) => panic!("multiple results returned"),
        };
        Ok(res)
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[inline]
        fn display_list(
            f: &mut std::fmt::Formatter,
            elems: &Vec<Expr>,
            sep: &str,
        ) -> std::fmt::Result {
            let mut it = elems.iter().peekable();
            while let Some(e) = it.next() {
                write!(f, "{}", e)?;
                if it.peek().is_some() {
                    write!(f, "{}", sep)?;
                }
            }
            Ok(())
        }

        match *self {
            Expr::String(ref s) => write!(f, "{}", s),
            Expr::StringEnc(ref s) => write!(f, "'{}'", s.escape_default()),
            Expr::Integer(n) => write!(f, "{}", n),
            Expr::Float(n) => write!(f, "{}", n),
            Expr::Boolean(b) => write!(f, "{}", b),
            Expr::Null => write!(f, "null"),
            Expr::Concat(ref elems) => {
                write!(f, "(")?;
                display_list(f, elems, " + ")?;
                write!(f, ")")?;
                Ok(())
            }
            Expr::Neg(ref a) => write!(f, "-({})", a),
            Expr::Add(ref a, ref b) => write!(f, "({} + {})", a, b),
            Expr::Sub(ref a, ref b) => write!(f, "({} - {})", a, b),
            Expr::Mul(ref a, ref b) => write!(f, "({} * {})", a, b),
            Expr::Div(ref a, ref b) => write!(f, "({} / {})", a, b),
            Expr::Not(ref a) => write!(f, "!({})", a),
            Expr::And(ref a, ref b) => write!(f, "({} and {})", a, b),
            Expr::Or(ref a, ref b) => write!(f, "({} or {})", a, b),
            Expr::StartsWith(ref a, ref b) => write!(f, "({} ^= {})", a, b),
            Expr::EndsWith(ref a, ref b) => write!(f, "({} $= {})", a, b),
            Expr::Contains(ref a, ref b) => write!(f, "({} *= {})", a, b),
            Expr::Eq(ref a, ref b) => write!(f, "({} == {})", a, b),
            Expr::Ne(ref a, ref b) => write!(f, "({} != {})", a, b),
            Expr::Gt(ref a, ref b) => write!(f, "({} > {})", a, b),
            Expr::Ge(ref a, ref b) => write!(f, "({} >= {})", a, b),
            Expr::Lt(ref a, ref b) => write!(f, "({} < {})", a, b),
            Expr::Le(ref a, ref b) => write!(f, "({} <= {})", a, b),
            Expr::Root => write!(f, "$"),
            Expr::Current => write!(f, "@"),
            Expr::Parent => write!(f, "^"),
            Expr::All => write!(f, "*"),
            Expr::Ancestors(ref l) => write!(f, "^**{}", l),
            Expr::Descendants(ref l) => write!(f, ".**{}", l),
            Expr::Property(ref e) => write!(f, ".{:#}", e),
            Expr::Index(ref e) => write!(f, "[{}]", e),
            Expr::Range(ref r) => write!(f, "{}", r),
            Expr::Group(ref elems) => {
                write!(f, "(")?;
                display_list(f, elems, ", ")?;
                write!(f, ")")?;
                Ok(())
            }
            Expr::Sequence(ref elems) => {
                for e in elems.iter() {
                    write!(f, "{}", e)?;
                }
                Ok(())
            }
            Expr::MethodCall(ref call) => {
                write!(f, ".{}(", call.id().name())?;
                display_list(f, call.args().as_vec(), ", ")?;
                write!(f, ")")?;
                Ok(())
            }
            Expr::FuncCall(ref call) => {
                write!(f, "{}(", call.id().name())?;
                display_list(f, call.args().as_vec(), ", ")?;
                write!(f, ")")?;
                Ok(())
            }
            Expr::Var(ref e) => match **e {
                Expr::String(ref s) => write!(f, "${}", s),
                _ => write!(f, "${{{}}}", e),
            },
            Expr::Env(ref e) => {
                write!(f, "env:(")?;
                write!(f, "{})", e)?;
                Ok(())
            }
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Expr) -> bool {
        if std::ptr::eq(self, other) {
            true
        } else {
            match (self, other) {
                (&Expr::String(ref s1), &Expr::String(ref s2)) => s1 == s2,
                (&Expr::StringEnc(ref s1), &Expr::StringEnc(ref s2)) => s1 == s2,
                (&Expr::Integer(n1), &Expr::Integer(n2)) => n1 == n2,
                (&Expr::Float(n1), &Expr::Float(n2)) => n1.to_bits() == n2.to_bits(),
                (&Expr::Boolean(b1), &Expr::Boolean(b2)) => b1 == b2,
                (&Expr::Null, &Expr::Null) => true,
                (&Expr::Concat(ref elems1), &Expr::Concat(ref elems2)) => elems1 == elems2,
                (&Expr::Neg(ref a1), &Expr::Neg(ref a2)) => a1 == a2,
                (&Expr::Add(ref a1, ref b1), &Expr::Add(ref a2, ref b2)) => a1 == a2 && b1 == b2,
                (&Expr::Sub(ref a1, ref b1), &Expr::Sub(ref a2, ref b2)) => a1 == a2 && b1 == b2,
                (&Expr::Mul(ref a1, ref b1), &Expr::Mul(ref a2, ref b2)) => a1 == a2 && b1 == b2,
                (&Expr::Div(ref a1, ref b1), &Expr::Div(ref a2, ref b2)) => a1 == a2 && b1 == b2,
                (&Expr::Not(ref a1), &Expr::Not(ref a2)) => a1 == a2,
                (&Expr::And(ref a1, ref b1), &Expr::And(ref a2, ref b2)) => a1 == a2 && b1 == b2,
                (&Expr::Or(ref a1, ref b1), &Expr::Or(ref a2, ref b2)) => a1 == a2 && b1 == b2,
                (&Expr::StartsWith(ref a1, ref b1), &Expr::StartsWith(ref a2, ref b2)) => {
                    a1 == a2 && b1 == b2
                }
                (&Expr::EndsWith(ref a1, ref b1), &Expr::EndsWith(ref a2, ref b2)) => {
                    a1 == a2 && b1 == b2
                }
                (&Expr::Contains(ref a1, ref b1), &Expr::Contains(ref a2, ref b2)) => {
                    a1 == a2 && b1 == b2
                }
                (&Expr::Eq(ref a1, ref b1), &Expr::Eq(ref a2, ref b2)) => a1 == a2 && b1 == b2,
                (&Expr::Ne(ref a1, ref b1), &Expr::Ne(ref a2, ref b2)) => a1 == a2 && b1 == b2,
                (&Expr::Gt(ref a1, ref b1), &Expr::Gt(ref a2, ref b2)) => a1 == a2 && b1 == b2,
                (&Expr::Ge(ref a1, ref b1), &Expr::Ge(ref a2, ref b2)) => a1 == a2 && b1 == b2,
                (&Expr::Lt(ref a1, ref b1), &Expr::Lt(ref a2, ref b2)) => a1 == a2 && b1 == b2,
                (&Expr::Le(ref a1, ref b1), &Expr::Le(ref a2, ref b2)) => a1 == a2 && b1 == b2,
                (&Expr::Root, &Expr::Root) => true,
                (&Expr::Current, &Expr::Current) => true,
                (&Expr::Parent, &Expr::Parent) => true,
                (&Expr::All, &Expr::All) => true,
                (&Expr::Ancestors(ref l1), &Expr::Ancestors(ref l2)) => l1 == l2,
                (&Expr::Descendants(ref l1), &Expr::Descendants(ref l2)) => l1 == l2,
                (&Expr::Property(ref e1), &Expr::Property(ref e2)) => e1 == e2,
                (&Expr::Index(ref e1), &Expr::Index(ref e2)) => e1 == e2,
                (&Expr::Range(ref r1), &Expr::Range(ref r2)) => r1 == r2,
                (&Expr::Group(ref elems1), &Expr::Group(ref elems2)) => elems1 == elems2,
                (&Expr::Sequence(ref elems1), &Expr::Sequence(ref elems2)) => elems1 == elems2,
                (&Expr::MethodCall(ref call1), &Expr::MethodCall(ref call2)) => call1 == call2,
                (&Expr::FuncCall(ref call1), &Expr::FuncCall(ref call2)) => call1 == call2,
                (&Expr::Var(ref e1), &Expr::Var(ref e2)) => e1 == e2,
                (_, _) => false,
            }
        }
    }
}

impl Eq for Expr {}

impl Hash for Expr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u8(self.tag());

        match *self {
            Expr::String(ref s) => s.hash(state),
            Expr::StringEnc(ref s) => s.hash(state),
            Expr::Integer(n) => n.hash(state),
            Expr::Float(n) => n.to_bits().hash(state),
            Expr::Boolean(b) => b.hash(state),
            Expr::Null => {}
            Expr::Concat(ref elems) => elems.hash(state),
            Expr::Neg(ref a) => a.hash(state),
            Expr::Add(ref a, ref b) => {
                a.hash(state);
                b.hash(state);
            }
            Expr::Sub(ref a, ref b) => {
                a.hash(state);
                b.hash(state);
            }
            Expr::Mul(ref a, ref b) => {
                a.hash(state);
                b.hash(state);
            }
            Expr::Div(ref a, ref b) => {
                a.hash(state);
                b.hash(state);
            }
            Expr::Not(ref a) => a.hash(state),
            Expr::And(ref a, ref b) => {
                a.hash(state);
                b.hash(state);
            }
            Expr::Or(ref a, ref b) => {
                a.hash(state);
                b.hash(state);
            }
            Expr::StartsWith(ref a, ref b) => {
                a.hash(state);
                b.hash(state);
            }
            Expr::EndsWith(ref a, ref b) => {
                a.hash(state);
                b.hash(state);
            }
            Expr::Contains(ref a, ref b) => {
                a.hash(state);
                b.hash(state);
            }
            Expr::Eq(ref a, ref b) => {
                a.hash(state);
                b.hash(state);
            }
            Expr::Ne(ref a, ref b) => {
                a.hash(state);
                b.hash(state);
            }
            Expr::Gt(ref a, ref b) => {
                a.hash(state);
                b.hash(state);
            }
            Expr::Ge(ref a, ref b) => {
                a.hash(state);
                b.hash(state);
            }
            Expr::Lt(ref a, ref b) => {
                a.hash(state);
                b.hash(state);
            }
            Expr::Le(ref a, ref b) => {
                a.hash(state);
                b.hash(state);
            }
            Expr::Root => {}
            Expr::Current => {}
            Expr::Parent => {}
            Expr::All => {}
            Expr::Ancestors(ref l) => l.hash(state),
            Expr::Descendants(ref l) => l.hash(state),
            Expr::Property(ref e) => e.hash(state),
            Expr::Index(ref e) => e.hash(state),
            Expr::Range(ref r) => r.hash(state),
            Expr::Group(ref elems) => elems.hash(state),
            Expr::Sequence(ref elems) => elems.hash(state),
            Expr::MethodCall(ref call) => call.hash(state),
            Expr::FuncCall(ref call) => call.hash(state),
            Expr::Var(ref e) => e.hash(state),
            Expr::Env(ref e) => e.hash(state),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod expr {
        use super::*;

        #[test]
        fn size_of_should_be_32() {
            assert_eq!(std::mem::size_of::<Expr>(), 32);
        }

        #[test]
        fn tag() {
            assert_eq!(0, Expr::String("test".to_string()).tag());
            assert_eq!(1, Expr::StringEnc("\"test\"".to_string()).tag());
            assert_eq!(2, Expr::Integer(std::i64::MIN).tag());
            assert_eq!(3, Expr::Float(std::f64::INFINITY).tag());
            assert_eq!(4, Expr::Boolean(true).tag());
            assert_eq!(5, Expr::Null.tag());
            assert_eq!(6, Expr::Concat(Vec::new()).tag());
            assert_eq!(7, Expr::Neg(box Expr::Null).tag());
            assert_eq!(8, Expr::Add(box Expr::Null, box Expr::Null).tag());
            assert_eq!(9, Expr::Sub(box Expr::Null, box Expr::Null).tag());
            assert_eq!(10, Expr::Mul(box Expr::Null, box Expr::Null).tag());
            assert_eq!(11, Expr::Div(box Expr::Null, box Expr::Null).tag());
            assert_eq!(12, Expr::Not(box Expr::Null).tag());
            assert_eq!(13, Expr::And(box Expr::Null, box Expr::Null).tag());
            assert_eq!(14, Expr::Or(box Expr::Null, box Expr::Null).tag());
            assert_eq!(15, Expr::StartsWith(box Expr::Null, box Expr::Null).tag());
            assert_eq!(16, Expr::EndsWith(box Expr::Null, box Expr::Null).tag());
            assert_eq!(17, Expr::Contains(box Expr::Null, box Expr::Null).tag());
            assert_eq!(18, Expr::Eq(box Expr::Null, box Expr::Null).tag());
            assert_eq!(19, Expr::Ne(box Expr::Null, box Expr::Null).tag());
            assert_eq!(20, Expr::Gt(box Expr::Null, box Expr::Null).tag());
            assert_eq!(21, Expr::Ge(box Expr::Null, box Expr::Null).tag());
            assert_eq!(22, Expr::Lt(box Expr::Null, box Expr::Null).tag());
            assert_eq!(23, Expr::Le(box Expr::Null, box Expr::Null).tag());
            assert_eq!(24, Expr::Root.tag());
            assert_eq!(25, Expr::Current.tag());
            assert_eq!(26, Expr::Parent.tag());
            assert_eq!(27, Expr::All.tag());
            assert_eq!(28, Expr::Ancestors(box LevelRange::default()).tag());
            assert_eq!(29, Expr::Descendants(box LevelRange::default()).tag());
            assert_eq!(30, Expr::Property(box Expr::Null).tag());
            assert_eq!(31, Expr::Index(box Expr::Null).tag());
            assert_eq!(32, Expr::Range(box NumberRange::default()).tag());
            assert_eq!(33, Expr::Group(Vec::new()).tag());
            assert_eq!(34, Expr::Sequence(Vec::new()).tag());
            assert_eq!(
                35,
                Expr::MethodCall(box MethodCall::new(MethodId::ToString, Vec::new())).tag()
            );
            assert_eq!(
                36,
                Expr::FuncCall(box FuncCall::new(FuncId::Sqrt, Vec::new())).tag()
            );
            assert_eq!(37, Expr::Var(box Expr::String("var1".to_string())).tag());
        }

        #[test]
        fn eq_hash_for_float_nan() {
            fn hash<H: Hash>(e: H) -> u64 {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                e.hash(&mut hasher);
                hasher.finish()
            }

            let e1 = Expr::Float(0.0 / 0.0);
            let e2 = Expr::Float(0.0 / 0.0);

            assert_eq!(e1, e2);
            assert_eq!(hash(e1), hash(e2));
        }

        #[test]
        fn metadata_file_path_components() {
            use std::path::Path;

            set_base_path(Path::new("/tmp"));

            let n = NodeRef::null();
            n.data_mut().set_file(Some(&FileInfo::new(
                Path::new("/tmp/some/path/file.json"),
                FileType::File,
                FileFormat::Json,
            )));
            let o = Opath::parse("@file_path_components").unwrap();
            let r = o.apply(&n, &n).unwrap();

            assert_eq!(r.len(), 1);
            assert_eq!(r.into_vec()[0].to_json(), r#"["some","path","file.json"]"#);
        }

        #[test]
        fn var_with_many_filtering() {
            let data = r#"{
                "hosts": {
                    "abc": {
                        "hostname": "abc",
                        "fqdn": "abc.kodegenix.pl"
                    },
                    "heffe": {
                        "hostname": "heffe",
                        "fqdn": "heffe.kodegenix.pl"
                    },
                    "zeus": {
                        "hostname": "zeus",
                        "fqdn": "zeus.kodegenix.pl"
                    }
                }
            }"#;
            let n = NodeRef::from_json(data).unwrap();
            let hosts_expr = Opath::parse("$.hosts.*").unwrap();
            let hosts = hosts_expr.apply(&n, &n).unwrap();
            let scope = ScopeMut::new();
            scope.set_var("hosts".into(), hosts);

            let e = Opath::parse("$hosts[@.hostname!='zeus']").unwrap();
            let res = e.apply_ext(&n, &n, scope.as_ref()).unwrap();

            assert!(res.is_many());
            assert_eq!(res.len(), 2);
        }
    }

    mod node_set {
        use super::*;

        #[test]
        fn can_serialize_empty() {
            let n = NodeSet::Empty;
            let s = serde_json::to_string(&n).unwrap();
            assert_eq!(s, r#"{"type":"empty"}"#);
        }

        #[test]
        fn can_deserialize_empty() {
            let n: NodeSet = serde_json::from_str(r#"{"type":"empty"}"#).unwrap();
            assert_eq!(n, NodeSet::Empty);
        }

        #[test]
        fn can_serialize_one() {
            let n = NodeSet::One(NodeRef::integer(123));
            let s = serde_json::to_string(&n).unwrap();
            assert_eq!(s, r#"{"type":"one","data":123}"#);
        }

        #[test]
        fn can_deserialize_one() {
            let n: NodeSet = serde_json::from_str(r#"{"type":"one","data":123}"#).unwrap();
            assert_eq!(n, NodeSet::One(NodeRef::integer(123)));
        }

        #[test]
        fn can_serialize_many() {
            let n = NodeSet::Many(vec![NodeRef::string("test"), NodeRef::integer(123)]);
            let s = serde_json::to_string(&n).unwrap();
            assert_eq!(s, r#"{"type":"many","data":["test",123]}"#);
        }

        #[test]
        fn can_deserialize_many() {
            let n: NodeSet =
                serde_json::from_str(r#"{"type":"many","data":["test",123]}"#).unwrap();
            assert_eq!(
                n,
                NodeSet::Many(vec![NodeRef::string("test"), NodeRef::integer(123)])
            );
        }
    }
}
