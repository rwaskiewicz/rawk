use clap::{App, Arg};
use std::io;
use std::io::Write;

fn main() {
    let matches = App::new("R-Awk")
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
    println!("R-AWK - a subset of awk written in Rust");

    let scanner = Scanner::new();
    let mut awk_line = String::new();
    let mut awk_input = String::new();

    loop {
        print!("RAWK > ");
        io::stdout().flush().expect("Unable to flush STDOUT!");

        io::stdin()
            .read_line(&mut awk_line)
            .expect("failed to get r-awk line");
        print!("R-AWK Line: {}", awk_line);
        scanner.scan(&awk_line);
        while !awk_input.contains("STOP!") {
            print!("Input Data (type STOP! to end data input) >> ");
            io::stdout().flush().expect("Unable to flush STDOUT!");

            awk_input.clear();

            io::stdin()
                .read_line(&mut awk_input)
                .expect("failed to get r-awk input!");
            print!("Received Data: {}", awk_input);
        }
        awk_input.clear();
        awk_line.clear();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // TODO: Make this test more realistic
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Debug)]
pub struct Scanner {
    lexeme_start: u32,
    lexeme_current: u32,
    current_line: u32,
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            current_line: 1,
            lexeme_current: 0,
            lexeme_start: 0,
        }
    }

    pub fn scan(&self, input: &str) {
        println!("It is time to scan! {}", input);
        for ch in input.chars() {
            println!("Character: {}", ch);
        }
    }
}
