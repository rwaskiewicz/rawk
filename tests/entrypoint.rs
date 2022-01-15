//! Integration tests for the entrypoint of the CLI

mod utils;

#[cfg(test)]
mod entrypoint {
    use crate::utils;

    #[test]
    fn prints_version_info_when_the_version_flag_provided() {
        utils::run_rawk(
            None,
            vec!["-V"],
            None,
            Some(format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).as_str()),
        );
    }

    #[test]
    fn prints_version_info_neither_program_nor_file_flag_provided() {
        utils::run_rawk(
            None,
            vec![],
            None,
            Some(format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).as_str()),
        );
    }

    #[test]
    fn prints_nothing_when_empty_program_provided() {
        // mark this as a 'quick' test to prevent us from awaiting user input
        utils::run_rawk(Some(""), vec!["-q"], None, None);
    }

    #[test]
    fn runs_an_awk_program_from_file() {
        utils::run_rawk(
            None,
            vec![
                "-f",
                "./awk_examples/field_variables/it_prints_all_line_parts.awk",
            ],
            Some("Alice 40 25"),
            Some("Alice 40 25"),
        );
    }

    #[test]
    fn runs_an_awk_program_from_multiple_files() {
        utils::run_rawk(
            None,
            vec![
                "-f",
                "./awk_examples/field_variables/it_prints_all_line_parts.awk",
                "-f",
                "./awk_examples/field_variables/it_prints_line_parts.awk",
            ],
            Some("Alice 40 25"),
            Some("Alice 40 25\n40 25"),
        );
    }

    #[test]
    #[should_panic(expected = "FileDoesNotExist")]
    fn panics_for_a_non_existent_file() {
        // TODO: Fix test util to not require Some() for expected output
        utils::run_rawk(None, vec!["-f", "./does_not_exist.awk"], None, Some(""));
    }

    #[test]
    #[should_panic(expected = "FileDoesNotExist")]
    fn panics_for_a_non_existent_file_many_given() {
        // TODO: Fix test util to not require Some() for expected output
        utils::run_rawk(
            None,
            vec![
                "-f",
                "./awk_examples/field_variables/it_prints_all_line_parts.awk",
                "-f",
                "./does_not_exist.awk",
            ],
            None,
            Some(""),
        );
    }
}
