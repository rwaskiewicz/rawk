//! Integration tests for logical operations

mod utils;

#[cfg(test)]
mod patterns_actions {
    use crate::utils;

    #[test]
    fn it_allows_more_than_one_pattern() {
        utils::assert_input_for_outputs(
            "{print \"Hello\";}{print \"World!\";}",
            vec![String::from("Hello"), String::from("World!")],
        );
    }

    #[test]
    fn it_correctly_manages_data_between_patterns() {
        utils::assert_input_for_outputs_with_data(
            "{print foo=$1;}{print foo*2;}{print foo;}",
            "32",
            vec![String::from("32"), String::from("64"), String::from("32")],
        );
    }

    #[test]
    fn it_correctly_manages_data_between_patterns_mutate() {
        utils::assert_input_for_outputs_with_data(
            "{print foo=$1;}{print foo*=2;}{print foo+2;}",
            "32",
            vec![String::from("32"), String::from("64"), String::from("66")],
        );
    }
}
