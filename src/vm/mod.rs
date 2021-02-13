use crate::chunk::{Chunk, OpCode};
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::token::Token;
use crate::value::Value;

use log::{debug, error, info};

#[derive(PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            stack: vec![],
        }
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            let old_ip = self.ip;
            self.ip += 1;

            let instruction: OpCode = self.chunk.code[old_ip].code.clone();

            debug!("VM switching on instruction '{:#?}'", &instruction);
            match instruction {
                OpCode::OpReturn => match self.stack.pop() {
                    Some(val) => {
                        info!("{}", val.to_string());
                        return InterpretResult::Ok;
                    }
                    None => {
                        error!("Error: Something went wrong trying to temp return");
                        return InterpretResult::RuntimeError;
                    }
                },
                OpCode::GreaterEqual => self.comparison_op(&instruction),
                OpCode::Greater => self.comparison_op(&instruction),
                OpCode::LessEqual => self.comparison_op(&instruction),
                OpCode::Less => self.comparison_op(&instruction),
                OpCode::DoubleEqual => self.comparison_op(&instruction),
                OpCode::NotEqual => self.comparison_op(&instruction),
                OpCode::Add => self.binary_op(&instruction),
                OpCode::Subtract => self.binary_op(&instruction),
                OpCode::Multiply => self.binary_op(&instruction),
                OpCode::Divide => self.binary_op(&instruction),
                OpCode::Modulus => self.binary_op(&instruction),
                OpCode::Exponentiation => self.binary_op(&instruction),
                OpCode::UnaryPlus => self.unary_op(&instruction),
                OpCode::UnaryMinus => self.unary_op(&instruction),
                OpCode::LogicalNot => self.unary_op(&instruction),
                OpCode::OpConstant(val) => self.stack.push(val),
            }
        }
        InterpretResult::Ok
    }

    pub fn interpret(&mut self, source: String) -> InterpretResult {
        let scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan();

        // TODO: This feels dirty
        self.chunk = Chunk::new();
        let mut parser = Parser::new(tokens.iter(), &mut self.chunk);

        if !parser.parse() {
            return InterpretResult::CompileError;
        }

        self.run()
    }

    fn binary_op(&mut self, op_code: &OpCode) {
        if !matches!(self.peek(0), Value::Number(_)) || !matches!(self.peek(1), Value::Number(_)) {
            eprintln!("Both operands must be numbers.");
            panic!("Both operands must be numbers."); // TODO: Return Runtime Error
        }

        if let Value::Number(b) = self.stack.pop().unwrap() {
            if let Value::Number(a) = self.stack.pop().unwrap() {
                match *op_code {
                    OpCode::Add => self.stack.push(Value::Number(a + b)),
                    OpCode::Subtract => self.stack.push(Value::Number(a - b)),
                    OpCode::Multiply => self.stack.push(Value::Number(a * b)),
                    OpCode::Divide => self.stack.push(Value::Number(a / b)),
                    OpCode::Modulus => self.stack.push(Value::Number(a % b)),
                    OpCode::Exponentiation => self.stack.push(Value::Number(a.powf(b))),
                    _ => panic!("Unknown op code given for binary '{:?}'", op_code),
                }
            }
        }
    }

    /// Perform a relational comparison between two operators on the stack
    ///
    /// When two operands are compared, either string comparison or numeric comparison may be
    /// used. This depends upon the attributes of the operands, according to the following
    /// symmetric matrix:
    ///         +----------------------------------------------
    ///         |       STRING          NUMERIC         STRNUM
    /// --------+----------------------------------------------
    ///         |
    /// STRING  |       string          string          string
    ///         |
    /// NUMERIC |       string          numeric         numeric
    ///         |
    /// STRNUM  |       string          numeric         numeric
    /// --------+----------------------------------------------
    /// [Source - GNU Awk Manual](https://www.gnu.org/software/gawk/manual/html_node/Variable-Typing.html)
    fn comparison_op(&mut self, op_code: &OpCode) {
        let is_string_comparison =
            matches!(self.peek(0), Value::String(_)) || matches!(self.peek(1), Value::String(_));

        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();

        if is_string_comparison {
            self.string_comparison(op_code, &*a.str_value(), &*b.str_value());
        } else {
            self.numeric_comparison(op_code, a.num_value(), b.num_value());
        }
    }

    /// Perform a relational comparison between two strings, and push the result onto the stack
    ///
    /// # Arguments
    /// - `op_code` the operation to perform
    /// - `a` the first argument of the comparison. Is placed on the left hand side of the expression
    /// - `b` the second argument of the comparison. Is placed on the right hand side of the expression
    fn string_comparison(&mut self, op_code: &OpCode, a: &str, b: &str) {
        match *op_code {
            OpCode::GreaterEqual => {
                let mut result: f32 = 0.0;
                if a >= b {
                    result = 1.0;
                }
                self.stack.push(Value::Number(result))
            }
            OpCode::Greater => {
                let mut result: f32 = 0.0;
                if a > b {
                    result = 1.0;
                }
                self.stack.push(Value::Number(result))
            }
            OpCode::LessEqual => {
                let mut result: f32 = 0.0;
                if a <= b {
                    result = 1.0;
                }
                self.stack.push(Value::Number(result))
            }
            OpCode::Less => {
                let mut result: f32 = 0.0;
                if a < b {
                    result = 1.0;
                }
                self.stack.push(Value::Number(result))
            }
            OpCode::DoubleEqual => {
                let mut result: f32 = 0.0;
                if a.eq(b) {
                    result = 1.0;
                }
                self.stack.push(Value::Number(result))
            }
            OpCode::NotEqual => {
                let mut result: f32 = 0.0;
                if a.ne(b) {
                    result = 1.0;
                }
                self.stack.push(Value::Number(result))
            }
            _ => panic!(
                "Unknown op code given for string comparison '{:?}'",
                op_code
            ),
        }
    }

    /// Perform a relational comparison between two numbers, and push the result onto the stack
    ///
    /// # Arguments
    /// - `op_code` the operation to perform
    /// - `a` the first argument of the comparison. Is placed on the left hand side of the expression
    /// - `b` the second argument of the comparison. Is placed on the right hand side of the expression
    fn numeric_comparison(&mut self, op_code: &OpCode, a: f32, b: f32) {
        match *op_code {
            OpCode::GreaterEqual => {
                let mut result: f32 = 0.0;
                if a >= b {
                    result = 1.0;
                }
                self.stack.push(Value::Number(result))
            }
            OpCode::Greater => {
                let mut result: f32 = 0.0;
                if a > b {
                    result = 1.0;
                }
                self.stack.push(Value::Number(result))
            }
            OpCode::LessEqual => {
                let mut result: f32 = 0.0;
                if a <= b {
                    result = 1.0;
                }
                self.stack.push(Value::Number(result))
            }
            OpCode::Less => {
                let mut result: f32 = 0.0;
                if a < b {
                    result = 1.0;
                }
                self.stack.push(Value::Number(result))
            }
            OpCode::DoubleEqual => {
                let mut result: f32 = 0.0;
                if (a - b).abs() == 0.0 {
                    result = 1.0;
                }
                self.stack.push(Value::Number(result))
            }
            OpCode::NotEqual => {
                let mut result: f32 = 0.0;
                if (a - b).abs() != 0.0 {
                    result = 1.0;
                }
                self.stack.push(Value::Number(result))
            }
            _ => panic!("Unknown op code given for comparison '{:?}'", op_code),
        }
    }

    fn unary_op(&mut self, op_code: &OpCode) {
        if matches!(self.peek(0), Value::Number(_)) {
            if let Value::Number(a) = self.stack.pop().unwrap() {
                match *op_code {
                    // Unary plus will be more useful for converting a string to a number
                    OpCode::UnaryPlus => {
                        if a == 0.0 {
                            self.stack.push(Value::Number(0.0))
                        } else {
                            self.stack.push(Value::Number(a))
                        }
                    }
                    OpCode::UnaryMinus => {
                        if a == 0.0 {
                            self.stack.push(Value::Number(0.0))
                        } else {
                            self.stack.push(Value::Number(-a))
                        }
                    }
                    OpCode::LogicalNot => {
                        let mut result: f32 = 1.0;
                        if a > 0.0 {
                            result = 0.0;
                        }
                        self.stack.push(Value::Number(result))
                    }
                    _ => panic!("Unknown op code given for unary '{:?}'", op_code),
                }
            }
        } else if matches!(self.peek(0), Value::String(_)) {
            if let Value::String(a) = self.stack.pop().unwrap() {
                match *op_code {
                    OpCode::LogicalNot => {
                        let mut result: f32 = 1.0;
                        if !a.is_empty() {
                            result = 0.0;
                        }
                        self.stack.push(Value::Number(result));
                    }
                    _ => panic!("Unknown op code given for unary '{:?}'", op_code),
                }
            }
        } else {
            let err_msg = "Unary operand must be a number or string.";
            eprintln!("{}", err_msg);
            panic!("{}", err_msg);
        }
    }

    fn peek(&mut self, distance: usize) -> &Value {
        let last_index = self.stack.len() - 1;
        return self
            .stack
            .get(last_index - distance)
            .expect("Unable to peek at stack!");
    }
}
