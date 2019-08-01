use super::*;

use kg_display::ListDisplay;

use crate::serial::fmt::toml::Terminal::BracketLeft;
use std::collections::VecDeque;
use std::string::ParseError;

pub type Error = ParseDiag;

pub type Token = LexToken<Terminal>;

#[derive(Debug, Display, Detail)]
#[diag(code_offset = 1200)]
pub enum ParseErrDetail {
    #[display(fmt = "invalid escape")]
    InvalidEscape { from: Position, to: Position },
    #[display(fmt = "invalid character '{input}'")]
    InvalidChar {
        input: char,
        from: Position,
        to: Position,
    },
    #[display(fmt = "invalid character '{input}', expected '{expected}'")]
    InvalidCharOne {
        input: char,
        from: Position,
        to: Position,
        expected: char,
    },
    #[display(
        fmt = "invalid character '{input}', expected one of: {expected}",
        expected = "ListDisplay(expected)"
    )]
    InvalidCharMany {
        input: char,
        from: Position,
        to: Position,
        expected: Vec<char>,
    },
    #[display(fmt = "invalid number literal: {err}")]
    InvalidIntegerLiteral {
        err: std::num::ParseIntError,
        from: Position,
        to: Position,
    },
    #[display(fmt = "invalid number literal: {err}")]
    InvalidFloatLiteral {
        err: std::num::ParseFloatError,
        from: Position,
        to: Position,
    },
    #[display(fmt = "unexpected end of input")]
    UnexpectedEoi { pos: Position },
    #[display(fmt = "unexpected end of line")]
    UnexpectedEol,
    #[display(fmt = "unexpected end of input, expected '{expected}'")]
    UnexpectedEoiOne { pos: Position, expected: char },
    #[display(
        fmt = "unexpected end of input, expected one of: {expected}",
        expected = "ListDisplay(expected)"
    )]
    UnexpectedEoiMany { pos: Position, expected: Vec<char> },
    #[display(fmt = "unexpected end of input, expected \"{expected}\"")]
    UnexpectedEoiOneString { pos: Position, expected: String },
    #[display(fmt = "unexpected symbol {token}")]
    UnexpectedToken { token: Token },
    #[display(fmt = "cannot use multiline string as key")]
    MultilineKey,
    #[display(fmt = "unexpected symbol {token}, expected {expected}")]
    UnexpectedTokenOne { token: Token, expected: Terminal },
    #[display(
        fmt = "unexpected symbol {token}, expected one of: {expected}",
        expected = "ListDisplay(expected)"
    )]
    UnexpectedTokenMany {
        token: Token,
        expected: Vec<Terminal>,
    },
    #[display(fmt = "array types mixed, expected type {expected}")]
    MixedArrayType { expected: ValueType },

    #[display(fmt = "key '{key}' defined multiple times")]
    RedefinedKey { key: String },
    #[display(fmt = "unclosed {a0}")]
    UnclosedGroup(Terminal),
}

impl ParseErrDetail {
    pub fn invalid_escape<T>(r: &mut dyn CharReader) -> Result<T, Error> {
        let p1 = r.position();
        let err = match r.next_char()? {
            Some(_) => {
                let p2 = r.position();
                parse_diag!(ParseErrDetail::InvalidEscape {
                    from: p1,
                    to: p2
                }, r, {
                    p1, p2 => "invalid escape",
                })
            }
            None => parse_diag!(ParseErrDetail::UnexpectedEoi {
                pos: p1,
            }, r, {
                p1, p1 => "unexpected end of input",
            }),
        };
        Err(err)
    }

    pub fn invalid_input<T>(r: &mut dyn CharReader) -> Result<T, Error> {
        let p1 = r.position();
        let invalid = r.peek_char(0)?.unwrap();
        let err = match r.next_char()? {
            Some(c) => {
                let p2 = r.position();
                parse_diag!(ParseErrDetail::InvalidChar {
                    input: invalid,
                    from: p1,
                    to: p2
                }, r, {
                    p1, p2 => "invalid character",
                })
            }
            None => parse_diag!(ParseErrDetail::UnexpectedEoi {
                pos: p1,
            }, r, {
                p1, p1 => "unexpected end of input",
            }),
        };
        Err(err)
    }

    pub fn invalid_input_one<T>(r: &mut dyn CharReader, expected: char) -> Result<T, Error> {
        let p1 = r.position();
        let err = match r.next_char()? {
            Some(c) => {
                let p2 = r.position();
                parse_diag!(ParseErrDetail::InvalidCharOne {
                    input: c,
                    from: p1,
                    to: p2,
                    expected,
                }, r, {
                    p1, p2 => "invalid character",
                })
            }
            None => parse_diag!(ParseErrDetail::UnexpectedEoiOne {
                pos: p1,
                expected,
            }, r, {
                p1, p1 => "unexpected end of input",
            }),
        };
        Err(err)
    }

    pub fn invalid_input_many<T>(r: &mut dyn CharReader, expected: Vec<char>) -> Result<T, Error> {
        let p1 = r.position();
        let err = match r.next_char()? {
            Some(c) => {
                let p2 = r.position();
                parse_diag!(ParseErrDetail::InvalidCharMany {
                    input: c,
                    from: p1,
                    to: p2,
                    expected,
                }, r, {
                    p1, p2 => "invalid character",
                })
            }
            None => parse_diag!(ParseErrDetail::UnexpectedEoiMany {
                pos: p1,
                expected,
            }, r, {
                p1, p1 => "unexpected end of input",
            }),
        };
        Err(err)
    }

