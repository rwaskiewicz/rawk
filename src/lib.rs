//! Library for invoking the VM with provided awk code.

mod chunk;
mod parser;
mod scanner;
mod token;
mod value;
mod vm;

use crate::vm::VM;

/// Thin wrapper for initializing a new VM, then interpreting the provided awk line
///
/// # Arguments
/// - `awk_line` - input from the user to be compiled/run. May not be valid awk code
pub fn startup_and_interpret_awk_line(awk_line: String) {
    let vm = VM::new();
    interpret_awk_line(vm, awk_line);
}

/// Function for interpreting a line of awk code
///
/// # Arguments
/// - `vm` - the instance of the VM to run the code
/// - `awk_line` - input from the user to be compiled/run. May not be valid awk code
fn interpret_awk_line(mut vm: VM, awk_line: String) {
    // TODO: It's probably more idiomatic to return a `Result`
    let _result = vm.interpret(awk_line);
}
