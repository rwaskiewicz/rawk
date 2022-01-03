/// Enum describing associativity of items in the grammar
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Associativity {
    NA,
    Left,
    Right,
}
