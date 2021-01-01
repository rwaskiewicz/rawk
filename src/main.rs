mod chunk;
mod parser;
mod scanner;
mod token;
mod value;
mod vm;

use crate::chunk::{Chunk, OpCode};
use crate::token::token::Token;
use crate::vm::VM;
use clap::{App, Arg};
use parser::Parser;
use scanner::Scanner;
use std::io;
use std::io::Write;

fn main() {
    let matches = App::new("r-awk")
        .version("0.0.1")
        .about("awk, implemented in Rust")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let file_name = matches.value_of("file");
    match file_name {
        None => {
            run_prompt();
        }
        Some(s) => {
            println!("TODO: Implement file parsing. Got file_name {}", s);
        }
    }

    println!("Hello, world!");
}

fn run_prompt() {
    println!("r-awk - a subset of awk written in Rust");

    loop {
        let mut awk_line = String::new();
        let mut awk_input = String::new();

        print!("r-awk > ");
        io::stdout().flush().expect("Unable to flush STDOUT!");

        io::stdin()
            .read_line(&mut awk_line)
            .expect("failed to get r-awk line");
        print!("r-awk line to process: {}", awk_line);

        let mut vm = VM::new();
        vm.interpret(awk_line);

        while !awk_input.contains("STOP!") {
            print!("Input Data (type STOP! to end data input) >> ");
            io::stdout().flush().expect("Unable to flush STDOUT!");

            awk_input.clear();

            io::stdin()
                .read_line(&mut awk_input)
                .expect("failed to get r-awk input!");
            print!("Received Data: {}", awk_input);
        }
    }
}
