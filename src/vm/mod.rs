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
            debug!("{:#?}", &instruction);
            match instruction {
                OpCode::OpReturn => match self.stack.pop() {
                    Some(val) => {
                        info!("{:#?}", val);
                        return InterpretResult::Ok;
                    }
                    None => {
                        error!("Error: Something went wrong trying to temp return");
                        return InterpretResult::RuntimeError;
                    }
                },
                OpCode::GreaterEqual => self.binary_op(&instruction),
                OpCode::Greater => self.binary_op(&instruction),
                OpCode::LessEqual => self.binary_op(&instruction),
                OpCode::Less => self.binary_op(&instruction),
                OpCode::DoubleEqual => self.binary_op(&instruction),
                OpCode::NotEqual => self.binary_op(&instruction),
                OpCode::Add => self.binary_op(&instruction),
                OpCode::Subtract => self.binary_op(&instruction),
                OpCode::Multiply => self.binary_op(&instruction),
                OpCode::Divide => self.binary_op(&instruction),
                OpCode::Modulus => self.binary_op(&instruction),
                OpCode::Exponentiation => self.binary_op(&instruction),
                OpCode::Negate => self.unary_op(),
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

    fn unary_op(&mut self) {
        if !matches!(self.peek(0), Value::Number(_)) {
            eprintln!("Unary operand must be a number.");
            panic!("Unary operand must be a number.");
        }
        if let Value::Number(a) = self.stack.pop().unwrap() {
            self.stack.push(Value::Number(-a));
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
