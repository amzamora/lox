mod chunk;
mod debug;
mod clox_value;

fn main() {
    let mut chunk: chunk::Chunk = chunk::Chunk{code: vec![], constants: vec![]};
    
    chunk.write_code(chunk::OpCode::OpConstant);
    chunk.write_code(chunk::OpCode::OpReturn);
    debug::dissamble_chunk(&chunk, "test chunk");
}
