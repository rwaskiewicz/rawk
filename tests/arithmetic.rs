//! Integration tests for arithmetic operations

mod utils;

#[cfg(test)]
mod arithmetic_tests {
    use crate::utils;

    #[test]
    fn it_sums_two_integers() {
        utils::assert_input("print 1+2;", predicates::str::contains("3"));
    }

    #[test]
    fn it_subtracts_two_integers() {
        utils::assert_input("print 1-2;", predicates::str::contains("-1"));
    }

    #[test]
    fn it_multiplies_two_integers() {
        utils::assert_input("print 3*2;", predicates::str::contains("6"));
    }

    #[test]
    fn it_divides_two_integers() {
        utils::assert_input("print 6/2;", predicates::str::contains("3"));
    }

    #[test]
    fn it_finds_the_modulo_of_two_integers() {
        utils::assert_input("print 3%2;", predicates::str::contains("1"));
    }

    #[test]
    fn it_finds_the_calculates_the_power_of_two_integers() {
        utils::assert_input("print 3^2;", predicates::str::contains("9"));
    }

    #[test]
    fn it_finds_the_calculates_the_power_of_three_integers() {
        utils::assert_input("print 3^2^3;", predicates::str::contains("6561"));
    }

    #[test]
    fn it_negates_a_positive_number_with_unary_minus() {
        utils::assert_input("print 9;", predicates::str::contains("9"))
    }

    #[test]
    fn it_negates_a_negative_number_with_unary_minus() {
        utils::assert_input("print -9;", predicates::str::contains("-9"))
    }

    #[test]
    fn it_does_not_negate_zero_with_unary_minus() {
        // echo '-0' | awk '{print -0}' yields 0 BUT
        // echo '-0' | awk '{print -$1}' yields -0
        utils::assert_input("print -0;", predicates::str::contains("0"))
    }

    #[test]
    fn it_keeps_a_negative_number_as_such_with_unary_plus() {
        utils::assert_input("print -+9;", predicates::str::contains("-9"))
    }

    #[test]
    fn it_keeps_a_negative_number_as_such_with_unary_plus_flipped() {
        utils::assert_input("print +-9;", predicates::str::contains("-9"))
    }

    #[test]
    fn it_does_not_alter_zero_with_unary_plus() {
        utils::assert_input("print +0;", predicates::str::contains("0"))
    }

    #[test]
    fn it_sums_an_integer_and_a_string() {
        utils::assert_input(
            "print 2.14 + \"1Hello\";",
            predicates::str::contains("3.14"),
        )
    }

    #[test]
    fn it_sums_a_float_and_a_string() {
        utils::assert_input(
            "print 2.14 + \"1.24Hello\";",
            predicates::str::contains("3.38"),
        )
    }

    #[test]
    fn it_sums_a_string_and_an_integer() {
        utils::assert_input("print \"02Hello\" + 2;", predicates::str::contains("4"))
    }

    #[test]
    fn it_sums_a_string_and_a_float() {
        utils::assert_input(
            "print \"5.55Hello\" + 1.21;",
            predicates::str::contains("6.76"),
        )
    }

    #[test]
    fn it_subtracts_an_integer_and_a_string() {
        utils::assert_input(
            "print 2.14 - \"1Hello\";",
            predicates::str::contains("1.14"),
        )
    }

    #[test]
    fn it_subtracts_a_float_and_a_string() {
        utils::assert_input(
            "print 2.14 - \"1.24Hello\";",
            predicates::str::contains(".9"),
        )
    }

    #[test]
    fn it_subtracts_a_string_and_an_integer() {
        utils::assert_input("print \"02Hello\" - 2;", predicates::str::contains("0"))
    }

    #[test]
    fn it_subtracts_a_string_and_a_float() {
        utils::assert_input(
            "print \"5.55Hello\" - 1.21;",
            predicates::str::contains("4.34"),
        )
    }

    #[test]
    fn it_multiplies_an_integer_and_a_string() {
        utils::assert_input(
            "print 2.14 * \"1Hello\";",
            predicates::str::contains("2.14"),
        )
    }

    #[test]
    fn it_multiplies_a_float_and_a_string() {
        utils::assert_input(
            "print 2.14 * \"1.24Hello\";",
            predicates::str::contains("2.6536"),
        )
    }

    #[test]
    fn it_multiplies_a_string_and_an_integer() {
        utils::assert_input("print \"02Hello\" * 2;", predicates::str::contains("4"))
    }

    #[test]
    fn it_multiplies_a_string_and_a_float() {
        utils::assert_input(
            "print \"5.55Hello\" * 1.21;",
            predicates::str::contains("6.7155"),
        )
    }

    #[test]
    fn it_divides_an_integer_and_a_string() {
        utils::assert_input(
            "print 2.14 / \"1Hello\";",
            predicates::str::contains("2.14"),
        )
    }

    #[test]
    fn it_divides_a_float_and_a_string() {
        utils::assert_input(
            "print 2.14 / \"1.24Hello\";",
            predicates::str::contains("1.7258"),
        )
    }

    #[test]
    fn it_divides_a_string_and_an_integer() {
        utils::assert_input("print \"02Hello\" / 2;", predicates::str::contains("1"))
    }

    #[test]
    fn it_divides_a_string_and_a_float() {
        utils::assert_input(
            "print \"5.55Hello\" / 1.21;",
            predicates::str::contains("4.5867"),
        )
    }

    #[test]
    fn it_modulos_an_integer_and_a_string() {
        utils::assert_input(
            "print 2.14 % \"1Hello\";",
            predicates::str::contains("0.14"),
        )
    }

    #[test]
    fn it_modulos_a_float_and_a_string() {
        utils::assert_input(
            "print 2.14 % \"1.24Hello\";",
            predicates::str::contains("0.9"),
        )
    }

    #[test]
    fn it_modulos_a_string_and_an_integer() {
        utils::assert_input("print \"02Hello\" % 2;", predicates::str::contains("0"))
    }

    #[test]
    fn it_modulos_a_string_and_a_float() {
        utils::assert_input(
            "print \"5.55Hello\" % 1.21;",
            predicates::str::contains("0.71"),
        )
    }

    #[test]
    fn it_exponentiates_an_integer_and_a_string() {
        utils::assert_input(
            "print 2.14 ^ \"1Hello\";",
            predicates::str::contains("2.14"),
        )
    }

    #[test]
    fn it_exponentiates_a_float_and_a_string() {
        utils::assert_input(
            "print 2.14 ^ \"1.24Hello\";",
            predicates::str::contains("2.568"),
        )
    }

    #[test]
    fn it_exponentiates_a_string_and_an_integer() {
        utils::assert_input("print \"02Hello\" ^ 2;", predicates::str::contains("4"))
    }

    #[test]
    fn it_exponentiates_a_string_and_a_float() {
        utils::assert_input(
            "print \"5.55Hello\" ^ 1.21;",
            predicates::str::contains("7.9541"),
        )
    }
}
