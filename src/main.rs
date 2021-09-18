use disassembler::*;
use std::env;
use std::fs;

mod disassembler;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let binary = fs::read(file_name).expect("Error reading file");

    let start = 0x200;
    for (pc, chunk) in binary.chunks(2).enumerate() {
        let opcode = ((chunk[0] as u16) << 8) | chunk[1] as u16;
        println!("{}", disassemble_opcode(start + pc * 2, opcode));
    }
}
