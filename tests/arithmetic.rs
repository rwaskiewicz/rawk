//! Integration tests for arithmetic operations

mod utils;

#[cfg(test)]
mod arithmetic_tests {
    use crate::utils;

    #[test]
    fn it_sums_two_integers() {
        utils::assert_input("1+2", predicates::str::contains("3"));
    }

    #[test]
    fn it_subtracts_two_integers() {
        utils::assert_input("1-2", predicates::str::contains("-1"));
    }

    #[test]
    fn it_multiplies_two_integers() {
        utils::assert_input("3*2", predicates::str::contains("6"));
    }

    #[test]
    fn it_divides_two_integers() {
        utils::assert_input("6/2", predicates::str::contains("3"));
    }

    #[test]
    fn it_finds_the_modulo_of_two_integers() {
        utils::assert_input("3%2", predicates::str::contains("1"));
    }

    #[test]
    fn it_finds_the_calculates_the_power_of_two_integers() {
        utils::assert_input("3^2", predicates::str::contains("9"));
    }

    #[test]
    fn it_finds_the_calculates_the_power_of_three_integers() {
        utils::assert_input("3^2^3", predicates::str::contains("6561"));
    }

    #[test]
    fn it_negates_a_positive_number_with_unary_minus() {
        utils::assert_input("9", predicates::str::contains("9"))
    }

    #[test]
    fn it_negates_a_negative_number_with_unary_minus() {
        utils::assert_input("-9", predicates::str::contains("-9"))
    }

    #[test]
    fn it_does_not_negate_zero_with_unary_minus() {
        // echo '-0' | awk '{print -0}' yields 0 BUT
        // echo '-0' | awk '{print -$1}' yields -0
        utils::assert_input("-0", predicates::str::contains("0"))
    }

    #[test]
    fn it_keeps_a_negative_number_as_such_with_unary_plus() {
        utils::assert_input("-+9", predicates::str::contains("-9"))
    }

    #[test]
    fn it_keeps_a_negative_number_as_such_with_unary_plus_flipped() {
        utils::assert_input("+-9", predicates::str::contains("-9"))
    }

    #[test]
    fn it_does_not_alter_zero_with_unary_plus() {
        utils::assert_input("+0", predicates::str::contains("0"))
    }
}
