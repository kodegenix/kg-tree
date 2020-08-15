use super::*;
use crate::opath::expr::func::FuncCallErrorDetail::{NonBinaryNode, RegexParse};
pub type FuncCallError = BasicDiag;

pub type FuncCallResult = Result<(), FuncCallError>;

#[derive(Debug, Display, Detail, PartialEq)]
#[diag(code_offset = 100)]
#[allow(dead_code)]
pub enum FuncCallErrorDetail {
    #[display(fmt = "unknown function '{name}'")]
    UnknownFunc { name: String },

    #[display(fmt = "unknown method '{name}' for type '{kind}'")]
    UnknownMethod { name: String, kind: Kind },

    #[display(
        fmt = "method '{id}' for type '{kind}' requires {required} parameters, but {supplied} were supplied"
    )]
    MethodCallInvalidArgCount {
        id: MethodId,
        kind: Kind,
        supplied: u32,
        required: u32,
    },

    #[display(
        fmt = "method '{id}' for type '{kind}' requires at least {required_min} parameters, but {supplied} were supplied"
    )]
    MethodCallInvalidArgCountMin {
        id: MethodId,
        kind: Kind,
        supplied: u32,
        required_min: u32,
    },

    #[display(
        fmt = "method '{id}' for type '{kind}' requires from {required_min} to {required_max} parameters, but {supplied} were supplied"
    )]
    MethodCallInvalidArgCountRange {
        id: MethodId,
        kind: Kind,
        supplied: u32,
        required_min: u32,
        required_max: u32,
    },

    #[display(
        fmt = "function '{id}' requires {required} parameters, but {supplied} were supplied"
    )]
    FuncCallInvalidArgCount {
        id: FuncId,
        supplied: u32,
        required: u32,
    },

    #[display(
        fmt = "function '{id}' requires at least {required_min} parameters, but {supplied} were supplied"
    )]
    FuncCallInvalidArgCountMin {
        id: FuncId,
        supplied: u32,
        required_min: u32,
    },

    #[display(
        fmt = "function '{id}' requires from {required_min} to {required_max} parameters, but {supplied} were supplied"
    )]
    FuncCallInvalidArgCountRange {
        id: FuncId,
        supplied: u32,
        required_min: u32,
        required_max: u32,
    },

    #[display(fmt = "cannot parse node from type {kind}")]
    NonBinaryNode { kind: Kind },

    #[display(fmt = "cannot parse regex: {err}")]
    RegexParse { err: regex::Error },

    #[display(fmt = "cannot parse expression")]
    ParseErr,

    #[display(fmt = "cannot parse expression")]
    NodeParse,

    #[display(fmt = "error while calling method '{id}' for type '{kind}'")]
    MethodCallCustom {
        id: MethodId,
        kind: Kind,
    },
    #[display(fmt = "error while calling function '{id}'")]
    FuncCallCustom { id: FuncId },
}

impl FuncCallErrorDetail {
    pub fn custom_func(id: &FuncId, err: BasicDiag) -> FuncCallError {
        FuncCallErrorDetail::FuncCallCustom {
            id: id.clone(),
        }.with_cause(err)
    }
    pub fn custom_method(id: &MethodId, kind: Kind, err: BasicDiag) -> FuncCallError {
        FuncCallErrorDetail::MethodCallCustom {
            id: id.clone(),
            kind,
        }.with_cause(err)
    }

    pub fn parse_err(err: ParseDiag) -> FuncCallError {
        FuncCallErrorDetail::ParseErr.with_cause(err)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FuncId {
    Array,
    Map,
    ReadFile,
    Parse,
    ParseInt,
    ParseFloat,
    ParseBinary,
    IsNaN,
    Sqrt,
    Json,
    Stringify,
    FindNew,
    FindOld,
    Custom(String),
}

impl FuncId {
    pub fn from(f: &str) -> FuncId {
        match f {
            "array" => FuncId::Array,
            "map" => FuncId::Map,
            "readFile" => FuncId::ReadFile,
            "parse" => FuncId::Parse,
            "parseInt" => FuncId::ParseInt,
            "parseFloat" => FuncId::ParseFloat,
            "parseBinary" => FuncId::ParseBinary,
            "isNaN" => FuncId::IsNaN,
            "sqrt" => FuncId::Sqrt,
            "json" => FuncId::Json,
            "stringify" => FuncId::Stringify,
            "findNew" => FuncId::FindNew,
            "findOld" => FuncId::FindOld,
            _ => FuncId::Custom(f.to_string()),
        }
    }

    pub fn name(&self) -> &str {
        match *self {
            FuncId::Array => "array",
            FuncId::Map => "map",
            FuncId::ReadFile => "readFile",
            FuncId::Parse => "parse",
            FuncId::ParseInt => "parseInt",
            FuncId::ParseFloat => "parseFloat",
            FuncId::ParseBinary => "parseBinary",
            FuncId::IsNaN => "isNaN",
            FuncId::Sqrt => "sqrt",
            FuncId::Json => "json",
            FuncId::Stringify => "stringify",
            FuncId::FindNew => "findNew",
            FuncId::FindOld => "findOld",
            FuncId::Custom(ref s) => s,
        }
    }
}

impl std::fmt::Display for FuncId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MethodId {
    Length,
    ToString,
    Find,
    Set,
    Delete,
    Extend,
    Push,
    Pop,
    Shift,
    Unshift,
    Join,
    Replace,
    Split,
    Custom(String),
}

impl MethodId {
    pub fn from(f: &str) -> MethodId {
        match f {
            "length" => MethodId::Length,
            "toString" => MethodId::ToString,
            "find" => MethodId::Find,
            "set" => MethodId::Set,
            "delete" => MethodId::Delete,
            "extend" => MethodId::Extend,
            "push" => MethodId::Push,
            "pop" => MethodId::Pop,
            "shift" => MethodId::Shift,
            "unshift" => MethodId::Unshift,
            "join" => MethodId::Join,
            "replace" => MethodId::Replace,
            "split" => MethodId::Split,
            _ => MethodId::Custom(f.to_string()),
        }
    }

    pub fn name(&self) -> &str {
        match *self {
            MethodId::Length => "length",
            MethodId::ToString => "toString",
            MethodId::Find => "find",
            MethodId::Set => "set",
            MethodId::Delete => "delete",
            MethodId::Extend => "extend",
            MethodId::Push => "push",
            MethodId::Pop => "pop",
            MethodId::Shift => "shift",
            MethodId::Unshift => "unshift",
            MethodId::Join => "join",
            MethodId::Replace => "replace",
            MethodId::Split => "split",
            MethodId::Custom(ref s) => s,
        }
    }
}

impl std::fmt::Display for MethodId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug)]
pub struct Args<'a> {
    args: &'a Vec<Expr>,
}

