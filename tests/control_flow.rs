//! Integration tests for control flow statements

pub mod utils;

#[cfg(test)]
mod control_flow {
    use crate::utils;

    #[test]
    fn it_skips_the_statement_when_the_expression_is_false() {
        utils::CodeRunner::init()
            .program("{if (0) print \"Should not print this\";}")
            .cli_options(vec!["-q"])
            .expect_empty_output()
            .assert()
    }

    #[test]
    fn it_allows_single_line_if_statements() {
        utils::CodeRunner::init()
            .program("{if (1) print \"Allows single line if statements\";}")
            .cli_options(vec!["-q"])
            .expect_output("Allows single line if statements")
            .assert()
    }

    #[test]
    fn it_supports_multiple_statements_in_the_if() {
        utils::CodeRunner::init()
            .program("{if (1) { foo = 2; print \"Should execute this many lines: \" foo; }}")
            .cli_options(vec!["-q"])
            .expect_output("Should execute this many lines: 2")
            .assert()
    }

    #[test]
    fn it_allows_multi_line_if_statements() {
        utils::CodeRunner::init()
            .program("{if (1)\nprint \"Allows single line if statements\";}")
            .cli_options(vec!["-q"])
            .expect_output("Allows single line if statements")
            .assert()
    }

    #[test]
    fn it_permits_nesting_if_statements() {
        utils::CodeRunner::init()
            .program("{if (1) if (2) print \"Nested if reached\";}")
            .cli_options(vec!["-q"])
            .expect_output("Nested if reached")
            .assert()
    }

    #[test]
    fn does_not_execute_a_child_if_statement_when_parent_false() {
        utils::CodeRunner::init()
            .program("{if (0) if (2) print \"Double if should not be reached\";}")
            .cli_options(vec!["-q"])
            .expect_empty_output()
            .assert();
    }

    #[test]
    fn it_binds_if_statements_that_are_ambiguous() {
        // the final if statement does not bind to the first one when no curly braces are used.
        // the code in the test is equivalent to:
        // if (0) {
        //   if (1) print "Should not print";
        // }
        // if (1) print "Should print";
        utils::CodeRunner::init()
            .program("{if (0) if (1) print \"Should not print\"; if (1) print \"Should print\";}")
            .cli_options(vec!["-q"])
            .expect_output("Should print")
            .assert()
    }

    #[test]
    fn it_supports_else_if_clauses() {
        utils::CodeRunner::init()
            .program(
                "{if (0) print \"You should not print me\"; else if (1) print \"else if hit\";}",
            )
            .cli_options(vec!["-q"])
            .expect_output("else if hit")
            .assert()
    }

    #[test]
    fn it_falls_through_else_if_when_falsy_() {
        utils::CodeRunner::init()
            .program("{foo = 123; if (0) foo = 456; else if (0) foo = 789; print foo;}")
            .cli_options(vec!["-q"])
            .expect_output("123")
            .assert()
    }

    #[test]
    fn it_supports_multiple_else_if_clauses() {
        utils::CodeRunner::init()
            .program("{foo = 123; if (0) foo = 456; else if (0) foo = 789; else if (1) foo = 9999; print foo;}")
            .cli_options(vec!["-q"])
            .expect_output("9999")
            .assert()
    }

    #[test]
    fn it_supports_else_clauses() {
        utils::CodeRunner::init()
            .program("{if (0) print \"should not print\"; else print \"else found\";}")
            .cli_options(vec!["-q"])
            .expect_output("else found")
            .assert()
    }

    #[test]
    fn skips_else_when_else_if_is_truthy() {
        utils::CodeRunner::init()
            .program(
                "{foo = 123; if (0) foo = 456; else if (1) foo = 789; else foo = 9999; print foo;}",
            )
            .cli_options(vec!["-q"])
            .expect_output("789")
            .assert()
    }

