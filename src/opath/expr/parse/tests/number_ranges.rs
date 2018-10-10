use opath::Expr::*;
use opath::NumberRange;
use opath::*;

#[test]
fn _int() {
    assert_expr!(":10", Range(Box::new(
        NumberRange {
            start: None,
            step : None,
            stop: Some(Integer(10))
        }
    )));
}

#[test]
fn int_int() {
    assert_expr!("1:10", Range(Box::new(
        NumberRange {
            start: Some(Integer(1)),
            step : None,
            stop: Some(Integer(10))
        }
    )));
}

#[test]
fn int_int_int() {
    assert_expr!("0:2:10", Range(Box::new(
        NumberRange {
            start: Some(Integer(0)),
            step : Some(Integer(2)),
            stop: Some(Integer(10))
        }
    )));
}

#[test]
fn int_float_int() {
    assert_expr!("0:0.1:10", Range(Box::new(
        NumberRange {
            start: Some(Integer(0)),
            step : Some(Float(0.1)),
            stop: Some(Integer(10))
        }
    )));
}

#[test]
fn int_neg_float_neg_float() {
    assert_expr!("0:-0.1:-0.6", Range(Box::new(
        NumberRange {
            start: Some(Integer(0)),
            step : Some(Float(-0.1)),
            stop: Some(Float(-0.6))
        }
    )));
}

#[test]
fn int_dot_dot_int() {
    assert_expr!("1..10", Range(Box::new(
        NumberRange {
            start: Some(Integer(1)),
            step : None,
            stop: Some(Integer(10))
        }
    )));
}

#[test]
fn dot_dot_int() {
    assert_expr!("..10", Range(Box::new(
        NumberRange {
            start: None,
            step : None,
            stop: Some(Integer(10))
        }
    )));
}

