use serde::{de, ser};

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Interpolation {
    Empty,
    Simple(String),
    Expr(Opath),
}

impl Interpolation {
    pub fn parse(input: Cow<str>) -> Result<Interpolation, OpathParseError> {
        match Parser::new().parse_str(&input)? {
            Interpolation::Empty => {
                if let Cow::Owned(s) = input {
                    Ok(Interpolation::Simple(s))
                } else {
                    Ok(Interpolation::Empty)
                }
            }
            i @ _ => Ok(i),
        }
    }

    pub fn parse_always(input: Cow<str>) -> Interpolation {
        let mut p = Parser::new();
        match input {
            Cow::Borrowed(s) => p.parse_str(s).unwrap_or(Interpolation::Empty),
            Cow::Owned(s) => match p.parse_str(&s) {
                Ok(i) => match i {
                    Interpolation::Empty => Interpolation::Simple(s),
                    _ => i,
                },
                Err(_err) => Interpolation::Simple(s),
            },
        }
    }

    pub fn parse_delims(
        input: Cow<str>,
        open_delim: &str,
        close_delim: &str,
    ) -> Result<Interpolation, OpathParseError> {
        match Parser::with_delims(open_delim, close_delim).parse_str(&input)? {
            Interpolation::Empty => {
                if let Cow::Owned(s) = input {
                    Ok(Interpolation::Simple(s))
                } else {
                    Ok(Interpolation::Empty)
                }
            }
            i @ _ => Ok(i),
        }
    }

    pub fn resolve(&self, root: &NodeRef, current: &NodeRef) -> Option<NodeRef> {
        match *self {
            Interpolation::Empty => None,
            Interpolation::Simple(ref s) => Some(NodeRef::string(s.as_str())),
            Interpolation::Expr(ref e) => Some(e.apply_one(root, current)),
        }
    }

    pub fn resolve_into(self, root: &NodeRef, current: &NodeRef) -> Option<NodeRef> {
        match self {
            Interpolation::Empty => None,
            Interpolation::Simple(s) => Some(NodeRef::string(s.as_str())),
            Interpolation::Expr(e) => Some(e.apply_one(root, current)),
        }
    }

    pub fn resolve_ext(&self, root: &NodeRef, current: &NodeRef, scope: &Scope) -> Option<NodeRef> {
        match *self {
            Interpolation::Empty => None,
            Interpolation::Simple(ref s) => Some(NodeRef::string(s.as_str())),
            Interpolation::Expr(ref e) => Some(e.apply_one_ext(root, current, scope)),
        }
    }

    pub fn resolve_ext_into(
        self,
        root: &NodeRef,
        current: &NodeRef,
        scope: &Scope,
    ) -> Option<NodeRef> {
        match self {
            Interpolation::Empty => None,
            Interpolation::Simple(s) => Some(NodeRef::string(s)),
            Interpolation::Expr(e) => Some(e.apply_one_ext(root, current, scope)),
        }
    }

    pub fn is_empty(&self) -> bool {
        match *self {
            Interpolation::Empty => true,
            _ => false,
        }
    }

    pub fn is_simple(&self) -> bool {
        match *self {
            Interpolation::Simple(_) => true,
            _ => false,
        }
    }

    pub fn is_expr(&self) -> bool {
        match *self {
            Interpolation::Expr(_) => true,
            _ => false,
        }
    }
}

impl ser::Serialize for Interpolation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match *self {
            Interpolation::Empty => serializer.serialize_none(),
            Interpolation::Simple(ref s) => serializer.serialize_str(s),
            Interpolation::Expr(ref e) => serializer.collect_str(&format_args!("`{}`", e)),
        }
    }
}

struct InterpolationVisitor;

impl<'de> de::Visitor<'de> for InterpolationVisitor {
    type Value = Interpolation;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Interpolation::parse_always(v.to_string().into()))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Interpolation::parse_always(v.to_string().into()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Interpolation::parse_always(v.into()))
    }
}

impl<'de> de::Deserialize<'de> for Interpolation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(InterpolationVisitor)
    }
}

#[derive(Debug)]
struct Delims {
    open: String,
    close: String,
    first_open: char,
    first_close: char,
    quotes: Vec<char>,
}

#[derive(Debug)]
pub struct Parser {
    expr_parser: self::parse::Parser,
    delims: Delims,
}

impl Parser {
    pub fn new() -> Parser {
        Parser::with_delims("<%", "%>")
    }

    pub fn with_delims(open_delim: &str, close_delim: &str) -> Parser {
        debug_assert!(!open_delim.is_empty());
        debug_assert_eq!(open_delim, open_delim.trim());
        debug_assert!(!close_delim.is_empty());
        debug_assert_eq!(close_delim, close_delim.trim());

        let mut quotes: Vec<char> = open_delim.chars().chain(close_delim.chars()).collect();
        quotes.sort();
        quotes.dedup();

        let delims = Delims {
            open: open_delim.to_string(),
            close: close_delim.to_string(),
            first_open: open_delim.chars().next().unwrap(),
            first_close: close_delim.chars().next().unwrap(),
            quotes,
        };

        Parser {
            expr_parser: self::parse::Parser::new().with_partial(true),
            delims,
        }
    }

