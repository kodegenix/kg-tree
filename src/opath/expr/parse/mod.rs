use super::*;

use std::collections::VecDeque;
use kg_display::ListDisplay;
use kg_diag::parse::num::{NumberParser, Number, Notation, Sign};

pub type Error = ParseDiag;

pub type Token = LexToken<Terminal>;

#[derive(Debug, Display, Detail)]
#[diag(code_offset = 300)]
pub enum ParseErrorDetail {
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
    #[display(fmt = "unexpected end of input")]
    UnexpectedEoi { pos: Position },
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
    #[display(fmt = "unclosed {a0}")]
    UnclosedGroup(Terminal),
}

impl ParseErrorDetail {
    pub fn invalid_escape<T>(r: &mut dyn CharReader) -> Result<T, Error> {
        let p1 = r.position();
        let err = match r.next_char()? {
            Some(_) => {
                let p2 = r.position();
                parse_diag!(ParseErrorDetail::InvalidEscape {
                    from: p1,
                    to: p2
                }, r, {
                    p1, p2 => "invalid escape",
                })
            }
            None => parse_diag!(ParseErrorDetail::UnexpectedEoi {
                pos: p1,
            }, r, {
                p1, p1 => "unexpected end of input",
            }),
        };
        Err(err)
    }

    pub fn invalid_input<T>(r: &mut dyn CharReader) -> Result<T, Error> {
        let p1 = r.position();
        let err = match r.next_char()? {
            Some(c) => {
                let p2 = r.position();
                parse_diag!(ParseErrorDetail::InvalidChar {
                    input: c,
                    from: p1,
                    to: p2
                }, r, {
                    p1, p2 => "invalid character",
                })
            }
            None => parse_diag!(ParseErrorDetail::UnexpectedEoi {
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
                parse_diag!(ParseErrorDetail::InvalidCharOne {
                    input: c,
                    from: p1,
                    to: p2,
                    expected,
                }, r, {
                    p1, p2 => "invalid character",
                })
            }
            None => parse_diag!(ParseErrorDetail::UnexpectedEoiOne {
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
                parse_diag!(ParseErrorDetail::InvalidCharMany {
                    input: c,
                    from: p1,
                    to: p2,
                    expected,
                }, r, {
                    p1, p2 => "invalid character",
                })
            }
            None => parse_diag!(ParseErrorDetail::UnexpectedEoiMany {
                pos: p1,
                expected,
            }, r, {
                p1, p1 => "unexpected end of input",
            }),
        };
        Err(err)
    }

    #[inline]
    pub fn unexpected_eoi_str<T>(r: &mut dyn CharReader, expected: String) -> Result<T, Error> {
        let pos = r.position();
        Err(parse_diag!(ParseErrorDetail::UnexpectedEoiOneString {
            pos,
            expected,
        }, r, {
            pos, pos => "unexpected end of input",
        }))
    }

    #[inline]
    pub fn unexpected_token<T>(token: Token, r: &mut dyn CharReader) -> Result<T, Error> {
        Err(parse_diag!(ParseErrorDetail::UnexpectedToken { token }, r, {
            token.from(), token.to() => "unexpected token"
        }))
    }

    #[inline]
    pub fn unexpected_token_one<T>(
        token: Token,
        expected: Terminal,
        r: &mut dyn CharReader,
    ) -> Result<T, Error> {
        Err(
            parse_diag!(ParseErrorDetail::UnexpectedTokenOne { token, expected }, r, {
                token.from(), token.to() => "unexpected token"
            }),
        )
    }

    #[inline]
    pub fn unexpected_token_many<T>(
        token: Token,
        expected: Vec<Terminal>,
        r: &mut dyn CharReader,
    ) -> Result<T, Error> {
        Err(
            parse_diag!(ParseErrorDetail::UnexpectedTokenMany { token, expected }, r, {
                token.from(), token.to() => "unexpected token"
            }),
        )
    }
}

#[inline]
pub(crate) fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$' || c == '@'
}

#[inline]
pub(crate) fn is_non_ident_char(c: Option<char>) -> bool {
    match c {
        None => true,
        Some(c) => !is_ident_char(c),
    }
}

#[derive(Debug, Display, PartialEq, Eq, Clone, Copy)]
pub enum Terminal {
    #[display(fmt = "END")]
    End,
    #[display(fmt = "'$'")]
    Root,
    #[display(fmt = "'@'")]
    Current,
    #[display(fmt = "'.'")]
    Dot,
    #[display(fmt = "'..'")]
    DoubleDot,
    #[display(fmt = "':'")]
    Colon,
    #[display(fmt = "'^'")]
    Caret,
    #[display(fmt = "'+'")]
    Plus,
    #[display(fmt = "'-'")]
    Minus,
    #[display(fmt = "'/'")]
    Slash,
    #[display(fmt = "'*'")]
    Star,
    #[display(fmt = "'**'")]
    DoubleStar,
    #[display(fmt = "'!' or 'not'")]
    Not,
    #[display(fmt = "'&&' or 'and'")]
    And,
    #[display(fmt = "'||' or 'or'")]
    Or,
    #[display(fmt = "'=='")]
    Eq,
    #[display(fmt = "'!='")]
    Ne,
    #[display(fmt = "'>'")]
    Gt,
    #[display(fmt = "'>='")]
    Ge,
    #[display(fmt = "'<'")]
    Lt,
    #[display(fmt = "'<='")]
    Le,
    #[display(fmt = "'$='")]
    StartsWith,
    #[display(fmt = "'^='")]
    EndsWith,
    #[display(fmt = "'*='")]
    Contains,
    #[display(fmt = "','")]
    Comma,
    #[display(fmt = "'('")]
    ParenLeft,
    #[display(fmt = "')'")]
    ParenRight,
    #[display(fmt = "'['")]
    BracketLeft,
    #[display(fmt = "']'")]
    BracketRight,
    #[display(fmt = "'{{'")]
    BraceLeft,
    #[display(fmt = "'}}'")]
    BraceRight,
    #[display(fmt = "'${{'")]
    VarBegin,
    #[display(fmt = "VARIABLE")]
    Var,
    #[display(fmt = "'env:'")]
    Env,
    #[display(fmt = "PROPERTY")]
    Property,
    #[display(fmt = "LITERAL")]
    Literal,
    #[display(fmt = "INT")]
    IntDecimal,
    #[display(fmt = "INTx")]
    IntHex,
    #[display(fmt = "INTo")]
    IntOctal,
    #[display(fmt = "INTb")]
    IntBinary,
    #[display(fmt = "FLOAT")]
    Float,
    #[display(fmt = "'true'")]
    True,
    #[display(fmt = "'false'")]
    False,
    #[display(fmt = "'null'")]
    Null,
}

impl LexTerm for Terminal {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Context {
    Expr,
    Property,
    Index,
    Range,
    Env,
    OpAndOr,
    OpCmp,
    OpAddSub,
    OpMulDivMod,
    OpNot,
    OpNeg,
}

#[derive(Debug)]
pub struct Parser {
    num_parser: NumberParser,
    partial: bool,
    multiline: bool,
    prev_pos: Position,
    next_pos: Position,
    token_queue: VecDeque<Token>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            num_parser: {
                let mut p = NumberParser::new();
                p.decimal.allow_plus = false;
                p.decimal.allow_minus = false;
                p.hex.allow_plus = false;
                p.hex.allow_minus = false;
                p.octal.allow_plus = false;
                p.octal.allow_minus = false;
                p.binary.allow_plus = false;
                p.binary.allow_minus = false;
                p
            },
            partial: false,
            multiline: true,
            prev_pos: Position::default(),
            next_pos: Position::default(),
            token_queue: VecDeque::new(),
        }
    }

