//! Integration tests for logical operations

mod utils;

#[cfg(test)]
mod logical_tests {
    use crate::utils;

    #[test]
    fn it_performs_logical_not_on_zero() {
        utils::assert_input("print !0;", predicates::str::contains("1"));
    }

    #[test]
    fn it_performs_logical_not_on_non_zero_integer() {
        utils::assert_input("print !9;", predicates::str::contains("0"));
    }

    #[test]
    fn it_performs_double_logical_not_on_zero() {
        utils::assert_input("print !!0;", predicates::str::contains("0"));
    }

    #[test]
    fn it_performs_double_logical_not_on_non_zero_integer() {
        utils::assert_input("print !!9;", predicates::str::contains("1"));
    }

    // "When an expression is used in a Boolean context, ... . Otherwise, a string value of the null
    // string shall be treated as false and any other value shall be treated as true."
    #[test]
    fn it_performs_unary_negation_on_empty_string() {
        utils::assert_input("print !\"\";", predicates::str::contains("1"));
    }

    #[test]
    fn it_performs_unary_negation_on_non_empty_string() {
        utils::assert_input("print !\"Hello World\";", predicates::str::contains("0"));
    }

    #[test]
    fn it_performs_unary_negation_on_string_0() {
        utils::assert_input("print !\"0\";", predicates::str::contains("0"));
    }

    #[test]
    fn it_performs_unary_negation_on_string_1() {
        utils::assert_input("print !\"1\";", predicates::str::contains("0"));
    }

    #[test]
    fn it_performs_logical_and_truthy() {
        utils::assert_input("print 1 && 1;", predicates::str::contains("1"));
    }

    #[test]
    fn it_performs_logical_and_falsy_lhs() {
        utils::assert_input("print 0 && 1;", predicates::str::contains("0"));
    }

    #[test]
    fn it_performs_logical_and_falsy_rhs() {
        utils::assert_input("print 1 && 0;", predicates::str::contains("0"));
    }

    #[test]
    fn it_performs_logical_and_with_str() {
        utils::assert_input("print 1 && \"0\";", predicates::str::contains("1"));
    }

    #[test]
    fn it_performs_logical_and_with_empty_str() {
        utils::assert_input("print 1 && \"\";", predicates::str::contains("0"));
    }

    #[test]
    fn it_performs_logical_and_with_expr() {
        utils::assert_input("print 1 && 1 - 1;", predicates::str::contains("0"));
    }
}
