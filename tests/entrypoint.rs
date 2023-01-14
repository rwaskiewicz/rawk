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
            .assert()
    }

    #[test]
    fn prints_version_info_neither_program_nor_file_flag_provided() {
        utils::CodeRunner::init()
            .expect_output(
                format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).as_str(),
            )
            .assert()
    }

    #[test]
    fn prints_nothing_when_empty_program_provided() {
        utils::CodeRunner::init()
            .program("")
            // mark this as a 'quick' test to prevent us from awaiting user input
            .cli_options(vec!["-q"])
            .expect_empty_output()
            .assert()
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
            .assert()
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
            .assert()
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

    #[test]
    fn accepts_a_data_file() {
        utils::CodeRunner::init()
            .program("{print $1, $2 * $3;}")
            .cli_options(vec!["./tests/data/hours1.dat"])
            .expect_output(
                r#"Alice 250
Bob 415
Charlie 610
Dan 0
Erin 660"#,
            )
            .assert();
    }

    #[test]
    fn prints_a_nothing_from_file_for_no_pattern_nor_action() {
        utils::CodeRunner::init()
            .program("")
            .cli_options(vec!["./tests/data/hours1.dat"])
            .expect_empty_output()
            .assert();
    }

    #[test]
    fn prints_a_data_file_for_no_action() {
        utils::CodeRunner::init()
            .program("1 > 0")
            .cli_options(vec!["./tests/data/hours1.dat"])
            .expect_output(
                r#"Alice    25.00  10
Bob      20.75  20
Charlie  15.25  40
Dan      21.50  0
Erin     22.00  30"#,
            )
            .assert();
    }

    #[test]
    fn accepts_an_awk_and_data_input_file() {
        utils::CodeRunner::init()
            .cli_options(vec![
                "-f",
                "./awk_examples/field_variables/it_prints_all_line_parts.awk",
                "./tests/data/hours1.dat",
            ])
            .expect_output(
                r#"Alice 25.00 10
Bob 20.75 20
Charlie 15.25 40
Dan 21.50 0
Erin 22.00 30"#,
            )
            .assert()
    }

    #[test]
    fn accepts_an_awk_and_data_input_files() {
        utils::CodeRunner::init()
            .cli_options(vec![
                "-f",
                "./awk_examples/field_variables/it_prints_all_line_parts.awk",
                "./tests/data/hours1.dat",
                "./tests/data/hours2.dat",
            ])
            .expect_output(
                r#"Alice 25.00 10
Bob 20.75 20
Charlie 15.25 40
Dan 21.50 0
Erin 22.00 30
Frank 15.00 11
Gerry 1.75 19
Hannah 25.50 40
Igor 0 10
Lauren 22.00 32"#,
            )
            .assert()
    }

    #[test]
    fn accepts_multiple_data_files() {
        utils::CodeRunner::init()
            .program("{print $1, $2 * $3;}")
            .cli_options(vec!["./tests/data/hours1.dat", "./tests/data/hours2.dat"])
            .expect_output(
                r#"Alice 250
Bob 415
Charlie 610
Dan 0
Erin 660
Frank 165
Gerry 33.25
Hannah 1020
Igor 0
Lauren 704"#,
            )
            .assert();
    }

    #[test]
    fn accepts_the_same_data_file_multiple_times() {
        utils::CodeRunner::init()
            .program("{print $1, $2 * $3;}")
            .cli_options(vec!["./tests/data/hours1.dat", "./tests/data/hours1.dat"])
            .expect_output(
                r#"Alice 250
Bob 415
Charlie 610
Dan 0
Erin 660
Alice 250
Bob 415
Charlie 610
Dan 0
Erin 660"#,
            )
            .assert();
    }

    #[test]
    fn processes_records_for_multiple_files_and_two_truthy_actions_correctly() {
        utils::CodeRunner::init()
            .program(r#"$2 * $3 > 600 {print $1,"needs to fill out a tax form";} $3 < 40 {print $1,"needs hours";}"#)
            .cli_options(vec!["./tests/data/hours1.dat", "./tests/data/hours2.dat"])
            .expect_output(r#"Alice needs hours
Bob needs hours
Charlie needs to fill out a tax form
Dan needs hours
Erin needs to fill out a tax form
Erin needs hours
Frank needs hours
Gerry needs hours
Hannah needs to fill out a tax form
Igor needs hours
Lauren needs to fill out a tax form
Lauren needs hours"#)
            .assert();
    }
    #[test]
    fn processes_records_for_multiple_files_and_multiple_actions_correctly() {
        utils::CodeRunner::init()
            .program(r#"{print "Looking at employee",$1;} $2 * $3 > 600 {print $1,"needs to fill out a tax form";} $2 * $3 < 600 {print $1,"does not need to fill out a tax form";}"#)
            .cli_options(vec!["./tests/data/hours1.dat", "./tests/data/hours2.dat"])
            .expect_output(r#"Looking at employee Alice
Alice does not need to fill out a tax form
Looking at employee Bob
Bob does not need to fill out a tax form
Looking at employee Charlie
Charlie needs to fill out a tax form
Looking at employee Dan
Dan does not need to fill out a tax form
Looking at employee Erin
Erin needs to fill out a tax form
Looking at employee Frank
Frank does not need to fill out a tax form
Looking at employee Gerry
Gerry does not need to fill out a tax form
Looking at employee Hannah
Hannah needs to fill out a tax form
Looking at employee Igor
Igor does not need to fill out a tax form
Looking at employee Lauren
Lauren needs to fill out a tax form"#)
            .assert();
    }

    #[test]
    fn panics_when_awk_file_and_program_literal_are_provided() {
        utils::CodeRunner::init()
            .cli_options(vec![
                "-f",
                "./awk_examples/field_variables/it_prints_all_line_parts.awk",
                "{print $0;}",
            ])
            .assert_fail();
    }
}
