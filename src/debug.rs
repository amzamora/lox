use crate::chunk;

pub fn dissamble_chunk(chunk: &chunk::Chunk, name: &str) {
   println!("== {} ==", name);
   let mut offset: usize = 0;
   while offset < chunk.code.len() {
    offset = dissamble_instruction(chunk, offset);
   } 
}

fn dissamble_instruction(chunk: &chunk::Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    let instruction = num::FromPrimitive::from_i8(chunk.code[offset]);
    match instruction {
        Some(chunk::OpCode::OpConstant) => simple_instruction("OP_CONSTANT", offset),
        Some(chunk::OpCode::OpReturn) => simple_instruction("OP_RETURN", offset),
        None => {
            println!("Unkwnon opcode {}", chunk.code[offset]);
            return offset + 1;
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    return offset + 1;
}