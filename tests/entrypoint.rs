//! Integration tests for the entrypoint of the CLI

pub mod utils;

#[cfg(test)]
mod entrypoint {
    use crate::utils;

    #[test]
    fn prints_version_info_when_the_version_flag_provided() {
        utils::CodeRunner::init()
            .cli_options(vec!["-V"])
            .expect_output(
                format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).as_str(),
            )
            .assert();
    }

    #[test]
    fn prints_version_info_neither_program_nor_file_flag_provided() {
        utils::CodeRunner::init()
            .expect_output(
                format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).as_str(),
            )
            .assert();
    }

    #[test]
    fn prints_nothing_when_empty_program_provided() {
        utils::CodeRunner::init()
            .program("")
            // mark this as a 'quick' test to prevent us from awaiting user input
            .cli_options(vec!["-q"])
            .expect_empty_output()
            .assert();
    }

    #[test]
    fn runs_an_awk_program_from_file() {
        utils::CodeRunner::init()
            .cli_options(vec![
                "-f",
                "./awk_examples/field_variables/it_prints_all_line_parts.awk",
            ])
            .stdin_data("Alice 40 25")
            .expect_output("Alice 40 25")
            .assert();
    }

    #[test]
    fn runs_an_awk_program_from_multiple_files() {
        utils::CodeRunner::init()
            .cli_options(vec![
                "-f",
                "./awk_examples/field_variables/it_prints_all_line_parts.awk",
                "-f",
                "./awk_examples/field_variables/it_prints_line_parts.awk",
            ])
            .stdin_data("Alice 40 25")
            .expect_output("Alice 40 25\n40 25")
            .assert();
    }

    #[test]
    fn panics_for_a_non_existent_file() {
        utils::CodeRunner::init()
            .cli_options(vec!["-f", "./does_not_exist.awk"])
            .assert_fail();
    }

    #[test]
    fn panics_for_a_non_existent_file_many_given() {
        utils::CodeRunner::init()
            .cli_options(vec![
                "-f",
                "./awk_examples/field_variables/it_prints_all_line_parts.awk",
                "-f",
                "./does_not_exist.awk",
            ])
            .assert_fail();
    }
}
