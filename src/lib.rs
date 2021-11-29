//! Library for invoking the VM with provided awk code.

use log::{debug, error};
use rustyline::error::ReadlineError;
use rustyline::Editor;

mod chunk;
mod parser;
pub mod runtime_config;
mod scanner;
mod token;
mod value;
mod vm;

use crate::runtime_config::RuntimeConfig;
use crate::scanner::Scanner;
use crate::token::Token;
use crate::vm::VM;

/// Invokes a REPL for awk code.
///
/// This function runs on a loop, which will be prematurely halted if `is_eval` is `true` (which is
/// a very fancy way to say that it runs once). Multiple statements/expressions may be placed on a
/// single line.
///
/// # Arguments
/// - `program` the user's program to run
/// - `runtime_config` the runtime configuration for the lifetime of the awk program
pub fn run_program(program: &str, runtime_config: RuntimeConfig) {
    let scanner = Scanner::new(String::from(program));
    let tokens: Vec<Token> = scanner.scan();

    let mut vm = VM::new();

    if runtime_config.is_quick {
        // TODO: Remove this when `BEGIN` is implemented
        let _result = vm.interpret(&tokens, &[]);
    } else if runtime_config.file_name.is_none() {
        loop {
            let data_received = read_user_data_from_terminal();
            let data_to_eval = split_user_data(&runtime_config.field_separator, data_received);
            let _result = vm.interpret(&tokens, &data_to_eval);

            if runtime_config.is_eval {
                // the eval should only run once
                break;
            }
        }
    } else {
        panic!("Reading a file is not implemented yet");
    }
}

/// Reads data from STDIN to be processed by a user's program
///
/// # Return value:
/// - the data read from STDIN
///
/// # Panics:
/// If the user inputs an end of file or interrupted character, or if for some reason a `Readline` error is returned by
/// the call to `readline()`
fn read_user_data_from_terminal() -> String {
    let mut rl = Editor::<()>::new();
    let data_input = rl.readline("$ ");
    let data_received = match data_input {
        Ok(data_line) => {
            debug!("data line to process: {}", data_line);
            data_line
        }
        Err(err) => {
            match err {
                // Yes, we know eprintln exists and these could be `error!()`, considering
                // them to be semantically different
                ReadlineError::Eof => println!("Eof received, exiting."),
                ReadlineError::Interrupted => println!("Interrupt received, exiting."),
                _ => error!("An error occurred: '{:?}'", err),
            }
            panic!();
        }
    };
    data_received
}

/// Splits data to be used as field variables.
///
/// The data is of the format [original_data, piece1, piece2, ...] where `original_data` matches `data_received`, and
/// each `piece` is the the N-th item to be split by a field separator, stored in `runtime_config`.
///
/// # Arguments:
/// - `field_separator` the field separator to use when splitting the `data_received`
/// - `data_received` the data to split
///
/// # Returns:
/// - the split data, as described above
fn split_user_data(field_separator: &str, data_received: String) -> Vec<String> {
    let mut split_data;
    if field_separator.is_empty() {
        // Case: field separator is an empty string
        split_data = data_received
            .split(field_separator)
            .map(String::from)
            .collect::<Vec<String>>();
        if split_data.len() >= 2 {
            // the default rust `split()` operation will add a leading/trailing empty string, remove it so we don't try
            // to implement split ourselves
            split_data.remove(split_data.len() - 1);
            split_data.remove(0);
        }
    } else if field_separator.eq(" ") {
        // Case: a single empty string, where we must strip all leading, trailing, and in-between whitespace
        split_data = data_received
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();
    } else if field_separator.len() == 1 {
        // Case: field separator is single character separator that is not (" ")
        split_data = data_received
            .split(field_separator)
            .map(String::from)
            .collect::<Vec<String>>();
    } else {
        // Case: field separator is more than one character, and should be treated as a regex
        panic!("TODO: Implement Regex Field Separators")
    }
    let mut data_to_eval = vec![data_received];
    data_to_eval.append(&mut split_data);
    data_to_eval
}

#[cfg(test)]
mod lib {
    use super::*;

    #[test]
    fn it_splits_data_by_single_char_fs() {
        let test_data = "  Hello,World  ,I\tam,someone! ";
        let split_data = split_user_data(&",", String::from(test_data));

        assert_eq!(split_data.len(), 5);
        assert_eq!(split_data[0], test_data);
        assert_eq!(split_data[1], "  Hello");
        assert_eq!(split_data[2], "World  ");
        assert_eq!(split_data[3], "I\tam");
        assert_eq!(split_data[4], "someone! ");
    }

    #[test]
    fn it_does_not_truncate_whitespace_for_fs() {
        let test_data = " Alice  ,40 ,25 ";
        let split_data = split_user_data(&",", String::from(test_data));

        assert_eq!(split_data.len(), 4);
        assert_eq!(split_data[0], test_data);
        assert_eq!(split_data[1], " Alice  ");
        assert_eq!(split_data[2], "40 ");
        assert_eq!(split_data[3], "25 ");
    }

