//! Integration tests for relational operations

mod utils;

#[cfg(test)]
mod relational_tests {
    use crate::utils;

    #[test]
    fn compares_two_integers_with_greater_than() {
        utils::assert_input(
            "1>2",
            predicates::str::contains(
                "Number(
        0.0,
    )",
            ),
        );
    }
}
