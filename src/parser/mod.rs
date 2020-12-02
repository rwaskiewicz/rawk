use super::token::token::Token;

pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse(&self, tokens: Vec<Token>) {
        println!("Got tokens {:?}", tokens)
    }
}
