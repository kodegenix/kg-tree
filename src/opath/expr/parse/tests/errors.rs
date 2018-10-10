use super::*;

fn parse_err(err: &ParseDiag) -> &ParseErr {
    if let Some(err) = err.detail().downcast_ref::<ParseErr>() {
        err
    } else {
        panic!("Unexpected type of error")
    }
}

#[test]
fn or_single_pipe() {
    let diag = Opath::parse("true |   true").unwrap_err();
    let err = parse_err(&diag);
        match *err {
            ParseErr::UnexpectedTokenOne { ref expected, ref token } => {
//                assert_eq!(expected, &["|"]);
//                assert_eq!(token.term(), "t");
            }
            _ => panic!("Wrong error kind")
        }

    assert_eq!(diag.quotes().len(), 1);
}

#[test]
fn or_single_pipe_eoi() {
    let diag = Opath::parse("true | ").unwrap_err();
    let err = parse_err(&diag);
    match *err {
        ParseErr::UnexpectedEoi { ref pos } => {
//            assert_eq!(pos, "????");
        }
        _ => panic!("Wrong error kind")
    }

    assert_eq!(diag.quotes().len(), 1);
}

#[test]
fn and_single_amp() {
    let diag = Opath::parse("true &   true").unwrap_err();
    let err = parse_err(&diag);
    match *err {
        ParseErr::UnexpectedTokenOne { ref expected, ref token } => {
//            assert_eq!(expected, &["&"]);
//            assert_eq!(token, "t");
        }
        _ => panic!("Wrong error kind")
    }

    assert_eq!(diag.quotes().len(), 1);
}

#[test]
fn and_single_amp_eoi() {
    let diag = Opath::parse("true & ").unwrap_err();
    let err = parse_err(&diag);
    match *err {
        ParseErr::UnexpectedEoi { ref pos } => {
//            assert_eq!(pos, "????");
        }
        _ => panic!("Wrong error kind")
    }

    assert_eq!(diag.quotes().len(), 1);
}

#[test]
fn no_digits_after_e_non_alphabetic() {
    let diag = Opath::parse("12.5e-;").unwrap_err();
    let err = parse_err(&diag);
    match *err {
        ParseErr::InvalidChar { ref input, ref from, ref to } => {
//            assert_eq!(found, &String::from(";"));
        }
        _ => panic!("Wrong error kind")
    }
    assert_eq!(diag.quotes().len(), 1);
}

#[test]
fn no_digits_after_e_alphabetic() {
    let diag = Opath::parse("12.5e+string").unwrap_err();
    let err = parse_err(&diag);
    match *err {
        ParseErr::InvalidChar { ref input, ref from, ref to } => {
//            assert_eq!(found, &String::from("string"));
        }
        _ => panic!("Wrong error kind")
    }
    assert_eq!(diag.quotes().len(), 1);
}

#[test]
fn no_digits_after_e_eoi() {
    let diag = Opath::parse("12.5e+").unwrap_err();
    let err = parse_err(&diag);
    match *err {
        ParseErr::UnexpectedEoi { ref pos } => {
//            assert_eq!(pos, "????");
        }
        _ => panic!("Wrong error kind")
    }
    assert_eq!(diag.quotes().len(), 1);
}

#[test]
fn scientific_notation_unexp_token() {
    let diag = Opath::parse("12.5e:").unwrap_err();
    let err = parse_err(&diag);
    match *err {
        ParseErr::UnexpectedTokenOne { ref expected, ref token } => {
//            assert_eq!(expected, &["+", "-", "digit"]);
//            assert_eq!(found, ":");
        }
        _ => panic!("Wrong error kind")
    }

    assert_eq!(diag.quotes().len(), 1);
}

#[test]
fn literal_eoi() {
    let diag = Opath::parse("'literal").unwrap_err();
    let err = parse_err(&diag);
    match *err {
        ParseErr::UnexpectedEoi { ref pos } => {
//            assert_eq!(pos, "????");
        }
        _ => panic!("Wrong error kind")
    }

    assert_eq!(diag.quotes().len(), 2);
}

#[test]
fn non_partial_parser_unexpected_token() {
    let mut parser = Parser::new().with_partial(false);

    let mut r = MemCharReader::new("'input' #a".as_bytes());

    let diag = parser.parse(&mut r).unwrap_err();
    let err = parse_err(&diag);

    match *err {
        ParseErr::UnexpectedTokenMany { ref expected, ref token } => {
            let exp: Vec<String> = vec![];
//            assert_eq!(found, "#");
//            assert_eq!(expected, &exp);
        }
        _ => panic!("Wrong error kind")
    }
    assert_eq!(diag.quotes().len(), 1);
}

#[test]
fn dot_notation() {
    let diag = Opath::parse("prop.: a").unwrap_err();
    let err = parse_err(&diag);
    match *err {
        ParseErr::UnexpectedTokenOne { ref expected, ref token } => {
//            assert_eq!(found, ":");
//            assert_eq!(expected, &["*", "**", "(", "'", "\""]);
        }
        _ => panic!("Wrong error kind")
    }
    assert_eq!(diag.quotes().len(), 1);
}

#[test]
fn level_range_close1() {
    let diag = Opath::parse("@.**{,1 @").unwrap_err();
    let err = parse_err(&diag);
    match *err {
        ParseErr::UnexpectedTokenOne { ref expected, ref token } => {
//            assert_eq!(found, "@");
//            assert_eq!(expected, &["}"]);
        }
        _ => panic!("Wrong error kind")
    }
    assert_eq!(diag.quotes().len(), 1);
}

#[test]
fn level_range_close2() {
    let diag = Opath::parse("@.**{1, 2@").unwrap_err();
    let err = parse_err(&diag);
    match *err {
        ParseErr::UnexpectedTokenOne { ref expected, ref token } => {
//            assert_eq!(found, "@");
//            assert_eq!(expected, &["}"]);
        }
        _ => panic!("Wrong error kind")
    }
    assert_eq!(diag.quotes().len(), 1);
}


#[test]
fn level_range() {
    let diag = Opath::parse("@.**{1 @").unwrap_err();
    let err = parse_err(&diag);
    match *err {
        ParseErr::UnexpectedTokenOne { ref expected, ref token } => {
//            assert_eq!(found, "@");
//            assert_eq!(expected, &["}", ","]);
        }
        _ => panic!("Wrong error kind")
    }
    assert_eq!(diag.quotes().len(), 1);
}

#[test]
fn parse_expr() {
    let diag = Opath::parse("^").unwrap_err();
    let err = parse_err(&diag);
    match *err {
        ParseErr::UnexpectedTokenOne { ref expected, ref token } => {
//            assert_eq!(found, "^");
//            assert_eq!(expected, &["$", "@", "-", "!", "not", "(", "[", "**", ":", ".."]);
        }
        _ => panic!("Wrong error kind")
    }
    assert_eq!(diag.quotes().len(), 1);
}

#[test]
fn parse_group_sep() {
    let diag = Opath::parse("(@.prop, @").unwrap_err();
    let err = parse_err(&diag);
    match *err {
        ParseErr::UnclosedGroup(ref separator) => {
//            assert_eq!(separator, "(");
        }
        _ => panic!("Wrong error kind")
    }
    assert_eq!(diag.quotes().len(), 2);
}
