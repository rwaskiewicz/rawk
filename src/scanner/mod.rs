use super::token::token_type::TokenType;

#[derive(Debug)]
pub struct Scanner {}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {}
    }

    pub fn scan(&self, input: &str) -> Vec<TokenType> {
        let mut tokens = Vec::new();
        let mut current_line = 1;
        let mut _lexeme_current = 0;
        let mut _lexeme_start = 0;

        for ch in input.chars() {
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
                    tokens.push(TokenType::LeftCurly);
                }
                '}' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::RightCurly
                    );
                    tokens.push(TokenType::RightCurly);
                }
                '[' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::LeftSquareBracket
                    );
                    tokens.push(TokenType::LeftSquareBracket);
                }
                ']' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::RightSquareBracket
                    );
                    tokens.push(TokenType::RightSquareBracket);
                }
                '(' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::LeftParenthesis
                    );
                    tokens.push(TokenType::LeftParenthesis);
                }
                ')' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::RightParenthesis
                    );
                    tokens.push(TokenType::RightParenthesis);
                }
                '\'' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::SingleQuote
                    );
                    tokens.push(TokenType::SingleQuote);
                }
                '\"' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::DoubleQuote
                    );
                    tokens.push(TokenType::DoubleQuote);
                }
                '>' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::GreaterThan
                    );
                    tokens.push(TokenType::GreaterThan);
                }
                '<' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::LessThan
                    );
                    tokens.push(TokenType::LessThan);
                }
                '=' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::Equals
                    );
                    tokens.push(TokenType::Equals);
                }
                '!' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::Bang
                    );
                    tokens.push(TokenType::Bang);
                }
                '$' => {
                    println!(
                        "Found a '{}', setting the type to {:?}",
                        ch,
                        TokenType::Sigil
                    );
                    tokens.push(TokenType::Sigil);
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

        println!("This is the final list of Tokens {:?}", tokens);
        tokens
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(tokens.iter().next(), Some(&TokenType::LeftCurly));
    }

    #[test]
    fn it_parses_a_right_curly_brace() {
        let tokens = Scanner::new().scan("}");

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.iter().next(), Some(&TokenType::RightCurly));
    }

    #[test]
    fn it_parses_a_left_square_bracket() {
        let tokens = Scanner::new().scan("[");

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.iter().next(), Some(&TokenType::LeftSquareBracket));
    }

    #[test]
    fn it_parses_a_right_square_bracket() {
        let tokens = Scanner::new().scan("]");

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.iter().next(), Some(&TokenType::RightSquareBracket));
    }

    #[test]
    fn it_parses_a_left_parenthesis() {
        let tokens = Scanner::new().scan("(");

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.iter().next(), Some(&TokenType::LeftParenthesis));
    }

    #[test]
    fn it_parses_a_right_parenthesis() {
        let tokens = Scanner::new().scan(")");

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.iter().next(), Some(&TokenType::RightParenthesis));
    }

    #[test]
    fn it_parses_a_single_quote() {
        let tokens = Scanner::new().scan("\'");

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.iter().next(), Some(&TokenType::SingleQuote));
    }

    #[test]
    fn it_parses_a_double_quote() {
        let tokens = Scanner::new().scan("\"");

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.iter().next(), Some(&TokenType::DoubleQuote));
    }

    #[test]
    fn it_parses_a_greater_than_caret() {
        let tokens = Scanner::new().scan(">");

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.iter().next(), Some(&TokenType::GreaterThan));
    }

    #[test]
    fn it_parses_a_less_than_caret() {
        let tokens = Scanner::new().scan("<");

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.iter().next(), Some(&TokenType::LessThan));
    }

    #[test]
    fn it_parses_an_assignment_token() {
        let tokens = Scanner::new().scan("=");

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.iter().next(), Some(&TokenType::Equals));
    }

    #[test]
    fn it_parses_a_bang() {
        let tokens = Scanner::new().scan("!");

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.iter().next(), Some(&TokenType::Bang));
    }

    #[test]
    fn it_parses_a_sigil() {
        let tokens = Scanner::new().scan("$");

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.iter().next(), Some(&TokenType::Sigil));
    }
}
