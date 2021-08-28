//! Integration tests for relational operations

mod utils;

#[cfg(test)]
mod relational_tests {
    use crate::utils;

    #[test]
    fn compares_two_integers_with_greater_than_false() {
        utils::assert_input("print 1>2;", "0");
    }

    #[test]
    fn compares_two_integers_with_greater_than_true() {
        utils::assert_input("print 2>1;", "1");
    }

    #[test]
    fn compares_two_integers_with_less_than_false() {
        utils::assert_input("print 2<1;", "0");
    }

    #[test]
    fn compares_two_integers_with_less_than_true() {
        utils::assert_input("print 1<2;", "1");
    }

    #[test]
    fn compares_two_integers_with_greater_equal_than_false() {
        utils::assert_input("print 1>=2;", "0");
    }

    #[test]
    fn compares_two_integers_with_greater_equal_than_true() {
        utils::assert_input("print 3>=2;", "1");
    }

    #[test]
    fn compares_two_integers_with_greater_equal_than_same_val_true() {
        utils::assert_input("print 2>=2;", "1");
    }

    #[test]
    fn compares_two_integers_with_less_equal_than_false() {
        utils::assert_input("print 1<=0;", "0");
    }

    #[test]
    fn compares_two_integers_with_less_equal_than_true() {
        utils::assert_input("print 1<=2;", "1");
    }

    #[test]
    fn compares_two_integers_with_less_equal_than_same_val_true() {
        utils::assert_input("print 2<=2;", "1");
    }

    #[test]
    fn compares_two_integers_with_double_equal_true() {
        utils::assert_input("print 2==2;", "1");
    }

    #[test]
    fn compares_two_integers_with_double_equal_false() {
        utils::assert_input("print 1==2;", "0");
    }

    #[test]
    fn compares_two_integers_with_not_equal_true() {
        utils::assert_input("print 1!=2;", "1");
    }

    #[test]
    fn compares_two_integers_with_not_equal_false() {
        utils::assert_input("print 2!=2;", "0");
    }

    #[test]
    fn compares_two_strings_double_equal_same_case() {
        utils::assert_input("print \"a\"==\"a\";", "1");
    }

    #[test]
    fn compares_two_strings_double_equal_same_case_multiple_char() {
        utils::assert_input("print \"abc\"==\"abc\";", "1");
    }

    #[test]
    fn compares_two_strings_double_equal_diff_case() {
        utils::assert_input("print \"a\"==\"A\";", "0");
    }

    #[test]
    fn compares_two_strings_double_equal_diff_case_multiple_char() {
        utils::assert_input("print \"abc\"==\"aBc\";", "0");
    }

    #[test]
    fn compares_two_strings_double_equal_empty_lhs() {
        utils::assert_input("print \"\"==\"a\";", "0");
    }

    #[test]
    fn compares_two_strings_double_equal_empty_rhs() {
        utils::assert_input("print \"a\"==\"\";", "0");
    }

    #[test]
    fn compares_two_strings_not_equal_same_case() {
        utils::assert_input("print \"a\"!=\"a\";", "0");
    }

    #[test]
    fn compares_two_strings_not_equal_same_case_multiple_char() {
        utils::assert_input("print \"abc\"!=\"abc\";", "0");
    }

    #[test]
    fn compares_two_strings_not_equal_diff_case() {
        utils::assert_input("print \"a\"!=\"A\";", "1");
    }

    #[test]
    fn compares_two_strings_not_equal_diff_case_multiple_char() {
        utils::assert_input("print \"abc\"!=\"aBc\";", "1");
    }

    #[test]
    fn compares_two_strings_not_equal_empty_lhs() {
        utils::assert_input("print \"\"!=\"a\";", "1");
    }

    #[test]
    fn compares_two_strings_not_equal_empty_rhs() {
        utils::assert_input("print \"a\"!=\"\";", "1");
    }

    #[test]
    fn compares_two_strings_greater_equal_same_case() {
        utils::assert_input("print \"a\">=\"a\";", "1");
    }

    #[test]
    fn compares_two_strings_greater_equal_same_case_multiple_char() {
        utils::assert_input("print \"abc\">=\"abc\";", "1");
    }

    #[test]
    fn compares_two_strings_greater_equal_diff_case() {
        utils::assert_input("print \"a\">=\"A\";", "1");
    }

    #[test]
    fn compares_two_strings_greater_equal_diff_case_multiple_char() {
        utils::assert_input("print \"abc\">=\"aBc\";", "1");
    }

    #[test]
    fn compares_two_strings_greater_equal_diff_case_multiple_char_diff_len() {
        utils::assert_input("print \"a\">=\"aBc\";", "0");
    }

    #[test]
    fn compares_two_strings_greater_equal_diff_case_multiple_char_diff_len_two() {
        utils::assert_input("print \"aBc\">=\"a\";", "1");
    }

    #[test]
    fn compares_two_strings_greater_equal_empty_lhs() {
        utils::assert_input("print \"\">=\"a\";", "0");
    }

    #[test]
    fn compares_two_strings_greater_equal_empty_rhs() {
        utils::assert_input("print \"a\">=\"\";", "1");
    }

    #[test]
    fn compares_two_strings_greater_same_case() {
        utils::assert_input("print \"a\">\"a\";", "0");
    }