    pub fn unexpected_token<T>(token: Token, r: &mut dyn CharReader) -> Result<T, Error> {
        Err(parse_diag!(ParseErrDetail::UnexpectedToken { token }, r, {
            token.from(), token.to() => "unexpected token"
        }))
    }

    pub fn multiline_key<T>(
        token: Token,
        r: &mut dyn CharReader,
    ) -> Result<T, Error> {
        Err(
            parse_diag!(ParseErrDetail::MultilineKey, r, {
                token.from(), token.to() => "invalid multline string usage"
            }),
        )
    }

    pub fn unexpected_token_one<T>(
        token: Token,
        expected: Terminal,
        r: &mut dyn CharReader,
    ) -> Result<T, Error> {
        Err(
            parse_diag!(ParseErrDetail::UnexpectedTokenOne { token, expected }, r, {
                token.from(), token.to() => "unexpected token"
            }),
        )
    }

    pub fn unexpected_token_many<T>(
        token: Token,
        expected: Vec<Terminal>,
        r: &mut dyn CharReader,
    ) -> Result<T, Error> {
        Err(
            parse_diag!(ParseErrDetail::UnexpectedTokenMany { token, expected }, r, {
                token.from(), token.to() => "unexpected token"
            }),
        )
    }

    pub fn mixed_array_type<T>(
        r: &mut dyn CharReader,
        node: NodeRef,
        expected: Kind,
    ) -> Result<T, Error> {
        let from = node.data().metadata().span().unwrap().from;
        let to = node.data().metadata().span().unwrap().to;
        Err(
            parse_diag!(ParseErrDetail::MixedArrayType { expected: expected.into() }, r, {
                from, to => "unexpected type"
            }),
        )
    }

    pub fn unexpected_eol<T>(r: &mut dyn CharReader) -> Result<T, Error> {
        let p = r.position();
        Err(parse_diag!(ParseErrDetail::UnexpectedEol, r, {
            p, p => "unexpected end of line"
        }))
    }

    pub fn key_redefined<T>(
        r: &mut dyn CharReader,
        redefined: Span,
        prev: Span,
        key: &str,
    ) -> Result<T, Error> {
        Err(
            parse_diag!(ParseErrDetail::RedefinedKey{key: key.to_string()}, r, {
                redefined.from, redefined.to => "key redefined here",
                prev.from, prev.to => "previously defined here",
            }),
        )
    }
    pub fn key_redefined_node<T>(
        r: &mut dyn CharReader,
        redefined: Span,
        prev_defined: &NodeRef,
        key: &str,
    ) -> Result<T, Error> {
        let prev = prev_defined
            .data()
            .metadata()
            .span()
            .expect("Node should always have span");

        return ParseErrDetail::key_redefined(r, redefined, prev, &key);
    }
    pub fn key_redefined_nodes<T>(
        r: &mut dyn CharReader,
        redefined: &NodeRef,
        prev_defined: &NodeRef,
        key: &str,
    ) -> Result<T, Error> {
        let prev = prev_defined
            .data()
            .metadata()
            .span()
            .expect("Node should always have span");

        let illegal = redefined
            .data()
            .metadata()
            .span()
            .expect("Node should always have span");
        return ParseErrDetail::key_redefined(r, illegal, prev, &key);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ValueType {
    Boolean,
    Integer,
    Float,
    String,
    Array,
    Table,
}

impl ValueType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ValueType::Boolean => "bool",
            ValueType::Integer => "integer",
            ValueType::Float => "float",
            ValueType::String => "string",
            ValueType::Array => "array",
            ValueType::Table => "table",
        }
    }
}

impl From<Kind> for ValueType {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Null => unreachable!(),
            Kind::Boolean => ValueType::Boolean,
            Kind::Integer => ValueType::Integer,
            Kind::Float => ValueType::Float,
            Kind::String => ValueType::String,
            Kind::Binary => unreachable!(),
            Kind::Array => ValueType::Array,
            Kind::Object => ValueType::Table,
        }
    }
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Display, PartialEq, Eq, Clone, Copy)]
pub enum Terminal {
    #[display(fmt = "END")]
    End,
    #[display(fmt = "NEWLINE")]
    Newline,
    #[display(fmt = "'#'")]
    Comment,
    #[display(fmt = "'='")]
    Equals,
    #[display(fmt = "BARE_KEY")]
    BareKey,
    #[display(fmt = "'{{'")]
    BraceLeft,
    #[display(fmt = "'}}'")]
    BraceRight,
    #[display(fmt = "'['")]
    BracketLeft,
    #[display(fmt = "']'")]
    BracketRight,
    #[display(fmt = "','")]
    Comma,
    #[display(fmt = "'.'")]
    Period,
    #[display(fmt = "STRING")]
    String {
        /// Indicates if string is basic or literal
        literal: bool,
        multiline: bool,
    },
    #[display(fmt = "INT")]
    Integer,
    #[display(fmt = "FLOAT")]
    Float,
    #[display(fmt = "'true'")]
    True,
    #[display(fmt = "'false'")]
    False,
}

impl Terminal {
    /// returns tuple `(literal, multiline)`
    fn unwrap_string(self) -> (bool, bool) {
        match self {
            Terminal::String {literal, multiline} => (literal, multiline),
            _ => { panic!("Not a string!") }
        }
    }
}

impl LexTerm for Terminal {}

