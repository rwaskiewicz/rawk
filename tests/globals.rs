//! Integration tests for the global variables

pub mod utils;

#[cfg(test)]
mod globals {
    use crate::utils;

    #[test]
    fn it_allows_nf_assign() {
        utils::CodeRunner::init()
            .program(r#"{ print "NF",NF; NF=23; print "NF",NF; }"#)
            .stdin_data("hello world")
            .expect_output(
                r#"NF 2
NF 23"#,
            )
            .assert();
    }

    #[test]
    fn it_allows_nf_in_pattern_for_file() {
        utils::CodeRunner::init()
            .program("NF > 2 { print $0; }")
            .cli_options(vec!["./tests/data/variable_data.dat"])
            .expect_output("I come in peace!")
            .assert();
    }

    #[test]
    fn it_allows_nf_in_pattern_for_data() {
        utils::CodeRunner::init()
            .program("NF > 2 { print $0; }")
            .stdin_data("I come in peace!")
            .expect_output("I come in peace!")
            .assert();
    }

    #[test]
    fn it_prints_nf() {
        utils::CodeRunner::init()
            .program("{ print NF; }")
            .stdin_data("hello world, i come in peace!")
            .expect_output("6")
            .assert();
    }

    #[test]
    fn it_prints_nf_in_field_variable() {
        utils::CodeRunner::init()
            .program("{ print $NF; }")
            .stdin_data("hello world")
            .expect_output("world")
            .assert();
    }

    #[test]
    fn it_prints_nf_in_field_variable_empty_input() {
        utils::CodeRunner::init()
            .program("{ print $NF; }")
            .cli_options(vec!["-q"])
            // we need the implicit newline here
            .expect_output("")
            .assert();
    }

    #[test]
    // TODO: Restore once field variable assignment is implemented
    #[ignore]
    fn foo() {
        // However, assigning to a nonexistent field (for example, $(NF+2)=5) shall increase the
        // value of NF; create any intervening fields with the uninitialized value...
        utils::CodeRunner::init()
            .program("{ $(NF+1)=5; print NF; print $0; }")
            .stdin_data("hello world")
            .expect_output(
                r#"3
hello world 5"#,
            )
            .assert();
    }

    // Uninitialized variables include all types of variables including scalars, array elements, and
    // fields. The definition of an uninitialized value in Variables and Special Variables is
    // necessary to describe the value placed on uninitialized variables and on fields that are
    // valid (for example, < $NF) but have no characters in them and to describe how these variables
    // are to be used in comparisons. A valid field, such as $1, that has no characters in it can be
    // obtained from an input line of "\t\t" when FS= '\t'. Historically, the comparison ($1<10) was
    // done numerically after evaluating $1 to the value zero.

    #[test]
    // TODO: Restore once we support multiline input in the test runner
    #[ignore]
    fn it_sets_nf_per_input_line() {
        utils::CodeRunner::init()
            .program("{ print NF; }")
            .stdin_data(
                r#"hello world
i
come in
peace"#,
            )
            .expect_output(
                r#"2
1
2
1"#,
            )
            .assert();
    }

    #[test]
    // TODO: Restore once we support multiline input in the test runner
    #[ignore]
    fn it_sets_nf_per_input_line_with_assign() {
        utils::CodeRunner::init()
            .program("{ if (NF == 2) { NF = 23; } print NF; }")
            .stdin_data(
                r#"hello world
i
come in
peace"#,
            )
            .expect_output(
                r#"23
1
23
1"#,
            )
            .assert();
    }
}