    #[test]
    fn compares_two_strings_greater_same_case_multiple_char() {
        utils::assert_input("print \"abc\">\"abc\";", "0");
    }

    #[test]
    fn compares_two_strings_greater_diff_case() {
        utils::assert_input("print \"a\">\"A\";", "1");
    }

    #[test]
    fn compares_two_strings_greater_diff_case_multiple_char() {
        utils::assert_input("print \"abc\">\"aBc\";", "1");
    }

    #[test]
    fn compares_two_strings_greater_diff_case_multiple_char_diff_len() {
        utils::assert_input("print \"a\">\"aBc\";", "0");
    }

    #[test]
    fn compares_two_strings_greater_diff_case_multiple_char_diff_len_two() {
        utils::assert_input("print \"aBc\">\"a\";", "1");
    }

    #[test]
    fn compares_two_strings_greater_empty_lhs() {
        utils::assert_input("print \"\">\"a\";", "0");
    }

    #[test]
    fn compares_two_strings_greater_empty_rhs() {
        utils::assert_input("print \"a\">\"\";", "1");
    }

    #[test]
    fn compares_two_strings_less_than_equal_same_case() {
        utils::assert_input("print \"a\"<=\"a\";", "1");
    }

    #[test]
    fn compares_two_strings_less_than_equal_same_case_multiple_char() {
        utils::assert_input("print \"abc\"<=\"abc\";", "1");
    }

    #[test]
    fn compares_two_strings_less_than_equal_diff_case() {
        utils::assert_input("print \"a\"<=\"A\";", "0");
    }

    #[test]
    fn compares_two_strings_less_than_equal_diff_case_multiple_char() {
        utils::assert_input("print \"abc\"<=\"aBc\";", "0");
    }

    #[test]
    fn compares_two_strings_less_than_equal_diff_case_multiple_char_diff_len() {
        utils::assert_input("print \"a\"<=\"aBc\";", "1");
    }

    #[test]
    fn compares_two_strings_less_than_equal_diff_case_multiple_char_diff_len_two() {
        utils::assert_input("print \"aBc\"<=\"a\";", "0");
    }

    #[test]
    fn compares_two_strings_less_than_equal_empty_lhs() {
        utils::assert_input("print \"\"<=\"a\";", "1");
    }

    #[test]
    fn compares_two_strings_less_than_equal_empty_rhs() {
        utils::assert_input("print \"a\"<=\"\";", "0");
    }

    #[test]
    fn compares_two_strings_less_than_same_case() {
        utils::assert_input("print \"a\"<\"a\";", "0");
    }

    #[test]
    fn compares_two_strings_less_than_same_case_multiple_char() {
        utils::assert_input("print \"abc\"<\"abc\";", "0");
    }

    #[test]
    fn compares_two_strings_less_than_diff_case() {
        utils::assert_input("print \"a\"<\"A\";", "0");
    }

    #[test]
    fn compares_two_strings_less_than_diff_case_multiple_char() {
        utils::assert_input("print \"abc\"<\"aBc\";", "0");
    }

    #[test]
    fn compares_two_strings_less_than_diff_case_multiple_char_diff_len() {
        utils::assert_input("print \"a\"<\"aBc\";", "1");
    }

    #[test]
    fn compares_two_strings_less_than_diff_case_multiple_char_diff_len_two() {
        utils::assert_input("print \"aBc\"<\"a\";", "0");
    }

    #[test]
    fn compares_two_strings_less_than_empty_lhs() {
        utils::assert_input("print \"\"<\"a\";", "1");
    }

    #[test]
    fn compares_two_strings_less_than_empty_rhs() {
        utils::assert_input("print \"a\"<\"\";", "0");
    }

    #[test]
    fn compares_a_number_and_string_double_equal() {
        utils::assert_input("print \"a\"==1;", "0");
    }

    #[test]
    fn compares_zero_and_empty_string_double_equal() {
        utils::assert_input("print \"\"==0;", "0");
    }

    #[test]
    fn compares_a_number_and_string_not_equal() {
        utils::assert_input("print \"a\"!=1;", "1");
    }

    #[test]
    fn compares_a_number_and_string_greater_equal() {
        utils::assert_input("print \"a\">=1;", "1");
    }

    #[test]
    fn compares_a_number_and_string_greater_equal_symmetric() {
        utils::assert_input("print 1>=\"a\";", "0");
    }

    #[test]
    fn compares_a_number_and_string_greater() {
        utils::assert_input("print \"a\">1;", "1");
    }

    #[test]
    fn compares_a_number_and_string_greater_symmetric() {
        utils::assert_input("print 1>\"a\";", "0");
    }

    #[test]
    fn compares_a_number_and_string_less_than_equal() {
        utils::assert_input("print \"a\"<=1;", "0");
    }

    #[test]
    fn compares_a_number_and_string_less_than_equal_symmetric() {
        utils::assert_input("print 1<=\"a\";", "1");
    }

    #[test]
    fn compares_a_number_and_string_less_than() {
        utils::assert_input("print \"a\"<1;", "0");
    }

    #[test]
    fn compares_a_number_and_string_less_than_symmetric() {
        utils::assert_input("print 1<\"a\";", "1");
    }
}