    pub fn with_partial(mut self, partial: bool) -> Self {
        self.set_partial(partial);
        self
    }

    pub fn with_multiline(mut self, multiline: bool) -> Self {
        self.set_multiline(multiline);
        self
    }

    pub fn is_partial(&self) -> bool {
        self.partial
    }

    pub fn set_partial(&mut self, partial: bool) {
        self.partial = partial
    }

    pub fn is_multiline(&self) -> bool {
        self.multiline
    }

    pub fn set_multiline(&mut self, multiline: bool) {
        self.multiline = multiline
    }

    fn lex(&mut self, r: &mut dyn CharReader) -> Result<Token, Error> {
        fn consume(r: &mut dyn CharReader, count: usize, term: Terminal) -> Result<Token, Error> {
            let p1 = r.position();
            r.skip_chars(count)?;
            let p2 = r.position();
            Ok(Token::new(term, p1, p2))
        }

        if self.multiline {
            r.skip_whitespace()?;
        } else {
            r.skip_whitespace_nonl()?;
        }

        if self.num_parser.is_at_start(r)? {
            let n = self.num_parser.parse_number(r)?;
            match n.term().notation() {
                Notation::Decimal => Ok(Token::new(Terminal::IntDecimal, n.from(), n.to())),
                Notation::Hex => Ok(Token::new(Terminal::IntHex, n.from(), n.to())),
                Notation::Octal => Ok(Token::new(Terminal::IntOctal, n.from(), n.to())),
                Notation::Binary => Ok(Token::new(Terminal::IntBinary, n.from(), n.to())),
                Notation::Float | Notation::Exponent => Ok(Token::new(Terminal::Float, n.from(), n.to())),
            }
        } else {
            match r.peek_char(0)? {
                None => Ok(Token::new(Terminal::End, r.position(), r.position())),
                Some(',') => consume(r, 1, Terminal::Comma),
                Some('(') => consume(r, 1, Terminal::ParenLeft),
                Some(')') => consume(r, 1, Terminal::ParenRight),
                Some('[') => consume(r, 1, Terminal::BracketLeft),
                Some(']') => consume(r, 1, Terminal::BracketRight),
                Some('{') => consume(r, 1, Terminal::BraceLeft),
                Some('}') => consume(r, 1, Terminal::BraceRight),
                Some(':') => consume(r, 1, Terminal::Colon),
                Some('+') => consume(r, 1, Terminal::Plus),
                Some('-') => consume(r, 1, Terminal::Minus),
                Some('/') => consume(r, 1, Terminal::Slash),
                Some('|') => {
                    let p1 = r.position();
                    r.next_char()?;
                    if let Some('|') = r.peek_char(0)? {
                        r.next_char()?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Or, p1, p2))
                    } else {
                        ParseErrorDetail::invalid_input_one(r, '|')
                    }
                }
                Some('&') => {
                    let p1 = r.position();
                    r.next_char()?;
                    if let Some('&') = r.peek_char(0)? {
                        r.next_char()?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::And, p1, p2))
                    } else {
                        ParseErrorDetail::invalid_input_one(r, '&')
                    }
                }
                Some('^') => {
                    let p1 = r.position();
                    r.next_char()?;
                    if let Some('=') = r.peek_char(0)? {
                        r.next_char()?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::StartsWith, p1, p2))
                    } else {
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Caret, p1, p2))
                    }
                }
                Some('=') => {
                    let p1 = r.position();
                    r.next_char()?;
                    if let Some('=') = r.peek_char(0)? {
                        r.next_char()?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Eq, p1, p2))
                    } else {
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Eq, p1, p2))
                    }
                }
                Some('!') => {
                    let p1 = r.position();
                    r.next_char()?;
                    if let Some('=') = r.peek_char(0)? {
                        r.next_char()?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Ne, p1, p2))
                    } else {
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Not, p1, p2))
                    }
                }
                Some('>') => {
                    let p1 = r.position();
                    r.next_char()?;
                    if let Some('=') = r.peek_char(0)? {
                        r.next_char()?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Ge, p1, p2))
                    } else {
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Gt, p1, p2))
                    }
                }
                Some('<') => {
                    let p1 = r.position();
                    r.next_char()?;
                    if let Some('=') = r.peek_char(0)? {
                        r.next_char()?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Le, p1, p2))
                    } else {
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Lt, p1, p2))
                    }
                }
                Some('.') => {
                    let p1 = r.position();
                    r.next_char()?;
                    if let Some('.') = r.peek_char(0)? {
                        r.next_char()?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::DoubleDot, p1, p2))
                    } else {
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Dot, p1, p2))
                    }
                }
                Some('*') => {
                    let p1 = r.position();
                    r.next_char()?;
                    match r.peek_char(0)? {
                        Some('*') => {
                            r.next_char()?;
                            let p2 = r.position();
                            Ok(Token::new(Terminal::DoubleStar, p1, p2))
                        }
                        Some('=') => {
                            r.next_char()?;
                            let p2 = r.position();
                            Ok(Token::new(Terminal::Contains, p1, p2))
                        }
                        _ => {
                            let p2 = r.position();
                            Ok(Token::new(Terminal::Star, p1, p2))
                        }
                    }
                }
                Some('@') => {
                    let p1 = r.position();
                    r.next_char()?;
                    let mut more = false;
                    r.skip_while(&mut |c| {
                        if is_ident_char(c) {
                            more = true;
                            true
                        } else {
                            false
                        }
                    })?;
                    let p2 = r.position();
                    if more {
                        Ok(Token::new(Terminal::Property, p1, p2))
                    } else {
                        Ok(Token::new(Terminal::Current, p1, p2))
                    }
                }
                Some('$') => {
                    let p1 = r.position();
                    r.next_char()?;
                    match r.peek_char(0)? {
                        Some('=') => {
                            r.next_char()?;
                            Ok(Token::new(Terminal::EndsWith, p1, r.position()))
                        }
                        Some('{') => {
                            r.next_char()?;
                            Ok(Token::new(Terminal::VarBegin, p1, r.position()))
                        }
                        _ => {
                            let mut more = false;
                            r.skip_while(&mut |c| {
                                if is_ident_char(c) {
                                    more = true;
                                    true
                                } else {
                                    false
                                }
                            })?;
                            let p2 = r.position();
                            if more {
                                Ok(Token::new(Terminal::Var, p1, p2))
                            } else {
                                Ok(Token::new(Terminal::Root, p1, p2))
                            }
                        }
                    }
                }
                Some('e') => {
                    if r.match_str("env:")? {
                        let p1 = r.position();
                        r.skip_chars(4)?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Env, p1, p2))
                    } else {
                        let p1 = r.position();
                        r.next_char()?;
                        r.skip_while(&mut is_ident_char)?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Property, p1, p2))
                    }
                }
                Some('n') => {
                    if r.match_str_term("null", &mut is_non_ident_char)? {
                        let p1 = r.position();
                        r.skip_chars(4)?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Null, p1, p2))
                    } else if r.match_str_term("not", &mut is_non_ident_char)? {
                        let p1 = r.position();
                        r.skip_chars(3)?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Not, p1, p2))
                    } else {
                        let p1 = r.position();
                        r.next_char()?;
                        r.skip_while(&mut is_ident_char)?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Property, p1, p2))
                    }
                }
                Some('t') => {
                    if r.match_str_term("true", &mut is_non_ident_char)? {
                        let p1 = r.position();
                        r.skip_chars(4)?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::True, p1, p2))
                    } else {
                        let p1 = r.position();
                        r.next_char()?;
                        r.skip_while(&mut is_ident_char)?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Property, p1, p2))
                    }
                }
                Some('f') => {
                    if r.match_str_term("false", &mut is_non_ident_char)? {
                        let p1 = r.position();
                        r.skip_chars(5)?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::False, p1, p2))
                    } else {
                        let p1 = r.position();
                        r.next_char()?;
                        r.skip_while(&mut is_ident_char)?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Property, p1, p2))
                    }
                }
                Some('a') => {
                    if r.match_str_term("and", &mut is_non_ident_char)? {
                        let p1 = r.position();
                        r.skip_chars(3)?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::And, p1, p2))
                    } else {
                        let p1 = r.position();
                        r.next_char()?;
                        r.skip_while(&mut is_ident_char)?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Property, p1, p2))
                    }
                }
                Some('o') => {
                    if r.match_str_term("or", &mut is_non_ident_char)? {
                        let p1 = r.position();
                        r.skip_chars(2)?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Or, p1, p2))
                    } else {
                        let p1 = r.position();
                        r.next_char()?;
                        r.skip_while(&mut is_ident_char)?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Property, p1, p2))
                    }
                }
                Some(c) if c.is_alphabetic() || c == '_' => {
                    let p1 = r.position();
                    r.next_char()?;
                    r.skip_while(&mut is_ident_char)?;
                    let p2 = r.position();
                    Ok(Token::new(Terminal::Property, p1, p2))
                }
                Some(c) if c == '\'' || c == '\"' => {
                    let p1 = r.position();
                    while let Some(k) = r.next_char()? {
                        if k == '\\' {
                            r.next_char()?;
                        } else if k == c {
                            break;
                        }
                    }
                    if r.eof() {
                        ParseErrorDetail::invalid_input_one(r, c)
                    } else {
                        r.next_char()?;
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Literal, p1, p2))
                    }
                }
                Some(_c) => {
                    let p1 = r.position();

                    // (jc) do not report lex errors when parsing partial input
                    if self.partial {
                        Ok(Token::new(Terminal::End, p1, p1))
                    } else {
                        ParseErrorDetail::invalid_input(r)
                    }
                }
            }
        }
    }

    fn next_token(&mut self, r: &mut dyn CharReader) -> Result<Token, Error> {
        if self.token_queue.is_empty() {
            let t = self.lex(r)?;
            self.prev_pos = self.next_pos;
            self.next_pos = t.to();
            Ok(t)
        } else {
            let t = self.token_queue.pop_front().unwrap();
            self.next_pos = t.to();
            Ok(t)
        }
    }

    fn push_token(&mut self, t: Token) {
        self.next_pos = self.prev_pos;
        self.token_queue.push_back(t);
    }

    fn expect_token(&mut self, r: &mut dyn CharReader, term: Terminal) -> Result<Token, Error> {
        let t = self.next_token(r)?;
        if t.term() == term {
            Ok(t)
        } else {
            ParseErrorDetail::unexpected_token_one(t, term, r)
        }
    }

    pub fn parse(&mut self, r: &mut dyn CharReader) -> Result<Opath, Error> {
        let p = r.position();
        self.token_queue.clear();
        self.next_pos = p;

        let e = self.parse_expr(r, Context::Expr);

        match e {
            Ok(e) => {
                if self.partial {
                    r.seek(self.next_pos)?;
                }
                Ok(Opath::new(e))
            }
            Err(err) => {
                if self.partial {
                    r.seek(p)?;
                }
                Err(err)
            }
        }
    }

    fn parse_expr(&mut self, r: &mut dyn CharReader, ctx: Context) -> Result<Expr, Error> {
        let t = self.next_token(r)?;

        let mut e = match t.term() {
            Terminal::Root | Terminal::Current => {
                self.push_token(t);
                self.parse_sequence(r, ctx)?
            }
            Terminal::Var => {
                self.push_token(t);
                self.parse_sequence(r, ctx)?
            }
            Terminal::VarBegin => {
                self.push_token(t);
                self.parse_var_expr(r, ctx)?
            }
            Terminal::Env => {
                self.push_token(t);
                self.parse_env_expr(r, ctx)?
            }
            Terminal::Property => {
                self.push_token(t);
                self.parse_sequence(r, ctx)?
            }
            Terminal::Literal => {
                self.push_token(t);
                self.parse_sequence(r, ctx)?
            }
            Terminal::Minus => {
                let e = self.parse_expr(r, Context::OpNeg)?;
                match e {
                    Expr::Integer(n) => Expr::Integer(-n),
                    Expr::Float(n) => Expr::Float(-n),
                    _ => Expr::Neg(Box::new(e)),
                }
            }
            Terminal::Not => {
                let e = self.parse_expr(r, Context::OpNot)?;
                match e {
                    Expr::Integer(n) => Expr::Boolean(n == 0),
                    Expr::Float(n) => Expr::Boolean(!n.is_normal()),
                    Expr::Boolean(b) => Expr::Boolean(!b),
                    _ => Expr::Not(Box::new(e)),
                }
            }
            Terminal::ParenLeft => {
                let c = if ctx == Context::Env {
                    Context::Env
                } else {
                    Context::Expr
                };
                self.push_token(t);
                self.parse_group(r, c)?
            }
            Terminal::BracketLeft => {
                let c = if ctx == Context::Env {
                    Context::Env
                } else {
                    Context::Expr
                };
                self.push_token(t);
                self.parse_sequence(r, c)?
            }
            Terminal::IntDecimal => {
                match self.num_parser.convert_number::<i64>(t.span(), Sign::None, Notation::Decimal, r) {
                    Ok(n) => Expr::Integer(n),
                    Err(_) => {
                        let n = self.num_parser.convert_number::<f64>(t.span(), Sign::None, Notation::Float, r)?;
                        Expr::Float(n)
                    }
                }
            }
            Terminal::IntHex => {
                let n = self.num_parser.convert_number::<i64>(t.span(), Sign::None, Notation::Hex, r)?;
                Expr::Integer(n)
            }
            Terminal::IntOctal => {
                let n = self.num_parser.convert_number::<i64>(t.span(), Sign::None, Notation::Octal, r)?;
                Expr::Integer(n)
            }
            Terminal::IntBinary => {
                let n = self.num_parser.convert_number::<i64>(t.span(), Sign::None, Notation::Binary, r)?;
                Expr::Integer(n)
            }
            Terminal::Float => {
                let n = self.num_parser.convert_number::<f64>(t.span(), Sign::None, Notation::Float, r)?;
                Expr::Float(n)
            }
            Terminal::True => Expr::Boolean(true),
            Terminal::False => Expr::Boolean(false),
            Terminal::Null => Expr::Null,
            Terminal::Star if ctx == Context::Index => Expr::All,
            Terminal::DoubleStar => {
                let l = self.parse_level_range(r)?.unwrap_or_default();
                Expr::Descendants(Box::new(l))
            }
            Terminal::Colon | Terminal::DoubleDot => {
                if ctx != Context::Range {
                    self.push_token(t);
                    let range = self.parse_number_range(None, r)?;
                    return Ok(Expr::Range(Box::new(range)));
                } else {
                    return ParseErrorDetail::unexpected_token(t, r);
                }
            }
            _ if ctx == Context::Range => {
                self.push_token(t);
                return Ok(Expr::Sequence(Vec::new()));
            }
            _ => {
                let mut expected = vec![
                    Terminal::Root,
                    Terminal::Current,
                    Terminal::Minus,
                    Terminal::Not,
                    Terminal::ParenLeft,
                    Terminal::BracketLeft,
                    Terminal::DoubleStar,
                ];
                if ctx != Context::Range {
                    expected.push(Terminal::Colon);
                    expected.push(Terminal::DoubleDot);
                }

                if ctx == Context::Index {
                    expected.push(Terminal::Star);
                }

                return ParseErrorDetail::unexpected_token_many(t, expected, r);
            }
        };

        loop {
            let t = self.next_token(r)?;

            match t.term() {
                Terminal::Plus => {
                    if ctx > Context::OpAddSub {
                        self.push_token(t);
                        return Ok(e);
                    } else {
                        let f = self.parse_expr(r, Context::OpAddSub)?;
                        e = Expr::Add(Box::new(e), Box::new(f))
                    }
                }
                Terminal::Minus => {
                    if ctx > Context::OpAddSub {
                        self.push_token(t);
                        return Ok(e);
                    } else {
                        let f = self.parse_expr(r, Context::OpAddSub)?;
                        e = Expr::Sub(Box::new(e), Box::new(f))
                    }
                }
                Terminal::Star => {
                    if ctx > Context::OpMulDivMod {
                        self.push_token(t);
                        return Ok(e);
                    } else {
                        let f = self.parse_expr(r, Context::OpMulDivMod)?;
                        e = Expr::Mul(Box::new(e), Box::new(f))
                    }
                }
                Terminal::Slash => {
                    if ctx > Context::OpMulDivMod {
                        self.push_token(t);
                        return Ok(e);
                    } else {
                        let f = self.parse_expr(r, Context::OpMulDivMod)?;
                        e = Expr::Div(Box::new(e), Box::new(f))
                    }
                }
                Terminal::Eq => {
                    if ctx > Context::OpCmp {
                        self.push_token(t);
                        return Ok(e);
                    } else {
                        let f = self.parse_expr(r, Context::OpCmp)?;
                        e = Expr::Eq(Box::new(e), Box::new(f))
                    }
                }
                Terminal::Ne => {
                    if ctx > Context::OpCmp {
                        self.push_token(t);
                        return Ok(e);
                    } else {
                        let f = self.parse_expr(r, Context::OpCmp)?;
                        e = Expr::Ne(Box::new(e), Box::new(f))
                    }
                }
                Terminal::Lt => {
                    if ctx > Context::OpCmp {
                        self.push_token(t);
                        return Ok(e);
                    } else {
                        let f = self.parse_expr(r, Context::OpCmp)?;
                        e = Expr::Lt(Box::new(e), Box::new(f))
                    }
                }
                Terminal::Le => {
                    if ctx > Context::OpCmp {
                        self.push_token(t);
                        return Ok(e);
                    } else {
                        let f = self.parse_expr(r, Context::OpCmp)?;
                        e = Expr::Le(Box::new(e), Box::new(f))
                    }
                }
                Terminal::Gt => {
                    if ctx > Context::OpCmp {
                        self.push_token(t);
                        return Ok(e);
                    } else {
                        let f = self.parse_expr(r, Context::OpCmp)?;
                        e = Expr::Gt(Box::new(e), Box::new(f))
                    }
                }
                Terminal::Ge => {
                    if ctx > Context::OpCmp {
                        self.push_token(t);
                        return Ok(e);
                    } else {
                        let f = self.parse_expr(r, Context::OpCmp)?;
                        e = Expr::Ge(Box::new(e), Box::new(f))
                    }
                }
                Terminal::StartsWith => {
                    if ctx > Context::OpCmp {
                        self.push_token(t);
                        return Ok(e);
                    } else {
                        let f = self.parse_expr(r, Context::OpCmp)?;
                        e = Expr::StartsWith(Box::new(e), Box::new(f))
                    }
                }
                Terminal::EndsWith => {
                    if ctx > Context::OpCmp {
                        self.push_token(t);
                        return Ok(e);
                    } else {
                        let f = self.parse_expr(r, Context::OpCmp)?;
                        e = Expr::EndsWith(Box::new(e), Box::new(f))
                    }
                }
                Terminal::Contains => {
                    if ctx > Context::OpCmp {
                        self.push_token(t);
                        return Ok(e);
                    } else {
                        let f = self.parse_expr(r, Context::OpCmp)?;
                        e = Expr::Contains(Box::new(e), Box::new(f))
                    }
                }
                Terminal::And => {
                    if ctx > Context::OpAndOr {
                        self.push_token(t);
                        return Ok(e);
                    } else {
                        let f = self.parse_expr(r, Context::OpAndOr)?;
                        e = Expr::And(Box::new(e), Box::new(f))
                    }
                }
                Terminal::Or => {
                    if ctx > Context::OpAndOr {
                        self.push_token(t);
                        return Ok(e);
                    } else {
                        let f = self.parse_expr(r, Context::OpAndOr)?;
                        e = Expr::Or(Box::new(e), Box::new(f))
                    }
                }
                Terminal::DoubleDot | Terminal::Colon => {
                    self.push_token(t);
                    if ctx < Context::Range {
                        let range = self.parse_number_range(Some(e), r)?;
                        return Ok(Expr::Range(Box::new(range)));
                    } else {
                        return Ok(e);
                    }
                }
                _ => {
                    self.push_token(t);
                    return Ok(e);
                }
            }
        }
    }

    #[inline]
    fn parse_expr_opt(
        &mut self,
        r: &mut dyn CharReader,
        ctx: Context,
    ) -> Result<Option<Expr>, Error> {
        let e = self.parse_expr(r, ctx)?;
        if let Expr::Sequence(ref elems) = e {
            if elems.is_empty() {
                return Ok(None);
            }
        }
        Ok(Some(e))
    }

    fn parse_literal(&mut self, t: Token, r: &mut dyn CharReader) -> Result<Expr, Error> {
        let p = r.position();
        r.seek(t.from())?;
        let mut s = String::with_capacity(t.to().offset - t.from().offset);
        let sep = match r.peek_char(0)? {
            Some(c) if c == '\'' || c == '\"' => {
                r.next_char()?;
                c
            }
            Some(_) => unreachable!(),
            None => return ParseErrorDetail::unexpected_eoi_str(r, "string literal".into()),
        };
        let mut enc = true;
        while let Some(c) = r.peek_char(0)? {
            if c == sep {
                r.next_char()?;
                break;
            } else if c == '\\' {
                enc = true;
                r.next_char()?;
                match r.peek_char(0)? {
                    Some('\\') => s.push('\\'),
                    Some('\'') => s.push('\''),
                    Some('\"') => s.push('\"'),
                    Some('t') => s.push('\t'),
                    Some('r') => s.push('\r'),
                    Some('n') => s.push('\n'),
                    _ => return ParseErrorDetail::invalid_escape(r),
                }
            } else if is_ident_char(c) {
                s.push(c);
            } else {
                enc = true;
                s.push(c);
            }
            r.next_char()?;
        }
        //debug_assert_eq!(r.position(), t.to());
        r.seek(p)?;
        Ok(if enc {
            Expr::StringEnc(s)
        } else {
            Expr::String(s)
        })
    }

    fn parse_func(&mut self, r: &mut dyn CharReader, _ctx: Context) -> Result<Expr, Error> {
        let mut args = Vec::new();
        let tname = self.expect_token(r, Terminal::Property)?;
        let id = func::FuncId::from(r.slice_pos(tname.from(), tname.to())?.as_ref());
        self.expect_token(r, Terminal::ParenLeft)?;
        loop {
            let t = self.next_token(r)?;
            match t.term() {
                Terminal::ParenRight => {
                    break;
                }
                Terminal::Comma if !args.is_empty() => {}
                _ => self.push_token(t),
            }

            let e = self.parse_expr(r, Context::Expr)?;
            args.push(e);
        }
        Ok(Expr::FuncCall(Box::new(FuncCall::new(id, args))))
    }

    fn parse_method(&mut self, r: &mut dyn CharReader, _ctx: Context) -> Result<Expr, Error> {
        let mut args = Vec::new();
        let tname = self.expect_token(r, Terminal::Property)?;
        let id = func::MethodId::from(r.slice_pos(tname.from(), tname.to())?.as_ref());
        self.expect_token(r, Terminal::ParenLeft)?;
        loop {
            let t = self.next_token(r)?;
            match t.term() {
                Terminal::ParenRight => {
                    break;
                }
                Terminal::Comma if !args.is_empty() => {}
                _ => self.push_token(t),
            }

            let e = self.parse_expr(r, Context::Expr)?;
            args.push(e);
        }
        Ok(Expr::MethodCall(Box::new(MethodCall::new(id, args))))
    }

    fn parse_sequence(&mut self, r: &mut dyn CharReader, ctx: Context) -> Result<Expr, Error> {
        let mut elems = Vec::new();

        let t = self.next_token(r)?;
        match t.term() {
            Terminal::Literal => elems.push(self.parse_literal(t, r)?),
            Terminal::Root => elems.push(Expr::Root),
            Terminal::Current => elems.push(Expr::Current),
            Terminal::BracketLeft => {
                self.push_token(t);
                elems.push(Expr::Current);
            }
            Terminal::Var => {
                let n = r.slice_pos(t.from(), t.to())?;
                elems.push(Expr::Var(Box::new(Expr::String(n[1..].to_string()))));
            }
            Terminal::Property => {
                if ctx == Context::Property || ctx == Context::Env {
                    let n = r.slice_pos(t.from(), t.to())?;
                    elems.push(Expr::String(n.to_string()));
                } else {
                    let tn = self.next_token(r)?;
                    if tn.term() == Terminal::ParenLeft {
                        self.push_token(t);
                        self.push_token(tn);
                        elems.push(self.parse_func(r, ctx)?);
                    } else {
                        self.push_token(tn);
                        let n = r.slice_pos(t.from(), t.to())?;
                        elems.push(Expr::Current);
                        elems.push(Expr::Property(Box::new(Expr::String(n.to_string()))));
                    }
                }
            }
            _ => {
                return ParseErrorDetail::unexpected_token(t, r);
            }
        }

        loop {
            let t = self.next_token(r)?;
            match t.term() {
                Terminal::Caret => {
                    let t = self.next_token(r)?;
                    match t.term() {
                        Terminal::DoubleStar => {
                            let l = match self.parse_level_range(r)? {
                                Some(l) => l,
                                None => LevelRange::default(),
                            };
                            elems.push(Expr::Ancestors(Box::new(l)));
                        }
                        _ => {
                            self.push_token(t);
                            elems.push(Expr::Parent);
                        }
                    }
                }
                Terminal::Dot => {
                    let t = self.next_token(r)?;
                    match t.term() {
                        Terminal::Literal => {
                            elems.push(Expr::Property(Box::new(self.parse_literal(t, r)?)));
                        }
                        Terminal::Property | Terminal::Var => {
                            let tn = self.next_token(r)?;
                            if tn.term() == Terminal::ParenLeft {
                                self.push_token(t);
                                self.push_token(tn);
                                elems.push(self.parse_method(r, ctx)?);
                            } else {
                                self.push_token(tn);
                                let n = r.slice_pos(t.from(), t.to())?;
                                let s = Expr::String(n.to_string());
                                elems.push(Expr::Property(Box::new(s)));
                            }
                        }
                        Terminal::Star => {
                            elems.push(Expr::Property(Box::new(Expr::All)));
                        }
                        Terminal::DoubleStar => {
                            let l = self.parse_level_range(r)?.unwrap_or_default();
                            elems.push(Expr::Descendants(Box::new(l)));
                        }
                        Terminal::ParenLeft => {
                            self.push_token(t);
                            let g = self.parse_group(r, Context::Property)?;
                            elems.push(Expr::Property(Box::new(g)));
                        }
                        _ => {
                            let expected = vec![
                                Terminal::Literal,
                                Terminal::Property,
                                Terminal::Var,
                                Terminal::Star,
                                Terminal::DoubleStar,
                                Terminal::ParenLeft,
                            ];
                            return ParseErrorDetail::unexpected_token_many(t, expected, r);
                        }
                    }
                }
                Terminal::BracketLeft => {
                    self.push_token(t);
                    elems.push(Expr::Index(Box::new(self.parse_group(r, Context::Index)?)));
                }
                _ => {
                    self.push_token(t);
                    break;
                }
            }
        }

        Ok(if elems.len() == 1 {
            elems.pop().unwrap()
        } else {
            Expr::Sequence(elems)
        })
    }

    fn parse_group(&mut self, r: &mut dyn CharReader, ctx: Context) -> Result<Expr, Error> {
        let t = self.next_token(r)?;
        let tsep = match t.term() {
            Terminal::ParenLeft => Terminal::ParenRight,
            Terminal::BracketLeft => Terminal::BracketRight,
            Terminal::BraceLeft => Terminal::BraceRight,
            _ => {
                let expected = vec![
                    Terminal::ParenLeft,
                    Terminal::BracketLeft,
                    Terminal::BraceLeft,
                ];
                return ParseErrorDetail::unexpected_token_many(t, expected, r);
            }
        };
        let op = t;

        let mut elems = Vec::new();

        loop {
            let e = self.parse_expr(r, ctx)?;
            elems.push(e);

            let t = self.next_token(r)?;
            match t.term() {
                Terminal::Comma => continue,
                term if term == tsep => break,
                _ => {
                    let err = parse_diag!(ParseErrorDetail::UnclosedGroup(tsep), r, {
                        op.from(), op.to() => "opened here",
                        t.from(), t.to() => "error occurred here",
                    });
                    return Err(err);
                }
            }
        }

        Ok(if elems.len() == 1 {
            elems.pop().unwrap()
        } else {
            Expr::Group(elems)
        })
    }

    fn parse_var_expr(&mut self, r: &mut dyn CharReader, _ctx: Context) -> Result<Expr, Error> {
        self.expect_token(r, Terminal::VarBegin)?;
        let e = self.parse_expr(r, Context::Expr)?;
        let _t = self.expect_token(r, Terminal::BraceRight)?;
        Ok(Expr::Var(Box::new(e)))
    }

    fn parse_env_expr(&mut self, r: &mut dyn CharReader, _ctx: Context) -> Result<Expr, Error> {
        self.expect_token(r, Terminal::Env)?;
        let e = self.parse_expr(r, Context::Env)?;
        Ok(Expr::Env(Box::new(e)))
    }

    fn parse_level_range(&mut self, r: &mut dyn CharReader) -> Result<Option<LevelRange>, Error> {
        let t = self.next_token(r)?;
        if t.term() == Terminal::BraceLeft {
            let mut l = LevelRange::default();
            let t = self.next_token(r)?;
            if t.term() == Terminal::Comma {
                let max = self.parse_expr(r, Context::Expr)?;
                l.set_max(max);
                self.expect_token(r, Terminal::BraceRight)?;
            } else {
                self.push_token(t);
                let min = self.parse_expr(r, Context::Expr)?;
                l.set_min(min);
                let t = self.next_token(r)?;
                match t.term() {
                    Terminal::BraceRight => {}
                    Terminal::Comma => {
                        let max = self.parse_expr(r, Context::Expr)?;
                        l.set_max(max);
                        self.expect_token(r, Terminal::BraceRight)?;
                    }
                    _ => {
                        let expected = vec![Terminal::BraceRight, Terminal::Comma];
                        return ParseErrorDetail::unexpected_token_many(t, expected, r);
                    }
                }
            }
            Ok(Some(l))
        } else {
            self.push_token(t);
            Ok(None)
        }
    }

    fn parse_number_range(
        &mut self,
        start: Option<Expr>,
        r: &mut dyn CharReader,
    ) -> Result<NumberRange, Error> {
        let mut range = NumberRange::default();
        range.set_start(start);
        let t = self.next_token(r)?;
        match t.term() {
            Terminal::Colon => {
                let s = self.parse_expr_opt(r, Context::Range)?;
                if s.is_some() {
                    let t = self.next_token(r)?;
                    if t.term() == Terminal::Colon {
                        range.set_step(s);
                        range.set_stop(self.parse_expr_opt(r, Context::Range)?);
                    } else {
                        self.push_token(t);
                        range.set_stop(s);
                    }
                }
            }
            Terminal::DoubleDot => {
                range.set_stop(self.parse_expr_opt(r, Context::Range)?);
            }
            _ => {
                let expected = vec![Terminal::Colon, Terminal::DoubleDot];
                return ParseErrorDetail::unexpected_token_many(t, expected, r);
            }
        }
        Ok(range)
    }
}

impl Default for Parser {
    fn default() -> Self {
        Parser::new()
    }
}

#[cfg(test)]
mod tests;