use crate::clox_value;

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    OpConstant,
    OpReturn,
}

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<clox_value::Value>,
}

impl Chunk {
    pub fn write_code(&mut self, code: OpCode) {
        self.code.push(code);
    }

    pub fn write_constant(&mut self, constant: clox_value::Value) -> usize {
        self.constants.push(constant);
        return self.constants.len() - 1;
    }
}