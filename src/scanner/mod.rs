use super::token::token::Token;
use super::token::token_type::TokenType;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Scanner {
    keywords: HashMap<&'static str, &'static TokenType>,
}

impl Scanner {
    pub fn new() -> Scanner {
        let mut keywords: HashMap<&str, &TokenType> = HashMap::new();
        keywords.insert("print", &TokenType::Print);
        keywords.shrink_to_fit();

        Scanner { keywords }
    }

    pub fn scan(&self, input: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut current_line = 1;

        let mut char_stream = input.chars().peekable();
        while let Some(_) = char_stream.peek() {
            let mut ch = char_stream.next().unwrap();
            println!("Inspecting Character: '{}'", ch);
            match ch {
                ' ' | '\r' | '\t' => println!("I can see and accept whitespace"),
                '\n' => {
                    current_line += 1;
                    println!(
                        "I can see the newline. The current_line number is now {}",
                        current_line
                    );
                }
                ';' => {
                    Scanner::report_scanned_character(ch, &TokenType::Semicolon);
                    tokens.push(Token::new(None, &TokenType::Semicolon));
                }
                ',' => {
                    Scanner::report_scanned_character(ch, &TokenType::Comma);
                    tokens.push(Token::new(None, &TokenType::Comma));
                }
                '#' => {
                    // consume the rest of the line, as we've found a comment
                    char_stream.find(|x| x == &'\n');
                    Scanner::report_scanned_character(ch, &TokenType::Pound);
                    tokens.push(Token::new(None, &TokenType::Pound));
                }
                '{' => {
                    Scanner::report_scanned_character(ch, &TokenType::LeftCurly);
                    tokens.push(Token::new(None, &TokenType::LeftCurly));
                }
                '}' => {
                    Scanner::report_scanned_character(ch, &TokenType::RightCurly);
                    tokens.push(Token::new(None, &TokenType::RightCurly));
                }
                '[' => {
                    Scanner::report_scanned_character(ch, &TokenType::LeftSquareBracket);
                    tokens.push(Token::new(None, &TokenType::LeftSquareBracket));
                }
                ']' => {
                    Scanner::report_scanned_character(ch, &TokenType::RightSquareBracket);
                    tokens.push(Token::new(None, &TokenType::RightSquareBracket));
                }
                '(' => {
                    Scanner::report_scanned_character(ch, &TokenType::LeftParenthesis);
                    tokens.push(Token::new(None, &TokenType::LeftParenthesis));
                }
                ')' => {
                    Scanner::report_scanned_character(ch, &TokenType::RightParenthesis);
                    tokens.push(Token::new(None, &TokenType::RightParenthesis));
                }
                '\'' => {
                    Scanner::report_scanned_character(ch, &TokenType::SingleQuote);
                    tokens.push(Token::new(None, &TokenType::SingleQuote));
                }
                '\"' => {
                    // TODO: Update this to find full strings
                    Scanner::report_scanned_character(ch, &TokenType::DoubleQuote);
                    tokens.push(Token::new(None, &TokenType::DoubleQuote));
                }
                '>' => {
                    // TODO: Support greater than or equal to '>='
                    Scanner::report_scanned_character(ch, &TokenType::GreaterThan);
                    tokens.push(Token::new(None, &TokenType::GreaterThan));
                }
                '<' => {
                    // TODO: Support less than or equal to '<='
                    Scanner::report_scanned_character(ch, &TokenType::LessThan);
                    tokens.push(Token::new(None, &TokenType::LessThan));
                }
                '=' => {
                    // TODO: Support equal to '=='
                    Scanner::report_scanned_character(ch, &TokenType::Equals);
                    tokens.push(Token::new(None, &TokenType::Equals));
                }
                '!' => {
                    // TODO: Support not equal to '!='
                    // TODO: Support ERE non-match '!~'
                    Scanner::report_scanned_character(ch, &TokenType::Bang);
                    tokens.push(Token::new(None, &TokenType::Bang));
                }
                '$' => {
                    Scanner::report_scanned_character(ch, &TokenType::Sigil);
                    tokens.push(Token::new(None, &TokenType::Sigil));
                }
                '+' => {
                    // TODO: Support addition assignment '+='
                    // TODO: Support post-increment '++'
                    // TODO: Support pre-increment '++'
                    Scanner::report_scanned_character(ch, &TokenType::Plus);
                    tokens.push(Token::new(None, &TokenType::Plus));
                }
                '-' => {
                    // TODO: Support subtraction assignment '-='
                    // TODO: Support post-decrement '--'
                    // TODO: Support pre-decrement '--'
                    Scanner::report_scanned_character(ch, &TokenType::Minus);
                    tokens.push(Token::new(None, &TokenType::Minus));
                }
                '*' => {
                    // TODO: Support multiplication assignment '*='
                    Scanner::report_scanned_character(ch, &TokenType::Star);
                    tokens.push(Token::new(None, &TokenType::Star));
                }
                '/' => {
                    // TODO: Support division assignment '/='
                    Scanner::report_scanned_character(ch, &TokenType::Slash);
                    tokens.push(Token::new(None, &TokenType::Slash));
                }
                '^' => {
                    // TODO: Support exponentiation assignment '^='
                    // super::report_scanned_token();
                    Scanner::report_scanned_character(ch, &TokenType::Caret);
                    tokens.push(Token::new(None, &TokenType::Caret));
                }
                '%' => {
                    // TODO: Support modulus assignment '%='
                    Scanner::report_scanned_character(ch, &TokenType::Modulus);
                    tokens.push(Token::new(None, &TokenType::Modulus));
                }
                '~' => {
                    Scanner::report_scanned_character(ch, &TokenType::Tilde);
                    tokens.push(Token::new(None, &TokenType::Tilde));
                }
                // TODO: Array membership
                // TODO: Logical AND
                // TODO: Logical OR
                // TODO: Ternary (? and :)
                _ => {
                    if ch.is_digit(10) {
                        let mut num_parsed = String::from("");
                        num_parsed.push(ch);

                        // TODO: Refactor this function out
                        while let Some(maybe_number) = char_stream.peek() {
                            if !maybe_number.is_digit(10) {
                                break;
                            }
                            if let Some(next_ch) = char_stream.next() {
                                ch = next_ch;
                                num_parsed.push(ch);
                            }
                        }

                        // TODO: Support scientific (exponential) notation like 0.707E-1, 1E1, 1e6
                        // Note that 'e' can be cased however, and may occur before or after the dot

                        // TODO: This still doesn't feel right
                        if let Some(maybe_dot) = char_stream.peek() {
                            if maybe_dot == &'.' {
                                if let Some(dot) = char_stream.next() {
                                    ch = dot;
                                    num_parsed.push(ch);
                                }
                            }

                            while let Some(maybe_number) = char_stream.peek() {
                                if !maybe_number.is_digit(10) {
                                    break;
                                }
                                if let Some(next_ch) = char_stream.next() {
                                    ch = next_ch;
                                    num_parsed.push(ch);
                                }
                            }
                        }

                        Scanner::report_scanned_string(&num_parsed, &TokenType::Number);
                        // TODO: Store this value in floating point
                        tokens.push(Token::new(Some(num_parsed), &TokenType::Number));
                    } else if ch.is_alphabetic() {
                        // TODO: Support underscore for variable names
                        let mut word_parsed = String::from(ch);

                        // TODO: This check may be too permissive
                        while let Some(maybe_alphanumeric) = char_stream.peek() {
                            if !maybe_alphanumeric.is_alphanumeric() && maybe_alphanumeric != &'_' {
                                break;
                            }
                            if let Some(next_ch) = char_stream.next() {
                                ch = next_ch;
                                word_parsed.push(ch);
                            }
                        }

                        let type_of_token = self
                            .keywords
                            .get(&word_parsed.as_str())
                            .unwrap_or(&&TokenType::Identifier);

                        Scanner::report_scanned_string(&word_parsed, &type_of_token);
                        tokens.push(Token::new(Some(word_parsed.clone()), &type_of_token));
                    } else {
                        println!("ALERT: We found a character we can not handle, '{}'", ch);
                    }
                }
            }
        }

        println!("This is the final list of Tokens {:?}", tokens);
        tokens
    }

