//! Integration tests for arithmetic operations

mod utils;

#[cfg(test)]
mod arithmetic_tests {
    use crate::utils;

    #[test]
    fn it_sums_two_integers() {
        utils::assert_input(
            "1+2",
            predicates::str::contains(
                "Number(
        3.0,
    )",
            ),
        );
    }
}
