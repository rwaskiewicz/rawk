//! Integration tests for relational operations

mod utils;

#[cfg(test)]
mod relational_tests {
    use crate::utils;

    #[test]
    fn compares_two_integers_with_greater_than_false() {
        utils::run_rawk(Some("{print 1>2;}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_two_integers_with_greater_than_true() {
        utils::run_rawk(Some("{print 2>1;}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_integers_with_less_than_false() {
        utils::run_rawk(Some("{print 2<1;}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_two_integers_with_less_than_true() {
        utils::run_rawk(Some("{print 1<2;}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_integers_with_greater_equal_than_false() {
        utils::run_rawk(Some("{print 1>=2;}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_two_integers_with_greater_equal_than_true() {
        utils::run_rawk(Some("{print 3>=2;}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_integers_with_greater_equal_than_same_val_true() {
        utils::run_rawk(Some("{print 2>=2;}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_integers_with_less_equal_than_false() {
        utils::run_rawk(Some("{print 1<=0;}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_two_integers_with_less_equal_than_true() {
        utils::run_rawk(Some("{print 1<=2;}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_integers_with_less_equal_than_same_val_true() {
        utils::run_rawk(Some("{print 2<=2;}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_integers_with_double_equal_true() {
        utils::run_rawk(Some("{print 2==2;}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_integers_with_double_equal_false() {
        utils::run_rawk(Some("{print 1==2;}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_two_integers_with_not_equal_true() {
        utils::run_rawk(Some("{print 1!=2;}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_integers_with_not_equal_false() {
        utils::run_rawk(Some("{print 2!=2;}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_two_strings_double_equal_same_case() {
        utils::run_rawk(Some("{print \"a\"==\"a\";}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_strings_double_equal_same_case_multiple_char() {
        utils::run_rawk(
            Some("{print \"abc\"==\"abc\";}"),
            vec!["-q"],
            None,
            Some("1"),
        );
    }

    #[test]
    fn compares_two_strings_double_equal_diff_case() {
        utils::run_rawk(Some("{print \"a\"==\"A\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_two_strings_double_equal_diff_case_multiple_char() {
        utils::run_rawk(
            Some("{print \"abc\"==\"aBc\";}"),
            vec!["-q"],
            None,
            Some("0"),
        );
    }

    #[test]
    fn compares_two_strings_double_equal_empty_lhs() {
        utils::run_rawk(Some("{print \"\"==\"a\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_two_strings_double_equal_empty_rhs() {
        utils::run_rawk(Some("{print \"a\"==\"\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_two_strings_not_equal_same_case() {
        utils::run_rawk(Some("{print \"a\"!=\"a\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_two_strings_not_equal_same_case_multiple_char() {
        utils::run_rawk(
            Some("{print \"abc\"!=\"abc\";}"),
            vec!["-q"],
            None,
            Some("0"),
        );
    }

    #[test]
    fn compares_two_strings_not_equal_diff_case() {
        utils::run_rawk(Some("{print \"a\"!=\"A\";}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_strings_not_equal_diff_case_multiple_char() {
        utils::run_rawk(
            Some("{print \"abc\"!=\"aBc\";}"),
            vec!["-q"],
            None,
            Some("1"),
        );
    }

    #[test]
    fn compares_two_strings_not_equal_empty_lhs() {
        utils::run_rawk(Some("{print \"\"!=\"a\";}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_strings_not_equal_empty_rhs() {
        utils::run_rawk(Some("{print \"a\"!=\"\";}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_strings_greater_equal_same_case() {
        utils::run_rawk(Some("{print \"a\">=\"a\";}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_strings_greater_equal_same_case_multiple_char() {
        utils::run_rawk(
            Some("{print \"abc\">=\"abc\";}"),
            vec!["-q"],
            None,
            Some("1"),
        );
    }

    #[test]
    fn compares_two_strings_greater_equal_diff_case() {
        utils::run_rawk(Some("{print \"a\">=\"A\";}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_strings_greater_equal_diff_case_multiple_char() {
        utils::run_rawk(
            Some("{print \"abc\">=\"aBc\";}"),
            vec!["-q"],
            None,
            Some("1"),
        );
    }

    #[test]
    fn compares_two_strings_greater_equal_diff_case_multiple_char_diff_len() {
        utils::run_rawk(Some("{print \"a\">=\"aBc\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_two_strings_greater_equal_diff_case_multiple_char_diff_len_two() {
        utils::run_rawk(Some("{print \"aBc\">=\"a\";}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_strings_greater_equal_empty_lhs() {
        utils::run_rawk(Some("{print \"\">=\"a\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_two_strings_greater_equal_empty_rhs() {
        utils::run_rawk(Some("{print \"a\">=\"\";}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_strings_greater_same_case() {
        utils::run_rawk(Some("{print \"a\">\"a\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_two_strings_greater_same_case_multiple_char() {
        utils::run_rawk(
            Some("{print \"abc\">\"abc\";}"),
            vec!["-q"],
            None,
            Some("0"),
        );
    }

    #[test]
    fn compares_two_strings_greater_diff_case() {
        utils::run_rawk(Some("{print \"a\">\"A\";}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_strings_greater_diff_case_multiple_char() {
        utils::run_rawk(
            Some("{print \"abc\">\"aBc\";}"),
            vec!["-q"],
            None,
            Some("1"),
        );
    }

    #[test]
    fn compares_two_strings_greater_diff_case_multiple_char_diff_len() {
        utils::run_rawk(Some("{print \"a\">\"aBc\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_two_strings_greater_diff_case_multiple_char_diff_len_two() {
        utils::run_rawk(Some("{print \"aBc\">\"a\";}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_strings_greater_empty_lhs() {
        utils::run_rawk(Some("{print \"\">\"a\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_two_strings_greater_empty_rhs() {
        utils::run_rawk(Some("{print \"a\">\"\";}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_strings_less_than_equal_same_case() {
        utils::run_rawk(Some("{print \"a\"<=\"a\";}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_strings_less_than_equal_same_case_multiple_char() {
        utils::run_rawk(
            Some("{print \"abc\"<=\"abc\";}"),
            vec!["-q"],
            None,
            Some("1"),
        );
    }

    #[test]
    fn compares_two_strings_less_than_equal_diff_case() {
        utils::run_rawk(Some("{print \"a\"<=\"A\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_two_strings_less_than_equal_diff_case_multiple_char() {
        utils::run_rawk(
            Some("{print \"abc\"<=\"aBc\";}"),
            vec!["-q"],
            None,
            Some("0"),
        );
    }

    #[test]
    fn compares_two_strings_less_than_equal_diff_case_multiple_char_diff_len() {
        utils::run_rawk(Some("{print \"a\"<=\"aBc\";}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_strings_less_than_equal_diff_case_multiple_char_diff_len_two() {
        utils::run_rawk(Some("{print \"aBc\"<=\"a\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_two_strings_less_than_equal_empty_lhs() {
        utils::run_rawk(Some("{print \"\"<=\"a\";}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_strings_less_than_equal_empty_rhs() {
        utils::run_rawk(Some("{print \"a\"<=\"\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_two_strings_less_than_same_case() {
        utils::run_rawk(Some("{print \"a\"<\"a\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_two_strings_less_than_same_case_multiple_char() {
        utils::run_rawk(
            Some("{print \"abc\"<\"abc\";}"),
            vec!["-q"],
            None,
            Some("0"),
        );
    }

    #[test]
    fn compares_two_strings_less_than_diff_case() {
        utils::run_rawk(Some("{print \"a\"<\"A\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_two_strings_less_than_diff_case_multiple_char() {
        utils::run_rawk(
            Some("{print \"abc\"<\"aBc\";}"),
            vec!["-q"],
            None,
            Some("0"),
        );
    }

    #[test]
    fn compares_two_strings_less_than_diff_case_multiple_char_diff_len() {
        utils::run_rawk(Some("{print \"a\"<\"aBc\";}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_strings_less_than_diff_case_multiple_char_diff_len_two() {
        utils::run_rawk(Some("{print \"aBc\"<\"a\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_two_strings_less_than_empty_lhs() {
        utils::run_rawk(Some("{print \"\"<\"a\";}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_two_strings_less_than_empty_rhs() {
        utils::run_rawk(Some("{print \"a\"<\"\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_a_number_and_string_double_equal() {
        utils::run_rawk(Some("{print \"a\"==1;}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_zero_and_empty_string_double_equal() {
        utils::run_rawk(Some("{print \"\"==0;}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_a_number_and_string_not_equal() {
        utils::run_rawk(Some("{print \"a\"!=1;}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_a_number_and_string_greater_equal() {
        utils::run_rawk(Some("{print \"a\">=1;}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_a_number_and_string_greater_equal_symmetric() {
        utils::run_rawk(Some("{print 1>=\"a\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_a_number_and_string_greater() {
        utils::run_rawk(Some("{print \"a\">1;}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_a_number_and_string_greater_symmetric() {
        utils::run_rawk(Some("{print 1>\"a\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_a_number_and_string_less_than_equal() {
        utils::run_rawk(Some("{print \"a\"<=1;}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_a_number_and_string_less_than_equal_symmetric() {
        utils::run_rawk(Some("{print 1<=\"a\";}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_a_number_and_string_less_than() {
        utils::run_rawk(Some("{print \"a\"<1;}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_a_number_and_string_less_than_symmetric() {
        utils::run_rawk(Some("{print 1<\"a\";}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_a_string_number_and_a_number_double_equal() {
        utils::run_rawk(Some("{print \"1.1\"==1.1;}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_a_string_number_and_a_number_double_equal_false() {
        utils::run_rawk(Some("{print 1==\"1.1\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_a_string_number_and_a_number_not_equals() {
        utils::run_rawk(Some("{print \"1.1\"!=1;}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_a_string_number_and_a_number_not_equals_false() {
        utils::run_rawk(Some("{print 1.1!=\"1.1\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_a_string_number_and_a_number_greater_than_equal() {
        utils::run_rawk(Some("{print \"1.1\">=1;}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_a_string_number_and_a_number_greater_than_equal_symmetric() {
        utils::run_rawk(Some("{print 1>=\"1.1\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_a_string_number_and_a_number_greater_than() {
        utils::run_rawk(Some("{print \"1.1\">1;}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_a_string_number_and_a_number_greater_than_lex() {
        utils::run_rawk(Some("{print \"2\">12;}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_a_string_number_and_a_number_greater_than_symmetric() {
        utils::run_rawk(Some("{print 1>\"1.1\";}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_a_string_number_and_a_number_less_than_equal() {
        utils::run_rawk(Some("{print \"1.1\"<=1;}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_a_string_number_and_a_number_less_than_equal_symmetric() {
        utils::run_rawk(Some("{print 1<=\"1.1\";}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_a_string_number_and_a_number_less_than() {
        utils::run_rawk(Some("{print \"1.1\"<1;}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_a_string_number_and_a_number_less_than_lex() {
        utils::run_rawk(Some("{print \"2\"<12;}"), vec!["-q"], None, Some("0"));
    }

    #[test]
    fn compares_a_string_number_and_a_number_less_than_symmetric() {
        utils::run_rawk(Some("{print 1<\"1.1\";}"), vec!["-q"], None, Some("1"));
    }

    #[test]
    fn compares_a_strnum_and_a_number_double_equal() {
        utils::run_rawk(Some("{print $1==1.1;}"), vec![], Some("1.1"), Some("1"));
    }

    #[test]
    fn compares_a_strnum_and_a_number_double_equal_false() {
        utils::run_rawk(Some("{print 1==$1;}"), vec![], Some("1.1"), Some("0"));
    }

    #[test]
    fn compares_a_strnum_and_a_number_not_equals() {
        utils::run_rawk(Some("{print $1!=1;}"), vec![], Some("1.1"), Some("1"));
    }

    #[test]
    fn compares_a_strnum_and_a_number_not_equals_false() {
        utils::run_rawk(Some("{print 1.1!=$1;}"), vec![], Some("1.1"), Some("0"));
    }

    #[test]
    fn compares_a_strnum_and_a_number_greater_than_equal() {
        utils::run_rawk(Some("{print $1>=1;}"), vec![], Some("1.1"), Some("1"));
    }

    #[test]
    fn compares_a_strnum_and_a_number_greater_than_equal_symmetric() {
        utils::run_rawk(Some("{print 1>=$1;}"), vec![], Some("1.1"), Some("0"));
    }

    #[test]
    fn compares_a_strnum_and_a_number_greater_than() {
        utils::run_rawk(Some("{print $1>1;}"), vec![], Some("1.1"), Some("1"));
    }

    #[test]
    fn compares_a_strnum_and_a_number_greater_than_lex() {
        utils::run_rawk(Some("{print $1>12;}"), vec![], Some("2"), Some("0"));
    }

    #[test]
    fn compares_a_strnum_and_a_number_greater_than_symmetric() {
        utils::run_rawk(Some("{print 1>$1;}"), vec![], Some("1.1"), Some("0"));
    }

    #[test]
    fn compares_a_strnum_and_a_number_less_than_equal() {
        utils::run_rawk(Some("{print $1<=1;}"), vec![], Some("1.1"), Some("0"));
    }

    #[test]
    fn compares_a_strnum_and_a_number_less_than_equal_symmetric() {
        utils::run_rawk(Some("{print 1<=$1;}"), vec![], Some("1.1"), Some("1"));
    }

    #[test]
    fn compares_a_strnum_and_a_number_less_than() {
        utils::run_rawk(Some("{print $1<1;}"), vec![], Some("1.1"), Some("0"));
    }

    #[test]
    fn compares_a_strnum_and_a_number_less_than_lex() {
        utils::run_rawk(Some("{print $1<12;}"), vec![], Some("2"), Some("1"));
    }

    #[test]
    fn compares_a_strnum_and_a_number_less_than_symmetric() {
        utils::run_rawk(Some("{print 1<$1;}"), vec![], Some("1.1"), Some("1"));
    }
}
