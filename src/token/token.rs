use crate::token::token_type::TokenType;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
}

impl Token {
    pub fn new(token_type: TokenType) -> Token {
        Token { token_type }
    }
}
