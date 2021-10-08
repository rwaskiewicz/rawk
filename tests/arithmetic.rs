//! Integration tests for arithmetic operations

mod utils;

#[cfg(test)]
mod arithmetic_tests {
    use crate::utils;

    #[test]
    fn it_sums_two_integers() {
        utils::assert_input("print 1+2;", "3");
    }

    #[test]
    fn it_subtracts_two_integers() {
        utils::assert_input("print 1-2;", "-1");
    }

    #[test]
    fn it_multiplies_two_integers() {
        utils::assert_input("print 3*2;", "6");
    }

    #[test]
    fn it_divides_two_integers() {
        utils::assert_input("print 6/2;", "3");
    }

    #[test]
    fn it_finds_the_modulo_of_two_integers() {
        utils::assert_input("print 3%2;", "1");
    }

    #[test]
    fn it_finds_the_calculates_the_power_of_two_integers() {
        utils::assert_input("print 3^2;", "9");
    }

    #[test]
    fn it_finds_the_calculates_the_power_of_three_integers() {
        utils::assert_input("print 3^2^3;", "6561");
    }

    #[test]
    fn it_negates_a_negative_number_with_unary_minus() {
        utils::assert_input("print -9;", "-9")
    }

    #[test]
    fn it_does_not_negate_zero_with_unary_minus() {
        // echo '-0' | awk '{print -0}' yields 0 BUT
        // echo '-0' | awk '{print -$1}' yields -0
        utils::assert_input("print -0;", "0")
    }

    #[test]
    fn it_keeps_a_negative_number_as_such_with_unary_plus() {
        utils::assert_input("print -+9;", "-9")
    }

    #[test]
    fn it_keeps_a_negative_number_as_such_with_unary_plus_flipped() {
        utils::assert_input("print +-9;", "-9")
    }

    #[test]
    fn it_does_not_alter_zero_with_unary_plus() {
        utils::assert_input("print +0;", "0")
    }

    #[test]
    fn it_sums_an_integer_and_a_string() {
        utils::assert_input("print 2.14 + \"1Hello\";", "3.14")
    }

    #[test]
    fn it_sums_a_float_and_a_string() {
        utils::assert_input("print 2.14 + \"1.24Hello\";", "3.38")
    }

    #[test]
    fn it_sums_a_string_and_an_integer() {
        utils::assert_input("print \"02Hello\" + 2;", "4")
    }

    #[test]
    fn it_sums_a_string_and_a_float() {
        utils::assert_input("print \"5.55Hello\" + 1.21;", "6.76")
    }

    #[test]
    fn it_subtracts_an_integer_and_a_string() {
        utils::assert_input("print 2.14 - \"1Hello\";", "1.1400001")
    }

    #[test]
    fn it_subtracts_a_float_and_a_string() {
        utils::assert_input("print 2.14 - \"1.24Hello\";", "0.9000001")
    }

    #[test]
    fn it_subtracts_a_string_and_an_integer() {
        utils::assert_input("print \"02Hello\" - 2;", "0")
    }

    #[test]
    fn it_subtracts_a_string_and_a_float() {
        utils::assert_input("print \"5.55Hello\" - 1.21;", "4.34")
    }

    #[test]
    fn it_multiplies_an_integer_and_a_string() {
        utils::assert_input("print 2.14 * \"1Hello\";", "2.14")
    }

    #[test]
    fn it_multiplies_a_float_and_a_string() {
        utils::assert_input("print 2.14 * \"1.24Hello\";", "2.6536002")
    }

    #[test]
    fn it_multiplies_a_string_and_an_integer() {
        utils::assert_input("print \"02Hello\" * 2;", "4")
    }

    #[test]
    fn it_multiplies_a_string_and_a_float() {
        utils::assert_input("print \"5.55Hello\" * 1.21;", "6.7155004")
    }

    #[test]
    fn it_divides_an_integer_and_a_string() {
        utils::assert_input("print 2.14 / \"1Hello\";", "2.14")
    }

    #[test]
    fn it_divides_a_float_and_a_string() {
        utils::assert_input("print 2.14 / \"1.24Hello\";", "1.7258065")
    }

    #[test]
    fn it_divides_a_string_and_an_integer() {
        utils::assert_input("print \"02Hello\" / 2;", "1")
    }

    #[test]
    fn it_divides_a_string_and_a_float() {
        utils::assert_input("print \"5.55Hello\" / 1.21;", "4.5867767")
    }

    #[test]
    fn it_modulos_an_integer_and_a_string() {
        utils::assert_input("print 2.14 % \"1Hello\";", "0.1400001")
    }

    #[test]
    fn it_modulos_a_float_and_a_string() {
        utils::assert_input("print 2.14 % \"1.24Hello\";", "0.9000001")
    }

