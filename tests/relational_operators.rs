//! Integration tests for relational operations

pub mod utils;

#[cfg(test)]
mod relational_tests {
    use crate::utils;

    #[test]
    fn compares_two_integers_with_greater_than_false() {
        utils::CodeRunner::init()
            .program("{print 1>2;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_integers_with_greater_than_true() {
        utils::CodeRunner::init()
            .program("{print 2>1;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_integers_with_less_than_false() {
        utils::CodeRunner::init()
            .program("{print 2<1;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_integers_with_less_than_true() {
        utils::CodeRunner::init()
            .program("{print 1<2;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_integers_with_greater_equal_than_false() {
        utils::CodeRunner::init()
            .program("{print 1>=2;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_integers_with_greater_equal_than_true() {
        utils::CodeRunner::init()
            .program("{print 3>=2;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_integers_with_greater_equal_than_same_val_true() {
        utils::CodeRunner::init()
            .program("{print 2>=2;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_integers_with_less_equal_than_false() {
        utils::CodeRunner::init()
            .program("{print 1<=0;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_integers_with_less_equal_than_true() {
        utils::CodeRunner::init()
            .program("{print 1<=2;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_integers_with_less_equal_than_same_val_true() {
        utils::CodeRunner::init()
            .program("{print 2<=2;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_integers_with_double_equal_true() {
        utils::CodeRunner::init()
            .program("{print 2==2;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_integers_with_double_equal_false() {
        utils::CodeRunner::init()
            .program("{print 1==2;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_integers_with_not_equal_true() {
        utils::CodeRunner::init()
            .program("{print 1!=2;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_integers_with_not_equal_false() {
        utils::CodeRunner::init()
            .program("{print 2!=2;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_double_equal_same_case() {
        utils::CodeRunner::init()
            .program("{print \"a\"==\"a\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_double_equal_same_case_multiple_char() {
        utils::CodeRunner::init()
            .program("{print \"abc\"==\"abc\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_double_equal_diff_case() {
        utils::CodeRunner::init()
            .program("{print \"a\"==\"A\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_double_equal_diff_case_multiple_char() {
        utils::CodeRunner::init()
            .program("{print \"abc\"==\"aBc\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_double_equal_empty_lhs() {
        utils::CodeRunner::init()
            .program("{print \"\"==\"a\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_double_equal_empty_rhs() {
        utils::CodeRunner::init()
            .program("{print \"a\"==\"\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_not_equal_same_case() {
        utils::CodeRunner::init()
            .program("{print \"a\"!=\"a\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_not_equal_same_case_multiple_char() {
        utils::CodeRunner::init()
            .program("{print \"abc\"!=\"abc\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_not_equal_diff_case() {
        utils::CodeRunner::init()
            .program("{print \"a\"!=\"A\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_not_equal_diff_case_multiple_char() {
        utils::CodeRunner::init()
            .program("{print \"abc\"!=\"aBc\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_not_equal_empty_lhs() {
        utils::CodeRunner::init()
            .program("{print \"\"!=\"a\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_not_equal_empty_rhs() {
        utils::CodeRunner::init()
            .program("{print \"a\"!=\"\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_greater_equal_same_case() {
        utils::CodeRunner::init()
            .program("{print \"a\">=\"a\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_greater_equal_same_case_multiple_char() {
        utils::CodeRunner::init()
            .program("{print \"abc\">=\"abc\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_greater_equal_diff_case() {
        utils::CodeRunner::init()
            .program("{print \"a\">=\"A\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_greater_equal_diff_case_multiple_char() {
        utils::CodeRunner::init()
            .program("{print \"abc\">=\"aBc\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_greater_equal_diff_case_multiple_char_diff_len() {
        utils::CodeRunner::init()
            .program("{print \"a\">=\"aBc\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_greater_equal_diff_case_multiple_char_diff_len_two() {
        utils::CodeRunner::init()
            .program("{print \"aBc\">=\"a\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_greater_equal_empty_lhs() {
        utils::CodeRunner::init()
            .program("{print \"\">=\"a\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_greater_equal_empty_rhs() {
        utils::CodeRunner::init()
            .program("{print \"a\">=\"\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_greater_same_case() {
        utils::CodeRunner::init()
            .program("{print \"a\">\"a\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_greater_same_case_multiple_char() {
        utils::CodeRunner::init()
            .program("{print \"abc\">\"abc\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_greater_diff_case() {
        utils::CodeRunner::init()
            .program("{print \"a\">\"A\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_greater_diff_case_multiple_char() {
        utils::CodeRunner::init()
            .program("{print \"abc\">\"aBc\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_greater_diff_case_multiple_char_diff_len() {
        utils::CodeRunner::init()
            .program("{print \"a\">\"aBc\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_greater_diff_case_multiple_char_diff_len_two() {
        utils::CodeRunner::init()
            .program("{print \"aBc\">\"a\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_greater_empty_lhs() {
        utils::CodeRunner::init()
            .program("{print \"\">\"a\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_greater_empty_rhs() {
        utils::CodeRunner::init()
            .program("{print \"a\">\"\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_less_than_equal_same_case() {
        utils::CodeRunner::init()
            .program("{print \"a\"<=\"a\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_less_than_equal_same_case_multiple_char() {
        utils::CodeRunner::init()
            .program("{print \"abc\"<=\"abc\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_less_than_equal_diff_case() {
        utils::CodeRunner::init()
            .program("{print \"a\"<=\"A\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_less_than_equal_diff_case_multiple_char() {
        utils::CodeRunner::init()
            .program("{print \"abc\"<=\"aBc\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_less_than_equal_diff_case_multiple_char_diff_len() {
        utils::CodeRunner::init()
            .program("{print \"a\"<=\"aBc\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_less_than_equal_diff_case_multiple_char_diff_len_two() {
        utils::CodeRunner::init()
            .program("{print \"aBc\"<=\"a\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_less_than_equal_empty_lhs() {
        utils::CodeRunner::init()
            .program("{print \"\"<=\"a\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_less_than_equal_empty_rhs() {
        utils::CodeRunner::init()
            .program("{print \"a\"<=\"\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_less_than_same_case() {
        utils::CodeRunner::init()
            .program("{print \"a\"<\"a\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_less_than_same_case_multiple_char() {
        utils::CodeRunner::init()
            .program("{print \"abc\"<\"abc\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_less_than_diff_case() {
        utils::CodeRunner::init()
            .program("{print \"a\"<\"A\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_less_than_diff_case_multiple_char() {
        utils::CodeRunner::init()
            .program("{print \"abc\"<\"aBc\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_less_than_diff_case_multiple_char_diff_len() {
        utils::CodeRunner::init()
            .program("{print \"a\"<\"aBc\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_less_than_diff_case_multiple_char_diff_len_two() {
        utils::CodeRunner::init()
            .program("{print \"aBc\"<\"a\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_two_strings_less_than_empty_lhs() {
        utils::CodeRunner::init()
            .program("{print \"\"<\"a\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_two_strings_less_than_empty_rhs() {
        utils::CodeRunner::init()
            .program("{print \"a\"<\"\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_a_number_and_string_double_equal() {
        utils::CodeRunner::init()
            .program("{print \"a\"==1;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_zero_and_empty_string_double_equal() {
        utils::CodeRunner::init()
            .program("{print \"\"==0;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_a_number_and_string_not_equal() {
        utils::CodeRunner::init()
            .program("{print \"a\"!=1;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_a_number_and_string_greater_equal() {
        utils::CodeRunner::init()
            .program("{print \"a\">=1;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_a_number_and_string_greater_equal_symmetric() {
        utils::CodeRunner::init()
            .program("{print 1>=\"a\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_a_number_and_string_greater() {
        utils::CodeRunner::init()
            .program("{print \"a\">1;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_a_number_and_string_greater_symmetric() {
        utils::CodeRunner::init()
            .program("{print 1>\"a\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_a_number_and_string_less_than_equal() {
        utils::CodeRunner::init()
            .program("{print \"a\"<=1;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_a_number_and_string_less_than_equal_symmetric() {
        utils::CodeRunner::init()
            .program("{print 1<=\"a\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_a_number_and_string_less_than() {
        utils::CodeRunner::init()
            .program("{print \"a\"<1;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_a_number_and_string_less_than_symmetric() {
        utils::CodeRunner::init()
            .program("{print 1<\"a\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_a_string_number_and_a_number_double_equal() {
        utils::CodeRunner::init()
            .program("{print \"1.1\"==1.1;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_a_string_number_and_a_number_double_equal_false() {
        utils::CodeRunner::init()
            .program("{print 1==\"1.1\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_a_string_number_and_a_number_not_equals() {
        utils::CodeRunner::init()
            .program("{print \"1.1\"!=1;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_a_string_number_and_a_number_not_equals_false() {
        utils::CodeRunner::init()
            .program("{print 1.1!=\"1.1\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_a_string_number_and_a_number_greater_than_equal() {
        utils::CodeRunner::init()
            .program("{print \"1.1\">=1;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_a_string_number_and_a_number_greater_than_equal_symmetric() {
        utils::CodeRunner::init()
            .program("{print 1>=\"1.1\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_a_string_number_and_a_number_greater_than() {
        utils::CodeRunner::init()
            .program("{print \"1.1\">1;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_a_string_number_and_a_number_greater_than_lex() {
        utils::CodeRunner::init()
            .program("{print \"2\">12;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_a_string_number_and_a_number_greater_than_symmetric() {
        utils::CodeRunner::init()
            .program("{print 1>\"1.1\";}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_a_string_number_and_a_number_less_than_equal() {
        utils::CodeRunner::init()
            .program("{print \"1.1\"<=1;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_a_string_number_and_a_number_less_than_equal_symmetric() {
        utils::CodeRunner::init()
            .program("{print 1<=\"1.1\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_a_string_number_and_a_number_less_than() {
        utils::CodeRunner::init()
            .program("{print \"1.1\"<1;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_a_string_number_and_a_number_less_than_lex() {
        utils::CodeRunner::init()
            .program("{print \"2\"<12;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_a_string_number_and_a_number_less_than_symmetric() {
        utils::CodeRunner::init()
            .program("{print 1<\"1.1\";}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_a_strnum_and_a_number_double_equal() {
        utils::CodeRunner::init()
            .program("{print $1==1.1;}")
            .stdin_data("1.1")
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_a_strnum_and_a_number_double_equal_false() {
        utils::CodeRunner::init()
            .program("{print 1==$1;}")
            .stdin_data("1.1")
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_a_strnum_and_a_number_not_equals() {
        utils::CodeRunner::init()
            .program("{print $1!=1;}")
            .stdin_data("1.1")
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_a_strnum_and_a_number_not_equals_false() {
        utils::CodeRunner::init()
            .program("{print 1.1!=$1;}")
            .stdin_data("1.1")
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_a_strnum_and_a_number_greater_than_equal() {
        utils::CodeRunner::init()
            .program("{print $1>=1;}")
            .stdin_data("1.1")
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_a_strnum_and_a_number_greater_than_equal_symmetric() {
        utils::CodeRunner::init()
            .program("{print 1>=$1;}")
            .stdin_data("1.1")
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_a_strnum_and_a_number_greater_than() {
        utils::CodeRunner::init()
            .program("{print $1>1;}")
            .stdin_data("1.1")
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_a_strnum_and_a_number_greater_than_lex() {
        utils::CodeRunner::init()
            .program("{print $1>12;}")
            .stdin_data("2")
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_a_strnum_and_a_number_greater_than_symmetric() {
        utils::CodeRunner::init()
            .program("{print 1>$1;}")
            .stdin_data("1.1")
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_a_strnum_and_a_number_less_than_equal() {
        utils::CodeRunner::init()
            .program("{print $1<=1;}")
            .stdin_data("1.1")
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_a_strnum_and_a_number_less_than_equal_symmetric() {
        utils::CodeRunner::init()
            .program("{print 1<=$1;}")
            .stdin_data("1.1")
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_a_strnum_and_a_number_less_than() {
        utils::CodeRunner::init()
            .program("{print $1<1;}")
            .stdin_data("1.1")
            .expect_output("0")
            .assert()
    }

    #[test]
    fn compares_a_strnum_and_a_number_less_than_lex() {
        utils::CodeRunner::init()
            .program("{print $1<12;}")
            .stdin_data("2")
            .expect_output("1")
            .assert()
    }

    #[test]
    fn compares_a_strnum_and_a_number_less_than_symmetric() {
        utils::CodeRunner::init()
            .program("{print 1<$1;}")
            .stdin_data("1.1")
            .expect_output("1")
            .assert()
    }
}
