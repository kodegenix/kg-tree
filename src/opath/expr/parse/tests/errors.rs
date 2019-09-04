use super::*;
use kg_diag::parse::ParseErrorDetail as DiagParseErrorDetail;

fn parse_err(err: &ParseDiag) -> &ParseErrorDetail {
    if let Some(err) = err.detail().downcast_ref::<ParseErrorDetail>() {
        err
    } else {
        panic!("Unexpected type of error")
    }
}

fn parse_error_detail(err: &ParseDiag) -> &DiagParseErrorDetail {
    if let Some(err) = err.detail().downcast_ref::<DiagParseErrorDetail>() {
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
            ParseErrorDetail::InvalidCharOne { .. } => {
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
        ParseErrorDetail::UnexpectedEoiOne { .. } => {
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
        ParseErrorDetail::InvalidCharOne { .. } => {
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
        ParseErrorDetail::UnexpectedEoiOne { .. } => {
//            assert_eq!(pos, "????");
        }
        _ => panic!("Wrong error kind")
    }

    assert_eq!(diag.quotes().len(), 1);
}

#[test]
fn no_digits_after_e_non_alphabetic() {
    let diag = Opath::parse("12.5e-;").unwrap_err();
    let err = parse_error_detail(&diag);
    match *err {
        DiagParseErrorDetail::UnexpectedInput { .. } => {
//            assert_eq!(found, &String::from(";"));
        }
        _ => panic!("Wrong error kind")
    }
    assert_eq!(diag.quotes().len(), 0);
}

#[test]
fn no_digits_after_e_alphabetic() {
    let diag = Opath::parse("12.5e+string").unwrap_err();
    let err = parse_error_detail(&diag);
    match *err {
        DiagParseErrorDetail::UnexpectedInput { .. } => {
//            assert_eq!(found, &String::from("string"));
        }
        _ => panic!("Wrong error kind")
    }
    assert_eq!(diag.quotes().len(), 0);
}

#[test]
fn no_digits_after_e_eof() {
    let diag = Opath::parse("12.5e+").unwrap_err();
    let err = parse_error_detail(&diag);
    match *err {
        DiagParseErrorDetail::UnexpectedEof { .. } => {
//            assert_eq!(pos, "????");
        }
        _ => panic!("Wrong error kind")
    }
    assert_eq!(diag.quotes().len(),0);
}

#[test]
fn scientific_notation_unexp_input() {
    let diag = Opath::parse("12.5e:").unwrap_err();
    let err = parse_error_detail(&diag);
    match *err {
        DiagParseErrorDetail::UnexpectedInput { .. } => {
//            assert_eq!(expected, &["+", "-", "digit"]);
//            assert_eq!(found, ":");
        }
        _ => panic!("Wrong error kind")
    }

    assert_eq!(diag.quotes().len(), 0);
}

#[test]
fn literal_eoi() {
    let diag = Opath::parse("'literal").unwrap_err();
    let err = parse_err(&diag);
    match *err {
        ParseErrorDetail::UnexpectedEoiOne { .. } => {
//            assert_eq!(pos, "????");
        }
        _ => panic!("Wrong error kind")
    }

    assert_eq!(diag.quotes().len(), 1);
}

#[test]
fn non_partial_parser_invalid_char() {
    let mut parser = Parser::new().with_partial(false);

    let mut r = MemCharReader::new("'input' #a".as_bytes());

    let diag = parser.parse(&mut r).unwrap_err();
    let err = parse_err(&diag);

    match *err {
        ParseErrorDetail::InvalidChar { .. } => {
//            let exp: Vec<String> = vec![];
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
        ParseErrorDetail::UnexpectedTokenMany { .. } => {
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
        ParseErrorDetail::UnexpectedTokenOne { .. } => {
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
        ParseErrorDetail::UnexpectedTokenOne { .. } => {
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
        ParseErrorDetail::UnexpectedTokenMany { .. } => {
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
        ParseErrorDetail::UnexpectedTokenMany { .. } => {
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
        ParseErrorDetail::UnclosedGroup(..) => {
//            assert_eq!(separator, "(");
        }
        _ => panic!("Wrong error kind")
    }
    assert_eq!(diag.quotes().len(), 2);
}