    #[test]
    fn it_supports_null_strings_in_if_condition() {
        utils::CodeRunner::init()
            .program("{if (\"\") print \"should not print\"; else print \"null string found\";}")
            .cli_options(vec!["-q"])
            .expect_output("null string found")
            .assert()
    }

    #[test]
    fn it_supports_non_null_strings_in_if_condition() {
        utils::CodeRunner::init()
            .program("{if (\"hello\") print \"should print\"; else print \"this should not\";}")
            .cli_options(vec!["-q"])
            .expect_output("should print")
            .assert()
    }

    #[test]
    fn it_supports_non_null_strnum_in_if_condition() {
        utils::CodeRunner::init()
            .program("{if ($1) print \"should print\"; else print \"this should not\";}")
            .stdin_data("hello")
            .expect_output("should print")
            .assert()
    }

    #[test]
    fn it_supports_null_strnum_in_if_condition() {
        utils::CodeRunner::init()
            .program("{if ($1) print \"should print\"; else print \"this should not\";}")
            .stdin_data("\n")
            .expect_output("this should not")
            .assert()
    }

    #[test]
    fn it_supports_strnum_in_if_condition_one() {
        utils::CodeRunner::init()
            .program("{if ($1) print \"should print\"; else print \"this should not\";}")
            .stdin_data("1")
            .expect_output("should print")
            .assert()
    }

    #[test]
    fn it_supports_strnum_in_if_condition_zero() {
        utils::CodeRunner::init()
            .program("{if ($1) print \"should print\"; else print \"this should not\";}")
            .stdin_data("0")
            .expect_output("this should not")
            .assert()
    }

    #[test]
    fn it_supports_multiple_statements_in_the_else() {
        utils::CodeRunner::init()
            .program("{if (0) print \"I should not print\"; else { foo = 2; print \"Should execute this many lines in else: \" foo; }}")
            .cli_options(vec!["-q"])
            .expect_output("Should execute this many lines in else: 2")
            .assert()
    }

    #[test]
    fn it_binds_else_to_the_correct_if_statement() {
        // the code in the test is equivalent to:
        // if (1) {
        //   if (0) { print "should not print"; }
        //   else { "else found"; }
        // }
        utils::CodeRunner::init()
            .program("{if (1) if (0) print \"should not print\"; else print \"else found\";}")
            .cli_options(vec!["-q"])
            .expect_output("else found")
            .assert()
    }

    #[test]
    fn it_binds_else_to_the_correct_if_statement_with_braces() {
        // the code in the test is equivalent to:
        // if (0) {
        //   if (1) {
        //     print "should not print";
        //   }
        // }
        // else { "else found for outer"; }
        utils::CodeRunner::init()
            .program("{if (0) { if (1) { print \"should not print\"; }} else { print \"else found for outer\";}}")
            .cli_options(vec!["-q"])
            .expect_output("else found for outer")
            .assert()
    }

    #[test]
    fn it_supports_logical_and_in_if_statements() {
        utils::CodeRunner::init()
            .program("{if (1 && 1) { print \"logical and is supported\"; }}")
            .cli_options(vec!["-q"])
            .expect_output("logical and is supported")
            .assert()
    }

    #[test]
    fn it_supports_logical_and_in_if_statements_to_be_falsy() {
        utils::CodeRunner::init()
            .program("{if (1 && 0) { print \"logical and is supported, but this should not run\"; } else { print \"but this should\"; }}")
            .cli_options(vec!["-q"])
            .expect_output("but this should")
            .assert()
    }

    #[test]
    fn it_allows_variable_assignment_in_if_with_logical_and() {
        utils::CodeRunner::init()
            .program("{if (1 && foo=2) { print foo; }}")
            .cli_options(vec!["-q"])
            .expect_output("2")
            .assert()
    }

    #[test]
    fn it_short_circuits_in_logical_and() {
        utils::CodeRunner::init()
            .program(
                "{if (1 && 0 && foo=0) { print \"this should not run\"; } else { print foo; }}",
            )
            .cli_options(vec!["-q"])
            .expect_output("")
            .assert()
    }

