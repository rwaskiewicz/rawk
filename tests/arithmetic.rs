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

    #[test]
    fn it_subtracts_two_integers() {
        utils::assert_input(
            "1-2",
            predicates::str::contains(
                "Number(
        -1.0,
    )",
            ),
        );
    }

    #[test]
    fn it_multiplies_two_integers() {
        utils::assert_input(
            "3*2",
            predicates::str::contains(
                "Number(
        6.0,
    )",
            ),
        );
    }

    #[test]
    fn it_divides_two_integers() {
        utils::assert_input(
            "6/2",
            predicates::str::contains(
                "Number(
        3.0,
    )",
            ),
        );
    }

    #[test]
    fn it_finds_the_modulo_of_two_integers() {
        utils::assert_input(
            "3%2",
            predicates::str::contains(
                "Number(
        1.0,
    )",
            ),
        );
    }

    #[test]
    fn it_finds_the_calculates_the_power_of_two_integers() {
        utils::assert_input(
            "3^2",
            predicates::str::contains(
                "Number(
        9.0,
    )",
            ),
        );
    }

    #[test]
    fn it_finds_the_calculates_the_power_of_three_integers() {
        utils::assert_input(
            "3^2^3",
            predicates::str::contains(
                "Number(
        6561.0,
    )",
            ),
        );
    }
}
