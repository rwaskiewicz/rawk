use crate::parser::Parser;
use crate::TokenType;

/// Enum describing associativity of items in the grammar
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Associativity {
    NA,
    Left,
    Right,
}

/// Enum that creates a hierarchy of precedences that are associated with a [TokenType].
///
/// Variants with lower values have a lower precedence than higher value variants.
///
/// [Parser::parse_precedence] relies heavily on this ordering to determine how may [Token]s to
/// parse at a given time.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Precedence {
    None,
    Assignment,     // '='
    Conditional,    // ternary - '?'
    LogicalOr,      // '||'
    LogicalAnd,     // '&&'
    Comparison,     // '>' '>=' '<' '<=' '==' '!=' // TODO: Where does append fit in?
    Concatenation,  // String concatenation, left associative
    Term,           // '+' '-'
    Factor,         // '*' '/' '%'
    Unary,          // '!' '+' '-'
    Exponentiation, // '^'
    FieldVariable,  // '$0', '$1', etc.
    Primary,
}

impl Precedence {
    /// Retrieves the next precedence level for a given variant
    ///
    /// # Arguments
    /// - `p` the precedence to get the next precedence level for
    ///
    /// # Return value
    /// - the next precedence level
    pub fn next_precedence(p: Precedence) -> Precedence {
        match p {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Conditional,
            Precedence::Conditional => Precedence::LogicalOr,
            Precedence::LogicalOr => Precedence::LogicalAnd,
            Precedence::LogicalAnd => Precedence::Comparison,
            Precedence::Comparison => Precedence::Concatenation,
            Precedence::Concatenation => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Exponentiation,
            Precedence::Exponentiation => Precedence::FieldVariable,
            Precedence::FieldVariable => Precedence::Primary,
            Precedence::Primary => Precedence::Primary,
        }
    }
}

/// Type describing functions that will be invoked at parse time.
///
/// The function that is invoked is predicated on a token that is read from the input stream. Some
/// functions shall require the `can_assign` variable to be plumbed to them to act appropriately in
/// the presence of a [`TokenType::Equals`] tokens.
///
/// # Arguments
/// - `can_assign` whether or not assignment to a variable is permitted
type ParseFn = fn(&mut Parser, can_assign: bool) -> ();

#[derive(Copy, Clone)]
pub struct ParseRule {
    // function to compile a _prefix expression_ starting with a token of some type
    // aka 'nuds' or 'null denotations'
    pub prefix_parse_fn: Option<ParseFn>,
    // function to compile an _infix expression_ whose left operand is followed by a token of that type
    // aka 'leds' or 'left denotations'
    pub infix_parse_fn: Option<ParseFn>,
    // the precedence of an _infix expression_ that uses a token as an operator
    pub infix_precedence: Precedence,
    // the associativity of an infix expression
    pub infix_associativity: Associativity,
}

// TODO: May not need this, it's all about access to the table, which could have if I can figure |
// out how to reference some of these things....
/// Helper function for indexing the table of parse rules
///
/// # Arguments
/// - `token_type` the type of the token to use as a part of the lookup
///
/// # Return value
/// - A reference to the [ParseRule] for the given token type
pub fn get_rule(token_type: &TokenType) -> &ParseRule {
    &PARSE_RULES[token_type.clone() as usize]
}

