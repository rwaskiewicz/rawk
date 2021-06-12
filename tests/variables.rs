//! Integration tests for user defined variables

mod utils;

#[cfg(test)]
mod variable_tests {
    use crate::utils;

    #[test]
    fn it_stores_a_value_and_reads_it_back() {
        utils::assert_input("price = \"4.99\"; print price;", "4.99");
    }

    #[test]
    fn it_stores_and_updates_a_value() {
        utils::assert_input("price = \"4.99\"; price = price + 1; print price;", "5.99");
    }

    #[test]
    fn it_reassigns_a_value() {
        utils::assert_input("price = \"4.99\"; price = 2; print price;", "2");
    }

    #[test]
    fn defines_a_from_mention() {
        utils::assert_input("print price;", "");
    }

    #[test]
    fn defines_an_empty_value() {
        utils::assert_input("price; print price;", "");
    }

    #[test]
    fn creates_a_new_var_from_simple_copy() {
        utils::assert_input("foo = 23; bar = foo; print bar;", "23");
    }

    #[test]
    fn creates_a_new_var_from_existing() {
        utils::assert_input("foo = 23; bar = foo * foo; print bar;", "529");
    }

    #[test]
    fn allows_assignment_in_print_statement() {
        utils::assert_input("print foo=3;", "3");
    }

    #[test]
    fn allows_assignment_in_print_statement_with_premature_ref_arithmetic() {
        utils::assert_input("print foo=3*2+foo;", "6");
    }

    // TODO: Validate that assignment does not work for cases like:
    // utils::assert_input("a=2;b=3;c=7;d=11;print a*b=c+d;", "Error at \'TODO: This is a shortsighted part of the lexeme\': Expect \';\' at the end of a statement.");

    // TODO Re-enable when comma support is added
    // #[test]
    // fn allows_assignment_in_print_statement() {
    //     utils::assert_input("print foo=3,2;print foo;;", "3 2\n3");
    // }
}
