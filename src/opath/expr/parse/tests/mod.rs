use super::*;

macro_rules! assert_expr {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                match crate::opath::Opath::parse(*left_val) {
                    Ok(opath) => {
                        let e = opath.expr();
                        assert_eq!(e, right_val)
                    }
                    Err(e) => {
                        panic!("Expression assertion failed: {:?}", e);
                    }
                }
            }
        }
    });
}

mod logical_operators;
mod comp_operators;
mod math_operators;
mod literals;
mod number_ranges;
mod prop_access;
mod operator_precedence;
mod filtering;
mod indexing;
mod errors;
