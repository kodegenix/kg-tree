use crate::opath::Expr::*;

#[test]
fn index_ge() {
    assert_expr!("@[@.@index >= 3]",
                Sequence(vec![
                        Current,
                        Index(
                            box Ge(
                                box Sequence(vec![
                                    Current,
                                    Property(box String("@index".into()))
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
                        Index(
                            box EndsWith(
                                box Sequence(vec![
                                    Current,
                                    Property(box String("@key".into()))
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
                            Index(
                            box Or(
                                    box EndsWith(
                                        box Sequence(vec![
                                            Current,
                                            Property(box String("@key".into()))
                                        ]),
                                        box String("name".into())
                                    ),
                                    box Ge(
                                        box Sequence(vec![
                                            Current,
                                            Property(box String("@index".into()))
                                        ]),
                                        box Integer(3)
                                        )
                                )
                            )

                    ]))
}
