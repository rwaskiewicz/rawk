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

    #[test]
    fn it_supports_multiple_statements_in_the_if() {
        utils::assert_input(
            "if (1) { foo = 2; print \"Should execute this many lines: \" foo; }",
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
    fn it_supports_else_if_clauses() {
        utils::assert_input(
            "if (0) print \"You should not print me\"; else if (1) print \"else if hit\";",
            "else if hit",
        );
    }

    #[test]
    fn it_falls_through_else_if_when_falsy_() {
        utils::assert_input(
            "foo = 123; if (0) foo = 456; else if (0) foo = 789; print foo;",
            "123",
        );
    }

    #[test]
    fn it_supports_multiple_else_if_clauses() {
        utils::assert_input("foo = 123; if (0) foo = 456; else if (0) foo = 789; else if (1) foo = 9999; print foo;", "9999");
    }

    #[test]
    fn it_supports_else_clauses() {
        utils::assert_input(
            "if (0) print \"should not print\"; else print \"else found\";",
            "else found",
        );
    }

    #[test]
    fn skips_else_when_else_if_is_truthy() {
        utils::assert_input(
            "foo = 123; if (0) foo = 456; else if (1) foo = 789; else foo = 9999; print foo;",
            "789",
        );
    }

    #[test]
    fn it_supports_null_strings() {
        utils::assert_input(
            "if (\"\") print \"should not print\"; else print \"null string found\";",
            "null string found",
        );
    }

    #[test]
    fn it_supports_multiple_statements_in_the_else() {
        utils::assert_input("if (0) print \"I should not print\"; else { foo = 2; print \"Should execute this many lines in else: \" foo; }",
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

    #[test]
    // if (0) {
    //   if (1) {
    //     print "should not print";
    //   }
    // }
    // else { "should print"; }
    fn it_binds_else_to_the_correct_if_statement_with_braces() {
        utils::assert_input(
            "if (0) { if (1) { print \"should not print\"; }} else { print \"else found for outer\";}",
            "else found for outer",
        );
    }

    #[test]
    fn it_supports_logical_and_in_if_statements() {
        utils::assert_input(
            "if (1 && 1) { print \"logical and is supported\"; }",
            "logical and is supported",
        );
    }

    #[test]
    fn it_supports_logical_and_in_if_statements_to_be_falsy() {
        utils::assert_input("if (1 && 0) { print \"logical and is supported, but this should not run\"; } else { print \"but this should\"; }", "but this should");
    }

    #[test]
    fn it_allows_variable_assignment_in_if_with_logical_and() {
        utils::assert_input("if (1 && foo=2) { print foo; }", "2");
    }

    #[test]
    fn it_short_circuits_in_logical_and() {
        utils::assert_input(
            "if (1 && 0 && foo=0) { print \"this should not run\"; } else { print foo; }",
            "",
        );
    }

    #[test]
    fn it_still_sets_var_when_assignment_is_falsy_with_logical_and() {
        utils::assert_input(
            "if (1 && foo=0) { print \"this should not run\"; } else { print foo; }",
            "0",
        );
    }

    #[test]
    fn it_supports_logical_or_in_if_statements() {
        utils::assert_input(
            "if (1 || 1) { print \"logical or is supported\"; }",
            "logical or is supported",
        );
    }

    #[test]
    fn it_supports_logical_or_in_if_statements_to_be_falsy() {
        utils::assert_input("if (0 || 0) { print \"logical or is supported, but this should not run\"; } else { print \"but this should\"; }", "but this should");
    }

    #[test]
    fn it_allows_variable_assignment_in_if_with_logical_or() {
        utils::assert_input("if (0 || foo=2) { print foo; }", "2");
    }

    #[test]
    fn it_short_circuits_in_logical_or() {
        utils::assert_input(
            "if (0 || 1 || foo=123) { print foo; } else { print \"this should not run\"; }",
            "",
        );
    }

    #[test]
    fn it_still_sets_var_when_assignment_is_falsy_with_logical_or_truthy() {
        utils::assert_input(
            "if (foo=1 || 0) { print foo; } else { print \"this should not run\"; }",
            "1",
        );
    }

    #[test]
    fn it_still_sets_var_when_assignment_is_falsy_with_logical_or_falsy() {
        utils::assert_input(
            "if (foo=0 || 0) { print \"this should not run\"; } else { print foo; }",
            "0",
        );
    }

    #[test]
    fn it_allows_while_to_be_first_token() {
        utils::assert_input("while(a == 1) { a=1; } print a;", "");
    }

    #[test]
    fn it_runs_a_valid_while_loop_to_completion() {
        utils::assert_input("j=10; while(j > 0) { j=j-1; } print j;", "0");
    }

    #[test]
    fn it_allows_a_while_loop_without_curly_braces() {
        utils::assert_input("j=10; while(j > 1) j=j-1; print j;", "1");
    }

    #[test]
    fn it_considers_an_undefined_variable_to_be_falsy_in_while_loops() {
        utils::assert_input("while(a == 0) { a=1; } print a;", "");
    }

    #[test]
    fn it_allows_assignment_that_breaks_a_while_loop() {
        utils::assert_input("while(a = 0) { a=1; } print a;", "0");
    }

    #[test]
    fn it_allows_continue_to_be_place_in_while_loop() {
        utils::assert_input("while(i<1) { i=i+2; continue; print \"This should never print\"; } print \"The value of i is\", i;", "The value of i is 2")
    }

    /// while (i < 2) {
    ///     i=i+1;
    ///     while (j < 3) {
    ///         j = j+1;
    ///         continue;
    ///         print "This is the j loop - this should not print";
    ///     }
    ///     continue;
    ///     print "This is the i loop - this should not print";
    /// }
    /// print "i is", i, "and j is", j;
    /// For quick debugging (without escaping quotes):
    /// while(i<2) { i=i+1; while (j < 3) { j = j+1; continue; print "This is the j loop - this should not print"; } continue; print "This is the i loop - this should not print"; } print "i is", i, "and j is", j;
    #[test]
    fn it_supports_multiple_continue_blocks_in_a_while_loop() {
        utils::assert_input("while(i<2) { i=i+1; while (j < 3) { j = j+1; continue; print \"This is the j loop - this should not print\"; } continue; print \"This is the i loop - this should not print\"; } print \"i is\", i, \"and j is\", j;", "i is 2 and j is 3");
    }

    #[test]
    fn it_allows_break_to_be_place_in_while_loop() {
        utils::assert_input("while(i<5) { i=i+3; break; print \"This should never print\"; } print \"The value of i is\", i;", "The value of i is 3")
    }

    /// i = 0;
    /// j = 0;
    /// while (i < 2) {
    ///     i=i+1;
    ///     while (j < 3) {
    ///         j = j+2;
    ///         break;
    ///         print "This is the j loop - this should not print";
    ///     }
    ///     break;
    ///     print "This is the i loop - this should not print";
    /// }
    /// print "i is", i, "and j is", j;
    /// For quick debugging (without escaping quotes):
    /// i = 0; j = 0; while(i<2) { i=i+1; while (j < 3) { j = j+1; break; print "This is the j loop - this should not print"; } break; print "This is the i loop - this should not print"; } print "i is", i, "and j is", j;
    #[test]
    fn it_supports_multiple_break_blocks_in_a_while_loop() {
        utils::assert_input("j=1; while(i<2) { i=i+1; while (j < 3) { j = j+1; break; print \"This is the j loop - this should not print\"; } break; print \"This is the i loop - this should not print\"; } print \"i is\", i, \"and j is\", j;", "i is 1 and j is 2");
    }

    #[test]
    fn it_supports_for_loop() {
        utils::assert_input(
            "result = \"\";for (i=0; i<10; i=i+1) {result = result i;} print result;",
            "0123456789",
        );
    }

    #[test]
    fn it_supports_for_loop_no_init() {
        utils::assert_input(
            "result = \"\";for (;i<10; i=i+1) {result = result i;} print result;",
            "123456789",
        );
    }

    #[test]
    fn it_supports_for_loop_no_condition() {
        utils::assert_input(
            "result = \"\";for (i=0;; i=i+1) {result = result i; if (i>=10) { break; } } print result;",
            "012345678910",
        );
    }

    #[test]
    fn it_supports_for_loop_no_incr() {
        utils::assert_input(
            "result = \"\";for (i=0; i<10;) {result = result i; i=i+1;} print result;",
            "0123456789",
        );
    }

    /// result = "hell";
    /// for (i=0; i<10; i=i+1) {
    ///     result = result "," i;
    ///     break;
    /// }
    #[test]
    fn it_supports_break_in_for_loop() {
        utils::assert_input(
            "result = \"hell\";for (i=0; i<10; i=i+1) {result = result i; break;} print result;",
            "hell0",
        );
    }

    /// for (i=1; i < 2; i=i+1) {
    ///     for (j=2 j < 3; j=j+2) {
    ///         break;
    ///         print "This is the j loop - this should not print";
    ///     }
    ///     break;
    ///     print "This is the i loop - this should not print";
    /// }
    /// print "i is", i, "and j is", j;
    #[test]
    fn it_supports_multiple_break_blocks_in_for_loop() {
        utils::assert_input("for (i=1; i < 2; i=i+1) { for (j=2; j < 3; j=j+2) { break; print \"This is the j loop - this should not print\"; } break; print \"This is the i loop - this should not print\";} print \"i is\", i, \"and j is\", j;" ,"i is 1 and j is 2", );
    }

    /// result = "hell";
    /// for (i=0; i<10; i=i+1) {
    ///     result = result "," i;
    ///     continue;
    ///     result = "???";
    /// }
    #[test]
    fn it_supports_continue_in_for_loop() {
        utils::assert_input(
            "result = \"hell\";for (i=0; i<10; i=i+1) {result = result i; continue; result=\"???\";} print result;",
            "hell0123456789",
        );
    }

    /// for (i=2; i<10; i=i+1) {
    ///     for (j=3; j<=12; j=j+1) {
    ///         continue;
    ///         print "This is the j loop - this should not print";
    ///     }
    ///     continue;
    ///     print "This is the i loop - this should not print";
    /// }
    /// print "i is", i, "and j is", j;
    #[test]
    fn it_supports_multiple_continue_blocks_in_for_loop() {
        utils::assert_input("for (i=2; i<10; i=i+1) { for (j=3; j<=12; j=j+1) { continue; print \"This is the j loop - this should not print\"; } continue; print \"This is the i loop - this should not print\"; } print \"i is\", i, \"and j is\", j;", "i is 10 and j is 13");
    }

    /// for (i=1; i<2; i=i+1) {
    ///     break;
    ///     i = 99;
    ///     continue;
    /// }
    /// print i;
    #[test]
    fn it_respects_break_before_continue_in_for() {
        utils::assert_input(
            "for (i=1; i<2; i=i+1) { break; i = 99; continue; } print i;",
            "1",
        );
    }

    /// for (i=0; i<=2; i=i+2) {
    ///     continue;
    ///     break;
    /// }
    /// print i;
    #[test]
    fn it_respects_continue_before_break_in_for() {
        utils::assert_input("for (i=0; i<=2; i=i+2) { continue; break; } print i;", "4");
    }
}
