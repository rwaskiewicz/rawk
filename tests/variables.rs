//! Integration tests for user defined variables

pub mod utils;

#[cfg(test)]
mod variable_tests {
    use crate::utils;

    #[test]
    fn it_stores_a_value_and_reads_it_back() {
        utils::CodeRunner::init()
            .program(r#"{price = "4.99"; print price;}"#)
            .cli_options(vec!["-q"])
            .expect_output("4.99")
            .assert()
    }

    #[test]
    fn it_stores_and_updates_a_value() {
        utils::CodeRunner::init()
            .program(r#"{price = "4.99"; price = price + 1; print price;}"#)
            .cli_options(vec!["-q"])
            .expect_output("5.99")
            .assert()
    }

    #[test]
    fn it_reassigns_a_value() {
        utils::CodeRunner::init()
            .program(r#"{price = "4.99"; price = 2; print price;}"#)
            .cli_options(vec!["-q"])
            .expect_output("2")
            .assert()
    }

    #[test]
    fn defines_a_variable_from_mention() {
        utils::CodeRunner::init()
            .program("{print price;}")
            .cli_options(vec!["-q"])
            .expect_output("")
            .assert()
    }

    #[test]
    fn defines_an_empty_value() {
        utils::CodeRunner::init()
            .program("{price; print price;}")
            .cli_options(vec!["-q"])
            .expect_output("")
            .assert()
    }

    #[test]
    fn creates_a_new_var_from_simple_copy() {
        utils::CodeRunner::init()
            .program("{foo = 23; bar = foo; print bar;}")
            .cli_options(vec!["-q"])
            .expect_output("23")
            .assert()
    }

    #[test]
    fn creates_a_new_var_from_existing() {
        utils::CodeRunner::init()
            .program("{foo = 23; bar = foo * foo; print bar;}")
            .cli_options(vec!["-q"])
            .expect_output("529")
            .assert()
    }

    #[test]
    fn allows_assignment_in_print_statement() {
        utils::CodeRunner::init()
            .program("{print foo=3;}")
            .cli_options(vec!["-q"])
            .expect_output("3")
            .assert()
    }

    #[test]
    fn allows_assignment_in_print_statement_with_premature_ref_arithmetic() {
        utils::CodeRunner::init()
            .program("{print foo=3*2+foo;}")
            .cli_options(vec!["-q"])
            .expect_output("6")
            .assert()
    }

    #[test]
    fn allows_assignment_in_print_statement_w_comma_operator() {
        utils::CodeRunner::init()
            .program("{print foo=3,2;print foo;}")
            .cli_options(vec!["-q"])
            .expect_output("3 2\n3")
            .assert()
    }
}
