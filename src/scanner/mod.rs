use super::token::token::Token;
use super::token::token_type::TokenType;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Scanner {
    input: String,
    keywords: HashMap<&'static str, &'static TokenType>,
}

impl Scanner {
    pub fn new(input: String) -> Scanner {
        let mut keywords: HashMap<&str, &TokenType> = HashMap::new();
        keywords.insert("BEGIN", &TokenType::Begin);
        keywords.insert("END", &TokenType::End);
        keywords.insert("break", &TokenType::Break);
        keywords.insert("continue", &TokenType::Continue);
        keywords.insert("delete", &TokenType::Delete);
        keywords.insert("do", &TokenType::Do);
        keywords.insert("else", &TokenType::Else);
        keywords.insert("exit", &TokenType::Exit);
        keywords.insert("for", &TokenType::For);
        keywords.insert("function", &TokenType::Function);
        keywords.insert("if", &TokenType::If);
        keywords.insert("in", &TokenType::In);
        keywords.insert("next", &TokenType::Next);
        keywords.insert("print", &TokenType::Print);
        keywords.insert("printf", &TokenType::Printf);
        keywords.insert("return", &TokenType::Return);
        keywords.insert("while", &TokenType::While);
        keywords.insert("GETLINE", &TokenType::GetLine);
        keywords.shrink_to_fit();

        Scanner { input, keywords }
    }

