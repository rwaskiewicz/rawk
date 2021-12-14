//! Integration tests for logical operations

mod utils;

#[cfg(test)]
mod patterns_actions {
    use crate::utils;

    #[test]
    fn it_supports_a_pattern_only() {
        utils::run_rawk(
            Some("1 > 0"),
            vec![],
            Some("Hello World"),
            Some("Hello World"),
        );
    }

    #[test]
    fn it_supports_skips_implicit_action() {
        utils::run_rawk(Some("1 - 1"), vec!["-q"], Some("Hello"), None);
    }

    #[test]
    fn it_allows_more_than_one_action() {
        utils::run_rawk(
            Some("{print \"Hello\";}{print \"World!\";}"),
            vec!["-q"],
            None,
            Some("Hello\nWorld!"),
        );
    }

    #[test]
    fn it_correctly_manages_data_between_actions() {
        utils::run_rawk(
            Some("{print foo=$1;}{print foo*2;}{print foo;}"),
            vec![],
            Some("32"),
            Some("32\n64\n32"),
        );
    }

    #[test]
    fn it_correctly_manages_data_between_actions_mutate() {
        utils::run_rawk(
            Some("{print foo=$1;}{print foo*=2;}{print foo+2;}"),
            vec![],
            Some("32"),
            Some("32\n64\n66"),
        );
    }

    #[test]
    fn it_supports_an_action_and_pattern() {
        utils::run_rawk(
            Some("1 > 0 {print \"Hello\";}"),
            vec!["-q"],
            None,
            Some("Hello"),
        );
    }

    #[test]
    fn it_supports_an_action_and_pattern_falsy() {
        utils::run_rawk(Some("0 > 1 {print \"Hello\";}"), vec!["-q"], None, None);
    }

    #[test]
    fn it_supports_an_action_and_pattern_null_str() {
        utils::run_rawk(Some("\"\" {print \"Hello\";}"), vec!["-q"], None, None);
    }

    #[test]
    fn it_supports_an_action_and_pattern_one() {
        utils::run_rawk(
            Some("1 {print \"Hello\";}"),
            vec!["-q"],
            None,
            Some("Hello"),
        );
    }

    #[test]
    fn it_supports_an_action_and_pattern_zero() {
        utils::run_rawk(Some("0 {print \"Hello\";}"), vec!["-q"], None, None);
    }

    #[test]
    fn it_supports_an_action_and_pattern_valid_str() {
        utils::run_rawk(
            Some("\"str\" {print \"Hello World\";}"),
            vec!["-q"],
            None,
            Some("Hello World"),
        );
    }

    #[test]
    fn it_supports_an_action_and_pattern_empty_assign() {
        utils::run_rawk(
            Some("foo=\"\" {print \"Hello World\";}"),
            vec!["-q"],
            None,
            None,
        );
    }

    #[test]
    fn it_supports_an_action_and_pattern_valid_assign() {
        utils::run_rawk(
            Some("foo=\"str\" {print \"Hello World\";}"),
            vec!["-q"],
            None,
            Some("Hello World"),
        );
    }
}
