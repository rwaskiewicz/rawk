//! Integration tests for field variables.

mod utils;

#[cfg(test)]
mod field_variables {
    use crate::utils;

    #[test]
    fn it_prints_the_whole_line() {
        // since we print $0, whitespace is preserved
        utils::run_rawk(
            Some("{print $0;}"),
            vec![],
            Some("Alice 40 25"),
            Some("Alice 40 25"),
        );
    }

    #[test]
    fn it_prints_nothing_for_out_of_bounds() {
        // since we do _not_ print $0, whitespace is not preserved
        utils::run_rawk(
            Some("{print  $1 $500 $2;}"),
            vec![],
            Some("Alice 40 25"),
            Some("Alice40"),
        );
    }

    #[test]
    fn it_prints_all_line_parts() {
        utils::run_rawk(
            Some("{print $1,$2,$3;}"),
            vec![],
            Some("Alice 40 25"),
            Some("Alice 40 25"),
        );
    }

    #[test]
    fn it_prints_line_parts() {
        utils::run_rawk(
            Some("{print $2,$3;}"),
            vec![],
            Some("Alice 40 25"),
            Some("40 25"),
        );
    }

    #[test]
    fn it_supports_concatenation() {
        utils::run_rawk(
            Some("{print $2 $3;}"),
            vec![],
            Some("Alice 40 25"),
            Some("4025"),
        );
    }

    #[test]
    fn it_nests_field_expressions() {
        //  $($(1+1)); -> $($2); -> $3 -> 5
        utils::run_rawk(Some("{print $($(1+1));}"), vec![], Some("0 3 5"), Some("5"));
    }

    #[test]
    fn it_does_not_truncate_whitespace_for_fs() {
        utils::run_rawk(
            Some("{print $1$2$3;}"),
            vec!["-F,"],
            Some(" Alice  ,40 ,25 "),
            Some(" Alice  40 25 "),
        );
    }

    #[test]
    fn it_supports_single_character_fs() {
        utils::run_rawk(
            Some("{print $1$2$3;}"),
            vec!["-F,"],
            Some("Alice4025"),
            Some("Alice4025"),
        );
    }

    #[test]
    fn it_counts_two_consecutive_fs_as_empty_record() {
        utils::run_rawk(
            Some("{print $1$2$3;}"),
            vec!["-F,"],
            Some("Hello,,World!"),
            Some("HelloWorld!"),
        );
    }

    #[test]
    fn it_splits_nothing_when_fs_not_found() {
        utils::run_rawk(
            Some("{print $1;}"),
            vec!["-F:"],
            Some("Hello,,World!"),
            Some("Hello,,World!"),
        );
    }

    #[test]
    fn it_splits_data_entirely_when_fs_matches_test_data_single_char() {
        utils::run_rawk(
            Some("{print \"|\"$1\"|\"$2\"|\";}"),
            vec!["-Fa"],
            Some("a"),
            Some("|||"),
        );
    }

    #[test]
    fn it_splits_data_when_fs_matches_leading_char() {
        utils::run_rawk(
            Some("{print $1$2$3;}"),
            vec!["-Fa"],
            Some("abac"),
            Some("bc"),
        );
    }

    #[test]
    fn it_splits_data_when_fs_matches_trailing_char() {
        utils::run_rawk(
            Some("{print $1$2$3;}"),
            vec!["-Fa"],
            Some("baca"),
            Some("bc"),
        );
    }
}
