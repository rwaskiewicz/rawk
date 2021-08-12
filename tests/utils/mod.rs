//! Common integration test utilities

use assert_cmd::Command;

/// Helper library for invoking r-awk and asserting on output in stderr
///
/// # Arguments:
/// - `input` the input that would have been received by the user
/// - `expected_value` the expected result to appear in stderr
///
/// # Panics:
/// If the `expected_value` cannot be found in stderr, the assertion (and test) will fail
pub fn assert_input(input: &str, expected_value: &str) {
    let mut expected_text = String::new();
    expected_text.push_str("^\\[INFO  rawk::vm\\] ");
    expected_text.push_str(expected_value);
    expected_text.push_str("\n$");
    Command::cargo_bin("rawk")
        .unwrap()
        .arg("-k")
        .write_stdin(input)
        .assert()
        .stderr(predicates::str::is_match(expected_text).unwrap());
}
