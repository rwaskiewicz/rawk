//! Integration tests for control flow statements

mod utils;

#[cfg(test)]
mod control_flow {
    use crate::utils;

    #[test]
    fn it_skips_the_statement_when_the_expression_is_false() {
        utils::assert_input_yields_empty("if (0) print \"Should not print this\";");
    }

    #[test]
    fn it_allows_single_line_if_statements() {
        utils::assert_input(
            "if (1) print \"Allows single line if statements\";",
            "Allows single line if statements",
        );
    }

    // TODO: Need actual block support
    #[test]
    #[ignore]
    fn it_supports_multiple_statements_in_the_if() {
        utils::assert_input(
            "if (1) { foo = 2; print \"Should execute this many lines: \" foo;",
            "Should execute this many lines: 2",
        );
    }

    // TODO: Need to allow the repl (which is the basis of this test fw) to handle line breaks
    #[test]
    #[ignore]
    fn it_allows_multi_line_if_statements() {
        utils::assert_input(
            "if (1)\nprint \"Allows single line if statements\";",
            "Allows single line if statements",
        );
    }

    #[test]
    fn it_permits_nesting_if_statements() {
        utils::assert_input(
            "if (1) if (2) print \"Double if reached\";",
            "Double if reached",
        );
    }

    #[test]
    fn does_not_execute_a_child_if_statment_when_parent_false() {
        utils::assert_input_yields_empty(
            "if (0) if (2) print \"Double if should not be reached\";",
        );
    }

    #[test]
    fn it_binds_if_statements_that_are_ambiguous() {
        // the final if statement does not bind to the first one:
        // if (0) {
        //   if (1) print "Should not print";
        // }
        // if (1) print "Should print";
        utils::assert_input(
            "if (0) if (1) print \"Should not print\"; if (1) print \"Should print\";",
            "Should print",
        );
    }

    #[test]
    fn it_supports_else_clauses() {
        utils::assert_input(
            "if (0) print \"should not print\"; else print \"else found\";",
            "else found",
        );
    }

    #[test]
    fn it_supports_null_strings() {
        utils::assert_input(
            "if (\"\") print \"should not print\"; else print \"null string found\";",
            "null string found",
        );
    }

    // TODO Need actual block support
    #[test]
    #[ignore]
    fn it_supports_multiple_statements_in_the_else() {
        utils::assert_input("if (0) print \"I should not print\"; else { foo = 2; print \"Should execute this many lines in else: \" foo;",
                            "Should execute this many lines in else: 2");
    }

    #[test]
    // if (1) {
    //   if (0) print "should not print";
    //   else "should print";
    // }
    fn it_binds_else_to_the_correct_if_statement() {
        utils::assert_input(
            "if (1) if (0) print \"should not print\"; else print \"else found\";",
            "else found",
        );
    }
}
