//! Integration tests for arithmetic operations

pub mod utils;

#[cfg(test)]
mod arithmetic_tests {
    use crate::utils;

    #[test]
    fn it_sums_two_integers() {
        utils::CodeRunner::init()
            .program("{print 1+2;}")
            .cli_options(vec!["-q"])
            .expect_output("3")
            .assert()
    }

    #[test]
    fn it_subtracts_two_integers() {
        utils::CodeRunner::init()
            .program("{print 1-2;}")
            .cli_options(vec!["-q"])
            .expect_output("-1")
            .assert()
    }

    #[test]
    fn it_multiplies_two_integers() {
        utils::CodeRunner::init()
            .program("{print 3*2;}")
            .cli_options(vec!["-q"])
            .expect_output("6")
            .assert()
    }

    #[test]
    fn it_divides_two_integers() {
        utils::CodeRunner::init()
            .program("{print 6/2;}")
            .cli_options(vec!["-q"])
            .expect_output("3")
            .assert()
    }

    #[test]
    fn it_finds_the_modulo_of_two_integers() {
        utils::CodeRunner::init()
            .program("{print 3%2;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_calculates_the_power_of_two_integers() {
        utils::CodeRunner::init()
            .program("{print 3^2;}")
            .cli_options(vec!["-q"])
            .expect_output("9")
            .assert()
    }

    #[test]
    fn it_calculates_the_power_of_three_integers() {
        utils::CodeRunner::init()
            .program("{print 3^2^3;}")
            .cli_options(vec!["-q"])
            .expect_output("6561")
            .assert()
    }

    #[test]
    fn it_negates_a_negative_number_with_unary_minus() {
        utils::CodeRunner::init()
            .program("{print -9;}")
            .cli_options(vec!["-q"])
            .expect_output("-9")
            .assert()
    }

    #[test]
    fn it_negates_a_negative_strnum_with_unary_minus() {
        utils::CodeRunner::init()
            .program("{print -$1;}")
            .stdin_data("9")
            .expect_output("-9")
            .assert()
    }

    #[test]
    fn it_does_not_negate_zero_with_unary_minus() {
        // echo '-0' | awk '{print -0}' yields 0
        utils::CodeRunner::init()
            .program("{print -0;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn it_negates_zero_with_strnum_unary_minus() {
        // echo '-0' | awk '{print -$1}' yields -0
        utils::CodeRunner::init()
            .program("{print -$1;}")
            .stdin_data("0")
            .expect_output("0")
            .assert()
    }

    #[test]
    fn it_keeps_a_negative_number_as_such_with_unary_plus() {
        utils::CodeRunner::init()
            .program("{print -+9;}")
            .cli_options(vec!["-q"])
            .expect_output("-9")
            .assert()
    }

    #[test]
    fn it_keeps_a_negative_strnum_as_such_with_unary_plus() {
        utils::CodeRunner::init()
            .program("{print -+$1;}")
            .stdin_data("9")
            .expect_output("-9")
            .assert()
    }

    #[test]
    fn it_keeps_a_negative_number_as_such_with_unary_plus_flipped() {
        utils::CodeRunner::init()
            .program("{print +-9;}")
            .cli_options(vec!["-q"])
            .expect_output("-9")
            .assert()
    }

    #[test]
    fn it_keeps_a_negative_strnum_as_such_with_unary_plus_flipped() {
        utils::CodeRunner::init()
            .program("{print +-$1;}")
            .stdin_data("9")
            .expect_output("-9")
            .assert()
    }

    #[test]
    fn it_does_not_alter_zero_with_unary_plus() {
        utils::CodeRunner::init()
            .program("{print +0;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn it_does_not_alter_zero_with_strnum_unary_plus() {
        utils::CodeRunner::init()
            .program("{print +$1;}")
            .stdin_data("0")
            .expect_output("0")
            .assert()
    }

    #[test]
    fn it_sums_an_integer_and_a_string() {
        utils::CodeRunner::init()
            .program(r#"{print 2.14 + "1Hello";}"#)
            .cli_options(vec!["-q"])
            .expect_output("3.14")
            .assert()
    }

    #[test]
    fn it_sums_a_float_and_a_string() {
        utils::CodeRunner::init()
            .program(r#"{print 2.14 + "1.24Hello";}"#)
            .cli_options(vec!["-q"])
            .expect_output("3.38")
            .assert()
    }

    #[test]
    fn it_sums_a_string_and_an_integer() {
        utils::CodeRunner::init()
            .program(r#"{print "02Hello" + 2;}"#)
            .cli_options(vec!["-q"])
            .expect_output("4")
            .assert()
    }

    #[test]
    fn it_sums_a_string_and_a_float() {
        utils::CodeRunner::init()
            .program(r#"{print "5.55Hello" + 1.21;}"#)
            .cli_options(vec!["-q"])
            .expect_output("6.76")
            .assert()
    }

    #[test]
    fn it_subtracts_an_integer_and_a_string() {
        utils::CodeRunner::init()
            .program(r#"{print 2.14 - "1Hello";}"#)
            .cli_options(vec!["-q"])
            .expect_output("1.1400001")
            .assert()
    }

    #[test]
    fn it_subtracts_a_float_and_a_string() {
        utils::CodeRunner::init()
            .program(r#"{print 2.14 - "1.24Hello";}"#)
            .cli_options(vec!["-q"])
            .expect_output("0.9000001")
            .assert()
    }

    #[test]
    fn it_subtracts_a_string_and_an_integer() {
        utils::CodeRunner::init()
            .program(r#"{print "02Hello" - 2;}"#)
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn it_subtracts_a_string_and_a_float() {
        utils::CodeRunner::init()
            .program(r#"{print "5.55Hello" - 1.21;}"#)
            .cli_options(vec!["-q"])
            .expect_output("4.34")
            .assert()
    }

    #[test]
    fn it_multiplies_an_integer_and_a_string() {
        utils::CodeRunner::init()
            .program(r#"{print 2.14 * "1Hello";}"#)
            .cli_options(vec!["-q"])
            .expect_output("2.14")
            .assert()
    }

    #[test]
    fn it_multiplies_a_float_and_a_string() {
        utils::CodeRunner::init()
            .program(r#"{print 2.14 * "1.24Hello";}"#)
            .cli_options(vec!["-q"])
            .expect_output("2.6536002")
            .assert()
    }

    #[test]
    fn it_multiplies_a_string_and_an_integer() {
        utils::CodeRunner::init()
            .program(r#"{print "02Hello" * 2;}"#)
            .cli_options(vec!["-q"])
            .expect_output("4")
            .assert()
    }

    #[test]
    fn it_multiplies_a_string_and_a_float() {
        utils::CodeRunner::init()
            .program(r#"{print "5.55Hello" * 1.21;}"#)
            .cli_options(vec!["-q"])
            .expect_output("6.7155004")
            .assert()
    }

    #[test]
    fn it_divides_an_integer_and_a_string() {
        utils::CodeRunner::init()
            .program(r#"{print 2.14 / "1Hello";}"#)
            .cli_options(vec!["-q"])
            .expect_output("2.14")
            .assert()
    }

    #[test]
    fn it_divides_a_float_and_a_string() {
        utils::CodeRunner::init()
            .program(r#"{print 2.14 / "1.24Hello";}"#)
            .cli_options(vec!["-q"])
            .expect_output("1.7258065")
            .assert()
    }

    #[test]
    fn it_divides_a_string_and_an_integer() {
        utils::CodeRunner::init()
            .program(r#"{print "02Hello" / 2;}"#)
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_divides_a_string_and_a_float() {
        utils::CodeRunner::init()
            .program(r#"{print "5.55Hello" / 1.21;}"#)
            .cli_options(vec!["-q"])
            .expect_output("4.5867767")
            .assert()
    }

    #[test]
    fn it_divides_by_zero() {
        utils::CodeRunner::init()
            .program(r#"{print 1 / 0;}"#)
            .cli_options(vec!["-q"])
            .expect_output("Error: Division by zero")
            .assert_fail()
    }

    #[test]
    fn it_modulos_an_integer_and_a_string() {
        utils::CodeRunner::init()
            .program(r#"{print 2.14 % "1Hello";}"#)
            .cli_options(vec!["-q"])
            .expect_output("0.1400001")
            .assert()
    }

    #[test]
    fn it_modulos_a_float_and_a_string() {
        utils::CodeRunner::init()
            .program(r#"{print 2.14 % "1.24Hello";}"#)
            .cli_options(vec!["-q"])
            .expect_output("0.9000001")
            .assert()
    }

    #[test]
    fn it_modulos_a_string_and_an_integer() {
        utils::CodeRunner::init()
            .program(r#"{print "02Hello" % 2;}"#)
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn it_modulos_a_string_and_a_float() {
        utils::CodeRunner::init()
            .program(r#"{print "5.55Hello" % 1.21;}"#)
            .cli_options(vec!["-q"])
            .expect_output("0.71000004")
            .assert()
    }

    #[test]
    fn it_modulos_by_zero() {
        utils::CodeRunner::init()
            .program(r#"{print 1 % 0;}"#)
            .cli_options(vec!["-q"])
            .expect_output("Error: Mod by zero")
            .assert_fail()
    }

    #[test]
    fn it_exponentiates_an_integer_and_a_string() {
        utils::CodeRunner::init()
            .program(r#"{print 2.14 ^ "1Hello";}"#)
            .cli_options(vec!["-q"])
            .expect_output("2.14")
            .assert()
    }

    #[test]
    fn it_exponentiates_a_float_and_a_string() {
        utils::CodeRunner::init()
            .program(r#"{print 2.14 ^ "1.24Hello";}"#)
            .cli_options(vec!["-q"])
            .expect_output("2.5686984")
            .assert()
    }

    #[test]
    fn it_exponentiates_a_string_and_an_integer() {
        utils::CodeRunner::init()
            .program(r#"{print "02Hello" ^ 2;}"#)
            .cli_options(vec!["-q"])
            .expect_output("4")
            .assert()
    }

    #[test]
    fn it_exponentiates_a_string_and_a_float() {
        utils::CodeRunner::init()
            .program(r#"{print "5.55Hello" ^ 1.21;}"#)
            .cli_options(vec!["-q"])
            .expect_output("7.9541645")
            .assert()
    }

    #[test]
    fn it_concatenates_two_strings() {
        utils::CodeRunner::init()
            .program(r#"{print "Hello" " World!";}"#)
            .cli_options(vec!["-q"])
            .expect_output("Hello World!")
            .assert()
    }

    #[test]
    fn it_concatenates_multiple_strings() {
        utils::CodeRunner::init()
            .program(r#"{print "Hello" " World!" " " "I come in peace!";}"#)
            .cli_options(vec!["-q"])
            .expect_output("Hello World! I come in peace!")
            .assert()
    }

    #[test]
    fn it_concatenates_a_number_and_string() {
        utils::CodeRunner::init()
            .program(r#"{print 1 " World";}"#)
            .cli_options(vec!["-q"])
            .expect_output("1 World")
            .assert()
    }

    #[test]
    fn it_concatenates_a_string_and_number() {
        utils::CodeRunner::init()
            .program(r#"{print "Hell" 0;}"#)
            .cli_options(vec!["-q"])
            .expect_output("Hell0")
            .assert()
    }

    #[test]
    fn it_concatenates_with_binary_str_addition() {
        utils::CodeRunner::init()
            .program(r#"{print 3 + "Hello" 4;}"#)
            .cli_options(vec!["-q"])
            .expect_output("34")
            .assert()
    }

    #[test]
    fn it_concatenates_using_comma_in_print_statement() {
        utils::CodeRunner::init()
            .program(r#"{print "hello","world";}"#)
            .cli_options(vec!["-q"])
            .expect_output("hello world")
            .assert()
    }

    #[test]
    fn it_concatenates_using_comma_with_undefined_variable() {
        utils::CodeRunner::init()
            .program(r#"{print hello,"world";}"#)
            .cli_options(vec!["-q"])
            .expect_output(" world")
            .assert()
    }

    #[test]
    fn it_concatenates_two_numbers_in_print_statement() {
        utils::CodeRunner::init()
            .program("{hello=23; print hello,hello;}")
            .cli_options(vec!["-q"])
            .expect_output("23 23")
            .assert()
    }

    #[test]
    fn it_coerces_a_variable_to_number() {
        utils::CodeRunner::init()
            .program(r#"{foo = "3"; print foo * 3;}"#)
            .cli_options(vec!["-q"])
            .expect_output("9")
            .assert()
    }

    #[test]
    fn it_coerces_a_variable_to_string() {
        utils::CodeRunner::init()
            .program(r#"{foo = 3; print foo "3";}"#)
            .cli_options(vec!["-q"])
            .expect_output("33")
            .assert()
    }

    #[test]
    fn it_ignores_unknown_variables_for_concatenation() {
        utils::CodeRunner::init()
            .program(r#"{zfoo=3; print "z" foo"z";}"#)
            .cli_options(vec!["-q"])
            .expect_output("zz")
            .assert()
    }

    #[test]
    fn it_uses_variables_in_concatenation() {
        utils::CodeRunner::init()
            .program(r#"{zfoo=3; print "z" zfoo"z";}"#)
            .cli_options(vec!["-q"])
            .expect_output("z3z")
            .assert()
    }

    #[test]
    fn it_supports_add_assign() {
        utils::CodeRunner::init()
            .program("{foo=3; bar+=foo; print bar;}")
            .cli_options(vec!["-q"])
            .expect_output("3")
            .assert()
    }

    #[test]
    fn it_supports_multiple_add_assign() {
        utils::CodeRunner::init()
            .program("{foo=3; foo+=foo+=foo; print foo;}")
            .cli_options(vec!["-q"])
            .expect_output("12")
            .assert()
    }

    #[test]
    fn it_supports_sub_assign() {
        utils::CodeRunner::init()
            .program("{foo=3; bar-=foo; print bar;}")
            .cli_options(vec!["-q"])
            .expect_output("-3")
            .assert()
    }

    #[test]
    fn it_supports_multiple_sub_assign() {
        utils::CodeRunner::init()
            .program("{foo=3; bar=1; foo-=bar-=foo; print foo;}")
            .cli_options(vec!["-q"])
            .expect_output("5")
            .assert()
    }

    #[test]
    fn it_supports_mul_assign() {
        utils::CodeRunner::init()
            .program("{foo=3; bar=7; bar*=foo; print bar;}")
            .cli_options(vec!["-q"])
            .expect_output("21")
            .assert()
    }

    #[test]
    fn it_supports_multiple_mul_assign() {
        utils::CodeRunner::init()
            .program("{foo=3; foo*=foo*=foo; print foo;}")
            .cli_options(vec!["-q"])
            .expect_output("81")
            .assert()
    }

    #[test]
    fn it_supports_div_assign() {
        utils::CodeRunner::init()
            .program("{foo=3; bar=6; bar/=foo; print bar;}")
            .cli_options(vec!["-q"])
            .expect_output("2")
            .assert()
    }

    #[test]
    fn it_supports_multiple_div_assign() {
        utils::CodeRunner::init()
            .program("{foo=81; bar=9; baz=3; foo/=bar/=baz; print foo;}")
            .cli_options(vec!["-q"])
            .expect_output("27")
            .assert()
    }

    #[test]
    fn it_supports_mod_assign() {
        utils::CodeRunner::init()
            .program("{foo=3; bar=7; bar%=foo; print bar;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_supports_multiple_mod_assign() {
        utils::CodeRunner::init()
            .program("{foo=7; bar=5; baz=3; foo%=bar%=baz; print foo;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_supports_pow_assign() {
        utils::CodeRunner::init()
            .program("{foo=3; foo^=foo; print foo;}")
            .cli_options(vec!["-q"])
            .expect_output("27")
            .assert()
    }

    #[test]
    fn it_supports_multiple_pow_assign() {
        utils::CodeRunner::init()
            .program("{foo=2; foo^=foo^=foo; print foo;}")
            .cli_options(vec!["-q"])
            .expect_output("256")
            .assert()
    }
}
