use opath::Expr::*;
use opath::*;

mod alternative {
    use super::*;

    #[test]
    fn booleans() {
        assert_expr!("true or false",
                    Or(
                        Box::new(Boolean(true)),
                        Box::new(Boolean(false))
                        )
                );
    }

    #[test]
    fn booleans_pipe() {
        assert_expr!("true || true",
                        Or(
                            Box::new(Boolean(true)),
                            Box::new(Boolean(true))
                            )
                    );
    }

    #[test]
    fn boolean_integer() {
        assert_expr!("true or 0",
                    Or(
                        Box::new(Boolean(true)),
                        Box::new(Integer(0))
                        )
                );
    }

    #[test]
    fn boolean_integer_pipe() {
        assert_expr!("true || 0",
                    Or(
                        Box::new(Boolean(true)),
                        Box::new(Integer(0))
                        )
                );
    }

    #[test]
    fn booleans_not_ctx() {
        assert_expr!("!(true or false)",
                    Not(Box::new(Or(
                        Box::new(Boolean(true)),
                        Box::new(Boolean(false))
                        )
                    ))
                );
    }

    #[test]
    fn booleans_not_ctx_pipe() {
        assert_expr!("!(true || false)",
                    Not(Box::new(Or(
                        Box::new(Boolean(true)),
                        Box::new(Boolean(false))
                        )
                    ))
                );
    }
}

mod conjunction {
    use super::*;

    #[test]
    fn booleans() {
        assert_expr!("true and true",
                And(
                    Box::new(Boolean(true)),
                    Box::new(Boolean(true))
                    )
            );
    }

    #[test]
    fn booleans_amp() {
        assert_expr!("true && true",
                And(
                    Box::new(Boolean(true)),
                    Box::new(Boolean(true))
                    )
            );
    }

    #[test]
    fn not_ctx() {
        assert_expr!("! (true and true)",
        Not(Box::new(
            And(
                Box::new(Boolean(true)),
                Box::new(Boolean(true))
                )
        ))
        );
    }

    #[test]
    fn not_ctx_amp() {
        assert_expr!("! (true && true)",
        Not(Box::new(
            And(
                Box::new(Boolean(true)),
                Box::new(Boolean(true))
                )
        ))
        );
    }
}


mod negation {
    use super::*;

    #[test]
    fn boolean_exclamation_mark() {
        assert_expr!("!true", Boolean(false));
        assert_expr!("!false", Boolean(true))
    }

    #[test]
    fn boolean() {
        assert_expr!("not true", Boolean(false));
        assert_expr!("not false", Boolean(true))
    }

    #[test]
    fn not_integer_exclamation_mark() {
        assert_expr!("!1", Boolean(false))
    }

    #[test]
    fn integer() {
        assert_expr!("not 1", Boolean(false))
    }

    #[test]
    fn zero_integer_exclamation_mark() {
        assert_expr!("!0", Boolean(true))
    }

    #[test]
    fn zero_integer() {
        assert_expr!("not 0", Boolean(true))
    }

    #[test]
    fn float_exclamation_mark() {
        assert_expr!("!1.1", Boolean(false))
    }

    #[test]
    fn float() {
        assert_expr!("not 1.1", Boolean(false))
    }

    #[test]
    fn zero_float_exclamation_mark() {
        assert_expr!("!0.0", Boolean(true))
    }

    #[test]
    fn zero_float() {
        assert_expr!("not 0.0", Boolean(true))
    }
}