impl<'a> Args<'a> {
    pub(super) fn new(args: &'a Vec<Expr>) -> Args<'a> {
        Args { args }
    }

    pub(super) fn as_vec(&self) -> &Vec<Expr> {
        self.args
    }

    pub fn count(&self) -> usize {
        self.args.len()
    }

    pub fn check_count_func(&self, id: &FuncId, min: u32, max: u32) -> FuncCallResult {
        let count = self.count() as u32;
        if min == max {
            if count != min {
                return Err(basic_diag!(FuncCallErrorDetail::FuncCallInvalidArgCount {
                    id: id.clone(),
                    required: min,
                    supplied: count
                }));
            }
        } else if min < max {
            if count < min || count > max {
                return Err(basic_diag!(
                    FuncCallErrorDetail::FuncCallInvalidArgCountRange {
                        id: id.clone(),
                        required_min: min,
                        required_max: max,
                        supplied: count
                    }
                ));
            }
        } else {
            if count < min {
                return Err(basic_diag!(
                    FuncCallErrorDetail::FuncCallInvalidArgCountMin {
                        id: id.clone(),
                        required_min: min,
                        supplied: count
                    }
                ));
            }
        }
        Ok(())
    }

    pub fn check_count_method(
        &self,
        id: &MethodId,
        kind: Kind,
        min: u32,
        max: u32,
    ) -> FuncCallResult {
        let count = self.count() as u32;
        if min == max {
            if count != min {
                return Err(basic_diag!(
                    FuncCallErrorDetail::MethodCallInvalidArgCount {
                        id: id.clone(),
                        kind,
                        required: min,
                        supplied: count
                    }
                ));
            }
        } else if min < max {
            if count < min || count > max {
                return Err(basic_diag!(
                    FuncCallErrorDetail::MethodCallInvalidArgCountRange {
                        id: id.clone(),
                        kind,
                        required_min: min,
                        required_max: max,
                        supplied: count
                    }
                ));
            }
        } else {
            if count < min {
                return Err(basic_diag!(
                    FuncCallErrorDetail::MethodCallInvalidArgCountMin {
                        id: id.clone(),
                        kind,
                        required_min: min,
                        supplied: count
                    }
                ));
            }
        }
        Ok(())
    }

    pub fn resolve(&self, consumable: bool, env: Env) -> ExprResult<Vec<NodeSet>> {
        let mut values = Vec::new();
        for arg in self.args.iter() {
            let mut out = NodeBuf::new();
            arg.apply_to(env, Context::Expr, &mut out)?;
            out.make_consumable(consumable);
            values.push(out.into_node_set());
        }
        Ok(values)
    }

    pub fn resolve_flat(&self, consumable: bool, env: Env) -> ExprResult<NodeSet> {
        let mut values = NodeBuf::new();
        for arg in self.args.iter() {
            arg.apply_to(env, Context::Expr, &mut values)?;
        }
        values.make_consumable(consumable);
        Ok(values.into_node_set())
    }

    pub fn resolve_column(&self, consumable: bool, column: usize, env: Env) -> ExprResult<NodeSet> {
        let mut values = NodeBuf::new();
        self.args[column].apply_to(env, Context::Expr, &mut values)?;
        values.make_consumable(consumable);
        Ok(values.into_node_set())
    }

    pub fn resolve_rows(
        &self,
        consumable: bool,
        max_cols: Option<usize>,
        default: NodeRef,
        env: Env,
    ) -> ExprResult<Vec<Vec<NodeRef>>> {
        let cols = if let Some(max) = max_cols {
            std::cmp::min(max, self.args.len())
        } else {
            self.args.len()
        };

        let mut values = Vec::with_capacity(cols);
        let mut min_len = std::usize::MAX;
        let mut empty = true;
        for arg in self.args.iter() {
            let mut vals = NodeBuf::new();
            arg.apply_to(env, Context::Expr, &mut vals)?;
            if vals.elems.len() > 0 {
                empty = false;
            }
            if vals.elems.len() > 1 && vals.elems.len() < min_len {
                min_len = vals.elems.len();
            }
            values.push(vals);
            if let Some(max) = max_cols {
                if values.len() >= max {
                    break;
                }
            }
        }
        if empty {
            Ok(Vec::new())
        } else {
            if min_len == std::usize::MAX {
                min_len = 1;
            }

            let mut rows = Vec::with_capacity(min_len);
            for r in 0..min_len {
                let mut columns = Vec::with_capacity(cols);
                for c in 0..cols {
                    let ref v = values[c];
                    let n = match v.elems.len() {
                        0 => &default,
                        1 => &v.elems[0],
                        _ => &v.elems[r],
                    };
                    columns.push(if !consumable || n.is_consumable() {
                        n.clone()
                    } else {
                        n.deep_copy()
                    });
                }
                rows.push(columns);
            }
            Ok(rows)
        }
    }

    pub fn resolve_rows_null(
        &self,
        consumable: bool,
        max_cols: Option<usize>,
        env: Env,
    ) -> ExprResult<Vec<Vec<NodeRef>>> {
        self.resolve_rows(consumable, max_cols, NodeRef::null(), env)
    }
}

pub trait FuncCallable: std::fmt::Debug + Sync + Send {
    fn call(&self, name: &str, args: Args, env: Env, out: &mut NodeBuf) -> FuncCallResult;

    fn clone(&self) -> Box<dyn FuncCallable>;
}

impl Clone for Box<dyn FuncCallable> {
    fn clone(&self) -> Self {
        self.as_ref().clone()
    }
}

#[derive(Clone, Copy)]
pub struct Func {
    fn_ptr: fn(&str, Args, Env, &mut NodeBuf) -> FuncCallResult,
}

impl Func {
    pub fn new(f: fn(&str, Args, Env, &mut NodeBuf) -> FuncCallResult) -> Func {
        Func { fn_ptr: f }
    }
}

impl FuncCallable for Func {
    fn call(&self, name: &str, args: Args, env: Env, out: &mut NodeBuf) -> FuncCallResult {
        (self.fn_ptr)(name, args, env, out)
    }

    fn clone(&self) -> Box<dyn FuncCallable> {
        Box::new(Clone::clone(self))
    }
}

impl std::fmt::Debug for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ptr = self.fn_ptr as *mut std::os::raw::c_void;
        let mut resolved = false;
        backtrace::resolve(ptr, |s| {
            if let (Some(addr), Some(name), Some(filename), Some(lineno)) =
                (s.addr(), s.name(), s.filename(), s.lineno())
            {
                write!(
                    f,
                    "Func({:p} - {} at {}:{})",
                    addr,
                    name,
                    filename.display(),
                    lineno
                )
                .unwrap();
            }
            resolved = true;
        });
        debug_assert!(resolved);
        Ok(())
    }
}

