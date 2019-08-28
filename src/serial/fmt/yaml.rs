use super::*;

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
    #[display(fmt = "'.'")]
    Dot,
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
    String, //TODO MC Add details
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
        '\n' => false,
        _ => true
    }
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

        let p1 = r.position();
        if self.line_start {
            r.skip_whitespace_nonl()?;
            self.line_start = false;
            if let Some(c) = r.peek_char(0)? {
                if c != '#' && c != '\n' && c != '\r' {
                    let p2 = r.position();
                    let indent = p2.offset - p1.offset;
                    if self.indent > indent {
                        self.indent = indent;
                        return Ok(Token::new(Terminal::Dedent, p1, p2))
                    } else if self.indent < indent {
                        self.indent = indent;
                        return Ok(Token::new(Terminal::Indent, p1, p2))
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
            Some(':') => consume(r, 1, Terminal::Colon),
            Some('-') => consume(r, 1, Terminal::Dash),
            Some('.') => consume(r, 1, Terminal::Dot),
            Some('?') => consume(r, 1, Terminal::QuestionMark),
            Some('*') => consume(r, 1, Terminal::Asterisk),
            Some('&') => consume(r, 1, Terminal::Ampersand),
            Some('!') => consume(r, 1, Terminal::ExclamationMark),
            Some('|') => consume(r, 1, Terminal::VerticalBar),
            Some('>') => consume(r, 1, Terminal::GraterThan),
            Some('%') => consume(r, 1, Terminal::Percent),
            Some('@') => consume(r, 1, Terminal::At),
            Some(' ') | Some('\t') => consume(r, 1, Terminal::Whitespace),
            Some('`') => consume(r, 1, Terminal::GraveAccent),
            Some('"') => consume(r, 1, Terminal::String),
            Some('\'') => consume(r, 1, Terminal::String),
            Some(_) => {
                let p1 = r.position();
                r.skip_while(&mut is_string)?;
                let p2 = r.position();
                Ok(Token::new(Terminal::String, p1, p2))
            },
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

                for t in $expected {
                    match parser.lex(&mut r){
                        Ok(token) => assert_eq!(token.term(), t),
                        Err(err) => {
                            println!("Cannot get token: {}", err)
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
            let input: &str = "---";

            let terms = vec![
                             Terminal::Dash,
                             Terminal::Dash,
                             Terminal::Dash];
            assert_terms!(input, terms);
        }

        #[test]
        fn document_end_lf() {
            let input: &str = "...";

            let terms = vec![
                Terminal::Dot,
                Terminal::Dot,
                Terminal::Dot];
            assert_terms!(input, terms);
        }

        #[test]
        fn key_value() {
            let input: &str = r#"key: value"#;

            let terms = vec![
                Terminal::String,
                Terminal::Colon,
                Terminal::Whitespace,
                Terminal::String,
                Terminal::End];
            assert_terms!(input, terms);
        }

        #[test]
        fn multiple_key_value() {
            let input: &str = r#"key1: value1
key2: value2"#;

            let terms = vec![
                Terminal::String,
                Terminal::Colon,
                Terminal::Whitespace,
                Terminal::String,
                Terminal::Newline,
                Terminal::String,
                Terminal::Colon,
                Terminal::Whitespace,
                Terminal::String,
                Terminal::End];
            assert_terms!(input, terms);
        }

        #[test]
        fn key_indent_with_tabs_key_value() {
            let input: &str = "key:\n\tkey: value";

            let terms = vec![
                Terminal::String,
                Terminal::Colon,
                Terminal::Newline,
                Terminal::Indent,
                Terminal::String,
                Terminal::Colon,
                Terminal::Whitespace,
                Terminal::String,
                Terminal::End];
            assert_terms!(input, terms);
        }

        fn key_indent_key_value() {
            let input: &str = r#"key:
   key: value"#;

            let terms = vec![
                Terminal::String,
                Terminal::Colon,
                Terminal::Newline,
                Terminal::Indent,
                Terminal::String,
                Terminal::Colon,
                Terminal::Whitespace,
                Terminal::String,
                Terminal::End];
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
        - empty line and \r\n at the end
        */
    }
}