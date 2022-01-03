//! Parser for our implementation of awk.
//!
//! The parser is implemented as a Pratt Parser, and is heavily modeled after the one given in
//! "Crafting Interpreters" in the second half of the book.

mod parse_rules;
mod precedence;

use crate::chunk::{Chunk, OpCode};
use crate::parser::parse_rules::Associativity;
use crate::parser::precedence::Precedence;
use crate::token::token_type::TokenType;
use crate::token::Token;
use crate::value::Value;
use log::error;
use std::slice::Iter;

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
    inner_most_loop_start: i32,
    inner_most_loop_end: i32,
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
            inner_most_loop_start: -1,
            inner_most_loop_end: -1,
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
            self.parse_pattern_action();
        }

        self.end_compiler();

        !self.had_error
    }

    /// Parse a single pattern-action
    fn parse_pattern_action(&mut self) {
        self.parse_pattern();
        self.parse_action();
    }

    /// Parse a single pattern
    fn parse_pattern(&mut self) {
        if self.match_token(&TokenType::Begin) {
            // BEGIN has been parsed, now handle the block
            self.emit_true();
            panic!("TODO: Implement BEGIN support");
        } else if self.match_token(&TokenType::End) {
            // END has been parsed, now handle the block
            self.emit_true();
            panic!("TODO: Implement END support");
        } else if self.peek_token(&TokenType::LeftCurly) {
            // we've run into an action earlier than we thought, emit true so we always run the action that will follow
            self.emit_true();
        } else {
            // we have a pattern to parse
            self.expression();
            // TODO: Support multiple patterns - https://www.gnu.org/software/gawk/manual/html_node/Ranges.html
        }
    }

    /// Emits a number one ('1') for the [TokenType::Number] token type
    fn emit_true(&mut self) {
        self.emit_constant(Value::Number(1.0));
    }

    /// Parse a single action
    fn parse_action(&mut self) {
        // emit a jump instruction as a placeholder to skip over the 'action' associated with the action in the
        // event the pattern condition is false (if one exists). we'll backpatch it soon with the correct offset.
        let pattern_false_jump = self.emit_jump(OpCode::JumpIfFalse(0xff, 0xff));

        if self.match_token(&TokenType::LeftCurly) {
            // parse the contents of the action
            self.block();
        } else {
            // no pattern - the action implicitly becomes 'print $0'
            self.emit_constant(Value::Number(0.0));
            self.emit_byte(OpCode::GetFieldVariable());
            self.emit_byte(OpCode::OpPrint);
            self.emit_byte(OpCode::Pop);
        }

        // we've passed through the action successfully, backpatch the jump that was emitted for the block
        self.patch_jump(pattern_false_jump);

        // always pop the pattern's result off the stack. this differs from handling an if statement, which has a
        // then clause that requires conditional popping
        self.emit_byte(OpCode::Pop);
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
            parse_rules::get_rule(self.previous_token.expect("missing token").token_type)
                .prefix_parse_fn;
        if maybe_prefix_rule.is_none() {
            self.error_at_previous("Expect expression.");
            return;
        }

        let can_assign: bool = precedence <= Precedence::LogicalAnd;
        let prefix_rule = maybe_prefix_rule.unwrap();
        prefix_rule(self, can_assign);

        while precedence
            <= parse_rules::get_rule(self.current_token.expect("Missing token!").token_type)
                .infix_precedence
        {
            self.advance();
            let infix_rule =
                parse_rules::get_rule(self.previous_token.expect("No Token was found!").token_type)
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
        if self.current_token.is_some() && self.peek_token(&TokenType::Eof) {
            return;
        }

        loop {
            self.current_token = self.tokens_iter.next();

            if !self.peek_token(&TokenType::Error) {
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
    /// # Arguments
    /// - `token_type` the token type to match
    ///
    /// # Return value
    /// - `true` if the current token type matches
    /// - `false` otherwise
    fn peek_token(&mut self, token_type: &TokenType) -> bool {
        token_type == self.current_token.unwrap().token_type
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
        if self.peek_token(token_type) {
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

    /// Parse a field variable reference
    fn field_variable(&mut self) {
        // a field variable is one that is prefixed with a dollar sign/sigil ('$')
        // because the value immediately following the '$' may be the result of an expression,
        // e.g. $(2+3), we must parse the right hand side first and push it onto the stack
        self.parse_precedence(Precedence::FieldVariable);
        // now that the result of the expression is on the stack, push the '$'
        self.emit_byte(OpCode::GetFieldVariable());
    }

    /// Parse a variable production
    fn variable_declaration(&mut self) {
        // store the variable name so it can be looked up later
        let global_variable_index = self.parse_variable();

        if self.match_token(&TokenType::Equals) {
            self.expression();
        } else if self.op_assign_match() {
            self.op_assign(global_variable_index);
        } else {
            self.emit_constant(Value::String("".into()));
        }
        self.consume(
            &TokenType::Semicolon,
            "Expect ';' after a variable declaration.",
        );

        self.define_variable(global_variable_index);
    }

    /// Evaluates whether or not the current token is an operator assignment
    ///
    /// # Return value
    /// true if the current token is an operator assignment, false otherwise
    fn op_assign_match(&mut self) -> bool {
        self.match_token(&TokenType::AddAssign)
            || self.match_token(&TokenType::SubAssign)
            || self.match_token(&TokenType::MulAssign)
            || self.match_token(&TokenType::DivAssign)
            || self.match_token(&TokenType::ModAssign)
            || self.match_token(&TokenType::PowAssign)
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
        } else if self.match_token(&TokenType::If) {
            self.if_statement();
        } else if self.match_token(&TokenType::While) {
            self.while_statement();
        } else if self.match_token(&TokenType::For) {
            self.for_statement();
        } else if self.match_token(&TokenType::Continue) {
            self.continue_statement();
        } else if self.match_token(&TokenType::Break) {
            self.break_statement();
        } else if self.match_token(&TokenType::LeftCurly) {
            self.block();
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

    /// Function for parsing a block of code contained by curly braces
    fn block(&mut self) {
        while !self.peek_token(&TokenType::RightCurly) && !self.peek_token(&TokenType::Eof) {
            self.declaration();
        }
        self.consume(&TokenType::RightCurly, "Expect '}' after block.");
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

    /// Parse an if statement
    fn if_statement(&mut self) {
        self.consume(&TokenType::LeftParenthesis, "Expect '(' after IF.");
        self.expression();
        self.consume(
            &TokenType::RightParenthesis,
            "Expect ')' at the end of IF statement.",
        );

        // emit a jump instruction as a placeholder to skip over the 'then' in the event the if condition is false.
        // we'll backpatch it soon with the correct offset.
        let if_was_false_jump = self.emit_jump(OpCode::JumpIfFalse(0xff, 0xff));
        self.statement();

        // when the if statement condition is truthy, pop the result off the stack
        self.emit_byte(OpCode::Pop);

        // in the event the if statement's condition is truthy, we need to jump over the else block rather than fall
        // through. put a placeholder in that will be able to skip the else keyword & the statement(s) that follow it.
        // there is an implicit 'else' here, even if there isn't one in the author's code.
        let else_jump = self.emit_jump(OpCode::Jump(0xff, 0xff));

        // we've passed through the then statement(s) backpatch the jump that was emitted for the if block. we needed
        // to emit the pop instruction for the condition being truthy to ensure we calculated the distance for
        // backpatching correctly.
        self.patch_jump(if_was_false_jump);

        // when the if statement condition is falsy, pop the result off the stack.
        self.emit_byte(OpCode::Pop);

        if self.match_token(&TokenType::Else) {
            self.statement();
        }

        // we've passed through the else block statement(s), backpatch the jump that was emitted for the else block
        self.patch_jump(else_jump);
    }

    /// Parsing method for while statements
    fn while_statement(&mut self) {
        // mark the location where the loop begins
        let while_start = self.compiling_chunk.code.len();

        // Store a reference to the active loop start for this call frame. The value on `self` will be mutated when the
        // body of the while statement is parsed
        let surrounding_loop_start = self.inner_most_loop_start;

        // Store where the loop starts should we run into a `continue` or `break` statement
        self.inner_most_loop_start = self.compiling_chunk.code.len() as i32;

        // Store a reference to the last loop's end place for this call frame. The value on `self` will be mutated when
        // the body of the while statement is parsed
        let surrounding_loop_end = self.inner_most_loop_end;

        self.consume(&TokenType::LeftParenthesis, "Expect '(' after 'while'.");
        self.expression();
        self.consume(
            &TokenType::RightParenthesis,
            "Expect ')' after 'while' condition.",
        );

        // emit a jump that will jump over the body of the loop should it's condition be false
        let while_condition_false = self.emit_jump(OpCode::JumpIfFalse(0xFF, 0xFF));

        // pop the result of the while condition off the stack if the condition was truthy
        self.emit_byte(OpCode::Pop);

        self.statement();
        // now that the body of the while loop has been parsed, emit a jump back to the start of the loop
        self.emit_loop(while_start);

        // in the event that we see a break statement during the course of the parsing, we need to patch it now that
        // we're at the end of the loop
        if self.inner_most_loop_end != -1 {
            self.patch_jump(self.inner_most_loop_end as usize);
        }
        self.inner_most_loop_end = surrounding_loop_end;

        // backpatch the jump for a falsy condition and pop the result off the stack
        self.patch_jump(while_condition_false);
        self.emit_byte(OpCode::Pop);

        // Restore the references to the active loops start after parsing the body of the while
        self.inner_most_loop_start = surrounding_loop_start;
    }

    /// Function for parsing a for loop
    fn for_statement(&mut self) {
        self.consume(&TokenType::LeftParenthesis, "Expect '(' after for.");
        // TODO Support `for (var in array) {}` when we get to arrays, that's why this for loop is funky ATM
        if self.match_token(&TokenType::Semicolon) {
            // assume there is no variable initialization occurring
        } else {
            self.expression();
            self.consume(&TokenType::Semicolon, "Expect ';'.");
            self.emit_byte(OpCode::Pop);
        }

        // Store a reference to the active loop start for this call frame. The value on `self` will be mutated when the
        // body of the for statement is parsed
        let surrounding_loop_start = self.inner_most_loop_start;

        // Store a reference to the last loop's end place for this call frame. The value on `self` will be mutated when
        // the body of the for statement is parsed
        let surrounding_loop_end = self.inner_most_loop_end;

        let mut loop_start = self.compiling_chunk.code.len();
        let mut for_loop_exit_jump: Option<usize> = None;

        if !self.match_token(&TokenType::Semicolon) {
            self.expression();
            self.consume(&TokenType::Semicolon, "Expect ';' after loop condition.");

            // if the condition is false, we need to jump out of the loop
            for_loop_exit_jump = Some(self.emit_jump(OpCode::JumpIfFalse(0xFF, 0xFF)));
            // if the condition is true, we need to pop the result off of the stack
            self.emit_byte(OpCode::Pop);
        }

        if !self.match_token(&TokenType::RightParenthesis) {
            // unconditionally jump over the incrementer, to the body of the loop
            let body_jump = self.emit_jump(OpCode::Jump(0xFF, 0xFF));
            let increment_clause_start = self.compiling_chunk.code.len();

            // compile the incrementer, then throw away the result since it's often assignment
            self.expression();
            self.emit_jump(OpCode::Pop);

            self.consume(
                &TokenType::RightParenthesis,
                "Expect ')' after for clauses.",
            );

            // this happens right after an increment, since an increment happens at the end of a loop (a little
            // weird, I know)
            // 1. take us back to the top of the for loop, right before the condition (which may not exist). this occurs
            // _after_ the increment
            self.emit_loop(loop_start);
            // 2. update the loop start to point to the increment clause
            loop_start = increment_clause_start;
            // 3. back patch the jump for the entire body
            self.patch_jump(body_jump);
        }

        // Store where the loop starts should we run into a `continue` or `break` statement
        self.inner_most_loop_start = loop_start as i32;

        self.statement();

        self.emit_loop(loop_start);

        // in the event that we see a break statement during the course of the parsing, we need to patch it now that
        // we're at the end of the loop
        if self.inner_most_loop_end != -1 {
            self.patch_jump(self.inner_most_loop_end as usize);
            self.emit_byte(OpCode::Pop);
        }
        self.inner_most_loop_end = surrounding_loop_end;

        // patch the jump if the condition is false
        if let Some(value) = for_loop_exit_jump {
            self.patch_jump(value);
            // if the condition is false, we still have that value on the stack
            self.emit_byte(OpCode::Pop);
        }

        // Restore the references to the active loops start after parsing the body of the while
        self.inner_most_loop_start = surrounding_loop_start;
    }

    /// Function for parsing the continue token
    fn continue_statement(&mut self) {
        if self.inner_most_loop_start <= -1 {
            self.error_at_previous("Can't use 'continue' outside of a loop.");
        }

        self.consume(&TokenType::Semicolon, "Expect ';' after continue");

        // casting is safer here, as we've ensure that the inner_most_loop_start >= 0 above
        self.emit_loop(self.inner_most_loop_start as usize);
    }

    /// Function for parsing the break token
    fn break_statement(&mut self) {
        if self.inner_most_loop_start <= -1 {
            self.error_at_previous("Can't use 'break' outside of a loop.");
        }

        self.consume(&TokenType::Semicolon, "Expect ';' after break");

        self.inner_most_loop_end = self.emit_jump(OpCode::Jump(0xFF, 0xFF)) as i32;
    }

    /// Emits a looping instruction to go backwards in the code
    ///
    /// # Arguments
    /// - `loop_start` the pointer to the the instruction where the loop began
    fn emit_loop(&mut self, loop_start: usize) {
        let offset = self.compiling_chunk.code.len() - loop_start + 1;
        let offset1 = (offset >> 8) & 0xff;
        let offset2 = offset & 0xff;
        self.emit_byte(OpCode::Loop(offset1, offset2));
    }

    /// Emit a jump instruction
    ///
    /// # Arguments
    /// - `instruction` the [OpCode] to emit
    ///
    /// # Return value
    /// the location of the emitted jump instruction in the current chunk
    fn emit_jump(&mut self, instruction: OpCode) -> usize {
        self.emit_byte(instruction);

        // return the offset, that is, where the instruction we'll later overwrites begins
        self.compiling_chunk.code.len()
    }

    /// Patch a jump instruction that was previously emitted
    ///
    /// # Arguments
    /// - `offset` the location of the jump instruction that was emitted
    fn patch_jump(&mut self, offset: usize) {
        // pull the placeholder instruction out to be patched
        let old_instruction = &self.compiling_chunk.code[offset - 1].code;

        // calculate the jump distance
        let jump = self.compiling_chunk.code.len() - offset;
        let new_offset1 = (jump >> 8) & 0xff;
        let new_offset2 = jump & 0xff;

        // create a patch instruction, using the old one to avoid messiness in moving values
        let patch_instruction = match &old_instruction {
            OpCode::JumpIfFalse(_, _) => OpCode::JumpIfFalse(new_offset1, new_offset2),
            OpCode::JumpIfTrue(_, _) => OpCode::JumpIfTrue(new_offset1, new_offset2),
            OpCode::Jump(_, _) => OpCode::Jump(new_offset1, new_offset2),
            _ => panic!(
                "Instruction {:?} cannot be used to patch a jump!",
                old_instruction
            ),
        };

        self.compiling_chunk.code[offset - 1].code = patch_instruction;
    }

    fn print_expr_list_opt(&mut self) {
        // TODO: Support empty
        self.print_expr_list();
    }

    fn print_expr_list(&mut self) {
        self.print_expr();

        while self.match_token(&TokenType::Comma) {
            // for every comma operator, concatenate the expr that follows the comma with a space
            // and the expr result
            self.emit_constant(Value::String(String::from(" ")));
            self.emit_byte(OpCode::Concatenate);
            self.expression();
            self.emit_byte(OpCode::Concatenate);
        }
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
            self.emit_byte(OpCode::SetGlobal(chunk_index));
        } else if can_assign && self.op_assign_match() {
            self.op_assign(chunk_index);
            self.emit_byte(OpCode::SetGlobal(chunk_index));
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

    /// Function for parsing operator assignment
    ///
    /// # Arguments
    /// - `global_variable_index` an index into the current compiling chunk's constants table
    fn op_assign(&mut self, global_variable_index: usize) {
        let operator_type = self.previous_token.expect("Missing token!").token_type;
        match operator_type {
            TokenType::AddAssign => {
                self.parse_precedence(Precedence::Assignment);
                self.emit_byte(OpCode::GetGlobal(global_variable_index));
                self.emit_byte(OpCode::Add);
            }
            TokenType::SubAssign => {
                self.emit_byte(OpCode::GetGlobal(global_variable_index));
                self.parse_precedence(Precedence::Assignment);
                self.emit_byte(OpCode::Subtract);
            }
            TokenType::MulAssign => {
                self.parse_precedence(Precedence::Assignment);
                self.emit_byte(OpCode::GetGlobal(global_variable_index));
                self.emit_byte(OpCode::Multiply);
            }
            TokenType::DivAssign => {
                self.emit_byte(OpCode::GetGlobal(global_variable_index));
                self.parse_precedence(Precedence::Assignment);
                self.emit_byte(OpCode::Divide);
            }
            TokenType::ModAssign => {
                self.emit_byte(OpCode::GetGlobal(global_variable_index));
                self.parse_precedence(Precedence::Assignment);
                self.emit_byte(OpCode::Modulus);
            }
            TokenType::PowAssign => {
                self.parse_precedence(Precedence::Assignment);
                self.emit_byte(OpCode::GetGlobal(global_variable_index));
                self.emit_byte(OpCode::Exponentiation);
            }
            _ => {}
        }
    }

    /// Function for parsing a binary infix expression.
    fn binary(&mut self) {
        let operator_type = self.previous_token.expect("Missing token!").token_type;

        // Compile the right operand
        let rule = parse_rules::get_rule(operator_type);
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
            _ => {}
        }
    }

    /// Parse a condition expression
    ///
    /// A condition expression takes the form `expr1 ? expr2 : expr3` where:
    /// - if `expr1` is truthy, `expr2` is evaluated and `expr3` is not evaluated
    /// - if `expr1` is falsy, `expr2` is not evaluated and `expr3` is evaluated
    fn conditional_expression(&mut self) {
        // expr1 has already been evaluated and is at the top of the stack.
        // we need to jump over expr2 if that value on the stack if falsy
        let over_expr_2_jump = self.emit_jump(OpCode::JumpIfFalse(0xFF, 0xFF));

        // if the value from expr1 is truthy, pop it off the stack now. we cannot wait to pop this value,
        // as that may lead to popping the result from expr2 being popped off the stack
        self.emit_byte(OpCode::Pop);

        // parse expr2
        self.expression();

        // we need to handle the colon (:) in this block
        self.consume(&TokenType::Colon, "Expect ':' after expression in ternary.");

        // if expr1 (which is on the stack) is truthy, we'll run through expr2. now we need to emit a jump over expr3
        // to ensure we don't evaluate that as well
        let over_expr_3_jump = self.emit_jump(OpCode::Jump(0xFF, 0xFF));

        // by now, both expr2 and the colon in the ternary have been parsed, so we are exactly where we need to be to
        // backpatch in the event expr1 was falsy
        self.patch_jump(over_expr_2_jump);

        // if the value from expr1 is falsy, pop it off the stack now. we cannot wait to pop this value,
        // as that may lead to popping the result from expr3 being popped off the stack
        self.emit_byte(OpCode::Pop);

        // parse expr3
        self.expression();

        // now that expr3 has been parsed, we can backpatch the jump we'd have taken if expr1 was truthy
        self.patch_jump(over_expr_3_jump);
    }

    /// Function for parsing logical or (||) to support short circuiting.
    ///
    /// When this function is reached, the left hand side of the expression should have already been parsed and its
    /// contents on the top of the stack. If that value is truthy, the entire statement is truthy.
    fn logical_or(&mut self) {
        // create a placeholder to jump to the end of the or statement should it's condition be true
        let end_jump = self.emit_jump(OpCode::JumpIfTrue(0xFF, 0xFF));

        // continue parsing the right hand side of the 'or'
        self.parse_precedence(Precedence::LogicalOr);

        // the LHS and RHS are now on the stack, we'll need to evaluate them in the VM, since awk's return value for
        // logical or is 1 or 0, not the return value of the last sub expression
        self.emit_byte(OpCode::LogicalOr);

        // we've now passed the or statement, backpatch the jump over the contents
        self.patch_jump(end_jump);
    }

    /// Function for parsing logical and (&&) to support short circuiting.
    ///
    /// When this function is reached, the left hand side of the expression should have already been parsed and its
    /// contents on the top of the stack. If that value is falsy, the entire statement is falsy.
    fn logical_and(&mut self) {
        // create a placeholder to jump to the end of the and statement should it's condition be false
        let end_jump = self.emit_jump(OpCode::JumpIfFalse(0xFF, 0xFF));

        // continue parsing the right hand side of the 'and'
        self.parse_precedence(Precedence::LogicalAnd);

        // the LHS and RHS are now on the stack, we'll need to evaluate them in the VM, since awk's return value for
        // logical and is 1 or 0, not the return value of the last sub expression
        self.emit_byte(OpCode::LogicalAnd);

        // we've now passed the and statement, backpatch the jump over the contents
        self.patch_jump(end_jump);
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
        self.emit_byte(OpCode::OpConstant(value));
    }

    /// Helper function to emit bytes
    ///
    /// # Arguments
    /// - `op_code` the value to emit the bytes for
    fn emit_byte(&mut self, op_code: OpCode) {
        let line_number = self
            .previous_token
            .unwrap_or_else(|| self.current_token.unwrap())
            .line;
        self.compiling_chunk.write_chunk(op_code, line_number);
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
        let mut error_msg = format!("[line {}] Error", unwrapped_token.line);
        match unwrapped_token.token_type {
            TokenType::Eof => {
                error_msg.push_str(" at end");
            }
            _ => {
                error_msg.push_str(&format!(
                    " at '{}'", // TODO: This is not exactly graceful on newlines
                    unwrapped_token.lexeme.as_ref().unwrap_or(&String::from(
                        "TODO: This is a shortsighted part of the lexeme"
                    ))
                ));
            }
        }

        error_msg.push_str(&format!(": {}", message));
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
