//! Integration tests that focus around blocks as found in actions

pub mod utils;

#[cfg(test)]
mod block_tests {
    use crate::utils;

    #[test]
    fn code_can_be_wrapped_in_a_block() {
        utils::CodeRunner::init()
            .program("{foo = 123; print foo;}")
            .cli_options(vec!["-q"])
            .expect_output("123")
            .assert()
    }

    #[test]
    fn it_shadows_variables_reassign_to_outer() {
        utils::CodeRunner::init()
            .program("{foo = 2; { foo = 3; } print foo;}")
            .cli_options(vec!["-q"])
            .expect_output("3")
            .assert()
    }

    #[test]
    fn inner_block_variables_do_not_get_collected() {
        utils::CodeRunner::init()
            .program("{foo = 2; { bar = 3; } print foo + bar;}")
            .cli_options(vec!["-q"])
            .expect_output("5")
            .assert()
    }
}
