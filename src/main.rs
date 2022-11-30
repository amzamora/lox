mod chunk;
mod debug;

fn main() {
    let mut chunk: chunk::Chunk = vec![];
    chunk.push(chunk::OpCode::OpReturn);
    debug::dissamble_chunk(&chunk, "test chunk");
}
