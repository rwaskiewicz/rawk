//! Integration tests for logical operations

pub mod utils;

#[cfg(test)]
mod logical_tests {
    use crate::utils;

    #[test]
    fn it_performs_logical_not_on_zero() {
        utils::CodeRunner::init()
            .program("{print !0;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_performs_logical_not_on_non_zero_integer() {
        utils::CodeRunner::init()
            .program("{print !9;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn it_performs_double_logical_not_on_zero() {
        utils::CodeRunner::init()
            .program("{print !!0;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn it_performs_double_logical_not_on_non_zero_integer() {
        utils::CodeRunner::init()
            .program("{print !!9;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_performs_unary_negation_on_empty_string() {
        // From https://pubs.opengroup.org/onlinepubs/7908799/xcu/awk.html:
        // > When an expression is used in a Boolean context, ... . Otherwise, a string value of the
        // > null string shall be treated as false and any other value shall be treated as true."
        utils::CodeRunner::init()
            .program("{print !\"\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_performs_unary_negation_on_non_empty_string() {
        utils::CodeRunner::init()
            .program("{print !\"Hello World\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn it_performs_unary_negation_on_string_0() {
        utils::CodeRunner::init()
            .program("{print !\"0\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn it_performs_unary_negation_on_string_1() {
        utils::CodeRunner::init()
            .program("{print !\"1\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn it_performs_logical_and_truthy_returns_true() {
        utils::CodeRunner::init()
            .program("{print 1 && 1;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_performs_logical_and_falsy_lhs_returns_false() {
        utils::CodeRunner::init()
            .program("{print 0 && 1;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn it_performs_logical_and_falsy_rhs_returns_false() {
        utils::CodeRunner::init()
            .program("{print 1 && 0;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn it_performs_logical_and_with_0_str_returns_true() {
        utils::CodeRunner::init()
            .program("{print 1 && \"0\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_performs_logical_and_with_empty_str_returns_false() {
        utils::CodeRunner::init()
            .program("{print 1 && \"\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn it_performs_logical_and_with_expr_returns_false() {
        utils::CodeRunner::init()
            .program("{print 1 && 1 - 1;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn it_short_circuits_logical_and_on_assignment() {
        utils::CodeRunner::init()
            .program("{foo = 2 && foo = 3; print foo;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_returns_the_result_of_chained_logical_ands_that_short_circuit() {
        utils::CodeRunner::init()
            .program("{a=1; b=0; print a && b && c=3;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn it_returns_the_result_of_chained_logical_ands_that_are_all_evaluated() {
        // result should be '1', the boolean result of the logical ands rather than one of the
        // values assigned in the sub-expression
        utils::CodeRunner::init()
            .program("{a=2; b=2; print a && b && c=3;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_performs_logical_or_truthy() {
        utils::CodeRunner::init()
            .program("{print 1 || 1;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_performs_logical_or_falsy_lhs_returns_true() {
        utils::CodeRunner::init()
            .program("{print 0 || 1;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_performs_logical_or_falsy_rhs_returns_true() {
        utils::CodeRunner::init()
            .program("{print 1 || 0;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_performs_logical_or_with_str_returns_true() {
        utils::CodeRunner::init()
            .program("{print 1 || \"0\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_performs_logical_or_with_empty_str_returns_true() {
        utils::CodeRunner::init()
            .program("{print \"\" || 1;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_performs_logical_or_with_expr_returns_false() {
        utils::CodeRunner::init()
            .program("{print 0 || 1 - 1;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn it_short_circuits_logical_or_on_the_first_truthy_value() {
        utils::CodeRunner::init()
            .program("{1 || foo = 2; print foo;}")
            .cli_options(vec!["-q"])
            .expect_output("")
            .assert()
    }

    #[test]
    fn it_short_circuits_logical_or_on_assignment() {
        utils::CodeRunner::init()
            .program("{foo = 2 || foo = 3; print foo;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_returns_the_result_of_chained_logical_or_that_are_all_evaluated() {
        // result should be '1', the boolean result of the logical ors rather than one of the values
        // assigned in the sub-expression
        utils::CodeRunner::init()
            .program("{a=0; b=0; print a || b || c=3;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_binds_logical_and_logical_or_correctly() {
        utils::CodeRunner::init()
            .program("{print 1 && 1 || 0;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_falls_back_to_logical_or_when_logical_and_is_falsy() {
        utils::CodeRunner::init()
            .program("{print 0 && 0 || 1;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_short_circuits_logical_or_with_logical_and_on_the_first_truthy_value() {
        utils::CodeRunner::init()
            .program("{1 || 0 && foo = 2; print foo;}")
            .cli_options(vec!["-q"])
            .expect_output("")
            .assert()
    }
}