    #[test]
    fn it_modulos_a_string_and_an_integer() {
        utils::assert_input("print \"02Hello\" % 2;", "0")
    }

    #[test]
    fn it_modulos_a_string_and_a_float() {
        utils::assert_input("print \"5.55Hello\" % 1.21;", "0.71000004")
    }

    #[test]
    fn it_exponentiates_an_integer_and_a_string() {
        utils::assert_input("print 2.14 ^ \"1Hello\";", "2.14")
    }

    #[test]
    fn it_exponentiates_a_float_and_a_string() {
        utils::assert_input("print 2.14 ^ \"1.24Hello\";", "2.5686984")
    }

    #[test]
    fn it_exponentiates_a_string_and_an_integer() {
        utils::assert_input("print \"02Hello\" ^ 2;", "4")
    }

    #[test]
    fn it_exponentiates_a_string_and_a_float() {
        utils::assert_input("print \"5.55Hello\" ^ 1.21;", "7.9541645")
    }

    #[test]
    fn it_concatenates_two_strings() {
        utils::assert_input("print \"Hello\" \" World!\";", "Hello World!");
    }

    #[test]
    fn it_concatenates_multiple_strings() {
        utils::assert_input(
            "print \"Hello\" \" World!\" \" \" \"I come in peace!\";",
            "Hello World! I come in peace!",
        );
    }

    #[test]
    fn it_concatenates_a_number_and_string() {
        utils::assert_input("print 1 \" World\";", "1 World");
    }

    #[test]
    fn it_concatenates_a_string_and_number() {
        utils::assert_input("print \"Hell\" 0;", "Hell0");
    }

    #[test]
    fn it_concatenates_with_binary_str_addition() {
        utils::assert_input("print 3 + \"Hello\" 4;", "34");
    }

    #[test]
    fn it_concatenates_using_comma() {
        utils::assert_input("print \"hello\",\"world\";", "hello world");
    }

    #[test]
    fn it_concatenates_using_comma_with_undefined_variable() {
        utils::assert_input("print hello,\"world\";", " world");
    }

    #[test]
    fn it_concatenates_using_with_two_numbers() {
        utils::assert_input("hello=23; print hello,hello;", "23 23");
    }

    #[test]
    fn it_coerces_a_variable_to_number() {
        utils::assert_input("foo = \"3\"; print foo * 3;", "9");
    }

    #[test]
    fn it_coerces_a_variable_to_string() {
        utils::assert_input("foo = 3; print foo \"3\";", "33");
    }

    #[test]
    fn it_ignores_unknown_variables_for_concatenation() {
        utils::assert_input("zfoo=3; print \"z\" foo\"z\";", "zz");
    }

    #[test]
    fn it_uses_variables_in_concatenation() {
        utils::assert_input("zfoo=3; print \"z\" zfoo\"z\";", "z3z");
    }

    #[test]
    fn it_supports_add_assign() {
        utils::assert_input("foo=3; bar+=foo; print bar;", "3");
    }

    #[test]
    fn it_supports_multiple_add_assign() {
        utils::assert_input("foo=3; foo+=foo+=foo; print foo;", "12");
    }

    #[test]
    fn it_supports_sub_assign() {
        utils::assert_input("foo=3; bar-=foo; print bar;", "-3");
    }

    #[test]
    fn it_supports_multiple_sub_assign() {
        utils::assert_input("foo=3; bar=1; foo-=bar-=foo; print foo;", "5");
    }

    #[test]
    fn it_supports_mul_assign() {
        utils::assert_input("foo=3; bar=7; bar*=foo; print bar;", "21");
    }

    #[test]
    fn it_supports_multiple_mul_assign() {
        utils::assert_input("foo=3; foo*=foo*=foo; print foo;", "81");
    }

    #[test]
    fn it_supports_div_assign() {
        utils::assert_input("foo=3; bar=6; bar/=foo; print bar;", "2");
    }

    #[test]
    fn it_supports_multiple_div_assign() {
        utils::assert_input("foo=81; bar=9; baz=3; foo/=bar/=baz; print foo;", "27");
    }

    #[test]
    fn it_supports_mod_assign() {
        utils::assert_input("foo=3; bar=7; bar%=foo; print bar;", "1");
    }

    #[test]
    fn it_supports_multiple_mod_assign() {
        utils::assert_input("foo=7; bar=5; baz=3; foo%=bar%=baz; print foo;", "1");
    }

    #[test]
    fn it_supports_pow_assign() {
        utils::assert_input("foo=3; foo^=foo; print foo;", "27");
    }

    #[test]
    fn it_supports_multiple_pow_assign() {
        utils::assert_input("foo=2; foo^=foo^=foo; print foo;", "256");
    }
}
