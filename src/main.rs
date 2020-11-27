use clap::{App, Arg};
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

    let mut scanner = Scanner::new();
    let mut awk_line = String::new();
    let mut awk_input = String::new();

    loop {
        print!("r-awk > ");
        io::stdout().flush().expect("Unable to flush STDOUT!");

        io::stdin()
            .read_line(&mut awk_line)
            .expect("failed to get r-awk line");
        print!("r-awk line to process: {}", awk_line);
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
pub enum TokenType {
    LeftCurly,
    RightCurly,
    LeftSquareBracket,
    RightSquareBracket,
    LeftParenthesis,
    RightParenthesis,
    SingleQuote,
    DoubleQuote,
    Sigil,
    Bang,
    Equals,
    GreaterThan,
    LessThan,
}

#[derive(Debug)]
pub struct Scanner {
    lexeme_start: u32,
    lexeme_current: u32,
    current_line: u32,
    tokens: Vec<TokenType>,
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            current_line: 1,
            lexeme_current: 0,
            lexeme_start: 0,
            tokens: Vec::new(),
        }
    }

    pub fn scan(&mut self, input: &str) {
        for ch in input.chars() {
            println!("Inspecting Character: '{}'", ch);
            // TODO: This is _very_ basic switching that is not looking for combining operator '>='
            match ch {
                ' ' | '\r' | '\t' => println!("I can see and accept whitespace"),
                '\n' => {
                    self.current_line += 1;
                    println!("I can see the newline, which is now {}", self.current_line);
                }
                '{' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::LeftCurly
                    );
                    self.tokens.push(TokenType::LeftCurly);
                }
                '}' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::RightCurly
                    );
                    self.tokens.push(TokenType::RightCurly);
                }
                '[' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::LeftSquareBracket
                    );
                    self.tokens.push(TokenType::LeftSquareBracket);
                }
                ']' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::RightSquareBracket
                    );
                    self.tokens.push(TokenType::RightSquareBracket);
                }
                '(' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::LeftParenthesis
                    );
                    self.tokens.push(TokenType::LeftParenthesis);
                }
                ')' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::RightParenthesis
                    );
                    self.tokens.push(TokenType::RightParenthesis);
                }
                '\'' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::SingleQuote
                    );
                    self.tokens.push(TokenType::SingleQuote);
                }
                '\"' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::DoubleQuote
                    );
                    self.tokens.push(TokenType::DoubleQuote);
                }
                '>' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::GreaterThan
                    );
                    self.tokens.push(TokenType::GreaterThan);
                }
                '<' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::LessThan
                    );
                    self.tokens.push(TokenType::LessThan);
                }
                '=' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::Equals
                    );
                    self.tokens.push(TokenType::Equals);
                }
                '!' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::Bang
                    );
                    self.tokens.push(TokenType::Bang);
                }
                '$' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::Sigil
                    );
                    self.tokens.push(TokenType::Sigil);
                }
                _ => {
                    if ch.is_digit(10) {
                        println!("I see a number! {}", ch);
                    } else if ch.is_alphabetic() {
                        println!("I see an alpha {}", ch);
                    } else {
                        println!("ALERT: We found a character we can not handle, '{}'", ch);
                    }
                }
            }
        }

        println!("This is the final list of Tokens {:?}", self.tokens);
    }
}
