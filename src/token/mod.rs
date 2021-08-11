pub mod token_type;

use crate::token::token_type::TokenType;

/// A representation of a Token
#[derive(Debug, PartialEq)]
pub struct Token {
    pub lexeme: Option<String>,
    pub token_type: &'static TokenType,
    pub line: i32,
    pub start_idx: i32,
}

impl Token {
    /// Constructs a new `Token`
    ///
    /// # Arguments
    /// - `lexeme` the lexeme to store in the Token
    /// - `token_type` the [TokenType] associated with the Token
    /// - `line` the line number at which the Token appears in the corpus
    pub fn new(
        lexeme: Option<String>,
        token_type: &'static TokenType,
        line: i32,
        start_idx: i32,
    ) -> Token {
        Token {
            lexeme,
            token_type,
            line,
            start_idx,
        }
    }

    /// Constructs a new `Token` that represents a synthetic error
    ///
    /// The [Token#structfield.token_type] will automatically be assigned [TokenType::Error]
    ///
    /// # Arguments
    /// - `message` the error message to store in the Token
    /// - `line` the line number at which the Token appears in the corpus
    /// - `start_idx` the index at which the Token appears in the given `line`
    pub fn error_token(message: String, line: i32, start_idx: i32) -> Token {
        Token::new(Some(message), &TokenType::Error, line, start_idx)
    }

    // pub fn length(&self) -> usize {
    //     self.lexeme.as_ref().unwrap_or("".into()).len()
    // }
}
