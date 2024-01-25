//! Common integration test utilities

use assert_cmd::assert::Assert;
use assert_cmd::Command;

/// Runner for r-awk testing
#[derive(Debug)]
#[must_use]
pub struct CodeRunner {
    program: Option<&'static str>,
    opts: Vec<&'static str>,
    data: &'static str,
    expected_value: String,
    debug: bool,
}

impl CodeRunner {
    /// Create a command to invoke r-awk
    pub fn init() -> Self {
        Self {
            program: None,
            // init with the 'eval' flag so a run is terminated after one piece of data is received
            opts: vec!["-k"],
            data: "",
            expected_value: String::from(""),
            debug: false,
        }
    }

    /// Enable debug mode for a single test
    ///
    /// This option is useful for gaining insight into what arguments are used to invoke r-awk, as
    /// well as inspecting the return status of the program invocation.
    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    /// Sets the awk program to run.
    ///
    /// Multiple invocations of this function will override previous calls.
    ///
    /// # Arguments
    /// - `program` the program that would have been received by the user as a positional argument
    pub fn program(mut self, program: &'static str) -> Self {
        self.program = Some(program);
        self
    }

    /// Add one or more CLI arguments to the r-awk invocation
    ///
    /// Multiple invocations of this function will _not_ override previous calls.
    ///
    /// # Arguments
    /// - `opts` command line options to pass to r-awk
    pub fn cli_options(mut self, opts: Vec<&'static str>) -> Self {
        self.opts.append(&mut opts.clone());
        self
    }

    /// Sets the data to be provided to r-awk via STDIN
    ///
    /// Multiple invocations of this function will override previous calls.
    ///
    /// # Arguments
    /// - `data` the data that would have been received from the user that the program should run
    /// against
    pub fn stdin_data(mut self, data: &'static str) -> Self {
        self.data = data;
        self
    }

    /// Set the expected output of a program run to be an empty string.
    ///
    /// This method does not determine which output stream the empty string should be evaluated
    /// against. To assert against `STDOUT`, see [`assert`].
    pub fn expect_empty_output(mut self) -> Self {
        self.expected_value = String::from("^$");
        self
    }

    /// Set the expected output of a program run to be the provided string.
    ///
    /// This method does not determine which output stream the provided string should be evaluated
    /// against. To assert against `STDOUT`, see [`assert`]. To assert against `STDERR`, see [`assert_fail`].
    ///
    /// # Arguments
    /// - `expected_value` the expected result to appear
    pub fn expect_output(mut self, expected_output: &str) -> Self {
        self.expected_value = format!("^{}\n$", expected_output);
        self
    }

    /// Runs r-awk, and asserts that the value printed to STDOUT matches the value provided by
    /// [`expect_output`]
    ///
    /// If no value was provided, assert an empty string was printed to STDOUT
    pub fn assert(mut self) {
        if self.expected_value.is_empty() {
            self = self.expect_empty_output();
        }

        self.build_assert()
            .success()
            .stdout(predicates::str::is_match(&(self.expected_value)).unwrap());
    }

    /// Runs r-awk, the program fails, and asserts that the value printed to STDERR matches the value provided by
    /// [`expect_output`]
    pub fn assert_fail(mut self) {
        self.build_assert()
            .failure()
            .stderr(predicates::str::is_match(&(self.expected_value)).unwrap());
    }

    /// Helper method for creating an [`Assert`] object from the current state of `Self`.
    ///
    /// This method invokes r-awk, which has two important ramifications:
    /// - This method does not perform any assertions against the results of invoking r-awk
    /// - Invocations may should not be considered idempotent.
    ///
    /// # Return value
    /// the result of running r-awk.
    fn build_assert(&mut self) -> Assert {
        let test_assert = Command::cargo_bin("rawk")
            .unwrap()
            .args(self.program)
            .args(&self.opts)
            .write_stdin(self.data)
            .assert();

        self.print_debug(&test_assert);

        test_assert
    }

    /// Helper method for printing test diagnostics
    ///
    /// # Arguments
    /// - `test_assert` the [`Assert`] object created after invoking r-awk.
    fn print_debug(&mut self, test_assert: &Assert) {
        if self.debug {
            println!("{:#?}", self);
            println!("{:#?}", test_assert);
        }
    }
}
