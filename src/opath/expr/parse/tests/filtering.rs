use crate::opath::Expr::*;
use crate::opath::Attr;

#[test]
fn index_ge() {
    assert_expr!("@[@.@index >= 3]",
                Sequence(vec![
                        Current,
                        IndexExpr(
                            box Ge(
                                box Sequence(vec![
                                    Current,
                                    Attribute(Attr::Index)
                                ]),
                                box Integer(3)
                            )
                        )
                ]))
}

#[test]
fn key_ends_with() {
    assert_expr!("@[@.@key $= 'name']",
                    Sequence(vec![
                        Current,
                        IndexExpr(
                            box EndsWith(
                                box Sequence(vec![
                                    Current,
                                    Attribute(Attr::Key)
                                ]),
                                box String("name".into())
                            )
                        )
                ]))
}

#[test]
fn key_ends_with_or_index() {
    assert_expr!("@[@.@key $= 'name' or @.@index >= 3]",
                    Sequence(vec![
                            Current,
                            IndexExpr(
                            box Or(
                                    box EndsWith(
                                        box Sequence(vec![
                                            Current,
                                            Attribute(Attr::Key)
                                        ]),
                                        box String("name".into())
                                    ),
                                    box Ge(
                                        box Sequence(vec![
                                            Current,
                                            Attribute(Attr::Index)
                                        ]),
                                        box Integer(3)
                                        )
                                )
                            )

                    ]))
}
