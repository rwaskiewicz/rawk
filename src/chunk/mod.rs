//! Module describing the various operations the VM can take with various debugging utilities

use crate::value;
use crate::value::Value;
use log::debug;

// TODO: Look further into byte alignment
/// Enum describing different operations (operation codes)
#[derive(Clone, Debug)]
pub enum OpCode {
    OpConstant(value::Value),
    GreaterEqual,
    Greater,
    LessEqual,
    Less,
    DoubleEqual,
    NotEqual,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Exponentiation,
    UnaryPlus,
    UnaryMinus,
    LogicalNot,
    OpReturn,
    OpPrint,
    Concatenate,
    LogicalAnd,
    LogicalOr,
    Pop,
    GetGlobal(usize),
    SetGlobal(usize),
    DefineGlobal(usize),
    GetFieldVariable(),
    JumpIfFalse(usize, usize),
    Jump(usize, usize),
    JumpIfTrue(usize, usize),
    Loop(usize, usize),
}

/// Struct describing an OpCode and the line in the original corpus it appears in
#[derive(Clone)]
pub struct CodeLine {
    pub code: OpCode,
    pub line: i32,
}

/// Representation of a series of operations
pub struct Chunk {
    pub code: Vec<CodeLine>, // TODO: Make this a pointer to a series of bytes. That way it's dense (cache friendly) and has constant time lookup and appending. Neat.
    pub constants: Vec<String>,
}

impl Chunk {
    /// Instantiates a new chunk
    pub fn new() -> Chunk {
        Chunk {
            code: vec![],
            constants: vec![],
        }
    }

    /// Writes an opcode and its associated line to the chunk
    ///
    /// # Arguments
    /// - `code` the op code to write
    /// - `line` the line number associated with the op code
    pub fn write_chunk(&mut self, code: OpCode, line: i32) {
        self.code.push(CodeLine { code, line });
    }

    /// Adds a constant to the chunk's constant table
    ///
    /// # Arguments
    /// - `constant` the value to add to the table
    ///
    /// # Return value
    /// the index of the item in the constant table
    pub fn add_constant(&mut self, constant: String) -> usize {
        match self.constants.binary_search(&constant) {
            Ok(index) => index,
            Err(_) => {
                self.constants.push(constant);
                self.constants.len() - 1
            }
        }
    }

    /// Disassembles the chunk to debugging purposes
    ///
    /// Note: The log level for the language must be set to DEBUG for this function to work
    ///
    /// # Arguments
    /// - `name` a name to give the chunk while debugging for readability/understanding purposes
    pub fn disassemble_chunk(&self, name: &str) {
        debug!("== {} ==", name);

        for (idx, _code_line) in self.code.iter().enumerate() {
            self.disassemble_instruction(idx);
        }
    }

