//! Integration tests for arithmetic operations

mod utils;

#[cfg(test)]
mod arithmetic_tests {
    use crate::utils;

    #[test]
    fn it_sums_two_integers() {
        utils::run_rawk(Some("{print 1+2;}"), vec!["-q"], None, Some("3"));
    }

    #[test]
    fn it_subtracts_two_integers() {
        utils::run_rawk(Some("{print 1-2;}"), vec!["-q"], None, Some("-1"));
    }

    #[test]
    fn it_multiplies_two_integers() {
        utils::run_rawk(Some("{print 3*2;}"), vec!["-q"], None, Some("6"));
    }

    #[test]
    fn it_divides_two_integers() {
        utils::run_rawk(Some("{print 6/2;}"), vec!["-q"], None, Some("3"));
    }

    #[test]
    fn it_finds_the_modulo_of_two_integers() {
        utils::run_rawk(Some("{print 3%2;}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn it_calculates_the_power_of_two_integers() {
        utils::run_rawk(Some("{print 3^2;}"), vec!["-q"], None, Some("9"));
    }

    #[test]
    fn it_calculates_the_power_of_three_integers() {
        utils::run_rawk(Some("{print 3^2^3;}"), vec!["-q"], None, Some("6561"));
    }

    #[test]
    fn it_negates_a_negative_number_with_unary_minus() {
        utils::run_rawk(Some("{print -9;}"), vec!["-q"], None, Some("-9"));
    }

    #[test]
    fn it_negates_a_negative_strnum_with_unary_minus() {
        utils::run_rawk(Some("{print -$1;}"), vec![], Some("9"), Some("-9"));
    }

    #[test]
    fn it_does_not_negate_zero_with_unary_minus() {
        // echo '-0' | awk '{print -0}' yields 0
        utils::run_rawk(Some("{print -0;}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn it_negates_zero_with_strnum_unary_minus() {
        // echo '-0' | awk '{print -$1}' yields -0
        utils::run_rawk(Some("{print -$1;}"), vec![], Some("0"), Some("0"));
    }

    #[test]
    fn it_keeps_a_negative_number_as_such_with_unary_plus() {
        utils::run_rawk(Some("{print -+9;}"), vec!["-q"], None, Some("-9"));
    }

    #[test]
    fn it_keeps_a_negative_strnum_as_such_with_unary_plus() {
        utils::run_rawk(Some("{print -+$1;}"), vec![], Some("9"), Some("-9"));
    }

    #[test]
    fn it_keeps_a_negative_number_as_such_with_unary_plus_flipped() {
        utils::run_rawk(Some("{print +-9;}"), vec!["-q"], None, Some("-9"));
    }

    #[test]
    fn it_keeps_a_negative_strnum_as_such_with_unary_plus_flipped() {
        utils::run_rawk(Some("{print +-$1;}"), vec![], Some("9"), Some("-9"));
    }

    #[test]
    fn it_does_not_alter_zero_with_unary_plus() {
        utils::run_rawk(Some("{print +0;}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn it_does_not_alter_zero_with_strnum_unary_plus() {
        utils::run_rawk(Some("{print +$1;}"), vec![], Some("0"), Some("0"));
    }

    #[test]
    fn it_sums_an_integer_and_a_string() {
        utils::run_rawk(
            Some("{print 2.14 + \"1Hello\";}"),
            vec!["-q"],
            None,
            Some("3.14"),
        );
    }

    #[test]
    fn it_sums_a_float_and_a_string() {
        utils::run_rawk(
            Some("{print 2.14 + \"1.24Hello\";}"),
            vec!["-q"],
            None,
            Some("3.38"),
        );
    }

    #[test]
    fn it_sums_a_string_and_an_integer() {
        utils::run_rawk(
            Some("{print \"02Hello\" + 2;}"),
            vec!["-q"],
            None,
            Some("4"),
        );
    }

    #[test]
    fn it_sums_a_string_and_a_float() {
        utils::run_rawk(
            Some("{print \"5.55Hello\" + 1.21;}"),
            vec!["-q"],
            None,
            Some("6.76"),
        );
    }

    #[test]
    fn it_subtracts_an_integer_and_a_string() {
        utils::run_rawk(
            Some("{print 2.14 - \"1Hello\";}"),
            vec!["-q"],
            None,
            Some("1.1400001"),
        );
    }

    #[test]
    fn it_subtracts_a_float_and_a_string() {
        utils::run_rawk(
            Some("{print 2.14 - \"1.24Hello\";}"),
            vec!["-q"],
            None,
            Some("0.9000001"),
        );
    }

    #[test]
    fn it_subtracts_a_string_and_an_integer() {
        utils::run_rawk(
            Some("{print \"02Hello\" - 2;}"),
            vec!["-q"],
            None,
            Some("0"),
        );
    }

    #[test]
    fn it_subtracts_a_string_and_a_float() {
        utils::run_rawk(
            Some("{print \"5.55Hello\" - 1.21;}"),
            vec!["-q"],
            None,
            Some("4.34"),
        );
    }

    #[test]
    fn it_multiplies_an_integer_and_a_string() {
        utils::run_rawk(
            Some("{print 2.14 * \"1Hello\";}"),
            vec!["-q"],
            None,
            Some("2.14"),
        );
    }

    #[test]
    fn it_multiplies_a_float_and_a_string() {
        utils::run_rawk(
            Some("{print 2.14 * \"1.24Hello\";}"),
            vec!["-q"],
            None,
            Some("2.6536002"),
        );
    }

    #[test]
    fn it_multiplies_a_string_and_an_integer() {
        utils::run_rawk(
            Some("{print \"02Hello\" * 2;}"),
            vec!["-q"],
            None,
            Some("4"),
        );
    }

    #[test]
    fn it_multiplies_a_string_and_a_float() {
        utils::run_rawk(
            Some("{print \"5.55Hello\" * 1.21;}"),
            vec!["-q"],
            None,
            Some("6.7155004"),
        );
    }

    #[test]
    fn it_divides_an_integer_and_a_string() {
        utils::run_rawk(
            Some("{print 2.14 / \"1Hello\";}"),
            vec!["-q"],
            None,
            Some("2.14"),
        );
    }

    #[test]
    fn it_divides_a_float_and_a_string() {
        utils::run_rawk(
            Some("{print 2.14 / \"1.24Hello\";}"),
            vec!["-q"],
            None,
            Some("1.7258065"),
        );
    }

    #[test]
    fn it_divides_a_string_and_an_integer() {
        utils::run_rawk(
            Some("{print \"02Hello\" / 2;}"),
            vec!["-q"],
            None,
            Some("1"),
        );
    }

    #[test]
    fn it_divides_a_string_and_a_float() {
        utils::run_rawk(
            Some("{print \"5.55Hello\" / 1.21;}"),
            vec!["-q"],
            None,
            Some("4.5867767"),
        );
    }

    #[test]
    fn it_modulos_an_integer_and_a_string() {
        utils::run_rawk(
            Some("{print 2.14 % \"1Hello\";}"),
            vec!["-q"],
            None,
            Some("0.1400001"),
        );
    }

    #[test]
    fn it_modulos_a_float_and_a_string() {
        utils::run_rawk(
            Some("{print 2.14 % \"1.24Hello\";}"),
            vec!["-q"],
            None,
            Some("0.9000001"),
        );
    }

    #[test]
    fn it_modulos_a_string_and_an_integer() {
        utils::run_rawk(
            Some("{print \"02Hello\" % 2;}"),
            vec!["-q"],
            None,
            Some("0"),
        );
    }

    #[test]
    fn it_modulos_a_string_and_a_float() {
        utils::run_rawk(
            Some("{print \"5.55Hello\" % 1.21;}"),
            vec!["-q"],
            None,
            Some("0.71000004"),
        );
    }

    #[test]
    fn it_exponentiates_an_integer_and_a_string() {
        utils::run_rawk(
            Some("{print 2.14 ^ \"1Hello\";}"),
            vec!["-q"],
            None,
            Some("2.14"),
        );
    }

    #[test]
    fn it_exponentiates_a_float_and_a_string() {
        utils::run_rawk(
            Some("{print 2.14 ^ \"1.24Hello\";}"),
            vec!["-q"],
            None,
            Some("2.5686984"),
        );
    }

    #[test]
    fn it_exponentiates_a_string_and_an_integer() {
        utils::run_rawk(
            Some("{print \"02Hello\" ^ 2;}"),
            vec!["-q"],
            None,
            Some("4"),
        );
    }

    #[test]
    fn it_exponentiates_a_string_and_a_float() {
        utils::run_rawk(
            Some("{print \"5.55Hello\" ^ 1.21;}"),
            vec!["-q"],
            None,
            Some("7.9541645"),
        );
    }

    #[test]
    fn it_concatenates_two_strings() {
        utils::run_rawk(
            Some("{print \"Hello\" \" World!\";}"),
            vec!["-q"],
            None,
            Some("Hello World!"),
        );
    }

    #[test]
    fn it_concatenates_multiple_strings() {
        utils::run_rawk(
            Some("{print \"Hello\" \" World!\" \" \" \"I come in peace!\";}"),
            vec!["-q"],
            None,
            Some("Hello World! I come in peace!"),
        );
    }

    #[test]
    fn it_concatenates_a_number_and_string() {
        utils::run_rawk(
            Some("{print 1 \" World\";}"),
            vec!["-q"],
            None,
            Some("1 World"),
        );
    }

    #[test]
    fn it_concatenates_a_string_and_number() {
        utils::run_rawk(Some("{print \"Hell\" 0;}"), vec!["-q"], None, Some("Hell0"));
    }

    #[test]
    fn it_concatenates_with_binary_str_addition() {
        utils::run_rawk(
            Some("{print 3 + \"Hello\" 4;}"),
            vec!["-q"],
            None,
            Some("34"),
        );
    }

    #[test]
    fn it_concatenates_using_comma_in_print_statement() {
        utils::run_rawk(
            Some("{print \"hello\",\"world\";}"),
            vec!["-q"],
            None,
            Some("hello world"),
        );
    }

    #[test]
    fn it_concatenates_using_comma_with_undefined_variable() {
        utils::run_rawk(
            Some("{print hello,\"world\";}"),
            vec!["-q"],
            None,
            Some(" world"),
        );
    }

    #[test]
    fn it_concatenates_two_numbers_in_print_statement() {
        utils::run_rawk(
            Some("{hello=23; print hello,hello;}"),
            vec!["-q"],
            None,
            Some("23 23"),
        );
    }

    #[test]
    fn it_coerces_a_variable_to_number() {
        utils::run_rawk(
            Some("{foo = \"3\"; print foo * 3;}"),
            vec!["-q"],
            None,
            Some("9"),
        );
    }

    #[test]
    fn it_coerces_a_variable_to_string() {
        utils::run_rawk(
            Some("{foo = 3; print foo \"3\";}"),
            vec!["-q"],
            None,
            Some("33"),
        );
    }

    #[test]
    fn it_ignores_unknown_variables_for_concatenation() {
        utils::run_rawk(
            Some("{zfoo=3; print \"z\" foo\"z\";}"),
            vec!["-q"],
            None,
            Some("zz"),
        );
    }

    #[test]
    fn it_uses_variables_in_concatenation() {
        utils::run_rawk(
            Some("{zfoo=3; print \"z\" zfoo\"z\";}"),
            vec!["-q"],
            None,
            Some("z3z"),
        );
    }

    #[test]
    fn it_supports_add_assign() {
        utils::run_rawk(
            Some("{foo=3; bar+=foo; print bar;}"),
            vec!["-q"],
            None,
            Some("3"),
        );
    }

    #[test]
    fn it_supports_multiple_add_assign() {
        utils::run_rawk(
            Some("{foo=3; foo+=foo+=foo; print foo;}"),
            vec!["-q"],
            None,
            Some("12"),
        );
    }

    #[test]
    fn it_supports_sub_assign() {
        utils::run_rawk(
            Some("{foo=3; bar-=foo; print bar;}"),
            vec!["-q"],
            None,
            Some("-3"),
        );
    }

    #[test]
    fn it_supports_multiple_sub_assign() {
        utils::run_rawk(
            Some("{foo=3; bar=1; foo-=bar-=foo; print foo;}"),
            vec!["-q"],
            None,
            Some("5"),
        );
    }

    #[test]
    fn it_supports_mul_assign() {
        utils::run_rawk(
            Some("{foo=3; bar=7; bar*=foo; print bar;}"),
            vec!["-q"],
            None,
            Some("21"),
        );
    }

    #[test]
    fn it_supports_multiple_mul_assign() {
        utils::run_rawk(
            Some("{foo=3; foo*=foo*=foo; print foo;}"),
            vec!["-q"],
            None,
            Some("81"),
        );
    }

    #[test]
    fn it_supports_div_assign() {
        utils::run_rawk(
            Some("{foo=3; bar=6; bar/=foo; print bar;}"),
            vec!["-q"],
            None,
            Some("2"),
        );
    }

    #[test]
    fn it_supports_multiple_div_assign() {
        utils::run_rawk(
            Some("{foo=81; bar=9; baz=3; foo/=bar/=baz; print foo;}"),
            vec!["-q"],
            None,
            Some("27"),
        );
    }

    #[test]
    fn it_supports_mod_assign() {
        utils::run_rawk(
            Some("{foo=3; bar=7; bar%=foo; print bar;}"),
            vec!["-q"],
            None,
            Some("1"),
        );
    }

    #[test]
    fn it_supports_multiple_mod_assign() {
        utils::run_rawk(
            Some("{foo=7; bar=5; baz=3; foo%=bar%=baz; print foo;}"),
            vec!["-q"],
            None,
            Some("1"),
        );
    }

    #[test]
    fn it_supports_pow_assign() {
        utils::run_rawk(
            Some("{foo=3; foo^=foo; print foo;}"),
            vec!["-q"],
            None,
            Some("27"),
        );
    }

    #[test]
    fn it_supports_multiple_pow_assign() {
        utils::run_rawk(
            Some("{foo=2; foo^=foo^=foo; print foo;}"),
            vec!["-q"],
            None,
            Some("256"),
        );
    }
}
