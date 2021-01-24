//! Integration tests for logical operations

mod utils;

#[cfg(test)]
mod logical_tests {
    use crate::utils;

    #[test]
    fn it_performs_logical_not_on_zero() {
        utils::assert_input(
            "!0",
            predicates::str::contains(
                "Number(
        1.0,
    )",
            ),
        );
    }

    #[test]
    fn it_performs_logical_not_on_non_zero_integer() {
        utils::assert_input(
            "!9",
            predicates::str::contains(
                "Number(
        0.0,
    )",
            ),
        );
    }

    #[test]
    fn it_performs_double_logical_not_on_zero() {
        utils::assert_input(
            "!!0",
            predicates::str::contains(
                "Number(
        0.0,
    )",
            ),
        );
    }

    #[test]
    fn it_performs_double_logical_not_on_non_zero_integer() {
        utils::assert_input(
            "!!9",
            predicates::str::contains(
                "Number(
        1.0,
    )",
            ),
        );
    }
}