    pub fn scan(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut current_line = 1;

        let mut char_stream = self.input.chars().peekable();
        while char_stream.peek().is_some() {
            // TODO: This is _technically_ OK I think, but heed the RustDoc and see if there's a better solution once the build is working again
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
                    Scanner::report_scanned_character(ch, &TokenType::DoubleQuote);

                    // Token::new(None, &TokenType::DoubleQuote)
                    let mut string_parsed = String::from("");
                    string_parsed.push(ch);
                    while let Some(_maybe_quote) = char_stream.peek() {
                        if let Some(next_ch) = char_stream.next() {
                            if next_ch == '\n' {
                                current_line += 1;
                            } else if next_ch == '\"' {
                                // TODO: Handle EOF/unterminated string
                                ch = next_ch;
                                string_parsed.push(ch);
                                break;
                            }
                            ch = next_ch;
                            string_parsed.push(ch);
                        }
                    }
                    let string_token = Token::new(Some(string_parsed), &TokenType::DoubleQuote);

                    tokens.push(string_token);
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
                '|' => {
                    Scanner::report_scanned_character(ch, &TokenType::Pipe);
                    tokens.push(Token::new(None, &TokenType::Pipe));
                }
                '?' => {
                    Scanner::report_scanned_character(ch, &TokenType::Question);
                    tokens.push(Token::new(None, &TokenType::Question));
                }
                ':' => {
                    Scanner::report_scanned_character(ch, &TokenType::Colon);
                    tokens.push(Token::new(None, &TokenType::Colon));
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

                        // TODO: Support scientific (exponential) notation like 0.707E-1, 1E1, 1e6, 1E
                        // Note that 'e' can be cased however, and may occur before or after the dot
                        // I'm unsure if that belongs in the parser or the scanner TBH - 1Ehello prints
                        // just '1' with `echo 'oo' | awk '{print 1Ehello}'`

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
        let tokens = Scanner::new(String::from("")).scan();

        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn it_parses_a_pound() {
        let tokens = Scanner::new(String::from("#")).scan();

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
        let tokens = Scanner::new(String::from("{print} # this print is important")).scan();
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
    fn it_parses_a_single_quote() {
        let tokens = Scanner::new(String::from("\'")).scan();

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
        let tokens = Scanner::new(String::from("\"Hello World!\"")).scan();

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: Some(String::from("\"Hello World!\"")),
                token_type: &TokenType::DoubleQuote
            })
        );
    }

    #[test]
    fn it_does_not_swallow_items_after_closing_a_double_quote() {
        let tokens = Scanner::new(String::from("\"Hello World!\"42")).scan();
        let mut token_iter = tokens.iter();

        assert_eq!(token_iter.len(), 2);
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: Some(String::from("\"Hello World!\"")),
                token_type: &TokenType::DoubleQuote
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                lexeme: Some(String::from("42")),
                token_type: &TokenType::Number
            })
        );
    }

    #[test]
    fn it_parses_a_slash() {
        let tokens = Scanner::new(String::from("/")).scan();

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
    fn it_parses_single_character_tokens() {
        let test_cases: [(&str, &TokenType); 22] = [
            ("{", &TokenType::LeftCurly),
            ("}", &TokenType::RightCurly),
            ("(", &TokenType::LeftParenthesis),
            (")", &TokenType::RightParenthesis),
            ("[", &TokenType::LeftSquareBracket),
            ("]", &TokenType::RightSquareBracket),
            (",", &TokenType::Comma),
            (";", &TokenType::Semicolon),
            // TODO: NEWLINE
            ("+", &TokenType::Plus),
            ("-", &TokenType::Minus),
            ("*", &TokenType::Star),
            ("%", &TokenType::Modulus),
            ("^", &TokenType::Caret),
            ("!", &TokenType::Bang),
            (">", &TokenType::GreaterThan),
            ("<", &TokenType::LessThan),
            ("|", &TokenType::Pipe),
            ("?", &TokenType::Question),
            (":", &TokenType::Colon),
            ("~", &TokenType::Tilde),
            ("$", &TokenType::Sigil),
            ("=", &TokenType::Equals),
        ];

        for test_case in test_cases.iter() {
            let token = test_case.0;
            let token_type = test_case.1;

            let tokens = Scanner::new(String::from(token)).scan();

            assert_eq!(tokens.len(), 1);
            assert_eq!(
                tokens.iter().next(),
                Some(&Token {
                    lexeme: None,
                    token_type
                })
            );
        }
    }

    #[test]
    fn it_parses_a_single_digit_number() {
        let tokens = Scanner::new(String::from("1")).scan();

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
        let tokens = Scanner::new(String::from("123")).scan();

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
        let tokens = Scanner::new(String::from("01")).scan();

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
        let tokens = Scanner::new(String::from("1.0")).scan();

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
        let tokens = Scanner::new(String::from("987.2")).scan();

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
        let tokens = Scanner::new(String::from("1.09876")).scan();

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
        let tokens = Scanner::new(String::from("1.")).scan();

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
        let tokens = Scanner::new(String::from("1 > 0")).scan();
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
        let tokens = Scanner::new(String::from("1,000")).scan();
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
    fn it_parses_keywords() {
        let test_cases: [(&str, &TokenType); 18] = [
            ("BEGIN", &TokenType::Begin),
            ("END", &TokenType::End),
            ("break", &TokenType::Break),
            ("continue", &TokenType::Continue),
            ("delete", &TokenType::Delete),
            ("do", &TokenType::Do),
            ("else", &TokenType::Else),
            ("exit", &TokenType::Exit),
            ("for", &TokenType::For),
            ("function", &TokenType::Function),
            ("if", &TokenType::If),
            ("in", &TokenType::In),
            ("next", &TokenType::Next),
            ("print", &TokenType::Print),
            ("printf", &TokenType::Printf),
            ("return", &TokenType::Return),
            ("while", &TokenType::While),
            ("GETLINE", &TokenType::GetLine),
        ];

        for test_case in test_cases.iter() {
            let token = test_case.0;
            let token_type = test_case.1;

            let tokens = Scanner::new(String::from(token)).scan();

            assert_eq!(tokens.len(), 1);
            assert_eq!(
                tokens.iter().next(),
                Some(&Token {
                    lexeme: Some(String::from(token)),
                    token_type
                })
            );
        }
    }

    #[test]
    fn it_parses_case_sensitive_keyboards_as_identifiers() {
        let tokens = Scanner::new(String::from("PRINT")).scan();

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                lexeme: Some(String::from("PRINT")),
                token_type: &TokenType::Identifier
            })
        );
    }

    #[test]
    fn it_parses_an_identifier_with_numbers() {
        let tokens = Scanner::new(String::from("h3ll0")).scan();

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
    fn it_parses_an_identifier_with_underscores() {
        let tokens = Scanner::new(String::from("hello_world")).scan();

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
    fn it_parses_an_identifier_with_uppercase_letters() {
        let tokens = Scanner::new(String::from("Hello")).scan();

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
    fn it_parses_an_identifier_with_numbers_separately() {
        let tokens = Scanner::new(String::from("1Hello")).scan();
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
        let tokens = Scanner::new(String::from("'1 > 0 { print; }' # print is cool")).scan();
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