    pub fn parse(&mut self, r: &mut dyn CharReader) -> Result<Interpolation, OpathParseError> {
        let mut expr = Vec::new();
        let mut buf = String::new();

        let mut p0 = r.position();

        while let Some(c) = r.next_char()? {
            match c {
                '\\' => {
                    let p1 = r.position();
                    match r.next_char()? {
                        Some(c) if self.delims.quotes.contains(&c) => {
                            if p1 > p0 {
                                buf.push_str(&r.slice_pos(p0, p1)?);
                            }
                            p0 = r.position();
                        }
                        _ => {}
                    }
                }
                c if c == self.delims.first_open => {
                    if r.match_str(&self.delims.open)? {
                        let p1 = r.position();
                        if p0 < p1 {
                            buf.push_str(&r.slice_pos(p0, p1)?);
                        }
                        if !buf.is_empty() {
                            expr.push(Expr::String(buf));
                            buf = String::new();
                        }
                        r.skip_chars(self.delims.open.len())?;

                        let e = self.expr_parser.parse(r)?;
                        r.skip_whitespace_nonl()?;
                        if r.match_str(&self.delims.close)? {
                            expr.push(e.into_expr());
                            r.skip_chars(self.delims.close.len())?;
                            p0 = r.position();
                        } else {
                            return self::parse::ParseErr::unexpected_eoi_str(
                                r,
                                self.delims.close.clone(),
                            );
                        }
                    }
                }
                _ => {}
            }
        }

        if !expr.is_empty() {
            let p1 = r.position();
            if p1 > p0 {
                buf.push_str(&r.slice_pos(p0, p1)?);
            }
            if !buf.is_empty() {
                expr.push(Expr::String(buf));
                buf = String::new();
            }
        } else if !buf.is_empty() {
            let p1 = r.position();
            if p1 > p0 {
                buf.push_str(&r.slice_pos(p0, p1)?);
            }
        }

        Ok(match expr.len() {
            0 => {
                if buf.is_empty() {
                    Interpolation::Empty
                } else {
                    Interpolation::Simple(buf)
                }
            }
            1 => match expr.pop().unwrap() {
                Expr::String(s) => Interpolation::Simple(s),
                Expr::StringEnc(s) => Interpolation::Simple(s),
                e @ _ => Interpolation::Expr(Opath::new(e)),
            },
            _ => Interpolation::Expr(Opath::new(Expr::Concat(expr))),
        })
    }

    pub fn parse_str(&mut self, input: &str) -> Result<Interpolation, OpathParseError> {
        let mut r = MemCharReader::new(input.as_bytes());
        self.parse(&mut r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_node<'a>() -> NodeRef {
        NodeRef::from_json(r#"{"username":"johnny", "email": "johnny@example.org"}"#).unwrap()
    }

    #[test]
    fn empty() {
        let s = "No interpolation";
        let n = test_node();

        let interp = Interpolation::parse(s.into()).unwrap();

        assert!(interp.is_empty());
        assert_eq!(interp.resolve(&n, &n), None);
    }

    #[test]
    fn with_escapes() {
        let s = "test \\<\\%\\> test";
        let n = test_node();

        let interp = Interpolation::parse(s.into()).unwrap();

        assert!(interp.is_simple());
        assert_eq!(interp.resolve(&n, &n).unwrap().as_string(), "test <%> test");
    }

    #[test]
    fn with_expressions_inside() {
        let s = "username: <% username %>, email: <% email %> was logged in.";
        let n = test_node();

        let interp = Interpolation::parse(s.into()).unwrap();

        assert!(interp.is_expr());
        assert_eq!(
            interp.resolve(&n, &n).unwrap().as_string(),
            "username: johnny, email: johnny@example.org was logged in."
        );
    }

    #[test]
    fn whole_expression() {
        let s = "<% username %>";
        let n = test_node();

        let interp = Interpolation::parse(s.into()).unwrap();

        assert!(interp.is_expr());
        assert_eq!(interp.resolve(&n, &n).unwrap().as_string(), "johnny");
    }

    #[test]
    fn with_expressions_and_escapes() {
        let s = "username: <% username %>, email: \\<%<%email%>%> was logged in.";
        let n = test_node();

        let interp = Interpolation::parse(s.into()).unwrap();

        assert!(interp.is_expr());
        assert_eq!(
            interp.resolve(&n, &n).unwrap().as_string(),
            "username: johnny, email: <%johnny@example.org%> was logged in."
        );
    }

    #[test]
    fn with_custom_delimiters() {
        let s = "username: ${username}$, email: ${email}$ was logged in.";
        let n = test_node();

        let interp = Interpolation::parse_delims(s.into(), "${", "}$").unwrap();

        assert!(interp.is_expr());
        assert_eq!(
            interp.resolve(&n, &n).unwrap().as_string(),
            "username: johnny, email: johnny@example.org was logged in."
        );
    }
}