fn is_bare(ch: char) -> bool {
    ('A' <= ch && ch <= 'Z')
        || ('a' <= ch && ch <= 'z')
        || ('0' <= ch && ch <= '9')
        || ch == '-'
        || ch == '_'
}

fn is_hex_char(ch: char) -> bool {
    ('A' <= ch && ch <= 'F') || ('a' <= ch && ch <= 'f') || ('0' <= ch && ch <= '9')
}

fn is_oct_char(ch: char) -> bool {
    ('0' <= ch && ch <= '7')
}

fn is_bin_char(ch: char) -> bool {
    ch == '0' || ch == '1'
}

fn is_sign(ch: char) -> bool {
    ch == '-' || ch == '+'
}

fn parse_integer(t: Token, value: Cow<str>) -> Result<i64, Error> {
    let value = value.replace("_", "");
    let mut radix = 10;
    let val = if value.starts_with("0x") {
        radix = 16;
        &value[2..]
    } else if value.starts_with("0o") {
        radix = 8;
        &value[2..]
    } else if value.starts_with("0b") {
        radix = 2;
        &value[2..]
    } else {
        &value[..]
    };

    let num: i64 =
        i64::from_str_radix(val, radix).map_err(|err| ParseErrDetail::InvalidIntegerLiteral {
            err,
            from: t.from(),
            to: t.to(),
        })?;
    Ok(num)
}

fn parse_float(t: Token, value: Cow<str>) -> Result<f64, Error> {
    if value == "inf" || value == "+inf" {
        return Ok(std::f64::INFINITY);
    }
    if value == "-inf" {
        return Ok(std::f64::NEG_INFINITY);
    }
    if value == "nan" || value == "+nan" || value == "-nan" {
        return Ok(std::f64::NAN);
    }

    let s = value.replace("_", "");

    let num: f64 = s
        .parse()
        .map_err(|err| ParseErrDetail::InvalidFloatLiteral {
            err,
            from: t.from(),
            to: t.to(),
        })?;
    Ok(num)
}

