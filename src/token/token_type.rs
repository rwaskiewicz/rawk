#[derive(Debug, PartialEq)]
pub enum TokenType {
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

    // TODO: built in fn
    GetLine, // 'GETLINE'

    // TODO: Two-character tokens.
    // %token ADD_ASSIGN SUB_ASSIGN MUL_ASSIGN DIV_ASSIGN MOD_ASSIGN POW_ASSIGN
    /*     '+='       '-='       '*='       '/='       '%='       '^=' */
    // Single Character Tokens
    LeftCurly,
    RightCurly,
    LeftParenthesis,
    RightParenthesis,
    LeftSquareBracket,
    RightSquareBracket,
    Comma,
    Semicolon,
    // TODO: NEWLINE
    Plus,
    Minus,
    Star,
    Modulus,
    Caret,
    Bang,
    GreaterThan,
    LessThan,
    Pipe,
    Question,
    Colon,
    Tilde,
    Sigil,
    Equals,

    // TODO: None of these are in the yacc spec...
    SingleQuote,
    DoubleQuote,
    Slash,
    Pound,

    // Variable Character Tokens
    Number,
    Identifier,
}
