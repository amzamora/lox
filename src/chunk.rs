use crate::clox_value;

#[derive(Debug, Clone, Copy, FromPrimitive)]
pub enum OpCode {
    OpConstant,
    OpReturn,
}

pub struct Chunk {
    pub code: Vec<i8>,
    pub constants: Vec<clox_value::Value>,
}

impl Chunk {
    pub fn write_code(&mut self, code: i8) {
        self.code.push(code);
    }

    pub fn write_constant(&mut self, constant: clox_value::Value) -> usize {
        self.constants.push(constant);
        return self.constants.len() - 1;
    }
}