    #[test]
    fn it_still_sets_var_when_assignment_is_falsy_with_logical_and() {
        utils::CodeRunner::init()
            .program("{if (1 && foo=0) { print \"this should not run\"; } else { print foo; }}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn it_supports_logical_or_in_if_statements() {
        utils::CodeRunner::init()
            .program("{if (1 || 1) { print \"logical or is supported\"; }}")
            .cli_options(vec!["-q"])
            .expect_output("logical or is supported")
            .assert()
    }

    #[test]
    fn it_supports_logical_or_in_if_statements_to_be_falsy() {
        utils::CodeRunner::init()
            .program("{if (0 || 0) { print \"logical or is supported, but this should not run\"; } else { print \"but this should\"; }}")
            .cli_options(vec!["-q"])
            .expect_output("but this should")
            .assert()
    }

    #[test]
    fn it_allows_variable_assignment_in_if_with_logical_or() {
        utils::CodeRunner::init()
            .program("{if (0 || foo=2) { print foo; }}")
            .cli_options(vec!["-q"])
            .expect_output("2")
            .assert()
    }

    #[test]
    fn it_short_circuits_in_logical_or() {
        utils::CodeRunner::init()
            .program(
                "{if (0 || 1 || foo=123) { print foo; } else { print \"this should not run\"; }}",
            )
            .cli_options(vec!["-q"])
            .expect_output("")
            .assert()
    }

    #[test]
    fn it_still_sets_var_when_assignment_is_falsy_with_logical_or_truthy() {
        utils::CodeRunner::init()
            .program("{if (foo=1 || 0) { print foo; } else { print \"this should not run\"; }}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_still_sets_var_when_assignment_is_falsy_with_logical_or_falsy() {
        utils::CodeRunner::init()
            .program("{if (foo=0 || 0) { print \"this should not run\"; } else { print foo; }}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn it_allows_while_to_be_first_token() {
        utils::CodeRunner::init()
            .program("{while(a == 1) { a=1; } print a;}")
            .cli_options(vec!["-q"])
            .expect_output("")
            .assert()
    }

    #[test]
    fn it_runs_a_valid_while_loop_to_completion() {
        utils::CodeRunner::init()
            .program("{j=10; while(j > 0) { j=j-1; } print j;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn it_allows_a_while_loop_without_curly_braces() {
        utils::CodeRunner::init()
            .program("{j=10; while(j > 1) j=j-1; print j;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_considers_an_undefined_variable_to_be_falsy_in_while_loops() {
        utils::CodeRunner::init()
            .program("{while(a == 0) { a=1; } print a;}")
            .cli_options(vec!["-q"])
            .expect_output("")
            .assert()
    }

    #[test]
    fn it_allows_assignment_that_breaks_a_while_loop() {
        utils::CodeRunner::init()
            .program("{while(a = 0) { a=1; } print a;}")
            .cli_options(vec!["-q"])
            .expect_output("0")
            .assert()
    }

    #[test]
    fn it_allows_continue_to_be_place_in_while_loop() {
        utils::CodeRunner::init()
            .program("{while(i<1) { i=i+2; continue; print \"This should never print\"; } print \"The value of i is\", i;}")
            .cli_options(vec!["-q"])
            .expect_output("The value of i is 2")
            .assert()
    }

    #[test]
    fn it_supports_multiple_continue_blocks_in_a_while_loop() {
        // the code in the test is equivalent to:
        // while (i < 2) {
        //     i=i+1;
        //     while (j < 3) {
        //         j = j+1;
        //         continue;
        //         print "This is the j loop - this should not print";
        //     }
        //     continue;
        //     print "This is the i loop - this should not print";
        // }
        // print "i is", i, "and j is", j;
        utils::CodeRunner::init()
            .program("{while(i<2) { i=i+1; while (j < 3) { j = j+1; continue; print \"This is the j loop - this should not print\"; } continue; print \"This is the i loop - this should not print\"; } print \"i is\", i, \"and j is\", j;}")
            .cli_options(vec!["-q"])
            .expect_output("i is 2 and j is 3")
            .assert()
    }

    #[test]
    fn it_allows_break_to_be_placed_in_while_loop() {
        utils::CodeRunner::init()
            .program("{while(i<5) { i=i+3; break; print \"This should never print\"; } print \"The value of i is\", i;}")
            .cli_options(vec!["-q"])
            .expect_output("The value of i is 3")
            .assert()
    }

    #[test]
    fn it_supports_multiple_break_blocks_in_a_while_loop() {
        // the code in the test is equivalent to:
        // i = 0;
        // j = 0;
        // while (i < 2) {
        //     i=i+1;
        //     while (j < 3) {
        //         j = j+2;
        //         break;
        //         print "This is the j loop - this should not print";
        //     }
        //     break;
        //     print "This is the i loop - this should not print";
        // }
        // print "i is", i, "and j is", j;
        // For quick debugging (without escaping quotes):
        // i = 0; j = 0; while(i<2) { i=i+1; while (j < 3) { j = j+1; break; print "This is the j loop - this should not print"; } break; print "This is the i loop - this should not print"; } print "i is", i, "and j is", j;
        utils::CodeRunner::init()
            .program("{j=1; while(i<2) { i=i+1; while (j < 3) { j = j+1; break; print \"This is the j loop - this should not print\"; } break; print \"This is the i loop - this should not print\"; } print \"i is\", i, \"and j is\", j;}")
            .cli_options(vec!["-q"])
            .expect_output("i is 1 and j is 2")
            .assert()
    }

    #[test]
    fn it_supports_for_loop() {
        utils::CodeRunner::init()
            .program("{result = \"\";for (i=0; i<10; i=i+1) {result = result i;} print result;}")
            .cli_options(vec!["-q"])
            .expect_output("0123456789")
            .assert()
    }

    #[test]
    fn it_supports_for_loop_no_init() {
        utils::CodeRunner::init()
            .program("{result = \"\";for (;i<10; i=i+1) {result = result i;} print result;}")
            .cli_options(vec!["-q"])
            .expect_output("123456789")
            .assert()
    }

    #[test]
    fn it_supports_for_loop_no_condition() {
        utils::CodeRunner::init()
            .program("{result = \"\";for (i=0;; i=i+1) {result = result i; if (i>=10) { break; } } print result;}")
            .cli_options(vec!["-q"])
            .expect_output("012345678910")
            .assert()
    }

    #[test]
    fn it_supports_for_loop_no_incr() {
        utils::CodeRunner::init()
            .program("{result = \"\";for (i=0; i<10;) {result = result i; i=i+1;} print result;}")
            .cli_options(vec!["-q"])
            .expect_output("0123456789")
            .assert()
    }

    #[test]
    fn it_supports_break_in_for_loop() {
        // the code in the test is equivalent to:
        // result = "hell";
        // for (i=0; i<10; i=i+1) {
        //     result = result "," i;
        //     break;
        // }
        utils::CodeRunner::init()
            .program("{result = \"hell\";for (i=0; i<10; i=i+1) {result = result i; break;} print result;}")
            .cli_options(vec!["-q"])
            .expect_output("hell0")
            .assert()
    }

    #[test]
    fn it_supports_multiple_break_blocks_in_for_loop() {
        // the code in the test is equivalent to:
        // for (i=1; i < 2; i=i+1) {
        //     for (j=2 j < 3; j=j+2) {
        //         break;
        //         print "This is the j loop - this should not print";
        //     }
        //     break;
        //     print "This is the i loop - this should not print";
        // }
        // print "i is", i, "and j is", j;
        utils::CodeRunner::init()
            .program("{for (i=1; i < 2; i=i+1) { for (j=2; j < 3; j=j+2) { break; print \"This is the j loop - this should not print\"; } break; print \"This is the i loop - this should not print\";} print \"i is\", i, \"and j is\", j;}")
            .cli_options(vec!["-q"])
            .expect_output("i is 1 and j is 2")
            .assert()
    }

    #[test]
    fn it_supports_continue_in_for_loop() {
        // the code in the test is equivalent to:
        // result = "hell";
        // for (i=0; i<10; i=i+1) {
        //     result = result "," i;
        //     continue;
        //     result = "???";
        // }
        utils::CodeRunner::init()
            .program("{result = \"hell\";for (i=0; i<10; i=i+1) {result = result i; continue; result=\"???\";} print result;}")
            .cli_options(vec!["-q"])
            .expect_output("hell0123456789")
            .assert()
    }

    #[test]
    fn it_supports_multiple_continue_blocks_in_for_loop() {
        // the code in the test is equivalent to:
        // for (i=2; i<10; i=i+1) {
        //     for (j=3; j<=12; j=j+1) {
        //         continue;
        //         print "This is the j loop - this should not print";
        //     }
        //     continue;
        //     print "This is the i loop - this should not print";
        // }
        // print "i is", i, "and j is", j;
        utils::CodeRunner::init()
            .program("{for (i=2; i<10; i=i+1) { for (j=3; j<=12; j=j+1) { continue; print \"This is the j loop - this should not print\"; } continue; print \"This is the i loop - this should not print\"; } print \"i is\", i, \"and j is\", j;}")
            .cli_options(vec!["-q"])
            .expect_output("i is 10 and j is 13")
            .assert()
    }

    #[test]
    fn it_respects_break_before_continue_in_for() {
        // the code in the test is equivalent to:
        // for (i=1; i<2; i=i+1) {
        //     break;
        //     i = 99;
        //     continue;
        // }
        // print i;
        utils::CodeRunner::init()
            .program("{for (i=1; i<2; i=i+1) { break; i = 99; continue; } print i;}")
            .cli_options(vec!["-q"])
            .expect_output("1")
            .assert()
    }

    #[test]
    fn it_respects_continue_before_break_in_for() {
        // the code in the test is equivalent to:
        // for (i=0; i<=2; i=i+2) {
        //     continue;
        //     break;
        // }
        // print i;
        utils::CodeRunner::init()
            .program("{for (i=0; i<=2; i=i+2) { continue; break; } print i;}")
            .cli_options(vec!["-q"])
            .expect_output("4")
            .assert()
    }

    #[test]
    fn it_supports_ternary_expressions() {
        // the code in the test is equivalent to:
        // val = 1;
        // result = val ? 100 : 50;
        // print result;
        utils::CodeRunner::init()
            .program("{val = 1; result = val ? 100 : 50; print result;}")
            .cli_options(vec!["-q"])
            .expect_output("100")
            .assert()
    }

    #[test]
    fn it_supports_falsy_ternary_expressions() {
        // the code in the test is equivalent to:
        // val = 0;
        // result = val ? 100 : 50;
        // print result;
        utils::CodeRunner::init()
            .program("{val = 0; result = val ? 100 : 50; print result;}")
            .cli_options(vec!["-q"])
            .expect_output("50")
            .assert()
    }

    #[test]
    fn it_supports_nested_ternary_expressions() {
        // the code in the test is equivalent to:
        // val = 1;
        // result = val ? (val > 1 ? 2 : 3) : 50;
        // print result;
        utils::CodeRunner::init()
            .program("{val = 1; result = val ? (val > 1 ? 2 : 3) : 50; print result;}")
            .cli_options(vec!["-q"])
            .expect_output("3")
            .assert()
    }

    #[test]
    fn it_supports_ternary_expressions_in_statement() {
        // the code in the test is equivalent to:
        // val = 1;
        // print "The value is", (val > 2 ? "greater than" : "less than or equal to"), 2;
        utils::CodeRunner::init()
            .program("{val = 1; print \"The value is\", (val > 2 ? \"greater than\" : \"less than or equal to\"), 2;}")
            .cli_options(vec!["-q"])
            .expect_output("The value is less than or equal to 2")
            .assert()
    }
}
