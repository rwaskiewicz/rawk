use crate::chunk::{Chunk, OpCode};
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::Token;
use crate::value::Value;

use log::{debug, error, info};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum InterpretError {
    CompileError,
    RuntimeError,
}

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            stack: vec![],
            globals: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), InterpretError> {
        let mut ret_ok = false;
        loop {
            if ret_ok {
                return Ok(());
            }

            let old_ip = self.ip;
            self.ip += 1;

            let instruction: OpCode = self.chunk.code[old_ip].code.clone();

            debug!("VM switching on instruction '{:#?}'", &instruction);
            match instruction {
                OpCode::OpPrint => match self.stack.pop() {
                    Some(val) => {
                        info!("{}", val.to_string());
                    }
                    None => {
                        error!("Error: The stack was empty when trying to print");
                        panic!("{:?}", InterpretError::RuntimeError);
                    }
                },
                OpCode::OpReturn => ret_ok = true,
                OpCode::GreaterEqual => self.comparison_op(&instruction),
                OpCode::Greater => self.comparison_op(&instruction),
                OpCode::LessEqual => self.comparison_op(&instruction),
                OpCode::Less => self.comparison_op(&instruction),
                OpCode::DoubleEqual => self.comparison_op(&instruction),
                OpCode::NotEqual => self.comparison_op(&instruction),
                OpCode::Add => self.arithmetic_op(&instruction),
                OpCode::Subtract => self.arithmetic_op(&instruction),
                OpCode::Multiply => self.arithmetic_op(&instruction),
                OpCode::Divide => self.arithmetic_op(&instruction),
                OpCode::Modulus => self.arithmetic_op(&instruction),
                OpCode::Exponentiation => self.arithmetic_op(&instruction),
                OpCode::Concatenate => self.concatenation_op(&instruction),
                OpCode::UnaryPlus => self.unary_op(&instruction),
                OpCode::UnaryMinus => self.unary_op(&instruction),
                OpCode::LogicalNot => self.unary_op(&instruction),
                OpCode::LogicalAnd => self.logical_op(&instruction),
                OpCode::LogicalOr => self.logical_op(&instruction),
                OpCode::OpConstant(val) => self.stack.push(val),
                OpCode::Pop => {
                    self.stack.pop();
                }
                OpCode::GetGlobal(chunk_index) => {
                    let variable_name = self.read_variable_name(chunk_index);
                    let default_val = &Value::String("".into());
                    let val = self.globals.get(&*variable_name).unwrap_or(default_val);
                    self.stack.push(val.clone());
                }
                OpCode::SetGlobal(chunk_index) => {
                    let variable_name = self.read_variable_name(chunk_index);
                    let val = self.peek(0).clone();
                    self.globals.insert(variable_name, val);
                }
                OpCode::DefineGlobal(chunk_index) => {
                    let variable_name = self.read_variable_name(chunk_index);
                    let val = self.peek(0).clone();
                    self.globals.insert(variable_name, val);
                    self.stack.pop();
                }
                OpCode::JumpIfFalse(offset1, offset2) => {
                    let if_result = self.peek(0).num_value() != 0.0;
                    if !if_result {
                        let offset = (offset1 >> 8) | offset2;
                        self.ip += offset;
                    }
                }
                OpCode::JumpIfTrue(offset1, offset2) => {
                    let if_result = self.peek(0).num_value() != 0.0;
                    if if_result {
                        // the top of the stack may not yield a one, force it
                        self.stack.pop();
                        self.stack.push(Value::Number(1.0));

                        let offset = (offset1 >> 8) | offset2;
                        self.ip += offset;
                    }
                }
                OpCode::Jump(offset1, offset2) => {
                    let offset = (offset1 >> 8) | offset2;
                    self.ip += offset;
                }
            }
        }
    }

    fn read_variable_name(&mut self, chunk_index: usize) -> String {
        self.chunk
            .constants
            .get(chunk_index)
            .expect("No variable found")
            .clone()
    }

    pub fn interpret(&mut self, source: String) -> Result<(), InterpretError> {
        let scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan();

        self.chunk = Chunk::new();
        self.ip = 0;
        let mut parser = Parser::new(tokens.iter(), &mut self.chunk);

        if !parser.parse() {
            return Err(InterpretError::CompileError);
        }

        self.run()
    }

    /// Perform an arithmetic operation on two values on the stack, placing the result on the stack
    ///
    /// The values on the stack shall be implicitly converted into their string representations for
    /// the operations supported by this method immediately following popping them from the stack
    ///
    /// # Arguments
    /// - `op_code` the operation to perform
    fn arithmetic_op(&mut self, op_code: &OpCode) {
        let b = self.stack.pop().unwrap().num_value();
        let a = self.stack.pop().unwrap().num_value();

        match *op_code {
            OpCode::Add => self.stack.push(Value::Number(a + b)),
            OpCode::Subtract => self.stack.push(Value::Number(a - b)),
            OpCode::Multiply => self.stack.push(Value::Number(a * b)),
            OpCode::Divide => self.stack.push(Value::Number(a / b)),
            OpCode::Modulus => self.stack.push(Value::Number(a % b)),
            OpCode::Exponentiation => self.stack.push(Value::Number(a.powf(b))),
            _ => panic!(
                "Unknown op code given for arithmetic operation '{:?}'",
                op_code
            ),
        }
    }

    /// Perform an a concatenation operation on two values on the stack, placing the result on the stack
    ///
    /// The values on the stack shall be implicitly converted into their string representations for
    /// the operations supported by this method immediately following popping them from the stack
    ///
    /// # Arguments
    /// - `op_code` the operation to perform
    fn concatenation_op(&mut self, op_code: &OpCode) {
        let b = self.stack.pop().unwrap().str_value();
        let a = self.stack.pop().unwrap().str_value();

        match *op_code {
            OpCode::Concatenate => {
                let mut concat_str = a;
                concat_str.push_str(&b);
                self.stack.push(Value::String(concat_str));
            }
            _ => panic!(
                "Unknown op code given for concatenation operation '{:?}'",
                op_code
            ),
        }
    }

    /// Perform logical comparison between two values on the stack
    ///
    /// # Arguments
    /// - `op_code` the operation to perform
    fn logical_op(&mut self, op_code: &OpCode) {
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();

        match *op_code {
            OpCode::LogicalAnd => {
                // TODO: Look into lazy evaluation further
                let result = a.truthy_value() && b.truthy_value();
                self.stack.push(Value::Number(result as i32 as f32));
            }
            OpCode::LogicalOr => {
                if a.truthy_value() {
                    self.stack.push(Value::Number(1.0));
                    return;
                }
                self.stack
                    .push(Value::Number(b.truthy_value() as i32 as f32));
            }
            _ => panic!(
                "Unknown op code given for logical operation '{:?}'",
                op_code
            ),
        }
    }

    /// Perform a relational comparison between two values on the stack
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
    ///
    /// # Arguments
    /// - `op_code` the operation to perform
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
