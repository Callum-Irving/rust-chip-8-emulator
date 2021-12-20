mod opcodes;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Chip8 {
    memory: [u8; 4096],
    // pub display: [[bool; 64]; 32],
    pub display: [u32; WIDTH * HEIGHT],
    pc: usize,
    i: usize,
    sp: usize,
    stack: [usize; 16],
    delay: u8,
    sound: u8,
    v: [u8; 16],
    pub keypad: [bool; 16],
    pub draw_flag: bool,
}

const SPRITES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

impl Chip8 {
    pub fn new() -> Self {
        let mut chip = Chip8 {
            memory: [0; 4096],
            //display: [[false; 64]; 32],
            display: [0; WIDTH * HEIGHT],
            pc: 512,
            i: 0,
            sp: 0,
            stack: [0; 16],
            delay: 0,
            sound: 0,
            v: [0; 16],
            keypad: [false; 16],
            draw_flag: false,
        };

        // Load sprites starting at mem location 80 (0x50)
        chip.memory[0x50..0xA0].copy_from_slice(&SPRITES);

        chip
    }

    pub fn load_binary(&mut self, binary: Vec<u8>) {
        for (i, byte) in binary.iter().enumerate() {
            self.memory[512 + i] = *byte;
        }
    }

    fn handle_opcode(&mut self, opcode: u16) {
        self.pc += 2;

        let first_byte = (opcode >> 8) as u8;
        let second_byte = opcode as u8;
        let x = (first_byte & 0x0F) as usize;
        let y = ((second_byte & 0xF0) >> 4) as usize;
        let addr = (opcode & 0x0FFF) as usize;

        match (first_byte & 0xF0) >> 4 {
            0x00 => match second_byte {
                0xE0 => self.op_00e0(), // CLS
                0xEE => self.op_00ee(), // RET
                _ => panic!("Unknown opcode: {:#04X}", opcode),
            },
            0x01 => self.op_1nnn(addr),           // JP
            0x02 => self.op_2nnn(addr),           // CALL
            0x03 => self.op_3xkk(x, second_byte), // SE
            0x04 => self.op_4xkk(x, second_byte), // SNE
            0x05 => self.op_5xy0(x, y),           // SE
            0x06 => self.op_6xkk(x, second_byte), // LD
            0x07 => self.op_7xkk(x, second_byte), // ADD
            0x08 => match second_byte & 0x0F {
                0x00 => self.op_8xy0(x, y), // LD
                0x01 => self.op_8xy1(x, y), // OR
                0x02 => self.op_8xy2(x, y), // AND
                0x03 => self.op_8xy3(x, y), // XOR
                0x04 => self.op_8xy4(x, y), // ADD
                0x05 => self.op_8xy5(x, y), // SUB
                0x06 => self.op_8xy6(x),    // SHR
                0x07 => self.op_8xy7(x, y), // SUBN
                0x0E => self.op_8xye(x),    // SHL
                _ => panic!("Unknown opcode: {:#04X}", opcode),
            },
            0x09 => self.op_9xy0(x, y),                     // SNE
            0x0A => self.op_annn(addr),                     // LD
            0x0B => self.op_bnnn(addr),                     // JP
            0x0C => self.op_cxkk(x, second_byte),           // TODO: RND
            0x0D => self.op_dxyn(x, y, second_byte & 0x0F), // DRW
            0x0E => match second_byte {
                0x9E => self.op_ex9e(x), // TODO: SKP
                0xA1 => self.op_exa1(x), // TODO: SKNP
                _ => panic!("Unknown opcode: {:#04X}", opcode),
            },
            0x0F => match second_byte {
                0x07 => self.op_fx07(x), // LD
                0x0A => self.op_fx0a(x), // LD
                0x15 => self.op_fx15(x), // LD
                0x18 => self.op_fx18(x), // LD
                0x1E => self.op_fx1e(x), // ADD
                0x29 => self.op_fx29(x), // LD
                0x33 => self.op_fx33(x), // LD
                0x55 => self.op_fx55(x), // LD
                0x65 => self.op_fx65(x), // LD
                _ => panic!("Unknown opcode: {:#04X}", opcode),
            },
            _ => panic!("Unknown opcode: {:#04X}", opcode),
        }
    }

    pub fn step(&mut self) {
        self.draw_flag = false;
        let opcode =
            (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16;
        self.handle_opcode(opcode);
    }

    pub fn decrement_timers(&mut self) {
        if self.delay > 0 {
            self.delay -= 1;
        }
    }

    pub fn dump_info(&self) {
        println!("PC: {}", self.pc);
        println!("V: {:?}", self.v);
        println!("I: {}", self.i);
        println!("SP: {}", self.sp);
        println!("Stack: {:?}", self.stack);
        println!("Delay: {}", self.delay);
        println!("Sound: {}", self.sound);
    }

    pub fn dump_mem(&self) {
        println!("Memory:");
        for chunk in self.memory.chunks(16) {
            println!("{:<#04X} {:<#04X} {:<#04X} {:<#04X} {:<#04X} {:<#04X} {:<#04X} {:<#04X} {:<#04X} {:<#04X} {:<#04X} {:<#04X} {:<#04X} {:<#04X} {:<#04X} {:<#04X}", chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7], chunk[8], chunk[9], chunk[10], chunk[11], chunk[12], chunk[13], chunk[14], chunk[15]);
        }
    }
}