pub trait MethodCallable: std::fmt::Debug + Sync + Send {
    fn call(&self, name: &str, args: Args, env: Env, out: &mut NodeBuf) -> FuncCallResult;

    fn mask(&self) -> KindMask;

    fn clone(&self) -> Box<dyn MethodCallable>;
}

impl Clone for Box<dyn MethodCallable> {
    fn clone(&self) -> Self {
        self.as_ref().clone()
    }
}

#[derive(Clone, Copy)]
pub struct Method {
    fn_ptr: fn(&str, Args, Env, &mut NodeBuf) -> FuncCallResult,
    mask: KindMask,
}

impl Method {
    pub fn new(mask: KindMask, f: fn(&str, Args, Env, &mut NodeBuf) -> FuncCallResult) -> Method {
        Method { fn_ptr: f, mask }
    }
}

impl MethodCallable for Method {
    fn call(&self, name: &str, args: Args, env: Env, out: &mut NodeBuf) -> FuncCallResult {
        (self.fn_ptr)(name, args, env, out)
    }

    fn mask(&self) -> KindMask {
        self.mask
    }

    fn clone(&self) -> Box<dyn MethodCallable> {
        Box::new(Clone::clone(self))
    }
}

impl std::fmt::Debug for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ptr = self.fn_ptr as *mut std::os::raw::c_void;
        let mut resolved = false;
        backtrace::resolve(ptr, |s| {
            if let (Some(addr), Some(name), Some(filename), Some(lineno)) =
                (s.addr(), s.name(), s.filename(), s.lineno())
            {
                write!(
                    f,
                    "Method({:p} - {} at {}:{})",
                    addr,
                    name,
                    filename.display(),
                    lineno
                )
                .unwrap();
            }
            resolved = true;
        });
        debug_assert!(resolved);
        Ok(())
    }
}

pub(super) fn apply_func_to(
    id: &FuncId,
    args: Args,
    env: Env,
    _ctx: Context,
    out: &mut NodeBuf,
) -> FuncCallResult {
    match *id {
        FuncId::Array => {
            let values = args.resolve_flat(true, env)?;
            out.add(NodeRef::array(values.into_iter().collect()));
            Ok(())
        }
        FuncId::Map => {
            if args.count() == 0 {
                out.add(NodeRef::object(Properties::new()));
            } else if args.count() == 1 {
                let values = args.resolve_column(false, 0, env)?;
                let mut map = Properties::with_capacity(values.len());
                for value in values.into_iter() {
                    if let Value::Object(ref props) = value.data().value() {
                        for (k, v) in props.iter() {
                            map.insert(k.clone(), v.clone());
                        }
                    }
                }
                out.add(NodeRef::object(map));
            } else {
                args.check_count_func(id, 2, 2)?;
                let keys = args.resolve_column(false, 0, env)?;
                let values = args.resolve_column(true, 1, env)?;
                let mut map = Properties::with_capacity(std::cmp::min(keys.len(), values.len()));
                for (k, v) in keys.into_iter().zip(values.into_iter()) {
                    map.insert(k.as_string().to_string().into(), v);
                }
                out.add(NodeRef::object(map));
            }
            Ok(())
        }
        FuncId::ReadFile => {
            args.check_count_func(id, 1, 2)?;

            if args.count() == 1 {
                let paths = args.resolve_column(false, 0, env)?;
                for p in paths.into_iter() {
                    let n =
                        NodeRef::from_file(&resolve_path_str(p.data().as_string().as_ref()), None)
                            .map_err(|err| FuncCallErrorDetail::custom_func(id, err))?;
                    out.add(n);
                }
            } else {
                let paths = args.resolve_column(false, 0, env)?;
                let formats = args.resolve_column(false, 1, env)?;

                for (p, f) in paths.into_iter().zip(formats.into_iter()) {
                    let format: FileFormat = f.data().as_string().as_ref().into();

                    let n = NodeRef::from_file(&resolve_path_str(p.data().as_string().as_ref()), Some(format))
                        .map_err(|err| FuncCallErrorDetail::custom_func(id, err))?;
                    out.add(n);
                }
            }
            Ok(())
        }
        FuncId::Json => {
            args.check_count_func(id, 1, 1)?;
            let res = args.resolve_flat(false, env)?;
            for n in res.into_iter() {
                let n = n.data();
                let s = n.as_string();

                let n = NodeRef::from_json(&s)
                    .map_err_as_cause(|| FuncCallErrorDetail::NodeParse)?;
                out.add(n);
            }
            Ok(())
        }
        FuncId::Parse => {
            args.check_count_func(id, 2, 2)?;
            let rows = args.resolve_rows_null(false, None, env)?;

            for r in rows {
                let ref content = r[0];
                let format: FileFormat = r[1].data().as_string().as_ref().into();

                let n = NodeRef::from_str(content.data().as_string(), format)
                    .map_err(|err| FuncCallErrorDetail::custom_func(id, err))?;
                out.add(n);
            }
            Ok(())
        }
        FuncId::Stringify => {
            args.check_count_func(id, 1, 3)?;
            match args.count() {
                1 => {
                    let row = args.resolve_flat(false, env)?;
                    let format = FileFormat::Json;
                    for n in row {
                        out.add(NodeRef::string(n.to_format(format, false)));
                    }
                }
                2 => {
                    let rows = args.resolve_rows_null(false, None, env)?;
                    for r in rows {
                        let ref n = r[0];
                        let format = {
                            let ref f = r[1];
                            if f.is_string() {
                                f.data().as_string().as_ref().into()
                            } else {
                                FileFormat::Json
                            }
                        };
                        out.add(NodeRef::string(n.to_format(format, false)));
                    }
                }
                3 => {
                    let rows = args.resolve_rows_null(false, None, env)?;
                    for r in rows {
                        let ref n = r[0];
                        let format = {
                            let ref f = r[1];
                            if f.is_string() {
                                f.data().as_string().as_ref().into()
                            } else {
                                FileFormat::Json
                            }
                        };
                        let pretty = r[2].as_boolean();
                        out.add(NodeRef::string(n.to_format(format, pretty)));
                    }
                }
                _ => unreachable!(),
            }
            Ok(())
        }
        FuncId::ParseInt => {
            args.check_count_func(id, 1, 2)?;

            let strs = args.resolve_column(false, 0, env)?;

            let mut radixes_1;
            let mut radixes_2;

            let radixes: &mut dyn Iterator<Item = u32> = if args.count() == 2 {
                radixes_1 = args
                    .resolve_column(false, 1, env)?
                    .into_iter()
                    .map(|r| r.as_integer().map_or(10, |base| base as u32))
                    .chain(std::iter::repeat(10u32));
                &mut radixes_1 as &mut dyn Iterator<Item = u32>
            } else {
                radixes_2 = std::iter::repeat(10u32);
                &mut radixes_2 as &mut dyn Iterator<Item = u32>
            };

            for (s, r) in strs.into_iter().zip(radixes.into_iter()) {
                let s = s.data();
                let s = s.as_string();
                let s = s.trim();
                let beg = if s.as_bytes()[0] == b'-' { 1 } else { 0 };
                let end = match s[beg..].find(|c: char| !c.is_digit(r)) {
                    Some(pos) => pos + beg,
                    None => s.len(),
                };
                match i64::from_str_radix(&s[0..end], r) {
                    Ok(num) => out.add(NodeRef::integer(num)),
                    Err(_) => out.add(NodeRef::float(std::f64::NAN)),
                }
            }
            Ok(())
        }
        FuncId::ParseFloat => {
            args.check_count_func(id, 1, 1)?;

            let strs = args.resolve_column(false, 0, env)?;

            for s in strs.into_iter() {
                let s = s.data();
                let s = s.as_string();
                let s = s.trim();

                match f64::from_str(s) {
                    Ok(num) => out.add(NodeRef::float(num)),
                    Err(_) => out.add(NodeRef::float(std::f64::NAN)),
                }
            }
            Ok(())
        }
        FuncId::ParseBinary => {
            args.check_count_func(id, 2, 2)?;

            let contents = args.resolve_column(false, 0, env)?;
            let formats = args.resolve_column(false, 1, env)?;

            for (c, f) in contents.into_iter().zip(formats.into_iter()) {
                let f: NodeRef = f;
                let c: NodeRef = c;
                let format: FileFormat = f.data().as_string().as_ref().into();

                let res = c.as_binary();

                if res.is_none() {
                    return Err(NonBinaryNode {
                        kind: c.data().kind(),
                    }
                    .into());
                }
                let bytes = res.unwrap();

                let n = NodeRef::from_bytes(bytes.as_slice(), format)
                    .map_err(|err| FuncCallErrorDetail::custom_func(id, err))?;
                out.add(n)

            }
            Ok(())
        }
        FuncId::IsNaN => {
            args.check_count_func(id, 1, 1)?;

            let nums = args.resolve_column(false, 0, env)?;

            for n in nums.into_iter() {
                let f = n.as_float();
                out.add(NodeRef::boolean(f64::is_nan(f)));
            }
            Ok(())
        }
        FuncId::Sqrt => {
            args.check_count_func(id, 1, 1)?;
            let res = args.resolve_flat(false, env)?;
            for n in res.into_iter() {
                out.add(NodeRef::float(n.as_float().sqrt()));
            }
            Ok(())
        }
        FuncId::FindNew => {
            if let Some(diff_env) = env.diff {
                args.check_count_func(id, 1, 1)?;
                let res = args.resolve_flat(false, env)?;
                for n in res.into_iter() {
                    out.add(diff_env.diff().find_new(&n, diff_env.old_root, env.root).unwrap_or_else(|| NodeRef::null()));
                }
                Ok(())
            } else {
                Err(basic_diag!(FuncCallErrorDetail::UnknownFunc {
                    name: id.to_string()
                }))
            }
        }
        FuncId::FindOld => {
            if let Some(diff_env) = env.diff {
                args.check_count_func(id, 1, 1)?;
                let res = args.resolve_flat(false, env)?;
                for n in res.into_iter() {
                    out.add(diff_env.diff().find_old(&n, diff_env.old_root, env.root).unwrap_or_else(|| NodeRef::null()));
                }
                Ok(())
            } else {
                Err(basic_diag!(FuncCallErrorDetail::UnknownFunc {
                    name: id.to_string()
                }))
            }
        }
        FuncId::Custom(ref name) => {
            if let Some(e) = env.scope() {
                if let Some(func) = e.get_func(name) {
                    return func.call(name, args, env, out);
                }
            }
            Err(basic_diag!(FuncCallErrorDetail::UnknownFunc {
                name: name.to_string()
            }))
        }
    }
}

