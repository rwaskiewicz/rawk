use crate::chunk::{Chunk, OpCode};
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::token::Token;
use crate::value::{As, Value, ValueType};

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
            dbg!("{}", instruction);
            match instruction {
                OpCode::OpReturn => match self.stack.pop() {
                    Some(val) => {
                        println!("{:#?}", val);
                        return InterpretResult::Ok;
                    }
                    None => {
                        eprintln!("Error: Something went wrong trying to temp return");
                        return InterpretResult::RuntimeError;
                    }
                },
                OpCode::Add => self.binary_op(&instruction),
                OpCode::Subtract => self.binary_op(&instruction),
                OpCode::Multiply => self.binary_op(&instruction),
                OpCode::Divide => self.binary_op(&instruction),
                OpCode::Negate => self.unary_op(),
                OpCode::OpConstant(val) => self.stack.push(val),
                _ => (),
            }
        }
        return InterpretResult::Ok;
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

        return self.run();
    }

    fn binary_op(&mut self, op_code: &OpCode) -> () {
        if self.peek(0).value_type != ValueType::Number
            || self.peek(1).value_type != ValueType::Number
        {
            eprintln!("Both operands must be numbers.");
            panic!("Both operands must be numbers."); // TODO: Return Runtime Error
        }
        unsafe {
            let b: f32 = self
                .stack
                .pop()
                .expect("This should be a number!")
                .type_as
                .number;
            let a: f32 = self
                .stack
                .pop()
                .expect("This should be a number!")
                .type_as
                .number;
            match op_code {
                &OpCode::Add => self
                    .stack
                    .push(Value::new(ValueType::Number, As { number: a + b })),
                &OpCode::Subtract => self
                    .stack
                    .push(Value::new(ValueType::Number, As { number: a - b })),
                &OpCode::Multiply => self
                    .stack
                    .push(Value::new(ValueType::Number, As { number: a * b })),
                &OpCode::Divide => self
                    .stack
                    .push(Value::new(ValueType::Number, As { number: a / b })),
                _ => panic!("Unknown op code given for binary {:?}", op_code),
            }
        }
    }

    fn unary_op(&mut self) -> () {
        if self.peek(0).value_type != ValueType::Number {
            eprintln!("Unary operand must be a number.");
            panic!("Unary operand must be a number.");
        }
        unsafe {
            let a: f32 = self
                .stack
                .pop()
                .expect("This should be a number!")
                .type_as
                .number;
            self.stack
                .push(Value::new(ValueType::Number, As { number: -a }));
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
