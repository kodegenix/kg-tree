use super::*;

mod alternative {
    use super::*;

    #[test]
    fn booleans() {
        assert_bool_op("true or false", true);
    }

    #[test]
    fn booleans_pipe() {
        assert_bool_op("true || true", true);
    }

    #[test]
    fn boolean_integer() {
        assert_bool_op("true or 0", true);
    }

    #[test]
    fn boolean_integer_pipe() {
        assert_bool_op("true || 0", true);
    }

    #[test]
    fn booleans_not_ctx() {
        assert_bool_op("!(true or false)", false);
    }

    #[test]
    fn booleans_not_ctx_pipe() {
        assert_bool_op("!(true || false)", false);
    }
}

mod conjunction {
    use super::*;

    #[test]
    fn booleans() {
        assert_bool_op("true and true", true);
    }

    #[test]
    fn booleans_amp() {
        assert_bool_op("true && true", true);
    }

    #[test]
    fn not_ctx() {
        assert_bool_op("! (true and true)", false);
    }

    #[test]
    fn not_ctx_amp() {
        assert_bool_op("! (true && true)", false);
    }
}

mod negation {
    use super::*;

    #[test]
    fn boolean_exclamation_mark() {
        assert_bool_op("!true", false);
        assert_bool_op("!false", true)
    }

    #[test]
    fn boolean() {
        assert_bool_op("not true", false);
        assert_bool_op("not false", true)
    }

    #[test]
    fn not_integer_exclamation_mark() {
        assert_bool_op("!1", false)
    }

    #[test]
    fn integer() {
        assert_bool_op("not 1", false)
    }

    #[test]
    fn zero_integer_exclamation_mark() {
        assert_bool_op("!0", true)
    }

    #[test]
    fn zero_integer() {
        assert_bool_op("not 0", true)
    }

    #[test]
    fn float_exclamation_mark() {
        assert_bool_op("!1.1", false)
    }

    #[test]
    fn float() {
        assert_bool_op("not 1.1", false)
    }

    #[test]
    fn zero_float_exclamation_mark() {
        assert_bool_op("!0.0", true)
    }

    #[test]
    fn zero_float() {
        assert_bool_op("not 0.0", true)
    }
}
