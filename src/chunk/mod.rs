use crate::value;

// TODO: Look further into byte alignment
/// Enum describing different operations (operation codes)
#[derive(Copy, Clone, Debug)]
pub enum OpCode {
    OpConstant(value::Value),
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    OpReturn,
}

#[derive(Copy, Clone)]
pub struct CodeLine {
    pub code: OpCode,
    pub line: i32,
}

pub struct Chunk {
    pub code: Vec<CodeLine>, // TODO: Make this a pointer to a series of bytes. That way it's dense (cache friendly) and has constant time lookup and appending. Neat.
    pub constants: Vec<value::Value>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: vec![],
            constants: vec![],
        }
    }

    pub fn write_chunk(&mut self, code: OpCode, line: i32) {
        self.code.push(CodeLine { code, line });
    }

    pub fn add_constant(&mut self, value: value::Value) -> value::Value {
        self.constants.push(value);
        // return the constant where the index was appended so we can locate it later
        return (self.constants.len() - 1) as value::Value;
    }

    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {} ==", name);

        for (idx, _code_line) in self.code.iter().enumerate() {
            self.disassemble_instruction(idx);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{}", format!("{:<04} ", offset));

        if offset > 0 {
            print!("   | ");
        } else {
            print!("{:4} ", &&self.code.get(offset).unwrap().line)
        }

        let instruction: &OpCode = &self.code.get(offset).unwrap().code;
        match instruction {
            OpCode::OpConstant(_) => {
                return self.constant_instruction("OP_CONSTANT", offset);
            }
            OpCode::OpReturn => {
                return Chunk::simple_instruction("OP_RETURN", offset);
            }
            _ => {
                println!("Unknown opcode {:#?}!", instruction);
                return offset + 1;
            }
        }
    }

    fn simple_instruction(name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.constants[offset]; // TODO: +1 ?
        println!("{} {}", name, constant);
        // self.print_value(self.constants[constant as usize]);
        return offset + 1; // TODO??? + 2
    }

    fn print_value(&self, value: value::Value) {
        print!("{}", value);
    }
}
