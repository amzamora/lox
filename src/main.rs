extern crate num;
#[macro_use]
extern crate num_derive;

mod chunk;
mod debug;
mod clox_value;

fn main() {
    let mut chunk: chunk::Chunk = chunk::Chunk{code: vec![], constants: vec![]};

    chunk.write_code(chunk::OpCode::OpConstant as i8);
    chunk.write_code(chunk::OpCode::OpReturn as i8);
    debug::dissamble_chunk(&chunk, "test chunk");
}
