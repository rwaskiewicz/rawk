#[derive(Debug, PartialEq)]
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