fn check_eol(r: &mut dyn CharReader, multiline: bool) -> Result<(), Error> {
    if multiline {
        return Ok(());
    }
    if let Some(c) = r.peek_char(0)? {
        if c == '\n' || c == '\r' {
            return ParseErrDetail::unexpected_eol(r);
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct Parser {
    token_queue: VecDeque<Token>,
    buf: String,
    static_arrays: Vec<NodeRef>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            token_queue: VecDeque::new(),
            buf: String::new(),
            static_arrays: vec![],
        }
    }

    fn lex(&mut self, r: &mut dyn CharReader) -> Result<Token, Error> {
        #[inline]
        fn process_scientific_notation(
            r: &mut dyn CharReader,
            p1: Position,
        ) -> Result<Token, Error> {
            r.next_char()?;
            match r.peek_char(0)? {
                Some('-') | Some('+') => {
                    r.skip_chars(1)?;
                    let mut has_digits = false;
                    r.skip_while_mut(&mut |c| {
                        if c.is_digit(10) {
                            has_digits = true;
                            true
                        } else {
                            false
                        }
                    })?;
                    if has_digits {
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Float, p1, p2))
                    } else {
                        ParseErrDetail::invalid_input_many(
                            r,
                            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
                        )
                    }
                }
                Some(c) if c.is_digit(10) => {
                    r.skip_chars(1)?;
                    r.skip_while(&|c| c.is_digit(10))?;
                    let p2 = r.position();
                    Ok(Token::new(Terminal::Float, p1, p2))
                }
                _ => ParseErrDetail::invalid_input_many(
                    r,
                    vec!['-', '+', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
                ),
            }
        }

        fn consume(r: &mut dyn CharReader, count: usize, term: Terminal) -> Result<Token, Error> {
            let p1 = r.position();
            r.skip_chars(count)?;
            let p2 = r.position();
            Ok(Token::new(term, p1, p2))
        }

        /// consume until newline or EOF
        fn consume_until_newline(r: &mut dyn CharReader, term: Terminal) -> Result<Token, Error> {
            let p1 = r.position();

            while let Some(c) = r.next_char()? {
                if c == '\n' {
                    break;
                }

                if c == '\r' {
                    if let Some('\n') = r.peek_char(1)? {
                        r.next_char()?;
                        break;
                    } else {
                        return ParseErrDetail::invalid_input(r);
                    }
                }
            }
            let p2 = r.position();
            Ok(Token::new(term, p1, p2))
        }

        /// consumes characters after specific prefix
        fn consume_int_prefix(
            r: &mut dyn CharReader,
            f: &dyn Fn(char) -> bool,
            p1: Position,
        ) -> Result<Token, Error> {
            r.next_char()?;
            if let Some('_') = r.next_char()? {
                // underscore is not allowed between prefix and number
                return ParseErrDetail::invalid_input(r);
            }
            r.skip_while(&|c| f(c) || c == '_')?;
            let p2 = r.position();
            return Ok(Token::new(Terminal::Integer, p1, p2));
        }

        fn consume_bare_key(r: &mut dyn CharReader) -> Result<LexToken<Terminal>, ParseDiag> {
            let p1 = r.position();
            while let Some(k) = r.next_char()? {
                if !is_bare(k) {
                    break;
                }
            }

            let p2 = r.position();
            Ok(Token::new(Terminal::BareKey, p1, p2))
        }

        r.skip_until(&|c: char| !(c.is_whitespace() && c != '\n' && c != '\r'))?;

        match r.peek_char(0)? {
            None => Ok(Token::new(Terminal::End, r.position(), r.position())),
            Some(',') => consume(r, 1, Terminal::Comma),
            Some('.') => consume(r, 1, Terminal::Period),
            Some('[') => consume(r, 1, Terminal::BracketLeft),
            Some(']') => consume(r, 1, Terminal::BracketRight),
            Some('{') => consume(r, 1, Terminal::BraceLeft),
            Some('}') => consume(r, 1, Terminal::BraceRight),
            Some('=') => consume(r, 1, Terminal::Equals),
            Some('#') => consume_until_newline(r, Terminal::Comment),
            Some('\n') => consume(r, 1, Terminal::Newline),
            Some('\r') => {
                if let Some('\n') = r.peek_char(1)? {
                    consume(r, 2, Terminal::Newline)
                } else {
                    ParseErrDetail::invalid_input(r)
                }
            }
            Some('i') => {
                if r.match_str_term("inf", &is_non_alphanumeric)? {
                    consume(r, 3, Terminal::Float)
                } else {
                    consume_bare_key(r)
                }
            }
            Some('n') => {
                if r.match_str_term("nan", &is_non_alphanumeric)? {
                    consume(r, 3, Terminal::Float)
                } else {
                    consume_bare_key(r)
                }
            }
            Some('t') => {
                if r.match_str_term("true", &is_non_alphanumeric)? {
                    consume(r, 4, Terminal::True)
                } else {
                    consume_bare_key(r)
                }
            }
            Some('f') => {
                if r.match_str_term("false", &is_non_alphanumeric)? {
                    consume(r, 5, Terminal::False)
                } else {
                    consume_bare_key(r)
                }
            }
            Some('\"') => {
                // handle basic strings
                let p1 = r.position();

                let mut multiline = false;
                if let Some('\"') = r.peek_char(1)? {
                    r.next_char()?;
                    if let Some('\"') = r.next_char()? {
                        multiline = true;
                    } else {
                        return ParseErrDetail::invalid_input_one(r, '\"');
                    }
                }

                while let Some(k) = r.next_char()? {
                    check_eol(r, multiline)?;
                    if k == '\\' {
                        r.next_char()?;
                    } else if k == '\"' {
                        if !multiline {
                            break;
                        } else {
                            if let Some('\"') = r.next_char()? {
                                if let Some('\"') = r.next_char()? {
                                    break;
                                }
                            }
                        }
                    }
                }
                if r.eof() {
                    ParseErrDetail::invalid_input_one(r, '\"')
                } else {
                    r.next_char()?;
                    let p2 = r.position();
                    Ok(Token::new(
                        Terminal::String {
                            literal: false,
                            multiline,
                        },
                        p1,
                        p2,
                    ))
                }
            }
            Some('\'') => {
                // handle literal strings
                let p1 = r.position();

                let mut multiline = false;
                if let Some('\'') = r.peek_char(1)? {
                    r.next_char()?;
                    if let Some('\'') = r.next_char()? {
                        multiline = true;
                    } else {
                        return ParseErrDetail::invalid_input_one(r, '\'');
                    }
                }

                while let Some(k) = r.next_char()? {
                    check_eol(r, multiline)?;
                    if k == '\'' {
                        if !multiline {
                            break;
                        } else {
                            if let Some('\'') = r.next_char()? {
                                if let Some('\'') = r.next_char()? {
                                    break;
                                }
                            }
                        }
                    }
                }

                if r.eof() {
                    ParseErrDetail::invalid_input_one(r, '\'')
                } else {
                    r.next_char()?;
                    let p2 = r.position();
                    Ok(Token::new(
                        Terminal::String {
                            literal: true,
                            multiline,
                        },
                        p1,
                        p2,
                    ))
                }
            }
            Some(c) if c.is_digit(10) || c == '+' || c == '-' => {
                let p1 = r.position();
                let next = r.peek_char(1)?;

                match (c, next) {
                    // Check integers prefix notation
                    ('0', Some('x')) => return consume_int_prefix(r, &|c| is_hex_char(c), p1),
                    ('0', Some('o')) => return consume_int_prefix(r, &|c| is_oct_char(c), p1),
                    ('0', Some('b')) => return consume_int_prefix(r, &|c| is_bin_char(c), p1),

                    // Check special floats
                    (first, Some('i')) if is_sign(first) => {
                        if r.match_str_term(&format!("{}inf", first), &is_non_alphanumeric)? {
                            return consume(r, 4, Terminal::Float);
                        }
                    }
                    (first, Some('n')) if is_sign(first) => {
                        if r.match_str_term(&format!("{}nan", first), &is_non_alphanumeric)? {
                            return consume(r, 4, Terminal::Float);
                        }
                    }
                    _ => {}
                }

                r.next_char()?;
                r.skip_while(&|c| c.is_digit(10) || c == '_')?;
                match r.peek_char(0)? {
                    Some('.') => {
                        if Some('.') == r.peek_char(1)? {
                            let p2 = r.position();
                            return Ok(Token::new(Terminal::Integer, p1, p2));
                        }
                        r.next_char()?;
                        r.skip_while(&|c| c.is_digit(10) || c == '_')?;
                        match r.peek_char(0)? {
                            Some('e') | Some('E') => process_scientific_notation(r, p1),
                            _ => {
                                let p2 = r.position();
                                Ok(Token::new(Terminal::Float, p1, p2))
                            }
                        }
                    }
                    Some('e') | Some('E') => process_scientific_notation(r, p1),
                    _ => {
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Integer, p1, p2))
                    }
                }
            }
            Some(c) if is_bare(c) => consume_bare_key(r),

            // TODO date types https://github.com/toml-lang/toml#offset-date-time
            Some(_) => ParseErrDetail::invalid_input(r),
        }
    }

    fn next_token(&mut self, r: &mut dyn CharReader) -> Result<Token, Error> {
        if let Some(t) = self.token_queue.pop_front() {
            Ok(t)
        } else {
            self.lex(r)
        }
    }

    fn push_token(&mut self, t: Token) {
        self.token_queue.push_back(t);
    }

    fn expect_token(&mut self, r: &mut dyn CharReader, term: Terminal) -> Result<Token, Error> {
        let t = self.next_token(r)?;
        if t.term() == term {
            Ok(t)
        } else {
            ParseErrDetail::unexpected_token_one(t, term, r)
        }
    }

    pub fn parse(&mut self, r: &mut dyn CharReader) -> Result<NodeRef, Error> {
        self.token_queue.clear();
        let mut root = NodeRef::object(LinkedHashMap::new());
        self.parse_inner(r, &mut root)?;
        Ok(root)
    }

    fn parse_inner(&mut self, r: &mut dyn CharReader, parent: &mut NodeRef) -> Result<(), Error> {
        let mut current = parent.clone();

        loop {
            let t = self.next_token(r)?;
            match t.term() {
                Terminal::BareKey | Terminal::Integer | Terminal::Float => {
                    self.push_token(t);
                    self.parse_kv(r, &mut current)?;
                }
                Terminal::String {
                    literal: _,
                    multiline,
                } => {
                    if multiline {
                        // FIXME better error
                        return ParseErrDetail::multiline_key(t, r);
                    }
                    self.push_token(t);
                    self.parse_kv(r, &mut current)?
                }
                Terminal::BracketLeft => {
                    let next = self.next_token(r)?;
                    if next.term() == BracketLeft {
                        self.push_token(t);
                        self.push_token(next);
                        current = self.parse_array_of_tables(r, parent)?;
                    } else {
                        self.push_token(t);
                        self.push_token(next);
                        current = self.parse_table(r, parent)?;
                    }
                }
                Terminal::Newline => {}
                Terminal::Comment => {}
                Terminal::End => return Ok(()),
                _ => {
                    return ParseErrDetail::unexpected_token_many(
                        t,
                        vec![
                            Terminal::String {
                                multiline: true,
                                literal: true,
                            },
                            Terminal::BareKey,
                            Terminal::BracketLeft,
                        ],
                        r,
                    );
                }
            };
        }
    }

    fn parse_key(
        &mut self,
        r: &mut dyn CharReader,
        parent: &mut NodeRef,
    ) -> Result<(NodeRef, String), Error> {
        let mut token = self.next_token(r)?;
        let mut current = parent.clone();
        let mut key;
        loop {
            key = match token.term() {
                Terminal::String { .. } => {
                    self.parse_string(token, r)?;
                    self.buf.clone()
                }
                Terminal::BareKey => r.slice_pos(token.from(), token.to())?.into_owned(),
                Terminal::Integer => {
                    let value = r.slice_pos(token.from(), token.to())?;
                    value.into()
                }
                Terminal::Float => {
                    // FIXME handle floats as keys.
                    // for example: `3.14159 = "pi"`
                    // should be parsed to: { "3": { "14159": "pi" } }
                    let value = r.slice_pos(token.from(), token.to())?;
                    value.into()
                }
                _ => {
                    return ParseErrDetail::unexpected_token_many(
                        token,
                        vec![
                            Terminal::BareKey,
                            Terminal::String {
                                multiline: true,
                                literal: true,
                            },
                        ],
                        r,
                    );
                }
            };
            let next = self.next_token(r)?;

            match next.term() {
                Terminal::Period => {
                    if let Some(child) = current.get_child_key(&key) {
                        if child.is_array() {
                            if self.is_static_array(&child) {
                                return ParseErrDetail::key_redefined_node(
                                    r,
                                    token.span(),
                                    &child,
                                    &key,
                                );
                            }
                            // array of tables must have at least one element
                            let idx = child.data().children_count().unwrap() - 1;
                            current = child.get_child_index(idx).unwrap();
                        } else if child.is_object() {
                            current = child;
                        } else {
                            return ParseErrDetail::key_redefined_node(
                                r,
                                token.span(),
                                &child,
                                &key,
                            );
                        }
                    } else {
                        let new = NodeRef::object(LinkedHashMap::new()).with_span(token.span());
                        current
                            .add_child(None, Some(key.into()), new.clone())
                            .unwrap();
                        current = new;
                    }
                    token = self.next_token(r)?;
                    continue;
                }
                _ => {
                    if let Some(child) = current.get_child_key(&key) {
                        if child.is_array() {
                            current = child;
                        } else {
                            return ParseErrDetail::key_redefined_node(
                                r,
                                token.span(),
                                &child,
                                &key,
                            );
                        }
                    }

                    self.push_token(next);

                    // TODO hashset
                    return Ok((current.clone(), key));
                }
            }
        }
    }

    fn parse_kv(&mut self, r: &mut dyn CharReader, parent: &mut NodeRef) -> Result<(), Error> {
        let (node, key) = self.parse_key(r, parent)?;
        self.expect_token(r, Terminal::Equals)?;
        let val = self.parse_value(r, parent)?;
        node.add_child(None, Some(key.into()), val).unwrap();
        let next = self.next_token(r)?;
        if next.term() == Terminal::Newline || next.term() == Terminal::End {
            Ok(())
        } else {
            ParseErrDetail::unexpected_token_many(next, vec![Terminal::Newline, Terminal::End], r)
        }
    }

    fn parse_value(
        &mut self,
        r: &mut dyn CharReader,
        parent: &mut NodeRef,
    ) -> Result<NodeRef, Error> {
        let t = self.next_token(r)?;

        match t.term() {
            Terminal::String { .. } => {
                self.parse_string(t, r)?;
                Ok(NodeRef::string(&self.buf).with_span(t.span()))
            }
            Terminal::BraceLeft => {
                self.push_token(t);
                self.parse_inline_table(r)
            }
            Terminal::BracketLeft => {
                self.push_token(t);
                self.parse_array(r, parent)
            }
            Terminal::Float => {
                let value = r.slice_pos(t.from(), t.to())?;
                let num = parse_float(t, value)?;
                Ok(NodeRef::float(num).with_span(t.span()))
            }
            Terminal::Integer => {
                let value = r.slice_pos(t.from(), t.to())?;
                let num = parse_integer(t, value)?;
                Ok(NodeRef::integer(num).with_span(t.span()))
            }
            Terminal::True => Ok(NodeRef::boolean(true).with_span(t.span())),
            Terminal::False => Ok(NodeRef::boolean(false).with_span(t.span())),
            _ => {
                return ParseErrDetail::unexpected_token_many(
                    t,
                    vec![
                        Terminal::String {
                            multiline: true,
                            literal: true,
                        },
                        Terminal::BraceLeft,
                        Terminal::BracketLeft,
                        Terminal::Float,
                        Terminal::Integer,
                    ],
                    r,
                );
            }
        }
    }

    fn parse_table(
        &mut self,
        r: &mut dyn CharReader,
        parent: &mut NodeRef,
    ) -> Result<NodeRef, Error> {
        let from = self.expect_token(r, Terminal::BracketLeft)?.from();
        let (node, key) = self.parse_key(r, parent)?;
        let to = self.expect_token(r, Terminal::BracketRight)?.to();

        if node.is_array() {
            // attempt to override existing array by table
            // array of tables cannot be empty
            let node = node
                .get_child_index(node.data().children_count().unwrap() - 1)
                .unwrap();
            return ParseErrDetail::key_redefined_node(r, Span::with_pos(from, to), &node, &key);
        }

        let table = NodeRef::object(LinkedHashMap::new()).with_span(Span::with_pos(from, to));
        node.add_child(None, Some(key.into()), table.clone())
            .unwrap();
        Ok(table)
    }

    fn parse_inline_table(&mut self, r: &mut dyn CharReader) -> Result<NodeRef, Error> {
        let from = self.expect_token(r, Terminal::BraceLeft)?.from();

        let mut table = NodeRef::object(LinkedHashMap::new());
        loop {
            let (mut node, key) = self.parse_key(r, &mut table)?;
            self.expect_token(r, Terminal::Equals)?;
            let val = self.parse_value(r, &mut node)?;
            node.add_child(None, Some(key.into()), val).unwrap();

            let next = self.next_token(r)?;

            match next.term() {
                Terminal::Comma => {}
                Terminal::BraceRight => {
                    table = table.with_span(Span::with_pos(from, next.to()));
                    break;
                }
                _ => {
                    return ParseErrDetail::unexpected_token_many(
                        next,
                        vec![Terminal::Comma, Terminal::BraceRight],
                        r,
                    );
                }
            }
        }

        Ok(table)
    }

    fn parse_array_of_tables(
        &mut self,
        r: &mut dyn CharReader,
        parent: &mut NodeRef,
    ) -> Result<NodeRef, Error> {
        let from = self.expect_token(r, Terminal::BracketLeft)?.from();
        self.expect_token(r, Terminal::BracketLeft)?;

        let (mut node, key) = self.parse_key(r, parent)?;

        self.expect_token(r, Terminal::BracketRight)?;
        let to = self.expect_token(r, Terminal::BracketRight)?.to();

        let mut table = NodeRef::object(LinkedHashMap::new()).with_span(Span::with_pos(from, to));

        if node.is_array() {
            if self.is_static_array(&node) {
                return ParseErrDetail::key_redefined_nodes(r, &node, &table, &key);
            }
            let idx = node.data().children_count().unwrap();
            node.add_child(Some(idx), None, table.clone()).unwrap();
        } else {
            let array = NodeRef::array(vec![table.clone()]);
            node.add_child(None, Some(key.into()), array).unwrap();
        }
        Ok(table)
    }

    fn parse_array(
        &mut self,
        r: &mut dyn CharReader,
        parent: &mut NodeRef,
    ) -> Result<NodeRef, Error> {
        let p1 = self.expect_token(r, Terminal::BracketLeft)?.from();
        let mut elems = Elements::new();
        let mut comma = false;
        loop {
            let t = self.next_token(r)?;
            match t.term() {
                Terminal::BracketRight => {
                    let span = Span {
                        from: p1,
                        to: t.to(),
                    };
                    let array = NodeRef::array(elems).with_span(span);
                    self.static_arrays.push(array.clone());
                    return Ok(array);
                }
                Terminal::Comma if comma => {
                    comma = false;
                }
                Terminal::Newline => {}
                _ if !comma => {
                    self.push_token(t);
                    let value = self.parse_value(r, parent)?;
                    if !elems.is_empty() {
                        let array_type = elems.get(0).unwrap().data().kind();
                        if array_type != value.data().kind() {
                            return ParseErrDetail::mixed_array_type(r, value, array_type);
                        }
                        elems.push(value);
                    } else {
                        elems.push(value);
                    }
                    comma = true;
                }
                _ => return ParseErrDetail::unexpected_token(t, r),
            }
        }
    }

    fn parse_string<'a>(&mut self, t: Token, r: &'a mut dyn CharReader) -> Result<(), Error> {
        fn prepare_multiline(val: &str) -> &str {
            // trim quotes
            let val = &val[3..val.len() - 3];

            // remove newline immediately following opening delimiter
            if val.starts_with("\n") {
                &val[1..]
            } else if val.starts_with("\r\n") {
                &val[2..]
            } else {
                val
            }
        }

        let (literal, multiline) = t.term().unwrap_string();
        let s = r.slice_pos(t.from(), t.to())?;

        let val = if multiline {
            prepare_multiline(&s)
        } else {
            &s[1..s.len() - 1]
        };

        if literal {
            self.buf.clear();
            self.buf.reserve(val.len());
            self.buf.push_str(val);
        } else {
            let mut chars=  val.chars().peekable();
            self.buf.clear();
            self.buf.reserve(val.len());
            while let Some(c) = chars.next() {
                if c == '\\' {
                    let c = chars.next();
                    match c {
                        Some('b') => self.buf.push('\u{0008}'), // backspace
                        Some('t') => self.buf.push('\t'),       // tab
                        Some('n') => self.buf.push('\n'),       // linefeed
                        Some('f') => self.buf.push('\u{000c}'), // form feed
                        Some('r') => self.buf.push('\r'),       // carriage return
                        Some('\"') => self.buf.push('\"'),      // quote
                        Some('\\') => self.buf.push('\\'),      // backslash
                        // FIXME ws https://github.com/toml-lang/toml#string
                        Some('u') => unimplemented!(),
                        Some('U') => unimplemented!(),
                        Some(c) if c.is_whitespace() => {
                            // handle line ending backslash
                            while let Some(c) = chars.peek() {
                                if c.is_whitespace() {
                                    chars.next();
                                } else {
                                    break
                                }
                            }
                        }
                        _ => return ParseErrDetail::invalid_escape(r),
                    }
                } else {
                    self.buf.push(c);
                }
            }
        }
        Ok(())
    }

    fn is_static_array(&self, node: &NodeRef) -> bool {
        let static_array = self.static_arrays.iter().find(|n| n.is_ref_eq(&node));
        static_array.is_some()
    }
}

