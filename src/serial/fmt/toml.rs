use super::*;

use kg_display::ListDisplay;

use std::collections::VecDeque;

pub type Error = ParseDiag;

pub type Token = LexToken<Terminal>;


#[derive(Debug, Display, Detail)]
#[diag(code_offset = 1200)]
pub enum ParseErr {
    #[display(fmt = "invalid escape")]
    InvalidEscape {
        from: Position,
        to: Position,
    },
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
    #[display(fmt = "invalid character '{input}', expected one of: {expected}", expected = "ListDisplay(expected)")]
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
    UnexpectedEoi {
        pos: Position,
    },
    #[display(fmt = "unexpected end of input, expected '{expected}'")]
    UnexpectedEoiOne {
        pos: Position,
        expected: char,
    },
    #[display(fmt = "unexpected end of input, expected one of: {expected}", expected = "ListDisplay(expected)")]
    UnexpectedEoiMany {
        pos: Position,
        expected: Vec<char>,
    },
    #[display(fmt = "unexpected end of input, expected \"{expected}\"")]
    UnexpectedEoiOneString {
        pos: Position,
        expected: String,
    },
    #[display(fmt = "unexpected symbol {token}")]
    UnexpectedToken {
        token: Token,
    },
    #[display(fmt = "unexpected symbol {token}, expected {expected}")]
    UnexpectedTokenOne {
        token: Token,
        expected: Terminal,
    },
    #[display(fmt = "unexpected symbol {token}, expected one of: {expected}", expected = "ListDisplay(expected)")]
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
            None => {
                parse_diag!(ParseErr::UnexpectedEoi {
                    pos: p1,
                }, r, {
                    p1, p1 => "unexpected end of input",
                })
            }
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
            None => {
                parse_diag!(ParseErr::UnexpectedEoi {
                    pos: p1,
                }, r, {
                    p1, p1 => "unexpected end of input",
                })
            }
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
            None => {
                parse_diag!(ParseErr::UnexpectedEoiOne {
                    pos: p1,
                    expected,
                }, r, {
                    p1, p1 => "unexpected end of input",
                })
            }
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
            None => {
                parse_diag!(ParseErr::UnexpectedEoiMany {
                    pos: p1,
                    expected,
                }, r, {
                    p1, p1 => "unexpected end of input",
                })
            }
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
    pub fn unexpected_token_one<T>(token: Token, expected: Terminal, r: &mut dyn CharReader) -> Result<T, Error> {
        Err(parse_diag!(ParseErr::UnexpectedTokenOne { token, expected }, r, {
            token.from(), token.to() => "unexpected token"
        }))
    }

