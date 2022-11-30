#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    OpReturn,
}

pub type Chunk = Vec<OpCode>;