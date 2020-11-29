#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single Character Tokens
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
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Modulus,
    Tilde,
    Semicolon,
    Comma,
    Pound,

    // Variable Character Tokens
    Number,
    Word,
}