    #[test]
    fn it_counts_two_consecutive_fs_as_empty_record() {
        let test_data = "Hello,,World!";
        let split_data = split_user_data(&",", String::from(test_data));

        assert_eq!(split_data.len(), 4);
        assert_eq!(split_data[0], test_data);
        assert_eq!(split_data[1], "Hello");
        assert_eq!(split_data[2], "");
        assert_eq!(split_data[3], "World!");
    }

    #[test]
    fn it_splits_nothing_when_fs_not_found() {
        let test_data = "  Hello World  I\tam  someone! ";
        let split_data = split_user_data(&",", String::from(test_data));

        assert_eq!(split_data.len(), 2);
        assert_eq!(split_data[0], test_data);
        assert_eq!(split_data[1], test_data);
    }

    #[test]
    fn it_splits_data_by_whitespace_when_no_fs_provided() {
        let test_data = "  Hello World  I\tam  someone! ";
        // awk/gawk/r-awk defaults to a single empty character, provide it in the test to satisfy the contract of th fn
        let split_data = split_user_data(&" ", String::from(test_data));

        assert_eq!(split_data.len(), 6);
        assert_eq!(split_data[0], test_data);
        assert_eq!(split_data[1], "Hello");
        assert_eq!(split_data[2], "World");
        assert_eq!(split_data[3], "I");
        assert_eq!(split_data[4], "am");
        assert_eq!(split_data[5], "someone!");
    }

    #[test]
    fn it_splits_data_by_character_when_null_string_fs_provided() {
        // https://www.gnu.org/software/gawk/manual/html_node/Single-Character-Fields.html
        // > Traditionally, the behavior of FS equal to "" was not defined. In this case, most versions of Unix awk
        // > simply treat the entire record as only having one field. (d.c.) In compatibility mode (see section
        // > Command-Line Options), if FS is the null string, then gawk also behaves this way.
        // r-awk chooses to respect FS="" as g-awk does
        let test_data = "Hello World";
        let split_data = split_user_data(&"", String::from(test_data));

        assert_eq!(split_data.len(), 12);
        assert_eq!(split_data[0], test_data);
        assert_eq!(split_data[1], "H");
        assert_eq!(split_data[2], "e");
        assert_eq!(split_data[3], "l");
        assert_eq!(split_data[4], "l");
        assert_eq!(split_data[5], "o");
        assert_eq!(split_data[6], " ");
        assert_eq!(split_data[7], "W");
        assert_eq!(split_data[8], "o");
        assert_eq!(split_data[9], "r");
        assert_eq!(split_data[10], "l");
        assert_eq!(split_data[11], "d");
    }

    #[test]
    fn it_splits_empty_data_when_fs_is_null_string() {
        let test_data = "";
        let split_data = split_user_data(&"", String::from(test_data));

        assert_eq!(split_data.len(), 1);
        assert_eq!(split_data[0], test_data);
    }

    #[test]
    fn it_splits_data_entirely_when_fs_matches_test_data_single_char() {
        let test_data = "a";
        let split_data = split_user_data(&"a", String::from(test_data));

        assert_eq!(split_data.len(), 3);
        assert_eq!(split_data[0], test_data);
        assert_eq!(split_data[1], "");
        assert_eq!(split_data[2], "");
    }

    #[test]
    fn it_splits_data_when_fs_matches_leading_char() {
        let test_data = "abac";
        let split_data = split_user_data(&"a", String::from(test_data));

        assert_eq!(split_data.len(), 4);
        assert_eq!(split_data[0], test_data);
        assert_eq!(split_data[1], "");
        assert_eq!(split_data[2], "b");
        assert_eq!(split_data[3], "c");
    }

    #[test]
    fn it_splits_data_when_fs_matches_trailing_char() {
        let test_data = "baca";
        let split_data = split_user_data(&"a", String::from(test_data));

        assert_eq!(split_data.len(), 4);
        assert_eq!(split_data[0], test_data);
        assert_eq!(split_data[1], "b");
        assert_eq!(split_data[2], "c");
        assert_eq!(split_data[3], "");
    }

    // TODO: Regex - test the case of FS = "[ \t\n]+" - which should _not_ strip leading spaces and add an integration test
    // See https://www.gnu.org/software/gawk/manual/html_node/Regexp-Field-Splitting.html
    #[test]
    #[ignore]
    fn it_splits_data_by_whitespace_via_regex() {
        let test_data = "  Hello World  I\tam  someone! ";
        let split_data = split_user_data(&"[ \t\n]+", String::from(test_data));

        assert_eq!(split_data.len(), 6);
        assert_eq!(split_data[0], test_data);
        assert_eq!(split_data[1], "  Hello");
        assert_eq!(split_data[2], "World");
        assert_eq!(split_data[3], "I");
        assert_eq!(split_data[4], "am");
        assert_eq!(split_data[5], "someone! ");
    }
}
