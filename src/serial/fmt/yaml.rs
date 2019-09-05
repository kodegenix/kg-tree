use super::*;

use kg_display::ListDisplay;

use std::collections::VecDeque;

pub type Error = ParseDiag;

pub type Token = LexToken<Terminal>;

#[derive(Debug, Display, Detail)]
#[diag(code_offset = 1400)]
pub enum ParseErrDetail{
    #[display(fmt = "invalid character '{input}'")]
    InvalidChar {
        input: char,
        from: Position,
        to: Position,
    },
    #[display(fmt = "unexpected end of input")]
    UnexpectedEoi { pos: Position },
    #[display(fmt = "invalid character '{input}', expected '{expected}'")]
    InvalidCharOne {
        input: char,
        from: Position,
        to: Position,
        expected: char,
    },
    #[display(fmt = "unexpected end of input, expected '{expected}'")]
    UnexpectedEoiOne { pos: Position, expected: char },
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
    #[display(
    fmt = "unexpected end of input, expected one of: {expected}",
    expected = "ListDisplay(expected)"
    )]
    UnexpectedEoiMany { pos: Position, expected: Vec<char> },
}

impl ParseErrDetail {
    pub fn invalid_input<T>(r: &mut dyn CharReader) -> Result<T, Error> {
        let p1 = r.position();
        let current = r.peek_char(0)?.unwrap();
        let err = match r.next_char()? {
            Some(_c) => {
                let p2 = r.position();
                parse_diag!(ParseErrDetail::InvalidChar {
                    input: current,
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
        let err = match (r.peek_char(0)?, r.next_char()?) {
            (Some(current), Some(_c)) => {
                let p2 = r.position();
                parse_diag!(ParseErrDetail::InvalidCharMany {
                    input: current,
                    from: p1,
                    to: p2,
                    expected,
                }, r, {
                    p1, p2 => "invalid character",
                })
            }
            _ => parse_diag!(ParseErrDetail::UnexpectedEoiMany {
                pos: p1,
                expected,
            }, r, {
                p1, p1 => "unexpected end of input",
            }),
        };
        Err(err)
    }

    pub fn unexpected_end_of_input<T>(r: &mut dyn CharReader) -> Result<T, Error> {
        let p1 = r.position();
        let err = parse_diag!(ParseErrDetail::UnexpectedEoi {
            pos: p1
        }, r, {
            p1, p1 => "unexpected end of input",
        });
        Err(err)
    }
}

#[derive(Debug, Display, PartialEq, Eq, Clone, Copy)]
pub enum Terminal {
    #[display(fmt = "END")]
    End,
    #[display(fmt = "NEWLINE")]
    Newline,
    #[display(fmt = "WHITESPACE")]
    Whitespace,
    #[display(fmt = "'%'")]
    Percent,
    #[display(fmt = "'-'")]
    Dash,
    #[display(fmt = "':'")]
    Colon,
    #[display(fmt = "INDENT")]
    Indent,
    #[display(fmt = "DEDENT")]
    Dedent,
    #[display(fmt = "'#'")]
    Comment,
    #[display(fmt = "'['")]
    BracketLeft,
    #[display(fmt = "']'")]
    BracketRight,
    #[display(fmt = "'{{'")]
    BraceLeft,
    #[display(fmt = "'}}'")]
    BraceRight,
    #[display(fmt = "','")]
    Comma,
    #[display(fmt = "'---'")]
    DocumentStart,
    #[display(fmt = "'...'")]
    DocumentEnd,
    #[display(fmt = "'&'")]
    Ampersand,
    #[display(fmt = "'*'")]
    Asterisk,
    #[display(fmt = "'?'")]
    QuestionMark,
    #[display(fmt = "'|'")]
    VerticalBar,
    #[display(fmt = "'>'")]
    GraterThan,
    #[display(fmt = "STRING")]
    String {
        escapes: bool
    },
    #[display(fmt = "INT")]
    Integer,
    #[display(fmt = "FLOAT")]
    Float,
    #[display(fmt = "'~'")]
    Null,
    #[display(fmt = "'true'")]
    True,
    #[display(fmt = "'false'")]
    False,
    #[display(fmt = "'!'")]
    ExclamationMark,
    #[display(fmt = "'@'")]
    At,
    #[display(fmt = "'`'")]
    GraveAccent,
}

impl LexTerm for Terminal {}

fn is_blank_or_break(c: char) -> bool {
    is_blank(c) || is_break(c)
}

fn is_blank(c: char) -> bool {
    c == ' ' || c == '\t'
}

fn is_break(c: char) -> bool {
    c == '\n' || c == '\r'
}

fn is_string(c: char) -> bool {
    match c {
        ':' => false,
        '?' => false,
        '\n' => false,
        ' ' => false,
        ',' => false,
        '[' => false,
        ']' => false,
        '{' => false,
        '}' => false,
        _ => true
    }
}

fn is_anchor(c: char) -> bool {
    match c {
        ' ' => false,
        '\t' => false,
        '[' => false,
        ']' => false,
        '{' => false,
        '}' => false,
        ',' => false,
        _ => true
    }
}

fn is_string_with_escapes(c: char) -> bool {
    match c {
        '"' => false,
        _ => true
    }
}

fn is_string_with_apostrophes(c: char) -> bool {
    match c {
        '\'' => false,
        _ => true
    }
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

#[derive(Debug)]
pub struct Parser {
    token_queue: VecDeque<Token>,
    buf: String,
    indent: usize,
    line_start: bool,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            token_queue: VecDeque::new(),
            buf: String::new(),
            indent: 0,
            line_start: true,
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
                        ParseErrDetail::invalid_input_many(
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
            r.skip_while(&mut |c| f(c) || c == '_')?;
            let p2 = r.position();
            return Ok(Token::new(Terminal::Integer, p1, p2));
        }

        fn consume_number(r: &mut dyn CharReader, c: char) -> Result<Token, Error> { // FIXME MC inf and nan must have dot '.' before themselves. Add Inf, INF, Nan, NAN.
            let p1 = r.position();
            let next = r.peek_char(1)?;

            match (c, next) {
                // Check integers prefix notation
                ('0', Some('x')) => return consume_int_prefix(r, &|c| is_hex_char(c), p1),
                ('0', Some('o')) => return consume_int_prefix(r, &|c| is_oct_char(c), p1),
                ('0', Some('b')) => return consume_int_prefix(r, &|c| is_bin_char(c), p1),

                (first, Some('.')) if is_sign(first) => {
                    if r.match_str_term(&format!("{}.inf", first), &mut is_non_alphanumeric)?
                        || r.match_str_term(&format!("{}.Inf", first), &mut is_non_alphanumeric)?
                        || r.match_str_term(&format!("{}.INF", first), &mut is_non_alphanumeric)? {
                        return consume(r, 5, Terminal::Float);
                    }
                }
                _ => {}
            }

            r.next_char()?;
            r.skip_while(&mut |c| c.is_digit(10) || c == '_')?;
            match r.peek_char(0)? {
                Some('.') => {
                    r.next_char()?;
                    r.skip_while(&mut |c| c.is_digit(10) || c == '_')?;
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

        fn consume_string(r: &mut dyn CharReader) -> Result<Token, Error> {
            let p1 = r.position();
            r.skip_while(&mut is_string)?;
            let p2 = r.position();
            Ok(Token::new(Terminal::String { escapes: false }, p1, p2))
        }

        //Used with indicators: '?', ':'.
        fn consume_string_starting_with_indicator(r: &mut dyn CharReader, t: Terminal) -> Result<Token, Error> {
            if let Some(nc) = r.peek_char(1)? {
                if is_blank_or_break(nc) {
                    return consume(r, 1, t);
                } else {
                    let p1 = r.position();
                    r.next_char()?;
                    r.skip_while(&mut is_string)?;
                    let p2 = r.position();
                    return Ok(Token::new(Terminal::String { escapes: false }, p1, p2));
                }
            } else {
                return consume(r, 1, t);
            }
        }

        fn consume_surrounded_string(r: &mut dyn CharReader, c: char, t: Terminal, f: &mut dyn FnMut(char) -> bool) -> Result<Token, Error> {
            let p1 = r.position();
            r.next_char()?;
            if r.eof() {
                return ParseErrDetail::invalid_input_one(r, c);
            }
            r.skip_while(f)?;
            if r.eof() {
                return ParseErrDetail::invalid_input_one(r, c);
            }
            r.next_char()?;
            let p2 = r.position();
            Ok(Token::new(t, p1, p2))
        }

        fn consume_anchor(r: &mut dyn CharReader, t: Terminal) -> Result<Token, Error> {
            if let Some(nc) = r.peek_char(1)? {
                if is_blank_or_break(nc) {
                    r.next_char()?;
                    ParseErrDetail::invalid_input(r)
                } else {
                    let p1 = r.position();
                    r.next_char()?;
                    r.skip_while(&mut is_anchor)?;
                    let p2 = r.position();
                    return Ok(Token::new(t, p1, p2));
                }
            } else {
                r.next_char()?;
                ParseErrDetail::unexpected_end_of_input(r)
            }
        }

        fn consume_reserved(r: &mut dyn CharReader, t: Terminal) -> Result<Token, Error> {
            if let Some(nc) = r.peek_char(1)? {
                if is_blank_or_break(nc) {
                    return consume(r, 1, t);
                } else {
                    ParseErrDetail::invalid_input_many(r, vec!['\n', '\r', ' ', '\t'])
                }
            } else {
                return consume(r, 1, t);
            }
        }

        let p1 = r.position();
        if self.line_start {
            if let Some('%') = r.peek_char(0)? {
                return consume_until_newline(r, Terminal::Percent);
            }
            r.skip_until(&mut |c: char| c != ' ')?;
            self.line_start = false;
            if let Some(c) = r.peek_char(0)? {
                if c.is_whitespace() && c != '\n' && c != '\r' {
                    return ParseErrDetail::invalid_input(r);
                }
                if c == '\r' {
                    if let Some(nc) = r.next_char()? {
                        if nc != '\n' {
                            return ParseErrDetail::invalid_input_one(r, '\n');
                        }
                    }
                } else if c != '#' && c != '\n' {
                    let p2 = r.position();
                    let indent = p2.offset - p1.offset;
                    if self.indent > indent {
                        self.indent = indent;
                        return Ok(Token::new(Terminal::Dedent, p1, p2));
                    } else if self.indent < indent {
                        self.indent = indent;
                        return Ok(Token::new(Terminal::Indent, p1, p2));
                    }
                }
            }
        }

        match r.peek_char(0)? {
            None => Ok(Token::new(Terminal::End, r.position(), r.position())),
            Some('#') => consume_until_newline(r, Terminal::Comment),
            Some(',') => consume(r, 1, Terminal::Comma),
            Some('[') => consume(r, 1, Terminal::BracketLeft),
            Some(']') => consume(r, 1, Terminal::BracketRight),
            Some('{') => consume(r, 1, Terminal::BraceLeft),
            Some('}') => consume(r, 1, Terminal::BraceRight),
            Some('!') => consume(r, 1, Terminal::ExclamationMark),
            Some('|') => consume(r, 1, Terminal::VerticalBar),
            Some('>') => consume(r, 1, Terminal::GraterThan),
            Some('@') => consume_reserved(r, Terminal::At),
            Some('`') => consume_reserved(r, Terminal::GraveAccent),
            Some(' ') | Some('\t') => consume(r, 1, Terminal::Whitespace),
            Some('\n') => {
                self.line_start = true;
                consume(r, 1, Terminal::Newline)
            }
            Some('\r') => {
                if let Some('\n') = r.peek_char(1)? {
                    self.line_start = true;
                    consume(r, 2, Terminal::Newline)
                } else {
                    ParseErrDetail::invalid_input(r)
                }
            }
            Some('&') => consume_anchor(r, Terminal::Ampersand),
            Some('*') => consume_anchor(r, Terminal::Asterisk),
            Some(':') => consume_string_starting_with_indicator(r, Terminal::Colon),
            Some('?') => consume_string_starting_with_indicator(r, Terminal::QuestionMark),
            Some(c) if c == '-' => {
                if r.match_str("---")? && r.position().column == 0 {
                    if let Some(c) = r.peek_char(3)? {
                        if is_break(c) {
                            return consume(r, 3, Terminal::DocumentStart);
                        } else {
                            r.skip_chars(3)?;
                            ParseErrDetail::invalid_input_many(r, vec!['\n', '\r'])
                        }
                    } else {
                        return consume(r, 3, Terminal::DocumentStart);
                    }
                } else {
                    if let Some(nc) = r.peek_char(1)? {
                        if is_blank_or_break(nc) {
                            return consume(r, 1, Terminal::Dash);
                        } else {
                            if nc.is_digit(10) || nc == '.' {
                                return consume_number(r, c);
                            } else {
                                return consume_string(r);
                            }
                        }
                    } else {
                        return consume(r, 1, Terminal::Dash);
                    }
                }
            }
            Some(c) if c == '.' => {
                if r.match_str("...")? && r.position().column == 0 {
                    if let Some(c) = r.peek_char(3)? {
                        if is_break(c) {
                            return consume(r, 3, Terminal::DocumentEnd);
                        } else {
                            r.skip_chars(3)?;
                            ParseErrDetail::invalid_input_many(r, vec!['\n', '\r'])
                        }
                    } else {
                        return consume(r, 3, Terminal::DocumentEnd);
                    }
                } else if r.match_str(".inf")?
                    || r.match_str(".Inf")?
                    || r.match_str(".INF")?
                    || r.match_str(".nan")?
                    || r.match_str(".NaN")?
                    || r.match_str(".NAN")? {
                    return consume(r, 4, Terminal::Float);
                } else {
                    if let Some(nc) = r.peek_char(1)? {
                        if nc.is_digit(10) {
                            unimplemented!() //Here should be: return consume_number(r, c);
                            //TODO MC Add handling .1 numbers in consume_number(r, c)
                        } else {
                            return consume_string(r);
                        }
                    } else {
                        return consume_string(r);
                    }
                }
            },
            Some('t') | Some('T') => {
                if r.match_str("true")? || r.match_str("True")? || r.match_str("TRUE")? {
                    return consume(r, 4, Terminal::True);
                } else {
                    return consume_string(r);
                }
            }
            Some('f') | Some('F') => {
                if r.match_str("false")? || r.match_str("False")? || r.match_str("FALSE")? {
                    return consume(r, 5, Terminal::False);
                } else {
                    return consume_string(r);
                }
            }
            Some('~') => consume(r, 1, Terminal::Null),
            Some('n') | Some('N') => {
                if r.match_str("null")? || r.match_str("Null")? || r.match_str("NULL")? {
                    return consume(r, 4, Terminal::Null);
                } else {
                    return consume_string(r);
                }
            }
            Some('"') => {
                return consume_surrounded_string(r, '"', Terminal::String { escapes: true }, &mut is_string_with_escapes);
            },
            Some('\'') => {
                return consume_surrounded_string(r, '\'', Terminal::String { escapes: false }, &mut is_string_with_apostrophes);
            },
            Some(c) if c.is_digit(10) || c == '+' => consume_number(r, c),
            Some(_) => consume_string(r),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod lex{
        use super::*;
        macro_rules! assert_terms {
            ($input: expr, $expected: expr) => {
                let mut r = MemCharReader::new($input.as_bytes());
                let mut parser = Parser::new();

                for (idx, t) in $expected.iter().enumerate() {
                    match parser.lex(&mut r){
                        Ok(token) => {
                            if &token.term() != t {
                                eprintln!("\nIndex: {}", idx);
                                assert_eq!(&token.term(), t);
                            }
                        }
                        Err(err) => {
                            println!("Cannot get token: {}", err);
                            panic!()
                        }
                    }
                }
            };
        }

        #[test]
        fn terminal_end() {
            let input: &str = "";

            let mut r = MemCharReader::new(input.as_bytes());
            let mut parser = Parser::new();

            let token = parser.lex(&mut r).unwrap();

            assert_eq!(Terminal::End, token.term());
        }

        #[test]
        fn line_feed() {
            let input: &str = "      \n";

            let terms = vec![Terminal::Newline, Terminal::End];
            assert_terms!(input, terms);
        }

        #[test]
        fn carriage_return_line_feed() {
            let input: &str = "      \r\n";

            let terms = vec![Terminal::Newline, Terminal::End];
            assert_terms!(input, terms);
        }

        #[test]
        fn comment() {
            let input: &str = "# comment";

            let terms = vec![Terminal::Comment, Terminal::End];
            assert_terms!(input, terms);
        }

        #[test]
        fn comment_lf() {
            let input: &str = "# comment\n";

            let terms = vec![Terminal::Comment, Terminal::Newline, Terminal::End];
            assert_terms!(input, terms);
        }

        #[test]
        fn comment_crlf() {
            let input: &str = "# comment\r\n";

            let terms = vec![Terminal::Comment, Terminal::Newline, Terminal::End];
            assert_terms!(input, terms);
        }

        #[test]
        fn document_start_lf() {
            let input: &str = "---\n";

            let terms = vec![
                             Terminal::DocumentStart,
                             Terminal::Newline,
                             Terminal::End];
            assert_terms!(input, terms);
        }

        #[test]
        fn document_start() {
            let input: &str = "---";

            let terms = vec![
                Terminal::DocumentStart,
                Terminal::End];
            assert_terms!(input, terms);
        }

        #[test]
        fn document_end_lf() {
            let input: &str = "...\n";

            let terms = vec![
                Terminal::DocumentEnd,
                Terminal::Newline,
                Terminal::End];
            assert_terms!(input, terms);
        }

        #[test]
        fn document_end() {
            let input: &str = "...";

            let terms = vec![
                Terminal::DocumentEnd,
                Terminal::End];
            assert_terms!(input, terms);
        }

        #[test]
        fn string_with_dot_at_the_begining() {
            let input: &str = ".key: value";

            let terms = vec![
                Terminal::String { escapes: false },
                Terminal::Colon,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::End];
            assert_terms!(input, terms);
        }

        #[test]
        fn string_with_colon_at_the_begining() {
            let input: &str = ":string";

            let terms = vec![
                Terminal::String { escapes: false },
                Terminal::End];
            assert_terms!(input, terms);
        }

        #[test]
        fn mapping_key_character() {
            let input: &str = r#"? key: value"#;

            let terms = vec![
                Terminal::QuestionMark,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::Colon,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn string_with_question_mark_at_the_begining() {
            let input: &str = r#"?string"#;

            let terms = vec![
                Terminal::String { escapes: false },
                Terminal::End];
            assert_terms!(input, terms);
        }

        #[test]
        fn key_value() {
            let input: &str = r#"key: value"#;

            let terms = vec![
                Terminal::String { escapes: false },
                Terminal::Colon,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn key_tab_value() {
            let input: &str = "key:\tvalue";

            let terms = vec![
                Terminal::String { escapes: false },
                Terminal::Colon,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn key_value_comment() {
            let input: &str = r#"key: value # comment"#;

            let terms = vec![
                Terminal::String { escapes: false },
                Terminal::Colon,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::Whitespace,
                Terminal::Comment,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn multiple_key_value() {
            let input: &str = r#"key1: value1
key2: value2"#;

            let terms = vec![
                Terminal::String { escapes: false },
                Terminal::Colon,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::Newline,
                Terminal::String { escapes: false },
                Terminal::Colon,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn key_indent_key_value() {
            let input: &str = r#"key:
   key: value"#;

            let terms = vec![
                Terminal::String { escapes: false },
                Terminal::Colon,
                Terminal::Newline,
                Terminal::Indent,
                Terminal::String { escapes: false },
                Terminal::Colon,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn dash_end() {
            let input: &str = r#"-"#;

            let terms = vec![
                Terminal::Dash,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn sequence() {
            let input: &str = r#"- one
- two
- three"#;

            let terms = vec![
                Terminal::Dash,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::Newline,
                Terminal::Dash,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::Newline,
                Terminal::Dash,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn sequence_in_mapping() {
            let input: &str = r#"key1:
   - one
   - two
key2:
   - three
   - four"#;

            let terms = vec![
                Terminal::String { escapes: false },
                Terminal::Colon,
                Terminal::Newline,
                Terminal::Indent,
                Terminal::Dash,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::Newline,
                Terminal::Dash,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::Newline,
                Terminal::Dedent,
                Terminal::String { escapes: false },
                Terminal::Colon,
                Terminal::Newline,
                Terminal::Indent,
                Terminal::Dash,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::Newline,
                Terminal::Dash,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn mapping_in_sequence() {
            let input: &str = r#"-
   key1: value1
-
   key2: value2"#;

            let terms = vec![
                Terminal::Dash,
                Terminal::Newline,
                Terminal::Indent,
                Terminal::String { escapes: false },
                Terminal::Colon,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::Newline,
                Terminal::Dedent,
                Terminal::Dash,
                Terminal::Newline,
                Terminal::Indent,
                Terminal::String { escapes: false },
                Terminal::Colon,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn sequence_in_sequence() {
            let input: &str = r#"- [one, two, three]"#;

            let terms = vec![
                Terminal::Dash,
                Terminal::Whitespace,
                Terminal::BracketLeft,
                Terminal::String { escapes: false },
                Terminal::Comma,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::Comma,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::BracketRight,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn mapping_in_mapping() {
            let input: &str = r#"key: {key: value}"#;

            let terms = vec![
                Terminal::String { escapes: false },
                Terminal::Colon,
                Terminal::Whitespace,
                Terminal::BraceLeft,
                Terminal::String { escapes: false },
                Terminal::Colon,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::BraceRight,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn sequence_with_indentation() {
            let input: &str = r#"[one,
two,
   three]"#;

            let terms = vec![
                Terminal::BracketLeft,
                Terminal::String { escapes: false },
                Terminal::Comma,
                Terminal::Newline,
                Terminal::String { escapes: false },
                Terminal::Comma,
                Terminal::Newline,
                Terminal::Indent,
                Terminal::String { escapes: false },
                Terminal::BracketRight,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn indentation_before_sequence() {
            let input: &str = r#"   [one,
two,
three]"#;

            let terms = vec![
                Terminal::Indent,
                Terminal::BracketLeft,
                Terminal::String { escapes: false },
                Terminal::Comma,
                Terminal::Newline,
                Terminal::Dedent,
                Terminal::String { escapes: false },
                Terminal::Comma,
                Terminal::Newline,
                Terminal::String { escapes: false },
                Terminal::BracketRight,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn string_with_dash_at_the_begining() {
            let input: &str = r#"-key: value"#;

            let terms = vec![
                Terminal::String { escapes: false },
                Terminal::Colon,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
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
6.626e-34
0."#;

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
                Terminal::Newline,
                Terminal::Float,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        //#[test] //TODO MC Uncomment
        fn float_with_dot_at_the_beginning() {
            let input: &str = r#".1"#;

            let terms = vec![
                Terminal::Float,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn bad_float_with_dot_at_the_beginning() {
            let input: &str = r#".a"#;

            let terms = vec![
                Terminal::String { escapes: false },
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn infs_and_nans() {
            let input: &str = r#".inf
.Inf
.INF
.nan
.NaN
.NAN"#;

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
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn bad_infs_and_nans() {
            let input: &str = r#".inF
.INf
.InF
.nAn
.Nan
.nAN"#;

            let terms = vec![
                Terminal::String { escapes: false },
                Terminal::Newline,
                Terminal::String { escapes: false },
                Terminal::Newline,
                Terminal::String { escapes: false },
                Terminal::Newline,
                Terminal::String { escapes: false },
                Terminal::Newline,
                Terminal::String { escapes: false },
                Terminal::Newline,
                Terminal::String { escapes: false },
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn infs_with_plus() {
            let input: &str = r#"+.inf
+.Inf
+.INF"#;

            let terms = vec![
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
        fn infs_with_minus() {
            let input: &str = r#"-.inf
-.Inf
-.INF"#;

            let terms = vec![
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
        fn bools() {
            let input: &str = r#"true
True
TRUE
false
False
FALSE"#;

            let terms = vec![
                Terminal::True,
                Terminal::Newline,
                Terminal::True,
                Terminal::Newline,
                Terminal::True,
                Terminal::Newline,
                Terminal::False,
                Terminal::Newline,
                Terminal::False,
                Terminal::Newline,
                Terminal::False,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn bad_bools() {
            let input: &str = r#"trUe
TruE
TrUE
faLse
FalsE
FALsE"#;

            let terms = vec![
                Terminal::String { escapes: false },
                Terminal::Newline,
                Terminal::String { escapes: false },
                Terminal::Newline,
                Terminal::String { escapes: false },
                Terminal::Newline,
                Terminal::String { escapes: false },
                Terminal::Newline,
                Terminal::String { escapes: false },
                Terminal::Newline,
                Terminal::String { escapes: false },
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn quoted_strings() {
            let input: &str = r#""string""#;

            let terms = vec![
                Terminal::String { escapes: true },
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn empty_quoted_string() {
            let input: &str = r#""""#;

            let terms = vec![
                Terminal::String { escapes: true },
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn quoted_string_with_special_chars() {
            let input: &str = r#""\:?
 ,[]{}#*&!|>%@`'""#;

            let terms = vec![
                Terminal::String { escapes: true },
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn string_with_apostrophes() {
            let input: &str = r#"'string'"#;

            let terms = vec![
                Terminal::String { escapes: false },
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn empty_string_with_apostrophes() {
            let input: &str = r#"''"#;

            let terms = vec![
                Terminal::String { escapes: false },
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn string_with_apostrophes_and_special_chars() {
            let input: &str = r#"'\:?
 ,[]{}#*&!|>%@`"'"#;

            let terms = vec![
                Terminal::String { escapes: false },
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn nulls() {
            let input: &str = r#"null
Null
NULL"#;

            let terms = vec![
                Terminal::Null,
                Terminal::Newline,
                Terminal::Null,
                Terminal::Newline,
                Terminal::Null,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn bad_nulls() {
            let input: &str = r#"nUll
NuLl
NUlL"#;

            let terms = vec![
                Terminal::String { escapes: false },
                Terminal::Newline,
                Terminal::String { escapes: false },
                Terminal::Newline,
                Terminal::String { escapes: false },
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn tilde() {
            let input: &str = r#"~"#;

            let terms = vec![
                Terminal::Null,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn ampersand() {
            let input: &str = r#"&anchor"#;

            let terms = vec![
                Terminal::Ampersand,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn asterisk() {
            let input: &str = r#"*anchor"#;

            let terms = vec![
                Terminal::Asterisk,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn directives() {
            let input: &str = r#"%YAML 1.2"#;

            let terms = vec![
                Terminal::Percent,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn at() {
            let input: &str = r#"@"#;

            let terms = vec![
                Terminal::At,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn at_and_string() {
            let input: &str = r#"@ string"#;

            let terms = vec![
                Terminal::At,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn grave_accent() {
            let input: &str = r#"`"#;

            let terms = vec![
                Terminal::GraveAccent,
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        #[test]
        fn grave_accent_and_string() {
            let input: &str = r#"` string"#;

            let terms = vec![
                Terminal::GraveAccent,
                Terminal::Whitespace,
                Terminal::String { escapes: false },
                Terminal::End,
            ];
            assert_terms!(input, terms);
        }

        //#[test]
        fn token_list() {
            // language=yaml
            let input: &str = r#"---
%:%:bbb: value

receipt: Oz-Ware Purchase Invoice
date: 2007-08-06
customer:
    given:   Dorothy
    family:  Gale

items:
    - part_no:   A4786
      descrip:   Water Bucket (Filled)
      price:     1.47
      quantity:  4

    - part_no:   E1628
      descrip:   High Heeled "Ruby" Slippers
      size:      8
      price:     100.27
      quantity:  1

bill-to: &id001
    street: |
            123 Tornado Alley
            Suite 16
    city:   East Centerville
    state:  KS

ship-to: *id001

specialDelivery: >
    Follow the Yellow Brick
    Road to the Emerald City.
    Pay no attention to the
    man behind the curtain.
..."#;

            let mut r = MemCharReader::new(input.as_bytes());
            let mut parser = Parser::new();

            loop {
                let token = parser.lex(&mut r).unwrap();
                if token.term() == Terminal::End {
                    break
                }
                eprintln!("token = {}", token);
            }
        }
        /*
        TODO MC Tests:
        - number .5
        - number 1.
        - string and \r at the end
        - test char after char for integers
        */
    }
}
