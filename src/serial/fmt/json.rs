use super::*;

use kg_display::ListDisplay;

use std::collections::VecDeque;

pub type Error = ParseDiag;

pub type Token = LexToken<Terminal>;

#[derive(Debug, Display, Detail)]
#[diag(code_offset = 400)]
pub enum ParseErr {
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
    #[display(fmt = "invalid UTF-8 character '{input}'")]
    InvalidControlUTF8Char {
        input: char,
        from: Position,
        to: Position,
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
    #[display(fmt = "key '{key}' defined multiple times")]
    RedefinedKey { key: String },
}

impl ParseErr {
    pub fn invalid_escape<T>(r: &mut dyn CharReader) -> Result<T, Error> {
        let p1 = r.position();
        let err = match r.next_char()? {
            Some(_) => {
                let p2 = r.position();
                parse_diag!(ParseErr::InvalidEscape {
                    from: p1,
                    to: p2
                }, r, {
                    p1, p2 => "invalid escape",
                })
            }
            None => unreachable!(), //Error UnexpectedEoi should be catch earlier in invalid_input_one
        };
        Err(err)
    }

    pub fn invalid_input<T>(r: &mut dyn CharReader) -> Result<T, Error> {
        let p1 = r.position();
        let current = r.peek_char(0)?.unwrap();
        let err = match r.next_char()? {
            Some(_c) => {
                let p2 = r.position();
                parse_diag!(ParseErr::InvalidChar {
                    input: current,
                    from: p1,
                    to: p2
                }, r, {
                    p1, p2 => "invalid character",
                })
            }
            None => parse_diag!(ParseErr::UnexpectedEoi {
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
            Some(_c) => unreachable!(), //There is only one possibility in method lex: unexpected end of input
            None => parse_diag!(ParseErr::UnexpectedEoiOne {
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
        let err = match (r.peek_char(0)?, r.next_char()?) {
            (Some(current), Some(_c)) => {
                let p2 = r.position();
                parse_diag!(ParseErr::InvalidCharMany {
                    input: current,
                    from: p1,
                    to: p2,
                    expected,
                }, r, {
                    p1, p2 => "invalid character",
                })
            }
            _ => parse_diag!(ParseErr::UnexpectedEoiMany {
                pos: p1,
                expected,
            }, r, {
                p1, p1 => "unexpected end of input",
            }),
        };
        Err(err)
    }

    pub fn invalid_control_utf8_input<T>(r: &mut dyn CharReader) -> Result<T, Error> {
        let p1 = r.position();
        let err = match (r.peek_char(0)?, r.next_char()?) {
            (Some(current), Some(_c)) => {
                let p2 = r.position();
                parse_diag!(ParseErr::InvalidControlUTF8Char {
                    input: current,
                    from: p1,
                    to: p2
                }, r, {
                    p1, p2 => "invalid control UTF-8 character",
                })
            }
            _ => unreachable!(), //Error is caught in method lex and UnexpectedEoiOne in returned
        };
        Err(err)
    }

    pub fn unexpected_eoi_str<T>(r: &mut dyn CharReader, expected: String) -> Result<T, Error> {
        let pos = r.position();
        Err(parse_diag!(ParseErr::UnexpectedEoiOneString {
            pos,
            expected,
        }, r, {
            pos, pos => "unexpected end of input",
        }))
    }

    pub fn unexpected_token<T>(token: Token, r: &mut dyn CharReader) -> Result<T, Error> {
        Err(parse_diag!(ParseErr::UnexpectedToken { token }, r, {
            token.from(), token.to() => "unexpected token"
        }))
    }

    pub fn unexpected_token_one<T>(
        token: Token,
        expected: Terminal,
        r: &mut dyn CharReader,
    ) -> Result<T, Error> {
        Err(
            parse_diag!(ParseErr::UnexpectedTokenOne { token, expected }, r, {
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
            parse_diag!(ParseErr::UnexpectedTokenMany { token, expected }, r, {
                token.from(), token.to() => "unexpected token"
            }),
        )
    }

    pub fn key_redefined<T>(
        r: &mut dyn CharReader,
        redefined: Span,
        prev: Span,
        key: &str,
    ) -> Result<T, Error> {
        Err(
            parse_diag!(ParseErr::RedefinedKey{key: key.to_string()}, r, {
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

        return ParseErr::key_redefined(r, redefined, prev, &key);
    }
}

#[derive(Debug, Display, PartialEq, Eq, Clone, Copy)]
pub enum Terminal {
    #[display(fmt = "END")]
    End,
    #[display(fmt = "'{{'")]
    BraceLeft,
    #[display(fmt = "'}}'")]
    BraceRight,
    #[display(fmt = "'['")]
    BracketLeft,
    #[display(fmt = "']'")]
    BracketRight,
    #[display(fmt = "':'")]
    Colon,
    #[display(fmt = "','")]
    Comma,
    #[display(fmt = "LITERAL")]
    Literal,
    #[display(fmt = "INT")]
    Integer,
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

fn is_hex_char(ch: char) -> bool {
    ('A' <= ch && ch <= 'F') || ('a' <= ch && ch <= 'f') || ('0' <= ch && ch <= '9')
}

#[derive(Debug)]
pub struct Parser {
    token_queue: VecDeque<Token>,
    buf: String,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            token_queue: VecDeque::new(),
            buf: String::new(),
        }
    }

    fn lex(&mut self, r: &mut dyn CharReader) -> Result<Token, Error> {
        fn process_scientific_notation(
            r: &mut dyn CharReader,
            p1: Position,
        ) -> Result<Token, Error> {
            r.next_char()?;
            match r.peek_char(0)? {
                Some('-') | Some('+') => {
                    r.skip_chars(1)?;
                    let mut has_digits = false;
                    r.skip_while(&mut |c| {
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
                        ParseErr::invalid_input_many(
                            r,
                            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
                        )
                    }
                }
                Some(c) if c.is_digit(10) => {
                    r.skip_chars(1)?;
                    r.skip_while(&mut |c| c.is_digit(10))?;
                    let p2 = r.position();
                    Ok(Token::new(Terminal::Float, p1, p2))
                }
                _ => ParseErr::invalid_input_many(
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

        r.skip_whitespace()?;

        match r.peek_char(0)? {
            None => Ok(Token::new(Terminal::End, r.position(), r.position())),
            Some(',') => consume(r, 1, Terminal::Comma),
            Some('[') => consume(r, 1, Terminal::BracketLeft),
            Some(']') => consume(r, 1, Terminal::BracketRight),
            Some('{') => consume(r, 1, Terminal::BraceLeft),
            Some('}') => consume(r, 1, Terminal::BraceRight),
            Some(':') => consume(r, 1, Terminal::Colon),
            Some('n') => {
                if r.match_str_term("null", &mut is_non_alphanumeric)? {
                    consume(r, 4, Terminal::Null)
                } else {
                    ParseErr::invalid_input(r)
                }
            }
            Some('t') => {
                if r.match_str_term("true", &mut is_non_alphanumeric)? {
                    consume(r, 4, Terminal::True)
                } else {
                    ParseErr::invalid_input(r)
                }
            }
            Some('f') => {
                if r.match_str_term("false", &mut is_non_alphanumeric)? {
                    consume(r, 5, Terminal::False)
                } else {
                    ParseErr::invalid_input(r)
                }
            }
            Some(c) if c.is_digit(10) || c == '+' || c == '-' => {
                let p1 = r.position();
                r.next_char()?;
                r.skip_while(&mut |c| c.is_digit(10))?;
                match r.peek_char(0)? {
                    Some('.') => {
                        r.next_char()?;
                        r.skip_while(&mut |c| c.is_digit(10))?;
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
            Some('\"') => {
                let p1 = r.position();
                while let Some(k) = r.next_char()? {
                    if k == '\\' {
                        r.next_char()?;
                    } else if k == '\"' {
                        break;
                    }
                }
                if r.eof() {
                    ParseErr::invalid_input_one(r, '\"')
                } else {
                    r.next_char()?;
                    let p2 = r.position();
                    Ok(Token::new(Terminal::Literal, p1, p2))
                }
            }
            Some(_) => ParseErr::invalid_input(r),
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
            ParseErr::unexpected_token_one(t, term, r)
        }
    }

    pub fn parse(&mut self, r: &mut dyn CharReader) -> Result<NodeRef, Error> {
        self.token_queue.clear();
        self.parse_value(r)
    }

    fn parse_value(&mut self, r: &mut dyn CharReader) -> Result<NodeRef, Error> {
        let t = self.next_token(r)?;
        match t.term() {
            Terminal::BraceLeft => {
                self.push_token(t);
                self.parse_object(r)
            }
            Terminal::BracketLeft => {
                self.push_token(t);
                self.parse_array(r)
            }
            Terminal::Null => Ok(NodeRef::null().with_span(t.span())),
            Terminal::True => Ok(NodeRef::boolean(true).with_span(t.span())),
            Terminal::False => Ok(NodeRef::boolean(false).with_span(t.span())),
            Terminal::Integer => {
                let s = r.slice_pos(t.from(), t.to())?;
                let num: i64 = s.parse().map_err(|err| ParseErr::InvalidIntegerLiteral {
                    err,
                    from: t.from(),
                    to: t.to(),
                })?;
                Ok(NodeRef::integer(num).with_span(t.span()))
            }
            Terminal::Float => {
                let s = r.slice_pos(t.from(), t.to())?;
                let num: f64 = s.parse().map_err(|err| ParseErr::InvalidFloatLiteral {
                    err,
                    from: t.from(),
                    to: t.to(),
                })?;
                Ok(NodeRef::float(num).with_span(t.span()))
            }
            Terminal::Literal => {
                self.parse_literal(t, r)?;
                Ok(NodeRef::string(self.buf.clone()).with_span(t.span()))
            }
            _ => ParseErr::unexpected_token_many(
                t,
                vec![
                    Terminal::BraceLeft,
                    Terminal::BracketLeft,
                    Terminal::Null,
                    Terminal::True,
                    Terminal::False,
                    Terminal::Integer,
                    Terminal::Float,
                    Terminal::Literal,
                ],
                r,
            ),
        }
    }

    fn parse_object(&mut self, r: &mut dyn CharReader) -> Result<NodeRef, Error> {
        let p1 = self.expect_token(r, Terminal::BraceLeft)?.from();
        let mut props = Properties::new();
        let mut comma = false;
        let mut literal = true;
        loop {
            let t = self.next_token(r)?;
            match t.term() {
                Terminal::BraceRight if (comma || literal) => {
                    let span = Span {
                        from: p1,
                        to: t.to(),
                    };
                    return Ok(NodeRef::object(props).with_span(span));
                }
                Terminal::Comma if comma => {
                    comma = false;
                }
                Terminal::Literal if !comma => {
                    self.parse_literal(t, r)?;
                    let key = Symbol::from(&self.buf);
                    self.expect_token(r, Terminal::Colon)?;
                    let value = self.parse_value(r)?;
                    if let Some(child) = props.get(&key) {
                        return ParseErr::key_redefined_node(r, t.span(), &child, &key);
                    }
                    props.insert(key, value);
                    comma = true;
                    literal = false;
                }
                _ if !literal && !comma => {
                    return ParseErr::unexpected_token_one(t, Terminal::Literal, r)
                }
                _ => {
                    return ParseErr::unexpected_token_many(
                        t,
                        if comma {
                            vec![Terminal::Comma, Terminal::BraceRight]
                        } else {
                            vec![Terminal::Literal, Terminal::BraceRight]
                        },
                        r,
                    )
                }
            }
        }
    }

    fn parse_array(&mut self, r: &mut dyn CharReader) -> Result<NodeRef, Error> {
        let p1 = self.expect_token(r, Terminal::BracketLeft)?.from();
        let mut elems = Elements::new();
        let mut comma = false;
        let mut bracket_right = true;
        loop {
            let t = self.next_token(r)?;
            match t.term() {
                Terminal::BracketRight if bracket_right => {
                    let span = Span {
                        from: p1,
                        to: t.to(),
                    };
                    return Ok(NodeRef::array(elems).with_span(span));
                }
                Terminal::Comma if comma => {
                    comma = false;
                    bracket_right = false;
                }
                _ if !comma => {
                    self.push_token(t);
                    let value = self.parse_value(r)?;
                    elems.push(value);
                    comma = true;
                    bracket_right = true;
                }
                _ => return ParseErr::unexpected_token(t, r),
            }
        }
    }

    fn parse_literal<'a>(&mut self, t: Token, r: &'a mut dyn CharReader) -> Result<(), Error> {
        r.seek(t.from())?;
        let end_offset = t.to().offset;
        r.skip_chars(1)?;
        let start_offset = r.position().offset;
        self.buf.clear();
        self.buf.reserve(end_offset - start_offset);
        while r.position().offset < end_offset - 1 {
            let c = r.next_char()?.unwrap();
            match c {
                '\\' => {
                    let c = r.next_char()?;
                    match c {
                        Some('\\') => self.buf.push('\\'),
                        Some('\'') => self.buf.push('\''),
                        Some('\"') => self.buf.push('\"'),
                        Some('t') => self.buf.push('\t'),
                        Some('r') => self.buf.push('\r'),
                        Some('n') => self.buf.push('\n'),
                        Some('b') => self.buf.push('\u{0008}'),
                        Some('f') => self.buf.push('\u{000c}'),
                        Some('u') => {
                            let p1 = r.position();
                            let mut val = String::new();
                            for _i in 0..4 {
                                if let Some(c) = r.next_char()? {
                                    if is_hex_char(c) {
                                        val.push(c)
                                    } else {
                                        return ParseErr::invalid_escape(r);
                                    }
                                } else {
                                    unreachable!() // Error UnexpectedEoiOne is returned earlier in lex method
                                }
                            }
                            // Earlier checks in code protect from error in from_str_radix, so no code coverage.
                            // map_err is present because IntErrorKind in ParseIntError is non-exhaustive
                            let num: u32 = u32::from_str_radix(&val, 16).map_err(|err| {
                                ParseErr::InvalidIntegerLiteral {
                                    err,
                                    from: p1,
                                    to: r.position(),
                                }
                            })?;

                            // http://unicode.org/glossary/#unicode_scalar_value
                            if (num <= 0xD7FFu32) || (num >= 0xE000u32 && num <= 0x10FFFFu32) {
                                let unicode_chars = num.to_be_bytes();
                                for c in &unicode_chars {
                                    if *c as char != 0 as char {
                                        self.buf.push(*c as char)
                                    }
                                }
                            } else {
                                return ParseErr::invalid_escape(r);
                            }
                        }
                        _ => return ParseErr::invalid_escape(r),
                    }
                }
                c if c as u32 <= 31 => return ParseErr::invalid_control_utf8_input(r),
                _ => self.buf.push(c),
            }
        }
        self.buf.pop();
        r.seek(t.to())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
