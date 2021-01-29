//! Integration tests for relational operations

mod utils;

#[cfg(test)]
mod relational_tests {
    use crate::utils;

    #[test]
    fn compares_two_integers_with_greater_than_false() {
        utils::assert_input("1>2", predicates::str::contains("0.0"));
    }

    #[test]
    fn compares_two_integers_with_greater_than_true() {
        utils::assert_input("2>1", predicates::str::contains("1.0"));
    }

    #[test]
    fn compares_two_integers_with_less_than_false() {
        utils::assert_input("2<1", predicates::str::contains("0.0"));
    }

    #[test]
    fn compares_two_integers_with_less_than_true() {
        utils::assert_input("1<2", predicates::str::contains("1.0"));
    }

    #[test]
    fn compares_two_integers_with_greater_equal_than_false() {
        utils::assert_input("1>=2", predicates::str::contains("0.0"));
    }

    #[test]
    fn compares_two_integers_with_greater_equal_than_true() {
        utils::assert_input("3>=2", predicates::str::contains("1.0"));
    }

    #[test]
    fn compares_two_integers_with_greater_equal_than_same_val_true() {
        utils::assert_input("2>=2", predicates::str::contains("1.0"));
    }

    #[test]
    fn compares_two_integers_with_less_equal_than_false() {
        utils::assert_input("1<=0", predicates::str::contains("0.0"));
    }

    #[test]
    fn compares_two_integers_with_less_equal_than_true() {
        utils::assert_input("1<=2", predicates::str::contains("1.0"));
    }

    #[test]
    fn compares_two_integers_with_less_equal_than_same_val_true() {
        utils::assert_input("2<=2", predicates::str::contains("1.0"));
    }

    #[test]
    fn compares_two_integers_with_double_equal_true() {
        utils::assert_input("2==2", predicates::str::contains("1.0"));
    }

    #[test]
    fn compares_two_integers_with_double_equal_false() {
        utils::assert_input("1==2", predicates::str::contains("0.0"));
    }

    #[test]
    fn compares_two_integers_with_not_equal_true() {
        utils::assert_input("1!=2", predicates::str::contains("1.0"));
    }

    #[test]
    fn compares_two_integers_with_not_equal_false() {
        utils::assert_input("2!=2", predicates::str::contains("0.0"));
    }
}
