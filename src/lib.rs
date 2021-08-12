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
/// - `is_eval` whether or not a single line of awk code is being interpreted. If so, this function
/// terminates after a single line of code from STDIN has been evaluated.
pub fn run_prompt(is_eval: bool) {
    let mut rl = Editor::<()>::new();
    let mut vm = VM::new();

    loop {
        let user_input = rl.readline("r-awk > ");
        match user_input {
            Ok(awk_line) => {
                debug!("r-awk line to process: {}", awk_line);
                let _result = vm.interpret(awk_line);
            }
            Err(err) => {
                match err {
                    ReadlineError::Eof => println!("Eof received, exiting."),
                    ReadlineError::Interrupted => println!("Interrupt received, exiting."),
                    _ => error!("An error occurred: '{:?}'", err),
                }
                break;
            }
        }
        // the eval should only run once
        if is_eval {
            break;
        }
    }
}
