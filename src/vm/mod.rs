use crate::chunk::{Chunk, OpCode};
use crate::parser::Parser;
use crate::token::Token;
use crate::value::Value;
use crate::ParsedDataInput;

use log::{debug, error};
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
        let mut vm = VM {
            chunk: Chunk::new(),
            ip: 0,
            stack: vec![],
            globals: HashMap::new(),
        };
        vm.globals.insert("NF".into(), Value::Number(0.0));
        vm
    }

    /// Runs code that has been compiled
    ///
    /// # Arguments
    /// - `data` user provided data to use when running the compiled code
    ///
    /// # Return value
    /// the result of running the provided source, expressed as an `InterpretError` if the code is
    /// unable to run to completion
    pub fn run(&mut self, all_data: &[ParsedDataInput]) -> Result<(), InterpretError> {
        // create an iterator that can be manually advanced, as some functionality (`getline`)
        // requires more than one advancement per instruction
        let mut data_iter = all_data.iter();
        // prime the pump with the first set of data
        let mut data = data_iter.next();
        self.globals.insert(
            "NF".into(),
            Value::Number(data.unwrap().parsed.len() as f32),
        );
        loop {
            let instruction: OpCode = self.chunk.code[self.ip].code.clone();
            self.ip += 1;

            debug!("VM switching on instruction '{:#?}'", &instruction);
            match instruction {
                OpCode::OpPrint => match self.stack.pop() {
                    Some(val) => {
                        println!("{}", val);
                    }
                    None => {
                        error!("Error: The stack was empty when trying to print");
                        break Err(InterpretError::RuntimeError);
                    }
                },
                OpCode::OpReturn => {
                    if !self.stack.is_empty() {
                        error!("The stack is not empty! {:?}", self.stack);
                        break Err(InterpretError::RuntimeError);
                    }

                    data = data_iter.next();
                    if data.is_none() {
                        // we are out of records and are done running
                        break Ok(());
                    } else {
                        // we have another record to process, reset the vm and start again
                        self.reset_vm();
                        // TODO: I don't love, but I don't know if we can get around this
                        //
                        // TODO: This recomputation affects and is affected by NF (the number of
                        // fields; see Examining Fields). For example, the value of NF is set to the
                        // number of the highest field you create. The exact format of $0 is also
                        // affected by a feature that has not been discussed yet: the output field
                        // separator, OFS, used to separate the fields (see Output Separators).
                        self.globals.insert(
                            "NF".into(),
                            Value::Number(data.unwrap().parsed.len() as f32),
                        );
                    }
                }
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
                    // TODO: Consider if we should return an error should we pop off the stack when it is empty
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
                OpCode::GetFieldVariable() => {
                    // the index may be the result of an expression - e.g. $(1+2), where the result
                    // (3) would be on the top of the stack. pop it off. if there is no value,
                    // that's illegal.
                    let index = match self.stack.pop() {
                        // https://www.gnu.org/software/gawk/manual/gawk.html#Nonconstant-Fields:
                        // > Negative field numbers are not allowed; trying to reference one
                        // > usually terminates the program. (The POSIX standard does not define
                        // > what happens when you reference a negative field number. gawk notices
                        // > this and terminates your program. Other awk implementations may behave
                        // > differently.)
                        // This awk will allow decimal accesses (for now) by loss of precision:
                        // $2.3 -> $2
                        Some(awk_value) => awk_value.num_value(),
                        None => {
                            error!("Error: The stack was empty when trying to determine a field reference lookup");
                            break Err(InterpretError::RuntimeError);
                        }
                    };
                    if index < 0.0 {
                        error!("r-awk: trying to access out of range field {}", index);
                        break Err(InterpretError::RuntimeError);
                    } else if (index - index.trunc()).abs() > 0.0 {
                        error!("r-awk: trying to access non integer index {}", index);
                        break Err(InterpretError::RuntimeError);
                    }

                    let safer_index = index as usize;
                    let data = data.unwrap();
                    if safer_index <= data.parsed.len() {
                        let value = if safer_index == 0 {
                            data.original.clone()
                        } else {
                            data.parsed[safer_index - 1].clone()
                        };
                        if value.trim().parse::<f32>().is_ok() {
                            self.stack.push(Value::StrNum(value));
                        } else {
                            self.stack.push(Value::String(value));
                        }
                    } else {
                        // if the index that user specified does not exist, push something on the
                        // stack in case we're doing something like `print $9999;'
                        self.stack.push(Value::String(String::from("")));
                    }
                }
                OpCode::JumpIfFalse(offset1, offset2) => {
                    let condition_result = self.peek(0).truthy_value();
                    if !condition_result {
                        let offset = (offset1 >> 8) | offset2;
                        self.ip += offset;
                    }
                }
                OpCode::JumpIfTrue(offset1, offset2) => {
                    let condition_result = self.peek(0).num_value() != 0.0;
                    if condition_result {
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
                OpCode::Loop(offset1, offset2) => {
                    let offset = (offset1 >> 8) | offset2;
                    self.ip -= offset;
                }
            }
        }
    }

    fn reset_vm(&mut self) {
        self.ip = 0;
        self.stack = vec![];
    }

    fn read_variable_name(&mut self, chunk_index: usize) -> String {
        self.chunk
            .constants
            .get(chunk_index)
            .expect("No variable found")
            .clone()
    }

    /// Entrypoint for the VM
    ///
    /// # Arguments
    /// - `tokens` the tokens generated by parsing the user's program
    /// - `data` any user provided data to run the user's program against
    ///
    /// # Return value
    /// the result of running the provided source, expressed as an `InterpretError` if the code is unable to run to
    /// completion
    pub fn interpret(
        &mut self,
        tokens: &[Token],
        data: &[ParsedDataInput],
    ) -> Result<(), InterpretError> {
        self.chunk = Chunk::new();
        self.ip = 0;
        let mut parser = Parser::new(tokens.iter(), &mut self.chunk);

        if !parser.parse() {
            return Err(InterpretError::CompileError);
        }

        self.run(data)
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
            self.string_comparison(op_code, &a.str_value(), &b.str_value());
        } else {
            // implicitly convert `Value::StrNum` to numbers
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
        if matches!(self.peek(0), Value::Number(_)) || matches!(self.peek(0), Value::StrNum(_)) {
            let num_like = self.stack.pop().unwrap().num_value();
            match *op_code {
                // Unary plus will be more useful for converting a string to a number
                OpCode::UnaryPlus => {
                    if num_like == 0.0 {
                        self.stack.push(Value::Number(0.0))
                    } else {
                        self.stack.push(Value::Number(num_like))
                    }
                }
                OpCode::UnaryMinus => {
                    if num_like == 0.0 {
                        self.stack.push(Value::Number(0.0))
                    } else {
                        self.stack.push(Value::Number(-num_like))
                    }
                }
                OpCode::LogicalNot => {
                    let mut result: f32 = 1.0;
                    if num_like > 0.0 {
                        result = 0.0;
                    }
                    self.stack.push(Value::Number(result))
                }
                _ => panic!(
                    "Unknown op code given for unary on num/strnum: '{:?}'",
                    op_code
                ),
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
                    OpCode::UnaryMinus | OpCode::UnaryPlus => {
                        // if we go out of bounds on a field variable, push zero
                        self.stack.push(Value::Number(0.0))
                    }
                    _ => panic!(
                        "Unknown op code given for unary operation on string: '{:?}'",
                        op_code
                    ),
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