    fn report_scanned_character(ch: char, token_type: &TokenType) {
        println!("Found a '{}', setting the type to '{:?}'", ch, token_type);
    }

    fn report_scanned_string(ch: &str, token_type: &TokenType) {
        println!("Found a '{}', setting the type to '{:?}'", ch, token_type);
    }
}

#[cfg(test)]
mod lexing {
    use super::*;

    #[test]
    fn it_returns_an_empty_vector_for_empty_input() {
        let tokens = Scanner::new().scan("");

        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn it_parses_a_semicolon() {
        let tokens = Scanner::new().scan(";");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::Semicolon
            })
        );
    }

    #[test]
    fn it_parses_a_comma() {
        let tokens = Scanner::new().scan(",");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::Comma
            })
        );
    }

    #[test]
    fn it_parses_a_pound() {
        let tokens = Scanner::new().scan("#");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::Pound
            })
        );
    }

    #[test]
    fn it_parses_comment_as_a_string() {
        let tokens = Scanner::new().scan("{print} # this print is important");
        let mut token_iter = tokens.iter();

        assert_eq!(tokens.len(), 4);
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::LeftCurly
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: Some(String::from("print")),
                token_type: &TokenType::Print
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::RightCurly
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::Pound
            })
        );
    }

    #[test]
    fn it_parses_a_left_curly_brace() {
        let tokens = Scanner::new().scan("{");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::LeftCurly
            })
        );
    }

    #[test]
    fn it_parses_a_right_curly_brace() {
        let tokens = Scanner::new().scan("}");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::RightCurly
            })
        );
    }

    #[test]
    fn it_parses_a_left_square_bracket() {
        let tokens = Scanner::new().scan("[");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::LeftSquareBracket
            })
        );
    }

    #[test]
    fn it_parses_a_right_square_bracket() {
        let tokens = Scanner::new().scan("]");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::RightSquareBracket
            })
        );
    }

    #[test]
    fn it_parses_a_left_parenthesis() {
        let tokens = Scanner::new().scan("(");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::LeftParenthesis
            })
        );
    }

    #[test]
    fn it_parses_a_right_parenthesis() {
        let tokens = Scanner::new().scan(")");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::RightParenthesis
            })
        );
    }

    #[test]
    fn it_parses_a_single_quote() {
        let tokens = Scanner::new().scan("\'");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::SingleQuote
            })
        );
    }

    #[test]
    fn it_parses_a_double_quote() {
        let tokens = Scanner::new().scan("\"");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::DoubleQuote
            })
        );
    }

    #[test]
    fn it_parses_a_greater_than_caret() {
        let tokens = Scanner::new().scan(">");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::GreaterThan
            })
        );
    }

    #[test]
    fn it_parses_a_less_than_caret() {
        let tokens = Scanner::new().scan("<");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::LessThan
            })
        );
    }

    #[test]
    fn it_parses_an_assignment_token() {
        let tokens = Scanner::new().scan("=");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::Equals
            })
        );
    }

    #[test]
    fn it_parses_a_bang() {
        let tokens = Scanner::new().scan("!");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::Bang
            })
        );
    }

    #[test]
    fn it_parses_a_sigil() {
        let tokens = Scanner::new().scan("$");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::Sigil
            })
        );
    }

    #[test]
    fn it_parses_a_plus() {
        let tokens = Scanner::new().scan("+");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::Plus
            })
        );
    }

    #[test]
    fn it_parses_a_minus() {
        let tokens = Scanner::new().scan("-");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::Minus
            })
        );
    }

    #[test]
    fn it_parses_a_star() {
        let tokens = Scanner::new().scan("*");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::Star
            })
        );
    }

    #[test]
    fn it_parses_a_slash() {
        let tokens = Scanner::new().scan("/");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::Slash
            })
        );
    }

    #[test]
    fn it_parses_a_caret() {
        let tokens = Scanner::new().scan("^");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::Caret
            })
        );
    }

    #[test]
    fn it_parses_a_modulus() {
        let tokens = Scanner::new().scan("%");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::Modulus
            })
        );
    }

    #[test]
    fn it_parses_a_tilde() {
        let tokens = Scanner::new().scan("~");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::Tilde
            })
        );
    }

    #[test]
    fn it_parses_a_single_digit_number() {
        let tokens = Scanner::new().scan("1");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: Some(String::from("1")),
                token_type: &TokenType::Number
            })
        );
    }

    #[test]
    fn it_parses_a_double_digit_number() {
        let tokens = Scanner::new().scan("123");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: Some(String::from("123")),
                token_type: &TokenType::Number
            })
        );
    }

    #[test]
    fn it_parses_a_number_with_leading_zero() {
        let tokens = Scanner::new().scan("01");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: Some(String::from("01")),
                token_type: &TokenType::Number
            })
        );
    }

    #[test]
    fn it_parses_a_floating_point_number() {
        let tokens = Scanner::new().scan("1.0");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: Some(String::from("1.0")),
                token_type: &TokenType::Number
            })
        );
    }

    #[test]
    fn it_parses_a_floating_point_number_with_many_base_digits() {
        let tokens = Scanner::new().scan("987.2");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: Some(String::from("987.2")),
                token_type: &TokenType::Number
            })
        );
    }

    #[test]
    fn it_parses_a_floating_point_number_with_many_fractional_digits() {
        let tokens = Scanner::new().scan("1.09876");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: Some(String::from("1.09876")),
                token_type: &TokenType::Number
            })
        );
    }

    #[test]
    fn it_parses_a_number_without_a_fraction() {
        let tokens = Scanner::new().scan("1.");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: Some(String::from("1.")),
                token_type: &TokenType::Number
            })
        );
    }

    #[test]
    fn it_stops_parsing_a_number_at_whitespace() {
        let tokens = Scanner::new().scan("1 > 0");
        let mut token_iter = tokens.iter();

        assert_eq!(tokens.len(), 3);
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: Some(String::from("1")),
                token_type: &TokenType::Number
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::GreaterThan
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: Some(String::from("0")),
                token_type: &TokenType::Number
            })
        );
    }

    #[test]
    fn it_parses_a_number_with_a_comma_into_two() {
        let tokens = Scanner::new().scan("1,000");
        let mut token_iter = tokens.iter();

        assert_eq!(tokens.len(), 3);
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: Some(String::from("1")),
                token_type: &TokenType::Number
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::Comma
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: Some(String::from("000")),
                token_type: &TokenType::Number
            })
        );
    }

    #[test]
    fn it_parses_a_word() {
        let tokens = Scanner::new().scan("print");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: Some(String::from("print")),
                token_type: &TokenType::Print
            })
        );
    }

    #[test]
    fn it_parses_a_word_with_numbers() {
        let tokens = Scanner::new().scan("h3ll0");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: Some(String::from("h3ll0")),
                token_type: &TokenType::Identifier
            })
        );
    }

    #[test]
    fn it_parses_a_word_with_underscores() {
        let tokens = Scanner::new().scan("hello_world");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: Some(String::from("hello_world")),
                token_type: &TokenType::Identifier
            })
        );
    }

    #[test]
    fn it_parses_a_word_with_uppercase_letters() {
        let tokens = Scanner::new().scan("Hello");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: Some(String::from("Hello")),
                token_type: &TokenType::Identifier
            })
        );
    }

    #[test]
    fn it_parses_a_word_with_numbers_separately() {
        let tokens = Scanner::new().scan("1Hello");
        let mut token_iter = tokens.iter();

        assert_eq!(tokens.len(), 2);
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: Some(String::from("1")),
                token_type: &TokenType::Number
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: Some(String::from("Hello")),
                token_type: &TokenType::Identifier
            })
        );
    }

    #[test]
    fn it_parses_a_small_program() {
        let tokens = Scanner::new().scan("'1 > 0 { print; }' # print is cool");
        let mut token_iter = tokens.iter();

        assert_eq!(tokens.len(), 10);
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::SingleQuote
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: Some(String::from("1")),
                token_type: &TokenType::Number
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::GreaterThan
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: Some(String::from("0")),
                token_type: &TokenType::Number
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::LeftCurly
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: Some(String::from("print")),
                token_type: &TokenType::Print
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::Semicolon
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::RightCurly
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::SingleQuote
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: None,
                token_type: &TokenType::Pound
            })
        );
    }
}