/// Table of rules associating a [TokenType] to a prefix expression function pointer, infix
/// expression pointer, and a precedence.
///
/// When an infix expression function from this table is called, it's left hand side (LHS) has
/// already been compiled and the infix operator consumed.
const PARSE_RULES: [ParseRule; 65] = [
    // BEGIN
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // END
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // break
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // continue
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // delete
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // do
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // else
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // exit
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // for
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // function
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // if
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // in
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::Left,
    },
    // next
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // print
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // printf
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // return
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // while
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // GETLINE
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // AddAssign
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::Right,
    },
    // SubAssign
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::Right,
    },
    // MulAssign
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::Right,
    },
    // DivAssign
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::Right,
    },
    // ModAssign
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::Right,
    },
    // PowAssign
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::Right,
    },
    // (Logical) Or
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: Some(|parser, _can_assign| parser.logical_or()),
        infix_precedence: Precedence::LogicalOr,
        infix_associativity: Associativity::Left,
    },
    // (Logical) And
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: Some(|parser, _can_assign| parser.logical_and()),
        infix_precedence: Precedence::LogicalAnd,
        infix_associativity: Associativity::Left,
    },
    // NoMatch
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // DoubleEqual
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: Some(|parser, _can_assign| parser.binary()),
        infix_precedence: Precedence::Comparison,
        infix_associativity: Associativity::NA,
    },
    // LessEqual
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: Some(|parser, _can_assign| parser.binary()),
        infix_precedence: Precedence::Comparison,
        infix_associativity: Associativity::NA,
    },
    // GreaterEqual
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: Some(|parser, _can_assign| parser.binary()),
        infix_precedence: Precedence::Comparison,
        infix_associativity: Associativity::NA,
    },
    // NotEqual
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: Some(|parser, _can_assign| parser.binary()),
        infix_precedence: Precedence::Comparison,
        infix_associativity: Associativity::NA,
    },
    // Incr
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // Decr
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // Append
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // LeftCurly
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // RightCurly
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // LeftParenthesis
    ParseRule {
        prefix_parse_fn: Some(|parser, _can_assign| parser.grouping()),
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // RightParenthesis
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // LeftSquareBracket
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // RightSquareBracket
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // Comma
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // Semicolon
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // Plus
    ParseRule {
        prefix_parse_fn: Some(|parser, _can_assign| parser.unary()),
        infix_parse_fn: Some(|parser, _can_assign| parser.binary()),
        infix_precedence: Precedence::Term,
        infix_associativity: Associativity::Left,
    },
    // Minus
    ParseRule {
        prefix_parse_fn: Some(|parser, _can_assign| parser.unary()),
        infix_parse_fn: Some(|parser, _can_assign| parser.binary()),
        infix_precedence: Precedence::Term,
        infix_associativity: Associativity::Left,
    },
    // Star
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: Some(|parser, _can_assign| parser.binary()),
        infix_precedence: Precedence::Factor,
        infix_associativity: Associativity::Left,
    },
    // Modulus
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: Some(|parser, _can_assign| parser.binary()),
        infix_precedence: Precedence::Factor,
        infix_associativity: Associativity::Left,
    },
    // Caret
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: Some(|parser, _can_assign| parser.binary()),
        infix_precedence: Precedence::Exponentiation,
        infix_associativity: Associativity::Right,
    },
    // Bang
    ParseRule {
        prefix_parse_fn: Some(|parser, _can_assign| parser.unary()),
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // GreaterThan
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: Some(|parser, _can_assign| parser.binary()),
        infix_precedence: Precedence::Comparison,
        infix_associativity: Associativity::NA,
    },
    // LessThan
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: Some(|parser, _can_assign| parser.binary()),
        infix_precedence: Precedence::Comparison,
        infix_associativity: Associativity::NA,
    },
    // Pipe
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // Question
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: Some(|parser, _can_assign| parser.conditional_expression()),
        infix_precedence: Precedence::Conditional,
        infix_associativity: Associativity::Right,
    },
    // Colon
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::Right,
    },
    // Tilde
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // Sigil
    ParseRule {
        prefix_parse_fn: Some(|parser, _can_assign| parser.field_variable()),
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // Equals
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // SingleQuote
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // DoubleQuote
    ParseRule {
        prefix_parse_fn: Some(|parser, _can_assign| parser.string()),
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // Slash
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: Some(|parser, _can_assign| parser.binary()),
        infix_precedence: Precedence::Term,
        infix_associativity: Associativity::Left,
    },
    // Pound
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // number
    ParseRule {
        prefix_parse_fn: Some(|parser, _can_assign| parser.number()),
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // identifier
    ParseRule {
        prefix_parse_fn: Some(|parser, can_assign| parser.variable(can_assign)),
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // eof
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // error
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
        infix_associativity: Associativity::NA,
    },
    // string concatenation (synthetic)
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: Some(|parser, _can_assign| parser.binary()),
        infix_precedence: Precedence::Concatenation,
        infix_associativity: Associativity::Left,
    },
];
