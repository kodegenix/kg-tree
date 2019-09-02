use crate::opath::*;
use crate::opath::Expr::*;

#[test]
fn range_dot_dot_int_at() {
    assert_expr!("@[..10]",
    Sequence(vec![
        Current,
        Index(box Range(
            box NumberRange {
                start: None,
                step: None,
                stop: Some(Integer(10))
            }
        ))
    ]));
}

#[test]
fn range_dot_dot_int() {
    assert_expr!("[..10]",
    Sequence(vec![
        Current,
        Index(box Range(
            box NumberRange {
                start: None,
                step: None,
                stop: Some(Integer(10))
            }
        ))
    ]));
}

#[test]
fn index_at() {
    assert_expr!("@[5]", Sequence(vec![
        Current,
        Index(box Integer(5))
    ]));
}

#[test]
fn index() {
    assert_expr!("[5]", Sequence(vec![
        Current,
        Index(box Integer(5))
    ]));
}
