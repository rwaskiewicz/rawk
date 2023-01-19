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

//     #[test]
//     fn it_sets_nf_per_input_line() {
//         utils::CodeRunner::init()
//             .program("{ print NF; }")
//             .stdin_data(
//                 r#"hello world
// i
// come in
// peace"#,
//             )
//             .expect_output(
//                 r#"2
// 1
// 2
// 1"#,
//             )
//             .assert();
//     }
//
//     #[test]
//     fn it_sets_nf_per_input_line_with_assign() {
//         utils::CodeRunner::init()
//             .program("{ if (NF == 2) { NF = 23; } print NF; }")
//             .stdin_data(
//                 r#"hello world
// i
// come in
// peace"#,
//             )
//             .expect_output(
//                 r#"23
// 1
// 23
// 1"#,
//             )
//             .assert();
//     }
}
