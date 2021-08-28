//! Integration tests for logical operations

mod utils;

#[cfg(test)]
mod logical_tests {
    use crate::utils;

    #[test]
    fn it_performs_logical_not_on_zero() {
        utils::assert_input("print !0;", "1");
    }

    #[test]
    fn it_performs_logical_not_on_non_zero_integer() {
        utils::assert_input("print !9;", "0");
    }

    #[test]
    fn it_performs_double_logical_not_on_zero() {
        utils::assert_input("print !!0;", "0");
    }

    #[test]
    fn it_performs_double_logical_not_on_non_zero_integer() {
        utils::assert_input("print !!9;", "1");
    }

    // "When an expression is used in a Boolean context, ... . Otherwise, a string value of the null
    // string shall be treated as false and any other value shall be treated as true."
    #[test]
    fn it_performs_unary_negation_on_empty_string() {
        utils::assert_input("print !\"\";", "1");
    }

    #[test]
    fn it_performs_unary_negation_on_non_empty_string() {
        utils::assert_input("print !\"Hello World\";", "0");
    }

    #[test]
    fn it_performs_unary_negation_on_string_0() {
        utils::assert_input("print !\"0\";", "0");
    }

    #[test]
    fn it_performs_unary_negation_on_string_1() {
        utils::assert_input("print !\"1\";", "0");
    }

    #[test]
    fn it_performs_logical_and_truthy_returns_true() {
        utils::assert_input("print 1 && 1;", "1");
    }

    #[test]
    fn it_performs_logical_and_falsy_lhs_returns_false() {
        utils::assert_input("print 0 && 1;", "0");
    }

    #[test]
    fn it_performs_logical_and_falsy_rhs_returns_false() {
        utils::assert_input("print 1 && 0;", "0");
    }

    #[test]
    fn it_performs_logical_and_with_str_returns_true() {
        utils::assert_input("print 1 && \"0\";", "1");
    }

    #[test]
    fn it_performs_logical_and_with_empty_str_returns_false() {
        utils::assert_input("print 1 && \"\";", "0");
    }

    #[test]
    fn it_performs_logical_and_with_expr_returns_false() {
        utils::assert_input("print 1 && 1 - 1;", "0");
    }

    #[test]
    fn it_short_circuits_logical_and_on_assignment() {
        utils::assert_input("foo = 2 && foo = 3; print foo;", "1");
    }

    #[test]
    fn it_returns_the_result_of_chained_logical_ands_that_short_circuit() {
        utils::assert_input("a=1; b=0; print a && b && c=3;", "0");
    }

    #[test]
    fn it_returns_the_result_of_chained_logical_ands_that_are_all_evaluated() {
        // result should be '1', the boolean result of the logical ands rather than one of the values assigned in the subexpr
        utils::assert_input("a=2; b=2; print a && b && c=3;", "1");
    }

    #[test]
    fn it_performs_logical_or_truthy() {
        utils::assert_input("print 1 || 1;", "1");
    }

    #[test]
    fn it_performs_logical_or_falsy_lhs_returns_true() {
        utils::assert_input("print 0 || 1;", "1");
    }

    #[test]
    fn it_performs_logical_or_falsy_rhs_returns_true() {
        utils::assert_input("print 1 || 0;", "1");
    }

    #[test]
    fn it_performs_logical_or_with_str_returns_true() {
        utils::assert_input("print 1 || \"0\";", "1");
    }

    #[test]
    fn it_performs_logical_or_with_empty_str_returns_true() {
        utils::assert_input("print 1 || \"\";", "1");
    }

    #[test]
    fn it_performs_logical_or_with_expr_returns_false() {
        utils::assert_input("print 0 || 1 - 1;", "0");
    }

    #[test]
    fn it_short_circuits_logical_or_on_the_first_truthy_value() {
        utils::assert_input("1 || foo = 2; print foo;", "");
    }

    #[test]
    fn it_short_circuits_logical_or_on_assignment() {
        utils::assert_input("foo = 2 || foo = 3; print foo;", "1");
    }

    #[test]
    fn it_returns_the_result_of_chained_logical_or_that_are_all_evaluated() {
        // result should be '1', the boolean result of the logical ors rather than one of the values assigned in the subexpr
        utils::assert_input("a=0; b=0; print a || b || c=3;", "1");
    }

    #[test]
    fn it_binds_logical_and_logical_or_correctly() {
        utils::assert_input("print 1 && 1 || 0;", "1");
    }

    #[test]
    fn it_falls_back_to_logical_or_when_logical_and_is_falsy() {
        utils::assert_input("print 0 && 0 || 1;", "1");
    }

    #[test]
    fn it_short_circuits_logical_or_with_logical_and_on_the_first_truthy_value() {
        utils::assert_input("1 || 0 && foo = 2; print foo;", "");
    }
}
