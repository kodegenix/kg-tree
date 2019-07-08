use opath::*;
use opath::Expr::*;

#[test]
fn addition() {
    assert_expr!("2 + 3", Add(Box::new(Integer(2)), Box::new(Integer(3))))
}

#[test]
fn subtraction() {
    assert_expr!("2 - 3", Sub(Box::new(Integer(2)), Box::new(Integer(3))))
}

#[test]
fn multiplication() {
    assert_expr!("2 * 3", Mul(Box::new(Integer(2)), Box::new(Integer(3))))
}

#[test]
fn division() {
    assert_expr!("2 / 3", Div(Box::new(Integer(2)), Box::new(Integer(3))))
}

#[test]
fn minus_expr() {
    assert_expr!("-(2 / 3)",
                Neg(Box::new(
                    Div(
                        Box::new(Integer(2)),
                        Box::new(Integer(3))
                        )))
                )
}