    #[inline]
    pub fn unexpected_token_many<T>(token: Token, expected: Vec<Terminal>, r: &mut dyn CharReader) -> Result<T, Error> {
        Err(parse_diag!(ParseErr::UnexpectedTokenMany { token, expected }, r, {
            token.from(), token.to() => "unexpected token"
        }))
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
    #[display(fmt = "KEYLIKE")]
    Keylike,
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

impl LexTerm for Terminal {}


#[derive(Debug)]
pub struct Parser {
    token_queue: VecDeque<Token>,
    buf: String,
}

fn is_keylike(ch: char) -> bool {
    ('A' <= ch && ch <= 'Z')
        || ('a' <= ch && ch <= 'z')
        || ('0' <= ch && ch <= '9')
        || ch == '-'
        || ch == '_'
}

fn is_hex_char(ch: char) -> bool {
    ('A' <= ch && ch <= 'F')
    || ('a' <= ch && ch <= 'f')
    || ('0' <= ch && ch <= '9')
}

fn is_oct_char(ch: char) -> bool {
    ('0' <= ch && ch <= '7')
}

fn is_bin_char(ch: char) -> bool {
    ch == '0' || ch == '1'
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
        fn process_scientific_notation(r: &mut dyn CharReader, p1: Position) -> Result<Token, Error> {
            r.next_char()?;
            match r.peek_char(0)? {
                Some('-') | Some('+') => {
                    r.skip_chars(1)?;
                    let mut has_digits = false;
                    r.skip_while_mut(&mut |c| if c.is_digit(10) {
                        has_digits = true;
                        true
                    } else {
                        false
                    })?;
                    if has_digits {
                        let p2 = r.position();
                        Ok(Token::new(Terminal::Float, p1, p2))
                    } else {
                        ParseErr::invalid_input_many(r, vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
                    }
                }
                Some(c) if c.is_digit(10) => {
                    r.skip_chars(1)?;
                    r.skip_while(&|c| c.is_digit(10))?;
                    let p2 = r.position();
                    Ok(Token::new(Terminal::Float, p1, p2))
                }
                _ => ParseErr::invalid_input_many(r, vec!['-', '+', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9']),
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
                        return ParseErr::invalid_input(r);
                    }
                }
            }
            let p2 = r.position();
            Ok(Token::new(term, p1, p2))
        }

        /// consumes characters after specific prefix
        fn consume_int_prefix(r: &mut dyn CharReader, f: &dyn Fn(char) -> bool, p1: Position) -> Result<Token, Error> {
            if let Some('_') = r.next_char()? {
                // underscore is not allowed between prefix and number
                return ParseErr::invalid_input(r)
            }
            r.skip_while(&|c| f(c) || c == '_')?;
            let p2 = r.position();
            return Ok(Token::new(Terminal::Integer, p1, p2));
        }

        r.skip_until(&|c: char| {
            !(c.is_whitespace() && c != '\n' && c != '\r')
        })?;

        match r.peek_char(0)? {
            None => Ok(Token::new(Terminal::End, r.position(), r.position())),
            Some(',') => consume(r, 1, Terminal::Comma),
            Some('.') => consume(r, 1, Terminal::Period),
            Some('[') => consume(r, 1, Terminal::BracketLeft),
            Some(']') => consume(r, 1, Terminal::BracketRight),
            Some('{') => consume(r, 1, Terminal::BraceLeft),
            Some('}') => consume(r, 1, Terminal::BraceRight),
            Some(':') => consume(r, 1, Terminal::Colon),
            Some('=') => consume(r, 1, Terminal::Equals),
            Some('#') => consume_until_newline(r, Terminal::Comment),
            Some('\n') => consume(r, 1, Terminal::Newline),
            Some('\r') => {
                if let Some('\n') = r.peek_char(1)? {
                    consume(r, 2, Terminal::Newline)
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
            Some('\"') => {
                // handle basic strings
                let p1 = r.position();

                let mut multiline = false;
                if let Some('\"') = r.peek_char(1)? {
                    r.next_char()?;
                    if let Some('\"') = r.next_char()? {
                        multiline = true;
                    } else {
                        return ParseErr::invalid_input_one(r, '\"');
                    }
                }

                while let Some(k) = r.next_char()? {
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
                    ParseErr::invalid_input_one(r, '\"')
                } else {
                    r.next_char()?;
                    let p2 = r.position();
                    Ok(Token::new(Terminal::String {
                        literal: false,
                        multiline,
                    }, p1, p2))
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
                        return ParseErr::invalid_input_one(r, '\'');
                    }
                }

                while let Some(k) = r.next_char()? {
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
                    ParseErr::invalid_input_one(r, '\'')
                } else {
                    r.next_char()?;
                    let p2 = r.position();
                    Ok(Token::new(Terminal::String {
                        literal: true,
                        multiline,
                    }, p1, p2))
                }
            }
            Some(c) if c.is_digit(10) || c == '+' || c == '-' => {
                let p1 = r.position();
                let next = r.next_char()?;
                if c == '0' {
                    match next {
                        Some('x') => return consume_int_prefix(r, &|c| is_hex_char(c), p1),
                        Some('o') => return consume_int_prefix(r, &|c| is_oct_char(c), p1),
                        Some('b') => return consume_int_prefix(r, &|c| is_bin_char(c), p1),
                        _ => {}
                    }
                }
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
            Some(c) if is_keylike(c) => {
                let p1 = r.position();
                while let Some(k) = r.next_char()? {
                    if !is_keylike(k) {
                        break;
                    }
                }

                let p2 = r.position();
                Ok(Token::new(Terminal::Keylike, p1, p2))
            }

            // TODO date types https://github.com/toml-lang/toml#offset-date-time
            Some(_) => {
                ParseErr::invalid_input(r)
            }
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
        unimplemented!()
    }

    fn parse_object(&mut self, r: &mut dyn CharReader) -> Result<NodeRef, Error> {
        unimplemented!()
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
                _ => return ParseErr::unexpected_token(t, r)
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

    mod lex {
        use super::*;

        macro_rules! assert_term {
            ($input: expr, $term: expr) => {
                let mut r = MemCharReader::new(input.as_bytes());
                let mut parser = Parser::new();

                let token = parser.lex(&mut r).unwrap();

                assert_eq!(token.term(), term);
            }
        }

        macro_rules! assert_terms {
            ($input: expr, $expected: expr) => {
                let mut r = MemCharReader::new($input.as_bytes());
                let mut parser = Parser::new();

                for t in $expected {
                    let token = parser.lex(&mut r).expect("Cannot get token!");
                    assert_eq!(token.term(), t);
                }
            }
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

            let detail = err.detail().downcast_ref::<ParseErr>().unwrap();

            if let ParseErr::InvalidChar { .. } = detail {
                // Ok
            } else {
                panic!("InvalidChar Error expected")
            }
        }

        #[test]
        fn comment_lf() {
            let input: &str = "#comment\n";

            let terms = vec![
                Terminal::Comment,
                Terminal::Newline,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn comment_crlf() {
            let input: &str = "#comment\r\n";

            let terms = vec![
                Terminal::Comment,
                Terminal::Newline,
                Terminal::End,
            ];
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
        fn keylike() {
            let input: &str = r#"key
            bare_key
            bare-key"#;

            let terms = vec![
                Terminal::Keylike,
                Terminal::Newline,
                Terminal::Keylike,
                Terminal::Newline,
                Terminal::Keylike,
                Terminal::End,
            ];

            assert_terms!(input, terms);
        }

        #[test]
        fn dotted_key() {
            let input: &str = r#"key.after_dot.a"#;

            let terms = vec![
                Terminal::Keylike,
                Terminal::Period,
                Terminal::Keylike,
                Terminal::Period,
                Terminal::Keylike,
                Terminal::End,
            ];

            assert_terms!(input, terms);
        }

        #[test]
        fn true_token() {
            let input: &str = r#"true"#;

            let terms = vec![
                Terminal::True,
                Terminal::End,
            ];

            assert_terms!(input, terms);
        }

        #[test]
        fn basic_string() {
            let input: &str = r#""some basic string"
            "another basic string""#;

            let terms = vec![
                Terminal::String { literal: false, multiline: false },
                Terminal::Newline,
                Terminal::String { literal: false, multiline: false },
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
                Terminal::String { literal: false, multiline: true },
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
                Terminal::String { literal: false, multiline: true },
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
                Terminal::String { literal: false, multiline: true },
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
                Terminal::String { literal: false, multiline: true },
                Terminal::End,
            ];

            assert_terms!(input, terms);
        }

        #[test]
        fn literal_string() {
            let input: &str = r#"'literal string'"#;

            let terms = vec![
                Terminal::String { literal: true, multiline: false},
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
                Terminal::String { literal: true, multiline: true},
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
                Terminal::String { literal: true, multiline: true},
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

            let terms = vec![
                Terminal::Integer,
                Terminal::End,
            ];
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

            let terms = vec![
                Terminal::Float,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }
    }
}