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
    Identifier,

    // Keywords
    Begin, // 'BEGIN'
    End,   // 'END'

    Break,    // 'break'
    Continue, // 'continue'
    Delete,   // 'delete'
    Do,       // 'do'
    Else,     // 'else'

    Exit,     // 'exit'
    For,      // 'for'
    Function, // 'function'
    If,       // 'if'
    In,       // 'in'

    Next,   // 'next'
    Print,  // 'print'
    Printf, // 'printf'
    Return, // 'return'
    While,  // 'while'
}
