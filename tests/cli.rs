#[test]
fn cli_tests() {
    trycmd::TestCases::new()
        .case("tests/cli/*.md")
        .case("tests/cli/*.toml");
}

pub mod utils;

#[test]
fn other_cli_tests() {
    utils::CodeRunner::init()
        .cli_options(vec!["--help"])
        .expect_output(r#"awk, implemented in Rust

Usage: rawk [OPTIONS] [program]

Arguments:
  [program]

Options:
  -f, --file <file>         Runs an awk program
  -q, --quick               Runs a single line of awk code without data, then terminates
  -k, --eval                Runs a single line of awk code, then terminates
  -F <field_separator>      Sets the field separator character/regex for parsing data [default: " "]
  -h, --help                Print help information
  -V, --version             Print version information

"#,
        )
        .assert()
}

#[test]
fn other_cli_tests_2() {
    utils::CodeRunner::init()
        .expect_output(r#"rawk 0.1.0"#,
        )
        .assert()
}
