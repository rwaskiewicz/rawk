//! Common integration test utilities

use assert_cmd::Command;
use predicates::str::ContainsPredicate;

/// Helper library for invoking r-awk and asserting on output in stderr
///
/// # Arguments:
/// - `input` the input that would have been received by the user
/// - `expected_text` the expected result to appear in stderr
///
/// # Panics:
/// If the `expected_text` cannot be found in stderr, causing a test to fail
pub fn assert_input(input: &str, expected_text: ContainsPredicate) {
    Command::cargo_bin("rawk")
        .unwrap()
        .write_stdin(input)
        .assert()
        .stderr(expected_text);
}
