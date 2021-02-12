//! Integration tests for logical operations

mod utils;

#[cfg(test)]
mod logical_tests {
    use crate::utils;

    #[test]
    fn it_performs_logical_not_on_zero() {
        utils::assert_input("!0", predicates::str::contains("1"));
    }

    #[test]
    fn it_performs_logical_not_on_non_zero_integer() {
        utils::assert_input("!9", predicates::str::contains("0"));
    }

    #[test]
    fn it_performs_double_logical_not_on_zero() {
        utils::assert_input("!!0", predicates::str::contains("0"));
    }

    #[test]
    fn it_performs_double_logical_not_on_non_zero_integer() {
        utils::assert_input("!!9", predicates::str::contains("1"));
    }

    // When an expression is used in a Boolean context, ... . Otherwise, a string value of the null
    // string shall be treated as false and any other value shall be treated as true.
    #[test]
    fn it_performs_unary_negation_on_empty_string() {
        utils::assert_input("!\"\"", predicates::str::contains("1"));
    }

    #[test]
    fn it_performs_unary_negation_on_non_empty_string() {
        utils::assert_input("!\"Hello World\"", predicates::str::contains("0"));
    }

    #[test]
    fn it_performs_unary_negation_on_string_0() {
        utils::assert_input("!\"0\"", predicates::str::contains("0"));
    }

    #[test]
    fn it_performs_unary_negation_on_string_1() {
        utils::assert_input("!\"1\"", predicates::str::contains("0"));
    }
}
