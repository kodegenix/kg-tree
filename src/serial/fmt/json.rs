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
            None => parse_diag!(ParseErr::UnexpectedEoi {
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
                parse_diag!(ParseErr::InvalidChar {
                    input: c,
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
            Some(c) => {
                let p2 = r.position();
                parse_diag!(ParseErr::InvalidCharOne {
                    input: c,
                    from: p1,
                    to: p2,
                    expected,
                }, r, {
                    p1, p2 => "invalid character",
                })
            }
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
        let err = match r.next_char()? {
            Some(c) => {
                let p2 = r.position();
                parse_diag!(ParseErr::InvalidCharMany {
                    input: c,
                    from: p1,
                    to: p2,
                    expected,
                }, r, {
                    p1, p2 => "invalid character",
                })
            }
            None => parse_diag!(ParseErr::UnexpectedEoiMany {
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
        Err(parse_diag!(ParseErr::UnexpectedEoiOneString {
            pos,
            expected,
        }, r, {
            pos, pos => "unexpected end of input",
        }))
    }

    #[inline]
    pub fn unexpected_token<T>(token: Token, r: &mut dyn CharReader) -> Result<T, Error> {
        Err(parse_diag!(ParseErr::UnexpectedToken { token }, r, {
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
            parse_diag!(ParseErr::UnexpectedTokenOne { token, expected }, r, {
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
            parse_diag!(ParseErr::UnexpectedTokenMany { token, expected }, r, {
                token.from(), token.to() => "unexpected token"
            }),
        )
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
                        ParseErr::invalid_input_many(
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
                if r.match_str_term("null", &is_non_alphanumeric)? {
                    consume(r, 4, Terminal::Null)
                } else {
                    ParseErr::invalid_input(r)
                }
            }
            Some('t') => {
                if r.match_str_term("true", &is_non_alphanumeric)? {
                    consume(r, 4, Terminal::True)
                } else {
                    ParseErr::invalid_input(r)
                }
            }
            Some('f') => {
                if r.match_str_term("false", &is_non_alphanumeric)? {
                    consume(r, 5, Terminal::False)
                } else {
                    ParseErr::invalid_input(r)
                }
            }
            Some(c) if c.is_digit(10) || c == '+' || c == '-' => {
                let p1 = r.position();
                r.next_char()?;
                r.skip_while(&|c| c.is_digit(10))?;
                match r.peek_char(0)? {
                    Some('.') => {
                        if Some('.') == r.peek_char(1)? {
                            let p2 = r.position();
                            return Ok(Token::new(Terminal::Integer, p1, p2));
                        }
                        r.next_char()?;
                        r.skip_while(&|c| c.is_digit(10))?;
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
        loop {
            let t = self.next_token(r)?;
            match t.term() {
                Terminal::BraceRight => {
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
                    props.insert(key, value);
                    comma = true;
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
        loop {
            let t = self.next_token(r)?;
            match t.term() {
                Terminal::BracketRight => {
                    let span = Span {
                        from: p1,
                        to: t.to(),
                    };
                    return Ok(NodeRef::array(elems).with_span(span));
                }
                Terminal::Comma if comma => {
                    comma = false;
                }
                _ if !comma => {
                    self.push_token(t);
                    let value = self.parse_value(r)?;
                    elems.push(value);
                    comma = true;
                }
                _ => return ParseErr::unexpected_token(t, r),
            }
        }
    }

    fn parse_literal<'a>(&mut self, t: Token, r: &'a mut dyn CharReader) -> Result<(), Error> {
        let s = r.slice_pos(t.from(), t.to())?;
        self.buf.clear();
        self.buf.reserve(s.len());
        let mut chars = s[1..s.len() - 1].chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                let c = chars.next();
                match c {
                    Some('\\') => self.buf.push('\\'),
                    Some('\'') => self.buf.push('\''),
                    Some('\"') => self.buf.push('\"'),
                    Some('t') => self.buf.push('\t'),
                    Some('r') => self.buf.push('\r'),
                    Some('n') => self.buf.push('\n'),
                    _ => return ParseErr::invalid_escape(r),
                }
            } else {
                self.buf.push(c);
            }
        }
        Ok(())
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

}
