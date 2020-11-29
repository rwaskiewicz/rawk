use crate::token::token_type::TokenType;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub lexeme: Option<String>,
    pub token_type: TokenType,
}

impl Token {
    pub fn new(lexeme: Option<String>, token_type: TokenType) -> Token {
        Token { lexeme, token_type }
    }
}
