//! Integration tests that focus around blocks as found in actions

mod utils;

#[cfg(test)]
mod block_tests {
    use crate::utils;

    #[test]
    fn code_can_be_wrapped_in_a_block() {
        utils::assert_input("{foo = 123; print foo;}", "123");
    }

    #[test]
    fn it_shadows_variables_reassign_to_outer() {
        utils::assert_input("{foo = 2; { foo = 3; } print foo;}", "3");
    }

    #[test]
    fn inner_block_variables_do_not_get_collected() {
        utils::assert_input("{foo = 2; { bar = 3; } print foo + bar;}", "5");
    }
}
