//! Parser for our implementation of awk.
//!
//! The parser is implemented as a Pratt Parser, and is heavily modeled after the one given in
//! "Crafting Interpreters" in the second half of the book.

use crate::chunk::{Chunk, OpCode};
use crate::token::token_type::TokenType;
use crate::token::Token;
use crate::value::Value;
use log::error;
use std::fmt::Debug;
use std::slice::Iter;

/// Enum describing associativity of items in the grammar
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
enum Associativity {
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
enum Precedence {
    None,
    Assignment,     // '='
    LogicalOr,      // '||'
    LogicalAnd,     // '&&'
    Comparison,     // '>' '>=' '<' '<=' '==' '!=' // TODO: Where does append fit in?
    Concatenation,  // String concatenation, left associative
    Term,           // '+' '-'
    Factor,         // '*' '/' '%'
    Unary,          // '!' '+' '-'
    Exponentiation, // '^'
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
            Precedence::Assignment => Precedence::LogicalOr,
            Precedence::LogicalOr => Precedence::LogicalAnd,
            Precedence::LogicalAnd => Precedence::Comparison,
            Precedence::Comparison => Precedence::Concatenation,
            Precedence::Concatenation => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Exponentiation,
            Precedence::Exponentiation => Precedence::Primary,
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

// TODO: We may need to track the precedence of the prefix for awk
#[derive(Copy, Clone)]
struct ParseRule {
    // function to compile a _prefix expression_ starting with a token of some type
    // aka 'nuds' or 'null denotations'
    prefix_parse_fn: Option<ParseFn>,
    // function to compile an _infix expression_ whose left operand is followed by a token of that type
    // aka 'leds' or 'left denotations'
    infix_parse_fn: Option<ParseFn>,
    // the precedence of an _infix expression_ that uses a token as an operator
    infix_precedence: Precedence,
    // the associativity of an infix expression
    infix_associativity: Associativity,
}

/// A Pratt Parser for our awk implementation
///
/// The `Parser` holds some stateful information in addition to the tokens it must iterate over:
/// - `current_token` a reference to the current token being examined
/// - `previous_token` a reference to the token that preceded the current token being examined
pub struct Parser<'a> {
    current_token: Option<&'a Token>,
    previous_token: Option<&'a Token>,
    tokens_iter: Iter<'a, Token>,
    compiling_chunk: &'a mut Chunk,
    had_error: bool,
    panic_mode: bool,
}

impl<'a> Parser<'a> {
    /// Constructs a new `Parser`
    ///
    /// # Arguments
    /// - `tokens` a slice iterator of [Token]s
    pub fn new(tokens: Iter<'a, Token>, compiling_chunk: &'a mut Chunk) -> Parser<'a> {
        Parser {
            current_token: None,
            previous_token: None,
            tokens_iter: tokens,
            compiling_chunk,
            had_error: false,
            panic_mode: false,
        }
    }

    /// Entrypoint for parsing tokens.
    ///
    /// # Return value
    /// - `true` if the program was parsed successfully
    /// - `false` otherwise
    pub fn parse(&mut self) -> bool {
        // prime the pump, so that the `current_token` is defined
        self.advance();

        while !self.match_token(&TokenType::Eof) {
            self.declaration();
        }

        self.end_compiler();

        !self.had_error
    }

    /// Parses a series of tokens, based on the precedence associated with them.
    ///
    /// Parses any expression that is equal to or of higher precedence than the one that is provided
    /// argument. This is accomplished by:
    /// 1. Advancing the pointer to the current token at least once
    /// 2. Invoking the prefix expression parse function for the `previous_token`
    /// 3. Successively advancing the pointer to the current token and calling the infix expression
    /// parse function for the `current_token` while its precedence is lower than the one that is
    /// provided. Lower precedences will subsume more/larger expressions.
    ///
    /// # Arguments
    /// - `precedence` the precedence of the current token
    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        // the first token is always going to belong to some kind of prefix expression, by
        // definition - although it may be nested as an operand in 1+ infix expressions
        let maybe_prefix_rule =
            get_rule(self.previous_token.expect("missing token").token_type).prefix_parse_fn;
        if maybe_prefix_rule.is_none() {
            self.error_at_previous("Expect expression.");
            return;
        }

        let can_assign: bool = precedence <= Precedence::Assignment;
        let prefix_rule = maybe_prefix_rule.unwrap();
        prefix_rule(self, can_assign);

