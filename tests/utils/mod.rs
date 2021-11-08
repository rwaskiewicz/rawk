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
// Allow dead code here per https://github.com/rust-lang/rust/issues/46379
// I don't expect this to be long lived anyway
#[allow(dead_code)]
pub fn assert_input(input: &str, expected_value: &str) {
    let mut expected_text = String::new();
    expected_text.push_str("^\\[INFO  rawk::vm\\] ");
    expected_text.push_str(expected_value);
    expected_text.push_str("\n$");
    Command::cargo_bin("rawk")
        .unwrap()
        .arg(input)
        .arg("-q")
        .assert()
        .stderr(predicates::str::is_match(expected_text).unwrap());
}

/// Helper library for invoking r-awk, providing data, and asserting on output in stderr
///
/// # Arguments:
/// - `program` the program that would have been received by the user
/// - `data` the data that would have been received by the user that the `program` should run against
/// - `expected_value` the expected result to appear in stderr
///
/// # Panics:
/// If the `expected_value` cannot be found in stderr, the assertion (and test) will fail
// Allow dead code here per https://github.com/rust-lang/rust/issues/46379
// I don't expect this to be long lived anyway
#[allow(dead_code)]
pub fn assert_input_with_data(program: &str, data: &str, expected_value: &str) {
    let mut expected_text = String::new();
    expected_text.push_str("^\\[INFO  rawk::vm\\] ");
    expected_text.push_str(expected_value);
    expected_text.push_str("\n$");
    Command::cargo_bin("rawk")
        .unwrap()
        .arg(program)
        .arg("-k")
        .write_stdin(data)
        .assert()
        .stderr(predicates::str::is_match(expected_text).unwrap());
}

/// Helper library for invoking r-awk and asserting an empty output on stderr
///
/// # Arguments:
/// - `input` the input that would have been received by the user
///
/// # Panics:
/// If the result is non-empty
// Allow dead code here per https://github.com/rust-lang/rust/issues/46379
// I don't expect this to be long lived anyway
#[allow(dead_code)]
pub fn assert_input_yields_empty(input: &str) {
    let expected_text = String::from("^$");
    Command::cargo_bin("rawk")
        .unwrap()
        .arg(input)
        .arg("-q")
        .assert()
        .stderr(predicates::str::is_match(expected_text).unwrap());
}
