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

#[cfg(test)]
mod precedence {
    use super::*;

    #[test]
    fn next_precedence_returns_correct_val_for_none() {
        assert_eq!(
            Precedence::next_precedence(Precedence::None),
            Precedence::Assignment
        );
    }

    #[test]
    fn next_precedence_returns_correct_val_for_assignment() {
        assert_eq!(
            Precedence::next_precedence(Precedence::Assignment),
            Precedence::Conditional
        );
    }

    #[test]
    fn next_precedence_returns_correct_val_for_conditional() {
        assert_eq!(
            Precedence::next_precedence(Precedence::Conditional),
            Precedence::LogicalOr
        );
    }

    #[test]
    fn next_precedence_returns_correct_val_for_logical_or() {
        assert_eq!(
            Precedence::next_precedence(Precedence::LogicalOr),
            Precedence::LogicalAnd
        );
    }

    #[test]
    fn next_precedence_returns_correct_val_for_logical_and() {
        assert_eq!(
            Precedence::next_precedence(Precedence::LogicalAnd),
            Precedence::Comparison
        );
    }

    #[test]
    fn next_precedence_returns_correct_val_for_comparison() {
        assert_eq!(
            Precedence::next_precedence(Precedence::Comparison),
            Precedence::Concatenation
        );
    }

    #[test]
    fn next_precedence_returns_correct_val_for_concatenation() {
        assert_eq!(
            Precedence::next_precedence(Precedence::Concatenation),
            Precedence::Term
        );
    }

    #[test]
    fn next_precedence_returns_correct_val_for_term() {
        assert_eq!(
            Precedence::next_precedence(Precedence::Term),
            Precedence::Factor
        );
    }

    #[test]
    fn next_precedence_returns_correct_val_for_factor() {
        assert_eq!(
            Precedence::next_precedence(Precedence::Factor),
            Precedence::Unary
        );
    }

    #[test]
    fn next_precedence_returns_correct_val_for_unary() {
        assert_eq!(
            Precedence::next_precedence(Precedence::Unary),
            Precedence::Exponentiation
        );
    }

    #[test]
    fn next_precedence_returns_correct_val_for_exponentiation() {
        assert_eq!(
            Precedence::next_precedence(Precedence::Exponentiation),
            Precedence::FieldVariable
        );
    }

    #[test]
    fn next_precedence_returns_correct_val_for_field_variable() {
        assert_eq!(
            Precedence::next_precedence(Precedence::FieldVariable),
            Precedence::Primary
        );
    }

    #[test]
    fn next_precedence_returns_correct_val_for_primary() {
        assert_eq!(
            Precedence::next_precedence(Precedence::Primary),
            Precedence::Primary
        );
    }
}