pub(super) fn apply_method_to(
    id: &MethodId,
    args: Args,
    env: Env,
    _ctx: Context,
    out: &mut NodeBuf,
) -> FuncCallResult {
    #[inline]
    fn array_remove(
        index: Option<usize>,
        id: &MethodId,
        kind: Kind,
        args: Args,
        env: Env,
        out: &mut NodeBuf,
    ) -> FuncCallResult {
        if env.current().is_array() {
            args.check_count_method(id, kind, 0, 0)?;

            if let Some(removed) = env.current().remove_child(index, None).unwrap() {
                out.add(removed)
            } else {
                out.add(NodeRef::null())
            }

            Ok(())
        } else {
            Err(basic_diag!(FuncCallErrorDetail::UnknownMethod {
                name: id.name().to_string(),
                kind,
            }))
        }
    }

    #[inline]
    fn array_add(
        index: Option<usize>,
        id: &MethodId,
        kind: Kind,
        args: Args,
        env: Env,
        out: &mut NodeBuf,
    ) -> FuncCallResult {
        if env.current().is_array() {
            args.check_count_method(id, kind, 1, 1)?;
            let elems = args.resolve_column(true, 0, env)?;
            for elem in elems.into_iter() {
                env.current().add_child(index, None, elem).unwrap();
            }

            out.add(NodeRef::integer(
                env.current().data().children_count().unwrap() as i64,
            ));

            Ok(())
        } else {
            Err(basic_diag!(FuncCallErrorDetail::UnknownMethod {
                name: id.name().to_string(),
                kind,
            }))
        }
    }

    let kind = env.current().data().kind();

    match *id {
        MethodId::ToString => {
            out.add(NodeRef::string(env.current().as_string()));
            Ok(())
        }
        MethodId::Length => match env.current().data().value() {
            Value::Binary(ref e) => {
                out.add(NodeRef::integer(e.len() as i64));
                Ok(())
            }
            Value::String(ref s) => {
                out.add(NodeRef::integer(s.len() as i64));
                Ok(())
            }
            Value::Array(ref e) => {
                out.add(NodeRef::integer(e.len() as i64));
                Ok(())
            }
            Value::Object(ref p) => {
                out.add(NodeRef::integer(p.len() as i64));
                Ok(())
            }
            _ => Err(basic_diag!(FuncCallErrorDetail::UnknownMethod {
                name: id.name().to_string(),
                kind,
            })),
        },
        MethodId::Join => {
            fn wrap(node: &NodeRef, wrap_open: &str, wrap_close: &str, buf: &mut String) {
                if !wrap_open.is_empty() {
                    buf.push_str(wrap_open);
                }
                buf.push_str(node.data().as_string().as_ref());
                if !wrap_close.is_empty() {
                    buf.push_str(wrap_close);
                }
            }

            fn join(elems: &Vec<NodeRef>, sep: &str, wrap_open: &str, wrap_close: &str) -> String {
                let mut s = String::new();
                match sep.as_ref() {
                    "" => {
                        for e in elems.iter() {
                            wrap(e, wrap_open, wrap_close, &mut s);
                        }
                    }
                    _ => {
                        let mut elemi = elems.iter().peekable();
                        while let Some(e) = elemi.next() {
                            wrap(e, wrap_open, wrap_close, &mut s);
                            if elemi.peek().is_some() {
                                s.push_str(sep.as_ref());
                            }
                        }
                    }
                }
                s
            }

            if let Value::Array(ref elems) = *env.current().data().value() {
                args.check_count_method(id, env.current().data().kind(), 1, 2)?;
                let sep = {
                    let nsep = args.resolve_column(false, 0, env)?;
                    match nsep.into_one() {
                        Some(sep) => sep.data().as_string().to_string(),
                        None => String::new(),
                    }
                };
                let wrap = {
                    if args.count() == 2 {
                        let nwrap = args.resolve_column(false, 1, env)?;
                        match nwrap.into_one() {
                            Some(wrap) => wrap.data().as_string().to_string(),
                            None => String::new(),
                        }
                    } else {
                        String::new()
                    }
                };

                let s = join(elems, &sep, &wrap, &wrap);
                out.add(NodeRef::string(s));
                Ok(())
            } else {
                Err(basic_diag!(FuncCallErrorDetail::UnknownMethod {
                    name: id.name().to_string(),
                    kind: env.current().data().kind()
                }))
            }
        }
        MethodId::Push => array_add(None, id, kind, args, env, out),
        MethodId::Pop => array_remove(None, id, kind, args, env, out),
        MethodId::Unshift => array_add(Some(0), id, kind, args, env, out),
        MethodId::Find => {
            args.check_count_method(id, kind, 1, 1)?;
            let nres = args.resolve_column(false, 0, env)?;
            for n in nres.into_iter() {
                let d = n.data();
                let s = d.as_string();
                if s.is_empty() {
                    out.add(env.current().clone());
                } else if s.starts_with("@.") {
                    let opath = Opath::parse(&s).map_err(|d| FuncCallErrorDetail::parse_err(d))?;
                    opath.expr().apply_to(env, Context::Expr, out)?;
                } else {
                    let s = String::with_capacity(256) + "@." + &s;
                    let opath = Opath::parse(&s).map_err(|d| FuncCallErrorDetail::parse_err(d))?;
                    opath.expr().apply_to(env, Context::Expr, out)?;
                }
            }
            Ok(())
        }
        MethodId::Shift => array_remove(Some(0), id, kind, args, env, out),
        MethodId::Set => {
            if env.current().is_object() {
                args.check_count_method(id, kind, 2, 2)?;

                let keys = args.resolve_column(false, 0, env)?;
                let keys: Vec<_> = keys
                    .into_iter()
                    .map(|k| {
                        if k.is_consumable() {
                            k.into_string()
                        } else {
                            k.as_string()
                        }
                    })
                    .collect();

                let values = args.resolve_column(true, 1, env)?;

                env.current()
                    .add_children(
                        true,
                        keys.into_iter()
                            .zip(values.into_iter())
                            .map(|(k, v)| (None, Some(k.into()), v)),
                    )
                    .unwrap();

                Ok(())
            } else {
                Err(basic_diag!(FuncCallErrorDetail::UnknownMethod {
                    name: id.name().to_string(),
                    kind,
                }))
            }
        }
        MethodId::Delete => {
            if env.current().is_object() {
                args.check_count_method(id, kind, 1, 1)?;

                let keys = args.resolve_column(false, 0, env)?;
                let keys: Vec<_> = keys
                    .into_iter()
                    .map(|k| {
                        if k.is_consumable() {
                            k.into_string()
                        } else {
                            k.as_string()
                        }
                    })
                    .collect();

                env.current()
                    .remove_children(true, keys.into_iter().map(|k| (None, Some(k.into()))))
                    .unwrap();

                Ok(())
            } else {
                Err(basic_diag!(FuncCallErrorDetail::UnknownMethod {
                    name: id.name().to_string(),
                    kind,
                }))
            }
        }
        MethodId::Extend => {
            fn calc_index(i: Option<i64>, len: usize) -> Option<usize> {
                match i {
                    Some(i) if i < 0 => Some(std::cmp::max(len as i64 + i, 0) as usize),
                    Some(i) => Some(i as usize),
                    None => None,
                }
            }

            if env.current().is_parent() {
                args.check_count_method(id, kind, 1, 2)?;

                if args.count() == 1 {
                    let values = args.resolve_column(true, 0, env)?;
                    env.current()
                        .extend_multiple(values.into_iter().map(|n| (n, None)))
                        .unwrap();
                } else {
                    let values = args.resolve_column(true, 0, env)?;
                    let indices = args.resolve_column(false, 1, env)?;
                    let len = env.current().data().children_count().unwrap();
                    env.current()
                        .extend_multiple(
                            values
                                .into_iter()
                                .zip(indices.into_iter())
                                .map(|(n, i)| (n, calc_index(i.as_integer(), len))),
                        )
                        .unwrap();
                }
                Ok(())
            } else {
                Err(basic_diag!(FuncCallErrorDetail::UnknownMethod {
                    name: id.name().to_string(),
                    kind,
                }))
            }
        }
        MethodId::Custom(ref name) => {
            if let Some(e) = env.scope() {
                if let Some(method) = e.get_method(name) {
                    if method.mask().has(kind) {
                        return method.call(name, args, env, out);
                    }
                }
            }
            Err(basic_diag!(FuncCallErrorDetail::UnknownMethod {
                name: name.to_string(),
                kind,
            }))
        }
        MethodId::Replace => {
            use regex::Regex;

            if kind == Kind::String {
                args.check_count_method(id, kind, 1, 2)?;

                let re = args.resolve_column(true, 0, env)?.into_one().unwrap();
                let regex =
                    Regex::new(&re.data().as_string()).map_err(|err| RegexParse { err })?;
                let replacement = {
                    if args.count() == 2 {
                        args.resolve_column(true, 1, env)?
                            .into_one()
                            .unwrap()
                            .data()
                            .as_string()
                            .to_string()
                    } else {
                        String::new()
                    }
                };

                let value = env.current().data();
                let s = value.as_string();
                let result = regex.replace_all(&s, replacement.as_str());
                out.add(NodeRef::string(result));
                Ok(())
            } else {
                Err(basic_diag!(FuncCallErrorDetail::UnknownMethod {
                    name: id.name().to_string(),
                    kind,
                }))
            }
        }
        MethodId::Split => {
            use regex::Regex;

            if kind == Kind::String {
                args.check_count_method(id, kind, 1, 2)?;
                let re = args.resolve_column(true, 0, env)?.into_one().unwrap();
                let regex = Regex::new(re.data().as_string().as_ref())
                    .map_err(|err| RegexParse { err })?;

                let value = env.current().data();
                let s = value.as_string();
                for s in regex.split(&s) {
                    out.add(NodeRef::string(s));
                }
                Ok(())
            } else {
                Err(basic_diag!(FuncCallErrorDetail::UnknownMethod {
                    name: id.name().to_string(),
                    kind,
                }))
            }
        } //_ => unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_JSON: &str = r#"
        {
            "one": 1,
            "empty_object": {},
            "empty_array": [],
            "array": ["a","b"],
            "null_value": null,
            "nested": {
                "two": 2,
                "three_string": "3",
                "four": 4
            }
        }
    "#;

    fn test_node<'a>() -> NodeRef {
        NodeRef::from_json(TEST_JSON).unwrap()
    }

    mod args {
        use super::*;

        fn expr(e: &str) -> Expr {
            Opath::parse(e).unwrap().into_expr()
        }

        #[test]
        fn resolve() {
            let n = test_node();
            let a = vec![expr("$.**"), expr("'str'"), expr("1..10")];
            let args = Args::new(&a);

            let values = args.resolve(false, Env::new(&n, &n, None)).unwrap();

            assert_eq!(values.len(), 3);
            assert_eq!(values[0].len(), 11);
            assert_eq!(values[1].len(), 1);
            assert_eq!(values[2].len(), 10);

            let mut consumable = true;
            for vals in values.iter() {
                for v in vals.iter() {
                    consumable = consumable && v.is_consumable();
                }
            }
            assert_eq!(consumable, false);
        }

        #[test]
        fn resolve_consumable() {
            let n = test_node();
            let a = vec![expr("$.**"), expr("'str'"), expr("1..10")];
            let args = Args::new(&a);

            let values = args.resolve(true, Env::new(&n, &n, None)).unwrap();

            assert_eq!(values.len(), 3);
            assert_eq!(values[0].len(), 11);
            assert_eq!(values[1].len(), 1);
            assert_eq!(values[2].len(), 10);
            for vals in values.iter() {
                for v in vals.iter() {
                    assert!(v.is_consumable());
                }
            }
        }

        #[test]
        fn resolve_flat() {
            let n = test_node();
            let a = vec![expr("$.**"), expr("'str'"), expr("1..10")];
            let args = Args::new(&a);

            let values = args.resolve_flat(false, Env::new(&n, &n, None)).unwrap();

            assert_eq!(values.len(), 11 + 1 + 10);
            let mut consumable = true;
            for v in values.iter() {
                consumable = consumable && v.is_consumable();
            }
            assert_eq!(consumable, false);
        }

        #[test]
        fn resolve_flat_consumable() {
            let n = test_node();
            let a = vec![expr("$.**"), expr("'str'"), expr("1..10")];
            let args = Args::new(&a);

            let values = args.resolve_flat(true, Env::new(&n, &n, None)).unwrap();

            assert_eq!(values.len(), 11 + 1 + 10);
            for v in values.iter() {
                assert!(v.is_consumable());
            }
        }

        #[test]
        fn resolve_rows() {
            let n = test_node();
            let a = vec![expr("$.**"), expr("'str'"), expr("1..10")];
            let args = Args::new(&a);

            let values = args
                .resolve_rows_null(false, None, Env::new(&n, &n, None))
                .unwrap();
            assert_eq!(values.len(), 10);
            assert_eq!(values[0].len(), 3);
            assert_eq!(values[1].len(), 3);
            assert_eq!(values[2].len(), 3);
            assert_eq!(values[3].len(), 3);
            assert_eq!(values[4].len(), 3);
            assert_eq!(values[5].len(), 3);
            assert_eq!(values[6].len(), 3);
            assert_eq!(values[7].len(), 3);
            assert_eq!(values[8].len(), 3);
            assert_eq!(values[9].len(), 3);

            let mut consumable = true;
            for vals in values.iter() {
                for v in vals.iter() {
                    consumable = consumable && v.is_consumable();
                }
            }
            assert_eq!(consumable, false);
        }

        #[test]
        fn resolve_rows_consumable() {
            let n = test_node();
            let a = vec![expr("$.**"), expr("'str'"), expr("1..10")];
            let args = Args::new(&a);

            let values = args
                .resolve_rows_null(true, None, Env::new(&n, &n, None))
                .unwrap();

            assert_eq!(values.len(), 10);
            assert_eq!(values[0].len(), 3);
            assert_eq!(values[1].len(), 3);
            assert_eq!(values[2].len(), 3);
            assert_eq!(values[3].len(), 3);
            assert_eq!(values[4].len(), 3);
            assert_eq!(values[5].len(), 3);
            assert_eq!(values[6].len(), 3);
            assert_eq!(values[7].len(), 3);
            assert_eq!(values[8].len(), 3);
            assert_eq!(values[9].len(), 3);

            for vals in values.iter() {
                for v in vals.iter() {
                    assert!(v.is_consumable());
                }
            }
        }
    }

    mod func {
        use super::*;

        #[test]
        fn array() {
            let n = test_node();

            let expr = Opath::parse("array(@.*.@key)").unwrap();

            let res = expr.apply(&n, &n).unwrap();

            assert_eq!(res.len(), 1);

            let ref a = res.into_vec()[0];
            assert!(a.is_array());
            assert_eq!(a.data().children_count(), Some(6));
        }

        #[test]
        fn map() {
            let n = test_node();

            let expr = Opath::parse("map(@.*.@key, @.*.@index)").unwrap();

            let res = expr.apply(&n, &n).unwrap();

            assert_eq!(res.len(), 1);

            let ref o = res.into_vec()[0];
            assert!(o.is_object());
            assert_eq!(o.data().children_count(), Some(6));
        }

        mod find_new {
            use super::*;

            #[test]
            fn should_return_error_without_diff_env() {
                let expr = Opath::parse("findNew($.some)").unwrap();
                let root = NodeRef::object(Properties::new());
                let err = expr.apply(&root, &root).unwrap_err();
                let detail: &FuncCallErrorDetail = err.detail().downcast_ref().unwrap();
                assert_eq!(detail, &FuncCallErrorDetail::UnknownFunc { name: "findNew".into() });
            }

            //FIXME (jc) add further tests
        }

        mod find_old {
            use super::*;

            #[test]
            fn should_return_error_without_diff_env() {
                let expr = Opath::parse("findOld($.some)").unwrap();
                let root = NodeRef::object(Properties::new());
                let err = expr.apply(&root, &root).unwrap_err();
                let detail: &FuncCallErrorDetail = err.detail().downcast_ref().unwrap();
                assert_eq!(detail, &FuncCallErrorDetail::UnknownFunc { name: "findOld".into() });
            }

            //FIXME (jc) add further tests
        }

        mod parse_int {
            use super::*;

            #[test]
            fn multiple_args() {
                let n = NodeRef::from_json(r#"{
                                                        "nums":["10", "-12aaa", "FF", "fE", "101", "1A"],
                                                        "basis":[10, 10, 16, 16, 2, "aaa"]
                                                    }"#)
                    .unwrap();

                let expr = Opath::parse("parseInt(@.nums.*, @.basis.*)").unwrap();
                let mut res = expr.apply(&n, &n).unwrap().into_iter();

                assert_eq!(res.len(), 6);
                assert_eq!(res.next().unwrap().as_integer().unwrap(), 10);
                assert_eq!(res.next().unwrap().as_integer().unwrap(), -12);
                assert_eq!(res.next().unwrap().as_integer().unwrap(), 255);
                assert_eq!(res.next().unwrap().as_integer().unwrap(), 254);
                assert_eq!(res.next().unwrap().as_integer().unwrap(), 5);
                assert_eq!(res.next().unwrap().as_integer().unwrap(), 1);
            }

            #[test]
            fn integer_str() {
                let n = NodeRef::null();
                let expr = Opath::parse("parseInt('10')").unwrap();
                let res = expr.apply(&n, &n).unwrap().into_vec();

                assert_eq!(res.len(), 1);

                let res = res.get(0).unwrap();

                assert!(res.is_integer());
                assert_eq!(res.as_integer().unwrap(), 10)
            }

            #[test]
            fn radix() {
                let n = NodeRef::from_json(r#"{}"#).unwrap();
                let expr = Opath::parse("parseInt('10', 2)").unwrap();
                let res = expr.apply(&n, &n).unwrap().into_vec();

                assert_eq!(res.len(), 1);

                let res = res.get(0).unwrap();

                assert!(res.is_integer());
                assert_eq!(res.as_integer().unwrap(), 2)
            }

            #[test]
            fn radix_str() {
                let n = NodeRef::null();
                let expr = Opath::parse("parseInt('10', '2')").unwrap();
                let res = expr.apply(&n, &n).unwrap().into_vec();

                assert_eq!(res.len(), 1);

                let res = res.get(0).unwrap();

                assert!(res.is_integer());
                assert_eq!(res.as_integer().unwrap(), 2)
            }

            #[test]
            fn float() {
                let n = NodeRef::from_json(r#"{}"#).unwrap();
                let expr = Opath::parse("parseInt('10.33')").unwrap();
                let res = expr.apply(&n, &n).unwrap().into_vec();

                assert_eq!(res.len(), 1);

                let res = res.get(0).unwrap();

                assert!(res.is_integer());
                assert_eq!(res.as_integer().unwrap(), 10)
            }

            #[test]
            fn nan() {
                let n = NodeRef::from_json(r#"{}"#).unwrap();
                let expr = Opath::parse("parseInt('blaa')").unwrap();
                let res = expr.apply(&n, &n).unwrap().into_vec();

                assert_eq!(res.len(), 1);

                let res = res.get(0).unwrap();

                assert!(res.is_float());
                assert!(res.as_float().is_nan());
            }

            #[test]
            fn neg() {
                let n = NodeRef::from_json(r#"{}"#).unwrap();
                let expr = Opath::parse("parseInt('-10')").unwrap();
                let res = expr.apply(&n, &n).unwrap().into_vec();

                assert_eq!(res.len(), 1);

                let res = res.get(0).unwrap();

                assert!(res.is_integer());
                assert_eq!(res.as_integer().unwrap(), -10);
            }

            #[test]
            fn num_first() {
                let n = NodeRef::from_json(r#"{}"#).unwrap();
                let expr = Opath::parse("parseInt('10ab')").unwrap();
                let res = expr.apply(&n, &n).unwrap().into_vec();

                assert_eq!(res.len(), 1);

                let res = res.get(0).unwrap();

                assert!(res.is_integer());
                assert_eq!(res.as_integer().unwrap(), 10);
            }

            #[test]
            fn mul_tokens() {
                let n = NodeRef::from_json(r#"{}"#).unwrap();
                let expr = Opath::parse("parseInt('10 aaa')").unwrap();
                let res = expr.apply(&n, &n).unwrap().into_vec();

                assert_eq!(res.len(), 1);

                let res = res.get(0).unwrap();

                assert!(res.is_integer());
                assert_eq!(res.as_integer().unwrap(), 10)
            }

            #[test]
            fn whitespaces() {
                let n = NodeRef::from_json(r#"{}"#).unwrap();
                let expr = Opath::parse("parseInt('  10  ')").unwrap();
                let res = expr.apply(&n, &n).unwrap().into_vec();

                assert_eq!(res.len(), 1);

                let res = res.get(0).unwrap();

                assert!(res.is_integer());
                assert_eq!(res.as_integer().unwrap(), 10)
            }
        }

        mod custom {
            use super::*;

            #[test]
            fn filter_numbers() {
                fn filter_numbers(
                    _name: &str,
                    args: Args,
                    env: Env<'_>,
                    out: &mut NodeBuf,
                ) -> FuncCallResult {
                    let res = args.resolve_flat(false, env).unwrap();
                    for r in res.into_iter() {
                        if r.is_number() {
                            out.add(r);
                        }
                    }
                    Ok(())
                }

                let scope = ScopeMut::new()
                    .with_func("filter_numbers".into(), box Func::new(filter_numbers));
                let n = test_node();
                let expr = Opath::parse("filter_numbers($.**).@path").unwrap();
                let res = expr.apply_ext(&n, &n, scope.as_ref()).unwrap().into_vec();

                assert_eq!(res.len(), 3);
                assert_eq!(res[0].as_string(), "$.one");
                assert_eq!(res[1].as_string(), "$.nested.two");
                assert_eq!(res[2].as_string(), "$.nested.four");
            }
        }
    }

    mod method {
        use super::*;

        mod find {
            use super::*;

            #[test]
            fn nested_prop() {
                let n = test_node();

                let expr = Opath::parse("@.find('nested.two')").unwrap();

                let res = expr.apply(&n, &n).unwrap().into_vec();

                assert_eq!(res.len(), 1);
                assert_eq!(Opath::from(&res[0]).to_string(), "$.nested.two");
            }

            #[test]
            fn wildcard() {
                let n = test_node();

                let expr = Opath::parse("@.find('nested.*')").unwrap();

                let res = expr.apply(&n, &n).unwrap().into_vec();

                assert_eq!(res.len(), 3);
                assert_eq!(Opath::from(&res[0]).to_string(), "$.nested.two");
                assert_eq!(Opath::from(&res[1]).to_string(), "$.nested.three_string");
                assert_eq!(Opath::from(&res[2]).to_string(), "$.nested.four");
            }
        }

        #[test]
        fn join() {
            let n = test_node();
            let expr = Opath::parse("@.array.join(':')").unwrap();

            let res = expr.apply(&n, &n).unwrap().into_vec();
            assert_eq!(res.len(), 1);

            let ref s = res[0];
            assert!(s.is_string());
            assert_eq!(s.as_string(), "a:b");
        }

        #[test]
        fn set() {
            let n = test_node();
            let expr = Opath::parse("@.set('new_prop', 12)").unwrap();
            let res = expr.apply(&n, &n).unwrap().into_vec();
            assert_eq!(res.len(), 0);

            let expr = Opath::parse("@.new_prop").unwrap();
            let res = expr.apply(&n, &n).unwrap().into_vec();

            assert_eq!(res.len(), 1);

            let ref p = res[0];
            assert_eq!(p.as_integer(), Some(12));
        }

        #[test]
        fn delete() {
            let n = test_node();
            let e: &str = r#"
        {
            "one": 1,
            "empty_object": {},
            "empty_array": [],
            "array": ["a","b"],
            "null_value": null
        }
        "#;
            let expected = NodeRef::from_json(e).unwrap();

            let expr = Opath::parse("@.delete('nested')").unwrap();

            let _res = expr.apply(&n, &n);

            assert!(n.is_identical_deep(&expected));
        }

        #[test]
        fn push_root() {
            let n = test_node();
            let expr = Opath::parse("(@.array.push('new elem'), @.array[2])").unwrap();

            let res = expr.apply(&n, &n).unwrap().into_vec();

            assert_eq!(res.len(), 2);

            let len = res.get(0).unwrap();
            assert_eq!(len.as_integer().unwrap(), 3);

            let e = res.get(1).unwrap();

            assert!(e.is_string());
            assert_eq!(e.as_string(), "new elem");
        }

        #[test]
        fn push_non_root() {
            let n = test_node();
            let expr = Opath::parse("(@.array.push($.one), @.array[2])").unwrap();
            let res = expr.apply(&n, &n).unwrap().into_vec();

            assert_eq!(res.len(), 2);

            let len = res.get(0).unwrap();
            assert_eq!(len.as_integer().unwrap(), 3);

            let e = res.get(1).unwrap();

            assert!(e.is_number());
            assert_eq!(e.as_float(), 1.0);
        }

        #[test]
        fn pop_non_empty() {
            let n: &str = r#"["a", "b"]"#;
            let n = NodeRef::from_json(n).unwrap();

            let expected: &str = r#"["a"]"#;
            let expected = NodeRef::from_json(expected).unwrap();

            let expr = Opath::parse("@.pop()").unwrap();
            let res = expr.apply(&n, &n).unwrap().into_vec();

            assert_eq!(res.len(), 1);

            let res = res.get(0).unwrap();

            assert!(res.is_string());
            assert_eq!(res.as_string(), "b");

            assert!(&n.is_identical_deep(&expected));
        }

        #[test]
        fn pop_empty() {
            let n: &str = r#"[]"#;
            let n = NodeRef::from_json(n).unwrap();

            let expr = Opath::parse("@.pop()").unwrap();
            let res = expr.apply(&n, &n).unwrap().into_vec();

            assert_eq!(res.len(), 1);

            let res = res.get(0).unwrap();

            assert!(res.is_null());
        }

        #[test]
        fn shift_non_empty() {
            let n: &str = r#"["a", "b"]"#;
            let n = NodeRef::from_json(n).unwrap();

            let expected: &str = r#"["b"]"#;
            let expected = NodeRef::from_json(expected).unwrap();

            let expr = Opath::parse("@.shift()").unwrap();
            let res = expr.apply(&n, &n).unwrap().into_vec();

            assert_eq!(res.len(), 1);

            let res = res.get(0).unwrap();

            assert!(res.is_string());
            assert_eq!(res.as_string(), "a");

            assert!(n.is_identical_deep(&expected));
        }

        #[test]
        fn shift_empty() {
            let n: &str = r#"[]"#;
            let n = NodeRef::from_json(n).unwrap();

            let expr = Opath::parse("@.shift()").unwrap();
            let res = expr.apply(&n, &n).unwrap().into_vec();

            assert_eq!(res.len(), 1);

            let res = res.get(0).unwrap();

            assert!(res.is_null());
        }

        #[test]
        fn unshift_root() {
            let n = test_node();
            let expr = Opath::parse("(@.array.unshift('new elem'), @.array[0])").unwrap();
            let res = expr.apply(&n, &n).unwrap().into_vec();

            assert_eq!(res.len(), 2);

            let len = res.get(0).unwrap();
            assert_eq!(len.as_integer().unwrap(), 3);

            let e = res.get(1).unwrap();

            assert!(e.is_string());
            assert_eq!(e.as_string(), "new elem");
        }

        #[test]
        fn unshift_non_root() {
            let n = test_node();
            let expr = Opath::parse("(@.array.unshift($.one), @.array[0])").unwrap();
            let res = expr.apply(&n, &n).unwrap().into_vec();

            assert_eq!(res.len(), 2);

            let len = res.get(0).unwrap();
            assert_eq!(len.as_integer().unwrap(), 3);

            let e = res.get(1).unwrap();

            assert!(e.is_number());
            assert_eq!(e.as_float(), 1.0);
        }

        mod custom {
            use super::*;

            #[test]
            fn count_char() {
                fn count_char(
                    name: &str,
                    args: Args,
                    env: Env,
                    out: &mut NodeBuf,
                ) -> FuncCallResult {
                    if !env.current().is_string() {
                        Err(basic_diag!(FuncCallErrorDetail::UnknownMethod {
                            name: name.into(),
                            kind: env.current().data().kind(),
                        }))
                    } else {
                        let mut res = args
                            .resolve_flat(false, env)
                            .unwrap()
                            .into_iter()
                            .filter_map(|n| n.data().as_string().chars().next())
                            .collect::<Vec<char>>();
                        res.sort();
                        res.dedup();

                        let d = env.current().data();
                        let s = d.as_string();
                        let mut count = 0;
                        for c in s.chars() {
                            if let Ok(_) = res.binary_search(&c) {
                                count += 1;
                            }
                        }
                        out.add(NodeRef::integer(count as i64));
                        Ok(())
                    }
                }

                let scope = ScopeMut::new().with_method(
                    "count_char".into(),
                    box Method::new(KindMask::string(), count_char),
                );
                let n = NodeRef::null();

                let expr = Opath::parse("'ala ma kota'.count_char('a', 'k')").unwrap();
                let res = expr.apply_ext(&n, &n, scope.as_ref()).unwrap().into_vec();

                assert_eq!(res.len(), 1);
                assert_eq!(res[0].as_integer(), Some(5));

                let expr = Opath::parse("'ala ma kota'.count_char('o', 'k', 't')").unwrap();
                let res = expr.apply_ext(&n, &n, scope.as_ref()).unwrap().into_vec();

                assert_eq!(res.len(), 1);
                assert_eq!(res[0].as_integer(), Some(3));
            }
        }
    }
}
