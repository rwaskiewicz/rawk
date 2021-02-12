//! Integration tests for relational operations

mod utils;

#[cfg(test)]
mod relational_tests {
    use crate::utils;

    #[test]
    fn compares_two_integers_with_greater_than_false() {
        utils::assert_input("1>2", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_integers_with_greater_than_true() {
        utils::assert_input("2>1", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_integers_with_less_than_false() {
        utils::assert_input("2<1", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_integers_with_less_than_true() {
        utils::assert_input("1<2", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_integers_with_greater_equal_than_false() {
        utils::assert_input("1>=2", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_integers_with_greater_equal_than_true() {
        utils::assert_input("3>=2", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_integers_with_greater_equal_than_same_val_true() {
        utils::assert_input("2>=2", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_integers_with_less_equal_than_false() {
        utils::assert_input("1<=0", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_integers_with_less_equal_than_true() {
        utils::assert_input("1<=2", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_integers_with_less_equal_than_same_val_true() {
        utils::assert_input("2<=2", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_integers_with_double_equal_true() {
        utils::assert_input("2==2", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_integers_with_double_equal_false() {
        utils::assert_input("1==2", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_integers_with_not_equal_true() {
        utils::assert_input("1!=2", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_integers_with_not_equal_false() {
        utils::assert_input("2!=2", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_double_equal_same_case() {
        utils::assert_input("\"a\"==\"a\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_double_equal_same_case_multiple_char() {
        utils::assert_input("\"abc\"==\"abc\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_double_equal_diff_case() {
        utils::assert_input("\"a\"==\"A\"", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_double_equal_diff_case_multiple_char() {
        utils::assert_input("\"abc\"==\"aBc\"", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_double_equal_empty_lhs() {
        utils::assert_input("\"\"==\"a\"", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_double_equal_empty_rhs() {
        utils::assert_input("\"a\"==\"\"", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_not_equal_same_case() {
        utils::assert_input("\"a\"!=\"a\"", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_not_equal_same_case_multiple_char() {
        utils::assert_input("\"abc\"!=\"abc\"", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_not_equal_diff_case() {
        utils::assert_input("\"a\"!=\"A\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_not_equal_diff_case_multiple_char() {
        utils::assert_input("\"abc\"!=\"aBc\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_not_equal_empty_lhs() {
        utils::assert_input("\"\"!=\"a\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_not_equal_empty_rhs() {
        utils::assert_input("\"a\"!=\"\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_greater_equal_same_case() {
        utils::assert_input("\"a\">=\"a\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_greater_equal_same_case_multiple_char() {
        utils::assert_input("\"abc\">=\"abc\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_greater_equal_diff_case() {
        utils::assert_input("\"a\">=\"A\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_greater_equal_diff_case_multiple_char() {
        utils::assert_input("\"abc\">=\"aBc\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_greater_equal_diff_case_multiple_char_diff_len() {
        utils::assert_input("\"a\">=\"aBc\"", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_greater_equal_diff_case_multiple_char_diff_len_two() {
        utils::assert_input("\"aBc\">=\"a\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_greater_equal_empty_lhs() {
        utils::assert_input("\"\">=\"a\"", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_greater_equal_empty_rhs() {
        utils::assert_input("\"a\">=\"\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_greater_same_case() {
        utils::assert_input("\"a\">\"a\"", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_greater_same_case_multiple_char() {
        utils::assert_input("\"abc\">\"abc\"", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_greater_diff_case() {
        utils::assert_input("\"a\">\"A\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_greater_diff_case_multiple_char() {
        utils::assert_input("\"abc\">\"aBc\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_greater_diff_case_multiple_char_diff_len() {
        utils::assert_input("\"a\">\"aBc\"", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_greater_diff_case_multiple_char_diff_len_two() {
        utils::assert_input("\"aBc\">\"a\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_greater_empty_lhs() {
        utils::assert_input("\"\">\"a\"", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_greater_empty_rhs() {
        utils::assert_input("\"a\">\"\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_less_than_equal_same_case() {
        utils::assert_input("\"a\"<=\"a\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_less_than_equal_same_case_multiple_char() {
        utils::assert_input("\"abc\"<=\"abc\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_less_than_equal_diff_case() {
        utils::assert_input("\"a\"<=\"A\"", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_less_than_equal_diff_case_multiple_char() {
        utils::assert_input("\"abc\"<=\"aBc\"", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_less_than_equal_diff_case_multiple_char_diff_len() {
        utils::assert_input("\"a\"<=\"aBc\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_less_than_equal_diff_case_multiple_char_diff_len_two() {
        utils::assert_input("\"aBc\"<=\"a\"", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_less_than_equal_empty_lhs() {
        utils::assert_input("\"\"<=\"a\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_less_than_equal_empty_rhs() {
        utils::assert_input("\"a\"<=\"\"", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_less_than_same_case() {
        utils::assert_input("\"a\"<\"a\"", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_less_than_same_case_multiple_char() {
        utils::assert_input("\"abc\"<\"abc\"", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_less_than_diff_case() {
        utils::assert_input("\"a\"<\"A\"", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_less_than_diff_case_multiple_char() {
        utils::assert_input("\"abc\"<\"aBc\"", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_less_than_diff_case_multiple_char_diff_len() {
        utils::assert_input("\"a\"<\"aBc\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_less_than_diff_case_multiple_char_diff_len_two() {
        utils::assert_input("\"aBc\"<\"a\"", predicates::str::contains("0"));
    }

    #[test]
    fn compares_two_strings_less_than_empty_lhs() {
        utils::assert_input("\"\"<\"a\"", predicates::str::contains("1"));
    }

    #[test]
    fn compares_two_strings_less_than_empty_rhs() {
        utils::assert_input("\"a\"<\"\"", predicates::str::contains("0"));
    }
}
