use crate::opath::Expr::*;

#[test]
fn mul_add() {
    assert_expr!("3 * 2 + 2",
                Add(
                    Box::new(Mul(
                            Box::new(Integer(3)),
                            Box::new(Integer(2))
                    )),
                    Box::new(Integer(2))

                )
    )
}

#[test]
fn and_neg() {
    assert_expr!("2 and 2 - 2",
                And(
                    Box::new(Integer(2)),
                    Box::new(Sub(
                            Box::new(Integer(2)),
                            Box::new(Integer(2))
                    )),
                )
    )
}

#[test]
fn neg_mul() {
    assert_expr!("-2 * 2",
                    Mul(
                        Box::new(Integer(-2)),
                        Box::new(Integer(2))
                    )
    )
}

#[test]
fn neg_div() {
    assert_expr!("-2 / 2",
                    Div(
                        Box::new(Integer(-2)),
                        Box::new(Integer(2))
                    )
    )
}


#[test]
fn eq_not() {
    assert_expr!("2 == !2",
                    Eq(
                        Box::new(Integer(2)),
                        Box::new(Boolean(false))
                    )
    )
}

#[test]
fn ne_not() {
    assert_expr!("2 != !2",
                    Ne(
                        Box::new(Integer(2)),
                        Box::new(Boolean(false))
                    )
    )
}

#[test]
fn lt_not() {
    assert_expr!("2 < !2",
                    Lt(
                        Box::new(Integer(2)),
                        Box::new(Boolean(false))
                    )
    )
}

#[test]
fn le_not() {
    assert_expr!("2 <= !2",
                    Le(
                        Box::new(Integer(2)),
                        Box::new(Boolean(false))
                    )
    )
}

#[test]
fn gt_not() {
    assert_expr!("2 > !2",
                    Gt(
                        Box::new(Integer(2)),
                        Box::new(Boolean(false))
                    )
    )
}

#[test]
fn ge_not() {
    assert_expr!("2 >= !2",
                    Ge(
                        Box::new(Integer(2)),
                        Box::new(Boolean(false))
                    )
    )
}


#[test]
fn not_and() {
    assert_expr!("!2 and 2",
                    And(
                        Box::new(Boolean(false)),
                        Box::new(Integer(2)),
                    )
    )
}

#[test]
fn or_and() {
    assert_expr!("!2 or 2",
                    Or(
                        Box::new(Boolean(false)),
                        Box::new(Integer(2)),
                    )
    )
}
