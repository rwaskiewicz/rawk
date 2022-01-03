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
