//! Integration tests for field variables.

pub mod utils;

#[cfg(test)]
mod field_variables {
    use crate::utils;

    #[test]
    fn it_prints_the_whole_line() {
        // since we print $0, whitespace is preserved
        utils::CodeRunner::init()
            .program("{print $0;}")
            .stdin_data("Alice 40 25")
            .expect_output("Alice 40 25")
            .assert()
    }

    #[test]
    fn it_prints_nothing_for_out_of_bounds() {
        // since we do _not_ print $0, whitespace is not preserved
        utils::CodeRunner::init()
            .program("{print  $1 $500 $2;}")
            .stdin_data("Alice 40 25")
            .expect_output("Alice40")
            .assert()
    }

    #[test]
    fn it_prints_all_line_parts() {
        utils::CodeRunner::init()
            .program("{print $1,$2,$3;}")
            .stdin_data("Alice 40 25")
            .expect_output("Alice 40 25")
            .assert()
    }

    #[test]
    fn it_prints_line_parts() {
        utils::CodeRunner::init()
            .program("{print $2,$3;}")
            .stdin_data("Alice 40 25")
            .expect_output("40 25")
            .assert()
    }

    #[test]
    fn it_supports_concatenation() {
        utils::CodeRunner::init()
            .program("{print $2 $3;}")
            .stdin_data("Alice 40 25")
            .expect_output("4025")
            .assert()
    }

    #[test]
    fn it_nests_field_expressions() {
        // $($(1+1)); -> $($2); -> $3 -> 5
        utils::CodeRunner::init()
            .program("{print $($(1+1));}")
            .stdin_data("0 3 5")
            .expect_output("5")
            .assert()
    }

    #[test]
    fn it_does_not_truncate_whitespace_for_fs() {
        utils::CodeRunner::init()
            .program("{print $1$2$3;}")
            .cli_options(vec!["-F,"])
            .stdin_data(" Alice  ,40 ,25 ")
            .expect_output(" Alice  40 25 ")
            .assert()
    }

    #[test]
    fn it_supports_single_character_fs() {
        utils::CodeRunner::init()
            .program("{print $1$2$3;}")
            .cli_options(vec!["-F,"])
            .stdin_data("Alice4025")
            .expect_output("Alice4025")
            .assert()
    }

    #[test]
    fn it_counts_two_consecutive_fs_as_empty_record() {
        utils::CodeRunner::init()
            .program("{print $1$2$3;}")
            .cli_options(vec!["-F,"])
            .stdin_data("Hello,,World!")
            .expect_output("HelloWorld!")
            .assert()
    }

    #[test]
    fn it_splits_nothing_when_fs_not_found() {
        utils::CodeRunner::init()
            .program("{print $1;}")
            .cli_options(vec!["-F:"])
            .stdin_data("Hello,,World!")
            .expect_output("Hello,,World!")
            .assert()
    }

    #[test]
    fn it_splits_data_entirely_when_fs_matches_test_data_single_char() {
        utils::CodeRunner::init()
            .program(r#"{print "b"$1"b"$2"b";}"#)
            .cli_options(vec!["-Fa"])
            .stdin_data("a")
            .expect_output("bbb")
            .assert()
    }

    #[test]
    fn it_splits_data_when_fs_matches_leading_char() {
        utils::CodeRunner::init()
            .program("{print $1$2$3;}")
            .cli_options(vec!["-Fa"])
            .stdin_data("abac")
            .expect_output("bc")
            .assert()
    }

    #[test]
    fn it_splits_data_when_fs_matches_trailing_char() {
        utils::CodeRunner::init()
            .program("{print $1$2$3;}")
            .cli_options(vec!["-Fa"])
            .stdin_data("baca")
            .expect_output("bc")
            .assert()
    }

    #[test]
    fn it_supports_unary_operations() {
        utils::CodeRunner::init()
            .program("{print -$1;}")
            .stdin_data("40")
            .expect_output("-40")
            .assert()
    }

    #[test]
    fn it_supports_unary_operations_out_of_bounds() {
        utils::CodeRunner::init()
            .program("{print -$2;}")
            .stdin_data("40")
            .expect_output("0")
            .assert()
    }
}
