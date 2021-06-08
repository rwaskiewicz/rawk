#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

    // Two-character tokens.
    AddAssign,    // '+='
    SubAssign,    // '-='
    MulAssign,    // '*='
    DivAssign,    // '/='
    ModAssign,    // '%='
    PowAssign,    // '^='
    Or,           // '||'
    And,          // '&&'
    NoMatch,      // '!~'
    DoubleEqual,  // '=='
    LessEqual,    // '<='
    GreaterEqual, // '>='
    NotEqual,     // '!='
    Incr,         // '++'
    Decr,         // '--'
    Append,       // '>>'

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

    // End of File
    Eof,

    // Synthetic Error
    Error,

    // Synthetic Concatenation
    StringConcat,
}
