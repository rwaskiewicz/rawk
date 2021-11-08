//! Integration tests for field variables.

mod utils;

#[cfg(test)]
mod field_variables {
    use crate::utils;

    #[test]
    fn it_prints_the_whole_line() {
        // since we print $0, whitespace is preserved
        utils::assert_input_with_data("print $0;", "Alice 40 25", "Alice 40 25");
    }

    #[test]
    fn it_prints_nothing_for_out_of_bounds() {
        // since we do _not_ print $0, whitespace is not preserved
        utils::assert_input_with_data("print  $1 $500 $2;", "Alice 40 25", "Alice40");
    }

    #[test]
    fn it_prints_all_line_parts() {
        utils::assert_input_with_data("print $1,$2,$3;", "Alice 40 25", "Alice 40 25");
    }

    #[test]
    fn it_prints_line_parts() {
        utils::assert_input_with_data("print $2,$3;", "Alice 40 25", "40 25");
    }

    #[test]
    fn it_supports_concatenation() {
        utils::assert_input_with_data("print $2 $3;", "Alice 40 25", "4025");
    }

    #[test]
    fn it_nests_field_expressions() {
        //  $($(1+1)); -> $($2); -> $3 -> 5
        utils::assert_input_with_data("print $($(1+1));", "0 3 5", "5");
    }
}
