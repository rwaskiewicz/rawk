use super::token::token::Token;
use super::token::token_type::TokenType;

#[derive(Debug)]
pub struct Scanner {}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {}
    }

    pub fn scan(&self, input: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut current_line = 1;
        let mut _lexeme_current = 0;
        let mut _lexeme_start = 0;

        let mut char_stream = input.chars().peekable();
        while let Some(_) = char_stream.peek() {
            // for ch in char_stream {
            let mut ch = char_stream.next().unwrap();
            println!("Inspecting Character: '{}'", ch);
            // TODO: This is _very_ basic switching that is not looking for combining operator '>='
            match ch {
                ' ' | '\r' | '\t' => println!("I can see and accept whitespace"),
                '\n' => {
                    current_line += 1;
                    println!("I can see the newline, which is now {}", current_line);
                }
                '{' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::LeftCurly
                    );
                    tokens.push(Token::new(TokenType::LeftCurly));
                }
                '}' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::RightCurly
                    );
                    tokens.push(Token::new(TokenType::RightCurly));
                }
                '[' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::LeftSquareBracket
                    );
                    tokens.push(Token::new(TokenType::LeftSquareBracket));
                }
                ']' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::RightSquareBracket
                    );
                    tokens.push(Token::new(TokenType::RightSquareBracket));
                }
                '(' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::LeftParenthesis
                    );
                    tokens.push(Token::new(TokenType::LeftParenthesis));
                }
                ')' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::RightParenthesis
                    );
                    tokens.push(Token::new(TokenType::RightParenthesis));
                }
                '\'' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::SingleQuote
                    );
                    tokens.push(Token::new(TokenType::SingleQuote));
                }
                '\"' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::DoubleQuote
                    );
                    tokens.push(Token::new(TokenType::DoubleQuote));
                }
                '>' => {
                    // TODO: Support greater than or equal to '>='
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::GreaterThan
                    );
                    tokens.push(Token::new(TokenType::GreaterThan));
                }
                '<' => {
                    // TODO: Support less than or equal to '<='
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::LessThan
                    );
                    tokens.push(Token::new(TokenType::LessThan));
                }
                '=' => {
                    // TODO: Support equal to '=='
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::Equals
                    );
                    tokens.push(Token::new(TokenType::Equals));
                }
                '!' => {
                    // TODO: Support not equal to '!='
                    // TODO: Support ERE non-match '!~'
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::Bang
                    );
                    tokens.push(Token::new(TokenType::Bang));
                }
                '$' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::Sigil
                    );
                    tokens.push(Token::new(TokenType::Sigil));
                },
                '+' => {
                    // TODO: Support addition '+'
                    // TODO: Support addition assignment '+='
                    // TODO: Support post-increment '++'
                    // TODO: Support pre-increment '++'
                },
                '-' => {
                    // TODO: Support subtraction '-'
                    // TODO: Support subtraction assignment '-='
                    // TODO: Support post-decrement '--'
                    // TODO: Support pre-decrement '--'
                },
                '*' => {
                    // TODO: Support multiplication '*'
                    // TODO: Support multiplication assignment '*='
                },
                '/' => {
                    // TODO: Support division '/'
                    // TODO: Support division assignment '/='
                },
                '^' => {
                    // TODO: Support exponentiation '^'
                    // TODO: Support exponentiation assignment '^='
                }
                '%' => {
                    // TODO: Support modulus '%'
                    // TODO: Support modulus assignment '%='
                },
                '~' => {
                    // TODO: Support ERE match '~'
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

                        println!("I see a number! {}", num_parsed);
                        // TODO: _what_ number?
                        tokens.push(Token::new(TokenType::Number));
                    } else if ch.is_alphabetic() {
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

                        println!("I see a word '{}'", word_parsed);
                        tokens.push(Token::new(TokenType::Word));
                    } else {
                        println!("ALERT: We found a character we can not handle, '{}'", ch);
                    }
                }
            }
        }

        println!("This is the final list of Tokens {:?}", tokens);
        tokens
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
    fn it_parses_a_left_curly_brace() {
        let tokens = Scanner::new().scan("{");

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.iter().next(),
            Some(&Token {
                token_type: TokenType::LeftCurly
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
                token_type: TokenType::RightCurly
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
                token_type: TokenType::LeftSquareBracket
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
                token_type: TokenType::RightSquareBracket
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
                token_type: TokenType::LeftParenthesis
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
                token_type: TokenType::RightParenthesis
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
                token_type: TokenType::SingleQuote
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
                token_type: TokenType::DoubleQuote
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
                token_type: TokenType::GreaterThan
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
                token_type: TokenType::LessThan
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
                token_type: TokenType::Equals
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
                token_type: TokenType::Bang
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
                token_type: TokenType::Sigil
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
                token_type: TokenType::Number
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
                token_type: TokenType::Number
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
                token_type: TokenType::Number
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
                token_type: TokenType::Number
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
                token_type: TokenType::Number
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
                token_type: TokenType::Number
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
                token_type: TokenType::Number
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
                token_type: TokenType::Number
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                token_type: TokenType::GreaterThan
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                token_type: TokenType::Number
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
                token_type: TokenType::Word
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
                token_type: TokenType::Word
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
                token_type: TokenType::Word
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
                token_type: TokenType::Word
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
                token_type: TokenType::Number
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                token_type: TokenType::Word
            })
        );
    }

    #[test]
    fn it_parses_a_small_program() {
        let tokens = Scanner::new().scan("'1 > 0 { print }'");
        let mut token_iter = tokens.iter();

        assert_eq!(tokens.len(), 8);
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                token_type: TokenType::SingleQuote
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                token_type: TokenType::Number
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                token_type: TokenType::GreaterThan
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                token_type: TokenType::Number
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                token_type: TokenType::LeftCurly
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                token_type: TokenType::Word
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                token_type: TokenType::RightCurly
            })
        );
        assert_eq!(
            token_iter.next(),
            Some(&Token {
                token_type: TokenType::SingleQuote
            })
        );
    }
}
