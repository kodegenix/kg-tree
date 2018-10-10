use opath::Expr::*;
use opath::*;
use std::string;

#[test]
fn gt() {
    assert_expr!("2 > 3", Gt(Box::new(Integer(2)), Box::new(Integer(3))))
}

#[test]
fn ge() {
    assert_expr!("2 >= 3", Ge(Box::new(Integer(2)), Box::new(Integer(3))))
}

#[test]
fn lt() {
    assert_expr!("2 < 3", Lt(Box::new(Integer(2)), Box::new(Integer(3))))
}

#[test]
fn le() {
    assert_expr!("2 <= 3", Le(Box::new(Integer(2)), Box::new(Integer(3))))
}

#[test]
fn eq() {
    assert_expr!("2 == 3", Eq(Box::new(Integer(2)), Box::new(Integer(3))))
}

#[test]
fn ne() {
    assert_expr!("2 != 3", Ne(Box::new(Integer(2)), Box::new(Integer(3))))
}

#[test]
fn starts_with() {
    assert_expr!("'aaabbb' ^= 'aa'",
        StartsWith(
            box String(string::String::from("aaabbb")),
            box String(string::String::from("aa")),
        ))
}

#[test]
fn contains() {
    assert_expr!("'aaabbb' *= 'aa'",
        Contains(
            box String(string::String::from("aaabbb")),
            box String(string::String::from("aa")),
        ))
}

#[test]
fn ends_with() {
    assert_expr!("'aaabbb' $= 'aa'",
        EndsWith(
            box String(string::String::from("aaabbb")),
            box String(string::String::from("aa")),
        ))
}
