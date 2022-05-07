//! Integration tests for logical operations

pub mod utils;

#[cfg(test)]
mod patterns_actions {
    use crate::utils;

    #[test]
    fn it_supports_an_awk_pattern_with_no_action() {
        utils::CodeRunner::init()
            .program("1 > 0")
            .stdin_data("Hello World")
            .expect_output("Hello World")
            .assert()
    }

    #[test]
    fn it_supports_skipping_an_implicit_action() {
        utils::CodeRunner::init()
            .program("1 - 1")
            .cli_options(vec!["-q"])
            .stdin_data("Hello")
            .expect_empty_output()
            .assert()
    }

    #[test]
    fn it_allows_more_than_one_action() {
        utils::CodeRunner::init()
            .program(r#"{print "Hello";}{print "World!";}"#)
            .cli_options(vec!["-q"])
            .expect_output("Hello\nWorld!")
            .assert()
    }

    #[test]
    fn it_correctly_manages_data_between_actions() {
        utils::CodeRunner::init()
            .program("{print foo=$1;}{print foo*2;}{print foo;}")
            .stdin_data("32")
            .expect_output("32\n64\n32")
            .assert()
    }

    #[test]
    fn it_correctly_manages_data_between_actions_mutate() {
        utils::CodeRunner::init()
            .program("{print foo=$1;}{print foo*=2;}{print foo+2;}")
            .stdin_data("32")
            .expect_output("32\n64\n66")
            .assert()
    }

    #[test]
    fn it_supports_an_action_and_pattern() {
        utils::CodeRunner::init()
            .program(r#"1 > 0 {print "Hello";}"#)
            .cli_options(vec!["-q"])
            .expect_output("Hello")
            .assert()
    }

    #[test]
    fn it_supports_an_action_and_pattern_falsy() {
        utils::CodeRunner::init()
            .program(r#"0 > 1 {print "Hello";}"#)
            .cli_options(vec!["-q"])
            .expect_empty_output()
            .assert()
    }

    #[test]
    fn it_supports_an_action_and_pattern_null_str() {
        utils::CodeRunner::init()
            .program(r#""" {print "Hello";}"#)
            .cli_options(vec!["-q"])
            .expect_empty_output()
            .assert()
    }

    #[test]
    fn it_supports_an_action_and_pattern_one() {
        utils::CodeRunner::init()
            .program(r#"1 {print "Hello";}"#)
            .cli_options(vec!["-q"])
            .expect_output("Hello")
            .assert()
    }

    #[test]
    fn it_supports_an_action_and_pattern_zero() {
        utils::CodeRunner::init()
            .program(r#"0 {print "Hello";}"#)
            .cli_options(vec!["-q"])
            .expect_empty_output()
            .assert()
    }

    #[test]
    fn it_supports_an_action_and_pattern_valid_str() {
        utils::CodeRunner::init()
            .program(r#""str" {print "Hello World";}"#)
            .cli_options(vec!["-q"])
            .expect_output("Hello World")
            .assert()
    }

    #[test]
    fn it_supports_an_action_and_pattern_empty_assign() {
        utils::CodeRunner::init()
            .program(r#"foo="" {print "Hello World";}"#)
            .cli_options(vec!["-q"])
            .expect_empty_output()
            .assert()
    }

    #[test]
    fn it_supports_an_action_and_pattern_valid_assign() {
        utils::CodeRunner::init()
            .program(r#"foo="str" {print "Hello World";}"#)
            .cli_options(vec!["-q"])
            .expect_output("Hello World")
            .assert()
    }
}
