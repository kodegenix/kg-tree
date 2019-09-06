use crate::opath::Expr;

#[test]
fn positive_integer() {
    assert_expr!("123", Expr::Integer(123))
}

#[test]
fn negative_integer() {
    assert_expr!("-2", Expr::Integer(-2))
}

#[test]
fn positive_integer_overflow() {
    assert_expr!("18446744073709551616", Expr::Float(18446744073709552000.0))
}

#[test]
fn positive_float_dot() {
    assert_expr!("1.13", Expr::Float(1.13))
}

#[test]
fn positive_float_dot_e() {
    assert_expr!("1.13e-10", Expr::Float(1.13e-10))
}

#[test]
#[should_panic]
fn positive_float_only_dot() {
    assert_expr!(".e10", Expr::Float(1e10))
}

#[test]
fn positive_float_e() {
    assert_expr!("1E-2", Expr::Float(1E-2))
}

#[test]
fn negative_float_e() {
    assert_expr!("-1E-2", Expr::Float(-1E-2))
}

#[test]
fn string_quot() {
    assert_expr!("'string'", Expr::StringEnc(String::from("string")))
}

#[test]
fn string_quot_utf8() {
    assert_expr!("'ąćźę'", Expr::StringEnc(String::from("ąćźę")))
}

#[test]
fn string_double_quot() {
    assert_expr!("\"string\"", Expr::StringEnc(String::from("string")))
}

#[test]
fn boolean_true() {
    assert_expr!("true", Expr::Boolean(true))
}

#[test]
fn boolean_false() {
    assert_expr!("false", Expr::Boolean(false))
}

#[test]
fn null() {
    assert_expr!("null", Expr::Null)
}


