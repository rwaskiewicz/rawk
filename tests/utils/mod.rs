//! Common integration test utilities

use assert_cmd::Command;

/// Helper library for invoking r-awk, providing data, and asserting on output in stdout
///
/// # Arguments:
/// - `program` the program that would have been received by the user as a positional argument
/// - `opts` command line options to pass to r-awk
/// - `data` the data that would have been received by the user that the `program` should run
/// against. A value of `None` is considered to be a lack of data, whereas a value of `Some(_)` is
/// considered a piece of data (even if `_` evaluates to an empty string).
/// - `expected_value` the expected result to appear in stdout. A value of `None` is considered to
/// be a lack of output to stout, whereas a value of `Some(_)` is considered to be something that
/// should be received from stdout (even if `_` evaluates to an empty string).
///
/// # Panics:
/// If the `expected_value` cannot be found in stdout, the assertion (and test) will fail
pub fn run_rawk(
    program: Option<&str>,
    opts: Vec<&str>,
    data: Option<&str>,
    expected_value: Option<&str>,
) {
    let mut program_args = vec![];

    if program.is_some() {
        program_args.push(program.expect("no testing program could be unwrapped!"));
    }

    // push the `eval` key so a run is terminated after a single piece of data is received
    program_args.push("-k");

    program_args.append(&mut opts.clone());

    let expected_text = match expected_value {
        Some(content) => format!("^{}\n$", content),
        None => String::from("^$"),
    };

    Command::cargo_bin("rawk")
        .unwrap()
        .args(program_args)
        .write_stdin(data.unwrap_or(""))
        .assert()
        .stdout(predicates::str::is_match(expected_text).unwrap());
}
