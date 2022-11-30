use crate::chunk;

pub fn dissamble_chunk(chunk: &chunk::Chunk, name: &str) {
   println!("== {} ==", name);
   let mut offset: usize = 0;
   while offset < chunk.len() {
    offset = dissamble_instruction(chunk, offset);
   } 
}

fn dissamble_instruction(chunk: &chunk::Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    let instruction: chunk::OpCode = chunk[offset];
    match instruction {
        chunk::OpCode::OpReturn => simple_instruction("OP_RETURN", offset),
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    return offset + 1;
}