    /// Disassembles a single instruction in a chunk
    ///
    /// Note: The log level for the language must be set to DEBUG for this function to work
    ///
    /// # Arguments
    /// - `offset` the offset in the chunk to disassemble
    #[allow(unreachable_patterns)]
    fn disassemble_instruction(&self, offset: usize) -> usize {
        let mut instruction_info: String = String::from("");
        instruction_info.push_str(format!("{:<04} ", offset).as_str());

        if offset > 0 {
            instruction_info.push_str("   | ");
        } else {
            instruction_info
                .push_str(format!("{:4} ", &&self.code.get(offset).unwrap().line).as_str());
        }

        let instruction: &OpCode = &self.code.get(offset).unwrap().code;
        match instruction {
            OpCode::OpConstant(constant) => {
                self.constant_instruction(&instruction_info, "OpConstant", offset, constant)
            }
            OpCode::GreaterEqual => {
                Chunk::simple_instruction(&instruction_info, "GreaterEqual", offset)
            }
            OpCode::Greater => Chunk::simple_instruction(&instruction_info, "Greater", offset),
            OpCode::LessEqual => Chunk::simple_instruction(&instruction_info, "LessEqual", offset),
            OpCode::Less => Chunk::simple_instruction(&instruction_info, "Less", offset),
            OpCode::DoubleEqual => {
                Chunk::simple_instruction(&instruction_info, "DoubleEqual", offset)
            }
            OpCode::NotEqual => Chunk::simple_instruction(&instruction_info, "NotEqual", offset),
            OpCode::Add => Chunk::simple_instruction(&instruction_info, "Add", offset),
            OpCode::Subtract => Chunk::simple_instruction(&instruction_info, "Subtract", offset),
            OpCode::Multiply => Chunk::simple_instruction(&instruction_info, "Multiply", offset),
            OpCode::Divide => Chunk::simple_instruction(&instruction_info, "Divide", offset),
            OpCode::Modulus => Chunk::simple_instruction(&instruction_info, "Modulus", offset),
            OpCode::Exponentiation => {
                Chunk::simple_instruction(&instruction_info, "Exponentiation", offset)
            }
            OpCode::UnaryPlus => Chunk::simple_instruction(&instruction_info, "UnaryPlus", offset),
            OpCode::UnaryMinus => {
                Chunk::simple_instruction(&instruction_info, "UnaryMinus", offset)
            }
            OpCode::LogicalNot => {
                Chunk::simple_instruction(&instruction_info, "LogicalNot", offset)
            }
            OpCode::OpReturn => Chunk::simple_instruction(&instruction_info, "OpReturn", offset),
            OpCode::OpPrint => Chunk::simple_instruction(&instruction_info, "OpPrint", offset),
            OpCode::Concatenate => {
                Chunk::simple_instruction(&instruction_info, "Concatenate", offset)
            }
            OpCode::LogicalAnd => {
                Chunk::simple_instruction(&instruction_info, "LogicalAnd", offset)
            }
            OpCode::LogicalOr => Chunk::simple_instruction(&instruction_info, "LogicalOr", offset),
            OpCode::Pop => Chunk::simple_instruction(&instruction_info, "Pop", offset),
            OpCode::GetGlobal(_chunk_index) => {
                Chunk::simple_instruction(&instruction_info, "GetGlobal", offset)
            }
            OpCode::SetGlobal(_chunk_index) => {
                Chunk::simple_instruction(&instruction_info, "SetGlobal", offset)
            }
            OpCode::DefineGlobal(_chunk_index) => {
                Chunk::simple_instruction(&instruction_info, "DefineGlobal", offset)
            }
            _ => {
                debug!("Unknown opcode {:#?}!", instruction);
                offset + 1
            }
        }
    }

    /// Disassembles a simple instruction
    ///
    /// A simple instruction is one that has no additional associated data, such as a constant value
    ///
    /// # Arguments
    /// - `prelude` information regarding the chunk to print to the console
    /// - `name` the name of the instruction
    /// - `offset` the current offset that was read from
    ///
    /// # Return value
    /// A new offset to read the next instruction from
    fn simple_instruction(prelude: &str, name: &str, offset: usize) -> usize {
        debug!("{} {}", prelude, name);
        offset + 1
    }

    /// Disassembles a constant instruction
    ///
    /// A constant instruction is one that has an associated value, such as a constant
    ///
    /// # Arguments
    /// - `prelude` information regarding the chunk to print to the console
    /// - `name` the name of the instruction
    /// - `offset` the current offset that was read from
    /// - `value` the constant value associated with the instruction
    ///
    /// # Return value
    /// A new offset to read the next instruction from
    fn constant_instruction(
        &self,
        prelude: &str,
        name: &str,
        offset: usize,
        value: &Value,
    ) -> usize {
        debug!("{} {} {}", prelude, name, value);
        offset + 1
    }
}

#[cfg(test)]
mod chunk {
    use super::*;

    #[test]
    fn it_returns_the_index_of_a_constant_that_exists() {
        let mut chunk = Chunk::new();
        chunk.add_constant(String::from("foo"));
        chunk.add_constant(String::from("bar"));
        chunk.add_constant(String::from("baz"));
        let bar_index = chunk.add_constant(String::from("bar"));

        assert_eq!(bar_index, 1);
        assert_ne!(bar_index, chunk.constants.len());
    }

    #[test]
    fn it_places_a_never_seen_before_chunk_at_the_end_of_table() {
        let mut chunk = Chunk::new();

        chunk.add_constant(String::from("foo"));
        let foo_index = chunk.add_constant(String::from("foo"));
        assert_eq!(foo_index, 0);
        assert_eq!(foo_index, chunk.constants.len() - 1);

        chunk.add_constant(String::from("bar"));
        let bar_index = chunk.add_constant(String::from("bar"));
        assert_eq!(bar_index, 1);
        assert_eq!(bar_index, chunk.constants.len() - 1);
    }
}