impl Default for Parser {
    fn default() -> Self {
        Parser::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod lex {
        use super::*;

        macro_rules! assert_term {
            ($input: expr, $term: expr) => {
                let mut r = MemCharReader::new(input.as_bytes());
                let mut parser = Parser::new();

                let token = parser.lex(&mut r).unwrap();

                assert_eq!(token.term(), term);
            };
        }

        macro_rules! assert_terms {
            ($input: expr, $expected: expr) => {
                let mut r = MemCharReader::new($input.as_bytes());
                let mut parser = Parser::new();

                for t in $expected {
                    let token = parser.lex(&mut r).expect("Cannot get token!");
                    assert_eq!(token.term(), t);
                }
            };
        }

        #[test]
        fn new_line_lf() {
            let input: &str = "\n";

            let mut r = MemCharReader::new(input.as_bytes());
            let mut parser = Parser::new();

            let token = parser.lex(&mut r).unwrap();

            let p1 = Position::default();
            let p2 = Position {
                column: 0,
                line: 1,
                offset: 1,
            };

            assert_eq!(token.term(), Terminal::Newline);
            assert_eq!(token.from(), p1);
            assert_eq!(token.to(), p2);
        }

        #[test]
        fn new_line_crlf() {
            let input: &str = "\r\n";

            let mut r = MemCharReader::new(input.as_bytes());
            let mut parser = Parser::new();

            let token = parser.lex(&mut r).unwrap();

            let p1 = Position::default();
            let p2 = Position {
                column: 0,
                line: 1,
                offset: 2,
            };

            assert_eq!(token.term(), Terminal::Newline);
            assert_eq!(token.from(), p1);
            assert_eq!(token.to(), p2);
        }

        #[test]
        fn new_line_crlf_malformed() {
            let input: &str = "\rmalformed";

            let mut r = MemCharReader::new(input.as_bytes());
            let mut parser = Parser::new();

            let err = parser.lex(&mut r).unwrap_err();

            let detail = err.detail().downcast_ref::<ParseErrDetail>().unwrap();

            if let ParseErrDetail::InvalidChar { .. } = detail {
                // Ok
            } else {
                panic!("InvalidChar Error expected")
            }
        }

        #[test]
        fn comment_lf() {
            let input: &str = "#comment\n";

            let terms = vec![Terminal::Comment, Terminal::Newline, Terminal::End];
            assert_terms!(input, terms);
        }

        #[test]
        fn comment_crlf() {
            let input: &str = "#comment\r\n";

            let terms = vec![Terminal::Comment, Terminal::Newline, Terminal::End];
            assert_terms!(input, terms);
        }

        #[test]
        fn comment_eof() {
            let input: &str = r#"#comment
            #comment"#;

            let terms = vec![
                Terminal::Comment,
                Terminal::Newline,
                Terminal::Comment,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn bare_keys() {
            let input: &str = r#"key
            bare_key
            bare-key"#;

            let terms = vec![
                Terminal::BareKey,
                Terminal::Newline,
                Terminal::BareKey,
                Terminal::Newline,
                Terminal::BareKey,
                Terminal::End,
            ];

            assert_terms!(input, terms);
        }

        #[test]
        fn dotted_key() {
            let input: &str = r#"key.after_dot.a"#;

            let terms = vec![
                Terminal::BareKey,
                Terminal::Period,
                Terminal::BareKey,
                Terminal::Period,
                Terminal::BareKey,
                Terminal::End,
            ];

            assert_terms!(input, terms);
        }

        #[test]
        fn true_token() {
            let input: &str = r#"true"#;

            let terms = vec![Terminal::True, Terminal::End];

            assert_terms!(input, terms);
        }

        #[test]
        fn basic_string() {
            let input: &str = r#""some basic string"
            "another basic string""#;

            let terms = vec![
                Terminal::String {
                    literal: false,
                    multiline: false,
                },
                Terminal::Newline,
                Terminal::String {
                    literal: false,
                    multiline: false,
                },
                Terminal::End,
            ];

            assert_terms!(input, terms);
        }

        #[test]
        fn basic_multiline_string() {
            let input: &str = r#""""
            muli
            line
            string
            """"#;

            let terms = vec![
                Terminal::String {
                    literal: false,
                    multiline: true,
                },
                Terminal::End,
            ];

            assert_terms!(input, terms);
        }

        #[test]
        fn basic_multiline_string_d_quot() {
            let input: &str = r#""""
            muli
            line
            string with "quotation"
            """"#;

            let terms = vec![
                Terminal::String {
                    literal: false,
                    multiline: true,
                },
                Terminal::End,
            ];

            assert_terms!(input, terms);
        }

        #[test]
        fn basic_multiline_string_single_quot() {
            let input: &str = r#""""
            muli
            line
            string with 'quotation'
            """"#;

            let terms = vec![
                Terminal::String {
                    literal: false,
                    multiline: true,
                },
                Terminal::End,
            ];

            assert_terms!(input, terms);
        }

        #[test]
        fn basic_multiline_string_two_quot() {
            let input: &str = r#""""
            muli
            line
            string with two quotes - ""
            """"#;

            let terms = vec![
                Terminal::String {
                    literal: false,
                    multiline: true,
                },
                Terminal::End,
            ];

            assert_terms!(input, terms);
        }

        #[test]
        fn literal_string() {
            let input: &str = r#"'literal string'"#;

            let terms = vec![
                Terminal::String {
                    literal: true,
                    multiline: false,
                },
                Terminal::End,
            ];

            assert_terms!(input, terms);
        }

        #[test]
        fn literal_multiline_string() {
            let input: &str = r#"'''
            literal
            multiline string
             string
            '''"#;

            let terms = vec![
                Terminal::String {
                    literal: true,
                    multiline: true,
                },
                Terminal::End,
            ];

            assert_terms!(input, terms);
        }

        #[test]
        fn literal_multiline_string_single_quot() {
            let input: &str = r#"'''
            literal
            multiline string
             with single quote '
             and two single quotes ''
            '''"#;

            let terms = vec![
                Terminal::String {
                    literal: true,
                    multiline: true,
                },
                Terminal::End,
            ];

            assert_terms!(input, terms);
        }

        #[test]
        fn integers() {
            let input: &str = r#"+99
            42
            0
            -17"#;

            let terms = vec![
                Terminal::Integer,
                Terminal::Newline,
                Terminal::Integer,
                Terminal::Newline,
                Terminal::Integer,
                Terminal::Newline,
                Terminal::Integer,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn integers_underscores() {
            let input: &str = r#"1_000
            -5_349_221
            1_2_3_4_5"#;

            let terms = vec![
                Terminal::Integer,
                Terminal::Newline,
                Terminal::Integer,
                Terminal::Newline,
                Terminal::Integer,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn integers_hex() {
            let input: &str = r#"0xDEADBEEF
            0xdeadbeef
            0xdead_beef"#;

            let terms = vec![
                Terminal::Integer,
                Terminal::Newline,
                Terminal::Integer,
                Terminal::Newline,
                Terminal::Integer,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn integers_oct() {
            let input: &str = r#"0o01234567
            0o755"#;

            let terms = vec![
                Terminal::Integer,
                Terminal::Newline,
                Terminal::Integer,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn integers_bin() {
            let input: &str = r#"0b11010110"#;

            let terms = vec![Terminal::Integer, Terminal::End];
            assert_terms!(input, terms);
        }

        #[test]
        fn floats() {
            let input: &str = r#"+1.0
            3.1415
            -0.01
            5e+22
            1e6
            2E-2
            6.626e-34"#;

            let terms = vec![
                Terminal::Float,
                Terminal::Newline,
                Terminal::Float,
                Terminal::Newline,
                Terminal::Float,
                Terminal::Newline,
                Terminal::Float,
                Terminal::Newline,
                Terminal::Float,
                Terminal::Newline,
                Terminal::Float,
                Terminal::Newline,
                Terminal::Float,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn floats_underscores() {
            let input: &str = r#"224_617.44e-34"#;

            let terms = vec![Terminal::Float, Terminal::End];
            assert_terms!(input, terms);
        }
    }
}
