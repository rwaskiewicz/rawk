//! Library for invoking the VM with provided awk code.

use log::{debug, error};
use rustyline::error::ReadlineError;
use rustyline::Editor;

mod chunk;
mod parser;
mod scanner;
mod token;
mod value;
mod vm;

use crate::vm::VM;

/// Invokes a REPL for awk code.
///
/// This function runs on a loop, which will be prematurely halted if `is_eval` is `true` (which is
/// a very fancy way to say that it runs once). Multiple statements/expressions may be placed on a
/// single line.
///
/// # Arguments
/// - `program` the user's program to run
/// - `data` the data the user may have passed to process
/// - `is_eval` whether or not a single line of awk code is being interpreted. If so, this function
/// terminates after a single line of code from STDIN has been evaluated.
/// - `is_quick` whether or not a single line of awk code is being interpreted without data. If so,
/// this function terminates after no code from STDIN has been evaluated and is a temporary stand
/// in for `BEGIN`.
pub fn run_prompt(program: &str, data: &[String], is_eval: bool, is_quick: bool) {
    let mut rl = Editor::<()>::new();
    let mut vm = VM::new();

    if is_quick {
        // TODO: Remove this when `BEGIN` is implemented
        let _result = vm.interpret(String::from(program), &[]);
    } else if data.is_empty() {
        loop {
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

            let mut split_data = data_received
                // TODO: Replace this when FS is implemented
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<String>>();
            let mut data_to_eval = vec![data_received];
            data_to_eval.append(&mut split_data);

            let _result = vm.interpret(String::from(program).clone(), &data_to_eval);

            if is_eval {
                // the eval should only run once
                break;
            }
        }
    } else {
        // TODO this is now the case of passing in a file
        panic!("Reading a file is not implemented yet");
    }
}
