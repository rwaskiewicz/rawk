//! Integration tests for user defined variables

mod utils;

#[cfg(test)]
mod variable_tests {
    use crate::utils;

    #[test]
    fn it_stores_a_value_and_reads_it_back() {
        utils::run_rawk(
            Some("{price = \"4.99\"; print price;}"),
            vec!["-q"],
            None,
            Some("4.99"),
        );
    }

    #[test]
    fn it_stores_and_updates_a_value() {
        utils::run_rawk(
            Some("{price = \"4.99\"; price = price + 1; print price;}"),
            vec!["-q"],
            None,
            Some("5.99"),
        );
    }

    #[test]
    fn it_reassigns_a_value() {
        utils::run_rawk(
            Some("{price = \"4.99\"; price = 2; print price;}"),
            vec!["-q"],
            None,
            Some("2"),
        );
    }

    #[test]
    fn defines_a_variable_from_mention() {
        utils::run_rawk(Some("{print price;}"), vec!["-q"], None, Some(""));
    }

    #[test]
    fn defines_an_empty_value() {
        utils::run_rawk(Some("{price; print price;}"), vec!["-q"], None, Some(""));
    }

    #[test]
    fn creates_a_new_var_from_simple_copy() {
        utils::run_rawk(
            Some("{foo = 23; bar = foo; print bar;}"),
            vec!["-q"],
            None,
            Some("23"),
        );
    }

    #[test]
    fn creates_a_new_var_from_existing() {
        utils::run_rawk(
            Some("{foo = 23; bar = foo * foo; print bar;}"),
            vec!["-q"],
            None,
            Some("529"),
        );
    }

    #[test]
    fn allows_assignment_in_print_statement() {
        utils::run_rawk(Some("{print foo=3;}"), vec!["-q"], None, Some("3"));
    }

    #[test]
    fn allows_assignment_in_print_statement_with_premature_ref_arithmetic() {
        utils::run_rawk(Some("{print foo=3*2+foo;}"), vec!["-q"], None, Some("6"));
    }

    // TODO: Validate that assignment does not work for cases like:
    // utils::run_rawk(Some("{a=2;b=3;c=7;d=11;print a*b=c+d;}"), vec!["-q"], None,  Some("Error at \'TODO: This is a shortsighted part of the lexeme\': Expect \';\' at the end of a statement."));

    #[test]
    fn allows_assignment_in_print_statement_w_comma_operator() {
        utils::run_rawk(
            Some("{print foo=3,2;print foo;}"),
            vec!["-q"],
            None,
            Some("3 2\n3"),
        );
    }
}
