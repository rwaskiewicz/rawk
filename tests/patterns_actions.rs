//! Integration tests for logical operations

mod utils;

#[cfg(test)]
mod patterns_actions {
    use crate::utils;

    #[test]
    fn it_supports_a_pattern_only() {
        utils::assert_input_with_data("1 > 0", "Hello World", "Hello World");
    }

    #[test]
    fn it_supports_skips_implicit_action() {
        utils::assert_input_yields_empty_with_data("1 - 1", "Hello");
    }

    #[test]
    fn it_allows_more_than_one_action() {
        utils::assert_input_for_outputs(
            "{print \"Hello\";}{print \"World!\";}",
            vec![String::from("Hello"), String::from("World!")],
        );
    }

    #[test]
    fn it_correctly_manages_data_between_actions() {
        utils::assert_input_for_outputs_with_data(
            "{print foo=$1;}{print foo*2;}{print foo;}",
            "32",
            vec![String::from("32"), String::from("64"), String::from("32")],
        );
    }

    #[test]
    fn it_correctly_manages_data_between_actions_mutate() {
        utils::assert_input_for_outputs_with_data(
            "{print foo=$1;}{print foo*=2;}{print foo+2;}",
            "32",
            vec![String::from("32"), String::from("64"), String::from("66")],
        );
    }

    #[test]
    fn it_supports_an_action_and_pattern() {
        utils::assert_input("1 > 0 {print \"Hello\";}", "Hello");
    }

    #[test]
    fn it_supports_an_action_and_pattern_falsy() {
        utils::assert_input_yields_empty("0 > 1 {print \"Hello\";}");
    }

    #[test]
    fn it_supports_an_action_and_pattern_null_str() {
        utils::assert_input_yields_empty("\"\" {print \"Hello\";}");
    }

    #[test]
    fn it_supports_an_action_and_pattern_one() {
        utils::assert_input("1 {print \"Hello\";}", "Hello");
    }

    #[test]
    fn it_supports_an_action_and_pattern_zero() {
        utils::assert_input_yields_empty("0 {print \"Hello\";}");
    }

    #[test]
    fn it_supports_an_action_and_pattern_valid_str() {
        utils::assert_input("\"str\" {print \"Hello World\";}", "Hello World");
    }

    #[test]
    fn it_supports_an_action_and_pattern_empty_assign() {
        utils::assert_input_yields_empty("foo=\"\" {print \"Hello World\";}");
    }

    #[test]
    fn it_supports_an_action_and_pattern_valid_assign() {
        utils::assert_input("foo=\"str\" {print \"Hello World\";}", "Hello World");
    }
}