        while precedence
            <= get_rule(self.current_token.expect("Missing token!").token_type).infix_precedence
        {
            self.advance();
            let infix_rule = get_rule(self.previous_token.expect("No Token was found!").token_type)
                .infix_parse_fn
                .unwrap();

            infix_rule(self, can_assign);
        }

        // if '=' is the current token, we should have consumed it somehow...report the error
        if can_assign && self.match_token(&TokenType::Equals) {
            self.error_at_current("Invalid assignment target.");
        }
    }

    /// Advances the pointers the parser has to the current and the previous token
    ///
    /// The `previous_token` will assume ownership of the `current_token`, and the `current_token`
    /// will be retrieved from the next in the stream of tokens
    fn advance(&mut self) {
        self.previous_token = self.current_token;

        // our scanner will emit 'EOF' tokens once, so if we've detected it, don't try to consume
        // anything else
        if self.current_token.is_some() && self.current_token.unwrap().token_type == &TokenType::Eof
        {
            return;
        }

        loop {
            self.current_token = self.tokens_iter.next();

            if self.current_token.unwrap().token_type != &TokenType::Error {
                break;
            }
            self.error_at_current("An error token was discovered.");
        }
    }

    /// Determines whether the pointer to the `current_token` is of some expected type or not
    ///
    /// # Arguments
    ///
    /// * `token_type` - The expected token type that the parser's `current_token` is sitting on
    /// * `error_msg` - The error message that should be emitted if the `current_token`'s type does
    /// not match `token_type`
    fn consume(&mut self, token_type: &TokenType, error_msg: &str) {
        if self.current_token.expect("No token type").token_type == token_type {
            self.advance();
            return;
        }
        // the `current_token` didn't match, report the error
        self.error_at_current(error_msg);
    }

    /// Helper function for determining whether or not the current token is of the same type as the
    /// one provided
    ///
    /// If the provided token type matches, advance the current token
    ///
    /// # Arguments
    /// - `token_type` the token type to match
    ///
    /// # Return value
    /// - `true` if the current token type matches
    /// - `false` otherwise
    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if token_type == self.current_token.unwrap().token_type {
            self.advance();
            return true;
        }
        false
    }

    /// Parse a declaration production
    fn declaration(&mut self) {
        if self.match_token(&TokenType::Identifier) {
            self.variable_declaration();
        } else {
            self.statement();
        }
    }

    /// Parse a variable production
    fn variable_declaration(&mut self) {
        // store the variable name so it can be looked up later
        let global_variable_index = self.parse_variable();

        if self.match_token(&TokenType::Equals) {
            self.expression();
        } else {
            self.emit_constant(Value::String("".into()));
        }
        self.consume(
            &TokenType::Semicolon,
            "Expect ';' after a variable declaration.",
        );

        self.define_variable(global_variable_index);
    }

    /// Parses a variable's name and places it in the current chunk's constant table
    ///
    /// # Return value
    /// the index of the variable name in the constant table for easy lookup
    fn parse_variable(&mut self) -> usize {
        self.compiling_chunk.add_constant(
            self.previous_token
                .expect("No token was parsed")
                .lexeme
                .clone()
                .expect("Variable name was empty"),
        )
    }

    /// Output the bytecode instructions for a new variable definition
    ///
    /// # Arguments
    /// - `global_var_index` the index of the variable name in a chunk's constants table
    fn define_variable(&mut self, global_var_index: usize) {
        self.emit_byte(OpCode::DefineGlobal(global_var_index));
    }

    /// Function for parsing a statement
    fn statement(&mut self) {
        if self.match_token(&TokenType::Print) {
            self.print_statement();
        } else {
            // we're looking at an expression statement (as the name of the next LoC implies)
            self.expression();
            self.consume(
                &TokenType::Semicolon,
                "Expect ';' at the end of a statement.",
            );
            // discard the result
            self.emit_byte(OpCode::Pop);
        }
    }

    /// Function for parsing a print statement
    ///
    /// TODO: output_redirection support
    fn print_statement(&mut self) {
        self.simple_print_statement();
        self.consume(
            &TokenType::Semicolon,
            "Expect ';' at the end of a statement.",
        );
        self.emit_byte(OpCode::OpPrint);
    }

    fn simple_print_statement(&mut self) {
        self.print_expr_list_opt();
        // TODO: Support additional arms of this part of the grammar
        // | Print  '(' multiple_expr_list ')'
        // | Printf print_expr_list
        // | Printf '(' multiple_expr_list ')'
    }

    fn print_expr_list_opt(&mut self) {
        // TODO: Support empty
        self.print_expr_list();
    }

    fn print_expr_list(&mut self) {
        self.print_expr();
        // TODO: | print_expr_list ',' newline_opt print_expr
    }

    fn print_expr(&mut self) {
        // TODO: These rules don't align with the grammar. We'll need to move that over and support
        // the who shebang at some point
        // print_expr       : unary_print_expr
        //                  | non_unary_print_expr
        self.expression();
    }

    fn expression(&mut self) {
        // Parse the lowest precedence level that isn't [Precedence::None], which will subsume all
        // of the higher ones too. If we called this with None, it could consume tokens forever
        // e.g. (1) would fail trying to find an infix operator for ')'
        self.parse_precedence(Precedence::Assignment);
    }

    /// Emits an opcode to read the value of a global variable
    ///
    /// # Arguments
    /// - `can_assign` `true` if a value can be assigned back to a variable, `false` otherwise
    fn variable(&mut self, can_assign: bool) {
        let chunk_index = self.parse_variable();
        if can_assign && self.match_token(&TokenType::Equals) {
            self.expression();
            self.emit_byte(OpCode::SetGlobal(chunk_index))
        } else {
            self.emit_byte(OpCode::GetGlobal(chunk_index));
        }
    }

    /// Emits a number for the [TokenType::Number] token type
    ///
    /// Assumes that a [Token#structfield.token_type] with value of [TokenType::Number] has been
    /// detected and is currently pointed to in the [Parser#structfield.previous_token]
    fn number(&mut self) {
        let raw_lexeme = self
            .previous_token
            .expect("No token was found!")
            .lexeme
            .as_ref()
            .expect("No lexeme for number found!");
        let number: f32 = str::parse(raw_lexeme.as_str())
            .unwrap_or_else(|err| panic!("Unable to convert {} to f32 - {}", raw_lexeme, err));
        self.emit_constant(Value::Number(number));
    }

    /// Emits a string for the [TokenType::DoubleQuote] token type
    ///
    /// Assumes that a [Token#structfield.token_type] with value of [TokenType::DoubleQuote] has
    /// been detected and is currently pointed to in the [Parser#structfield.previous_token]
    fn string(&mut self) {
        self.emit_constant(Value::String(
            self.previous_token
                .unwrap()
                .lexeme
                .as_ref()
                .unwrap()
                .clone(),
        ));
    }

    /// Emits the correct token while parsing a unary expression - e.g. `-42`
    ///
    /// Assumes that a [Token#structfield_token_type] that can be used within a unary expression has
    /// been detected and is currently pointed to in the [Parser#structfield.previous_token]
    fn unary(&mut self) {
        let operator_type = self.previous_token.expect("missing token!").token_type;

        // parse the operand first, then we'll negate it after this method has returned
        // this also allows for nesting unary expressions, like `--2`
        self.parse_precedence(Precedence::Unary);

        if let TokenType::Plus = operator_type {
            self.emit_byte(OpCode::UnaryPlus)
        } else if let TokenType::Minus = operator_type {
            self.emit_byte(OpCode::UnaryMinus)
        } else if let TokenType::Bang = operator_type {
            self.emit_byte(OpCode::LogicalNot)
        }
    }

    /// Function for parsing a binary infix expression.
    fn binary(&mut self) {
        let operator_type = self.previous_token.expect("Missing token!").token_type;

        // Compile the right operand
        let rule = get_rule(operator_type);
        // Note: We _could_ define a function for each of the operators and not have to do the
        // calculation for the next precedence, calling `parse_precedence` with the correct level.
        // This only works because the operators are left-associative:
        // 1 + 2 + 3 + 4 becomes ((1 + 2) + 3) + 4
        // To enable right associativity, we'd call with the same precedence
        if rule.infix_associativity == Associativity::Right {
            self.parse_precedence(rule.infix_precedence);
        } else {
            self.parse_precedence(Precedence::next_precedence(rule.infix_precedence));
        }

        match operator_type {
            TokenType::GreaterEqual => self.emit_byte(OpCode::GreaterEqual),
            TokenType::GreaterThan => self.emit_byte(OpCode::Greater),
            TokenType::LessEqual => self.emit_byte(OpCode::LessEqual),
            TokenType::LessThan => self.emit_byte(OpCode::Less),
            TokenType::DoubleEqual => self.emit_byte(OpCode::DoubleEqual),
            TokenType::NotEqual => self.emit_byte(OpCode::NotEqual),
            TokenType::Plus => self.emit_byte(OpCode::Add),
            TokenType::Minus => self.emit_byte(OpCode::Subtract),
            TokenType::Star => self.emit_byte(OpCode::Multiply),
            TokenType::Slash => self.emit_byte(OpCode::Divide),
            TokenType::Modulus => self.emit_byte(OpCode::Modulus),
            TokenType::Caret => self.emit_byte(OpCode::Exponentiation),
            TokenType::StringConcat => self.emit_byte(OpCode::Concatenate),
            TokenType::And => self.emit_byte(OpCode::LogicalAnd),
            TokenType::Or => self.emit_byte(OpCode::LogicalOr),
            _ => {}
        }
    }

    /// Function for parsing a grouping, denoted by an expression surrounded by parenthesis.
    ///
    /// Assumes that a [Token#structfield.token_type] with a value of [TokenType::LeftParenthesis]
    /// has been detected and is currently pointed at in the [Parser#structfield.previous_token]
    ///
    /// As far as the backend of the language is concerned, there's nothing to this. It only allows
    /// us to use a lower precedence expression in when a higher one is expected. The implication
    /// of this is that it does _not_ emit any code!
    fn grouping(&mut self) {
        self.expression();
        self.consume(&TokenType::RightParenthesis, "Expect ')' token");
    }

    /// Helper function to emit the bytes associated with a constant
    ///
    /// # Arguments
    /// - `value` the constant to emit bytes for
    fn emit_constant(&mut self, value: Value) {
        // TODO: This is a tad different from CI
        self.emit_byte(OpCode::OpConstant(value));
    }

    /// Helper function to emit bytes
    ///
    /// # Arguments
    /// - `op_code` the value to emit the bytes for
    fn emit_byte(&mut self, op_code: OpCode) {
        self.compiling_chunk
            .write_chunk(op_code, self.previous_token.unwrap().line);
    }

    /// Helper function for reporting an error at the current token
    ///
    /// # Arguments
    /// - `message` the message to relay to the user
    fn error_at_current(&mut self, message: &str) {
        // TODO: This clone does not seem like the right thing to do
        self.error_at(&self.current_token.clone(), message)
    }

    /// Helper function for reporting an error at the previous token
    ///
    /// # Arguments
    /// - `message` the message to relay to the user
    fn error_at_previous(&mut self, message: &str) {
        // TODO: This clone does not seem like the right thing to do
        self.error_at(&self.previous_token.clone(), message)
    }

    /// Reports an error found for a provided token
    ///
    /// Information from the provided token will be extracted as a part of the error message, making
    /// it imperative that the line number, token type, etc. are accurately set
    ///
    /// Sets [Parser#structfield.had_error]
    ///
    /// If the compiler is currently in [Parser#structfield.panic_mode], error messages will be
    /// suppressed
    ///
    /// # Arguments
    /// - `token` the token that the error was reported to have occurred on
    /// - `message` the message to relay to the user
    fn error_at(&mut self, token: &Option<&Token>, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;

        let unwrapped_token = token.unwrap();
        let mut error_msg = format!(
            "[{}:{}] Error",
            unwrapped_token.line, unwrapped_token.start_idx
        );
        match unwrapped_token.token_type {
            TokenType::Eof => {
                error_msg.push_str(" at end");
            }
            _ => {
                error_msg.push_str(&format!(
                    " at '{}'", // TODO: This is not exactly graceful on newlines
                    unwrapped_token
                        .lexeme
                        .as_ref()
                        .unwrap_or(&String::from("???"))
                ));
            }
        }

        error_msg.push_str(&format!(". {}", message));
        error!("{}", error_msg.as_str());

        self.had_error = true;
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn);
    }

    fn end_compiler(&mut self) {
        self.compiling_chunk.disassemble_chunk("code");
        self.emit_return();
    }
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
fn get_rule(token_type: &TokenType) -> &ParseRule {
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
        infix_parse_fn: Some(|parser, _can_assign| parser.binary()),
        infix_precedence: Precedence::LogicalOr,
        infix_associativity: Associativity::Left,
    },
    // (Logical) And
    ParseRule {
        prefix_parse_fn: None,
        infix_parse_fn: Some(|parser, _can_assign| parser.binary()),
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
        infix_parse_fn: None,
        infix_precedence: Precedence::None,
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
        prefix_parse_fn: None,
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
