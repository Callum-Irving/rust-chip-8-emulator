use super::{Chip8, HEIGHT, WIDTH};
use rand::Rng;

impl Chip8 {
    /// 00E0 - CLS
    ///
    /// Clear the display.
    pub(super) fn op_00e0(&mut self) {
        self.display = [0; WIDTH * HEIGHT];
    }

    /// 00EE - RET
    ///
    /// Return from a subroutine.
    pub(super) fn op_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp];
    }

    /// 1nnn - JP addr
    ///
    /// Jump to location nnn.
    pub(super) fn op_1nnn(&mut self, addr: usize) {
        self.pc = addr;
    }

    /// 2nnn - CALL addr
    ///
    /// Call subroutine at nnn.
    pub(super) fn op_2nnn(&mut self, addr: usize) {
        self.stack[self.sp] = self.pc;
        self.sp += 1;
        self.pc = addr;
    }

    /// 3xkk - SE Vx, byte
    ///
    /// Skip next instruction if Vx = kk.
    pub(super) fn op_3xkk(&mut self, x: usize, byte: u8) {
        self.pc += if self.v[x] == byte { 2 } else { 0 };
    }

    /// 4xkk - SNE Vx, byte
    ///
    /// Skip next instruction if Vx != kk.
    pub(super) fn op_4xkk(&mut self, x: usize, byte: u8) {
        self.pc += if self.v[x] != byte { 2 } else { 0 };
    }

    /// 5xy0 - SE Vx, Vy
    ///
    /// Skip next instruction if Vx = Vy.
    pub(super) fn op_5xy0(&mut self, x: usize, y: usize) {
        self.pc += if self.v[x] == self.v[y] { 2 } else { 0 };
    }

    /// 6xkk - LD Vx, byte
    ///
    /// Set Vx = kk.
    pub(super) fn op_6xkk(&mut self, x: usize, byte: u8) {
        self.v[x] = byte;
    }

    /// 7xkk - ADD Vx, byte
    ///
    /// Set Vx = Vx + kk.
    pub(super) fn op_7xkk(&mut self, x: usize, byte: u8) {
        self.v[x] = self.v[x].wrapping_add(byte);
    }

    /// 8xy0 - LD Vx, Vy
    ///
    /// Set Vx = Vy.
    pub(super) fn op_8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }

    /// 8xy1 - OR Vx, Vy
    ///
    /// Set Vx = Vx OR Vy.
    pub(super) fn op_8xy1(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
    }

    /// 8xy2 - AND Vx, Vy
    ///
    /// Set Vx = Vx AND Vy.
    pub(super) fn op_8xy2(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
    }

    /// 8xy3 - XOR Vx, Vy
    ///
    /// Set Vx = Vx XOR Vy.
    pub(super) fn op_8xy3(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
    }

    /// 8xy4 - ADD Vx, Vy
    ///
    /// Set Vx = Vx + Vy, set VF = carry.
    pub(super) fn op_8xy4(&mut self, x: usize, y: usize) {
        let (res, overflow) = self.v[x].overflowing_add(self.v[y]);
        self.v[0x0F] = overflow as u8;
        self.v[x] = res;
    }

    /// 8xy5 - SUB Vx, Vy
    ///
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    pub(super) fn op_8xy5(&mut self, x: usize, y: usize) {
        let (res, overflow) = self.v[x].overflowing_sub(self.v[y]);
        self.v[0x0F] = !overflow as u8;
        self.v[x] = res;
    }

    /// 8xy6 - SHR Vx {, Vy}
    ///
    /// Set Vx = Vx SHR 1.
    pub(super) fn op_8xy6(&mut self, x: usize) {
        self.v[0x0F] = self.v[x] & 1;
        self.v[x] >>= 1;
    }

    /// 8xy7 - SUBN Vx, Vy
    ///
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    pub(super) fn op_8xy7(&mut self, x: usize, y: usize) {
        let (res, overflow) = self.v[y].overflowing_sub(self.v[x]);
        self.v[0x0F] = !overflow as u8;
        self.v[x] = res;
    }

    /// 8xyE - SHL Vx {, Vy}
    ///
    /// Set Vx = Vx SHL 1.
    pub(super) fn op_8xye(&mut self, x: usize) {
        self.v[0x0F] = (self.v[x] & 0x80) >> 7;
        self.v[x] <<= 1;
    }

    /// 9xy0 - SNE Vx, Vy
    ///
    /// Skip next instruction if Vx != Vy.
    pub(super) fn op_9xy0(&mut self, x: usize, y: usize) {
        self.pc += if self.v[x] != self.v[y] { 2 } else { 0 };
    }

    /// Annn - LD I, addr
    ///
    /// Set I = nnn.
    pub(super) fn op_annn(&mut self, addr: usize) {
        self.i = addr;
    }

    /// Bnnn - JP V0, addr
    ///
    /// Jump to location nnn + V0.
    pub(super) fn op_bnnn(&mut self, addr: usize) {
        self.pc = addr + self.v[0] as usize;
    }

    /// Cxkk - RND Vx, byte
    ///
    /// Set Vx = random byte AND kk.
    pub(super) fn op_cxkk(&mut self, x: usize, byte: u8) {
        let mut rng = rand::thread_rng();
        let random: u8 = rng.gen();
        self.v[x] = random & byte;
    }

    /// Dxyn - DRW Vx, Vy, nibble
    ///
    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    pub(super) fn op_dxyn(&mut self, x: usize, y: usize, nibble: u8) {
        for byte in 0..nibble {
            let sprite_y = (self.v[y] + byte) as usize % HEIGHT;
            for bit in 0..8 {
                let sprite_x = (self.v[x] + bit) as usize % WIDTH;
                let pixel = (self.memory[self.i + byte as usize] >> (7 - bit)) & 1;
                self.v[0x0F] |=
                    pixel & (self.display[sprite_y * WIDTH + sprite_x] == u32::MAX) as u8;
                self.display[sprite_y * WIDTH + sprite_x] ^= (pixel as u32) * u32::MAX;
            }
        }
        self.draw_flag = true;
    }

    /// Ex9E - SKP Vx
    ///
    /// Skip next instruction if key with the value of Vx is pressed.
    pub(super) fn op_ex9e(&mut self, x: usize) {
        if self.keypad[self.v[x] as usize] {
            self.pc += 2;
        }
    }

    /// ExA1 - SKNP Vx
    ///
    /// Skip next instruction if key with the value of Vx is not pressed.
    pub(super) fn op_exa1(&mut self, x: usize) {
        if !self.keypad[self.v[x] as usize] {
            self.pc += 2;
        }
    }

    /// Fx07 - LD Vx, DT
    ///
    /// Set Vx = delay timer value.
    pub(super) fn op_fx07(&mut self, x: usize) {
        self.v[x] = self.delay;
    }

    /// Fx0A - LD Vx, K
    ///
    /// Wait for a key press, store the value of the key in Vx.
    pub(super) fn op_fx0a(&mut self, x: usize) {
        let mut key_pressed = false;
        for (i, key) in self.keypad.iter().enumerate() {
            if *key {
                self.v[x] = i as u8;
                key_pressed = true;
                break;
            }
        }
        if !key_pressed {
            self.pc -= 2;
        }
    }

    /// Fx15 - LD DT, Vx
    ///
    /// Set delay timer = Vx.
    pub(super) fn op_fx15(&mut self, x: usize) {
        self.delay = self.v[x];
    }

    /// Fx18 - LD ST, Vx
    ///
    /// Set sound timer = Vx.
    pub(super) fn op_fx18(&mut self, x: usize) {
        self.sound = self.v[x];
    }

    /// Fx1E - ADD I, Vx
    ///
    /// Set I = I + Vx.
    pub(super) fn op_fx1e(&mut self, x: usize) {
        // TODO: Make self.i 16 bit
        // self.i += self.v[x] as usize;
        self.i = (self.i as u16).wrapping_add(self.v[x] as u16) as usize;
    }

    /// Fx29 - LD F, Vx
    ///
    /// Set I = location of sprite for digit Vx.
    pub(super) fn op_fx29(&mut self, x: usize) {
        self.i = self.v[x] as usize * 5 + 0x50;
    }

    /// Fx33 - LD B, Vx
    ///
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    pub(super) fn op_fx33(&mut self, x: usize) {
        self.memory[self.i] = self.v[x] / 100;
        self.memory[self.i + 1] = (self.v[x] % 100) / 10;
        self.memory[self.i + 2] = self.v[x] % 10;
    }

    /// Fx55 - LD [I], Vx
    ///
    /// Store registers V0 through Vx in memory starting at location I.
    pub(super) fn op_fx55(&mut self, x: usize) {
        // self.memory[(self.i)..(self.i + x + 1)].copy_from_slice(&self.v[0..(x + 1)]);
        for i in 0..x + 1 {
            self.memory[self.i + i] = self.v[i];
        }
    }

    /// Fx65 - LD Vx, [I]
    ///
    /// Read registers V0 through Vx from memory starting at location I.
    pub(super) fn op_fx65(&mut self, x: usize) {
        // self.v[0..(x + 1)].copy_from_slice(&self.memory[(self.i)..(self.i + x + 1)]);
        for i in 0..x + 1 {
            self.v[i] = self.memory[self.i + i];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_00e0() {
        let mut chip8 = Chip8::new();
        for i in 0..WIDTH * HEIGHT {
            chip8.display[i] = u32::MAX;
        }

        chip8.handle_opcode(0x00e0);

        assert_eq!(chip8.display, [0u32; WIDTH * HEIGHT]);
    }

    #[test]
    fn op_00ee() {
        let mut chip8 = Chip8::new();
        chip8.pc = 0x23;

        chip8.handle_opcode(0x2ABC);
        chip8.handle_opcode(0x00EE);

        assert_eq!(chip8.pc, 0x23 + 2);
        assert_eq!(chip8.sp, 0);
    }

    #[test]
    fn op_1nnn() {
        let mut chip8 = Chip8::new();
        chip8.handle_opcode(0x1ABC);
        assert_eq!(chip8.pc, 0xABC)
    }

    #[test]
    fn op_2nnn() {
        let mut chip8 = Chip8::new();
        chip8.pc = 0x23;
        chip8.handle_opcode(0x2ABC);
        assert_eq!(chip8.pc, 0xABC);
        assert_eq!(chip8.sp, 1);
        assert_eq!(chip8.stack[0], 0x23 + 2);
    }

    #[test]
    fn op_3xkk() {
        let mut chip8 = Chip8::new();
        chip8.pc = 0;
        chip8.v[1] = 0xFE;
        chip8.handle_opcode(0x31FE);
        assert_eq!(chip8.pc, 4);
        chip8.handle_opcode(0x31FA);
        assert_eq!(chip8.pc, 6);
    }

    #[test]
    fn op_4xkk() {
        let mut chip8 = Chip8::new();
        chip8.pc = 0;
        chip8.v[1] = 0xFE;
        chip8.handle_opcode(0x41FE);
        assert_eq!(chip8.pc, 2);
        chip8.handle_opcode(0x41FA);
        assert_eq!(chip8.pc, 6);
    }

    #[test]
    fn op_5xy0() {
        let mut chip8 = Chip8::new();
        chip8.pc = 0;
        chip8.v[0] = 12;
        chip8.v[1] = 12;
        chip8.handle_opcode(0x5010);
        assert_eq!(chip8.pc, 4);
        chip8.v[1] = 13;
        chip8.handle_opcode(0x5010);
        assert_eq!(chip8.pc, 6);
    }

    #[test]
    fn op_6xkk() {
        let mut chip8 = Chip8::new();
        chip8.handle_opcode(0x6123);
        assert_eq!(chip8.v[1], 0x23);
    }

    #[test]
    fn op_7xkk() {
        let mut chip8 = Chip8::new();
        chip8.v[1] = 3;
        chip8.handle_opcode(0x7101);
        assert_eq!(chip8.v[1], 4);
        chip8.v[1] = 0xFF;
        chip8.handle_opcode(0x7123);
        assert_eq!(chip8.v[1], 0x22);
    }

    #[test]
    fn op_8xy0() {
        let mut chip8 = Chip8::new();
        chip8.v[0] = 0x23;
        chip8.handle_opcode(0x8100);
        assert_eq!(chip8.v[1], 0x23);
    }

    #[test]
    fn op_8xy1() {
        let mut chip8 = Chip8::new();
        chip8.v[0] = 0x45;
        chip8.v[1] = 0x23;
        chip8.handle_opcode(0x8011);
        assert_eq!(chip8.v[0], 0x45 | 0x23);
    }

    #[test]
    fn op_8xy2() {
        let mut chip8 = Chip8::new();
        chip8.v[0] = 0x45;
        chip8.v[1] = 0x23;
        chip8.handle_opcode(0x8012);
        assert_eq!(chip8.v[0], 0x45 & 0x23);
    }

    #[test]
    fn op_8xy3() {
        let mut chip8 = Chip8::new();
        chip8.v[0] = 0x45;
        chip8.v[1] = 0x23;
        chip8.handle_opcode(0x8013);
        assert_eq!(chip8.v[0], 0x45 ^ 0x23);
    }

    #[test]
    fn op_8xy4() {
        let mut chip8 = Chip8::new();
        chip8.v[0] = 0x45;
        chip8.v[1] = 0x23;
        chip8.handle_opcode(0x8014);
        assert_eq!(chip8.v[0], 0x45 + 0x23);
        assert_eq!(chip8.v[0x0F], 0);
        chip8.v[0] = 0xFF;
        chip8.v[1] = 0x23;
        chip8.handle_opcode(0x8014);
        assert_eq!(chip8.v[0], 0x22);
        assert_eq!(chip8.v[0x0F], 1);
    }

    #[test]
    fn op_8xy5() {
        let mut chip8 = Chip8::new();
        chip8.v[0] = 0x45;
        chip8.v[1] = 0x23;
        chip8.handle_opcode(0x8015);
        assert_eq!(chip8.v[0], 0x45 - 0x23);
        assert_eq!(chip8.v[0x0F], 1);
        chip8.v[0] = 0x22;
        chip8.v[1] = 0x23;
        chip8.handle_opcode(0x8015);
        assert_eq!(chip8.v[0], 0xFF);
        assert_eq!(chip8.v[0x0F], 0);
    }

    #[test]
    fn op_8xy6() {
        let mut chip8 = Chip8::new();
        chip8.v[0] = 0x01;
        chip8.handle_opcode(0x8016);
        assert_eq!(chip8.v[0], 0);
        assert_eq!(chip8.v[0x0F], 1);
        chip8.v[0] = 0x02;
        chip8.handle_opcode(0x8016);
        assert_eq!(chip8.v[0], 1);
        assert_eq!(chip8.v[0x0F], 0);
    }

    #[test]
    fn op_8xy7() {
        let mut chip8 = Chip8::new();
        chip8.v[0] = 0x05;
        chip8.v[1] = 0x06;
        chip8.handle_opcode(0x8017);
        assert_eq!(chip8.v[0], 1);
        assert_eq!(chip8.v[0x0F], 1);
        chip8.v[0] = 0x06;
        chip8.v[1] = 0x05;
        chip8.handle_opcode(0x8017);
        assert_eq!(chip8.v[0], 0xFF);
        assert_eq!(chip8.v[0x0F], 0);
    }

    #[test]
    fn op_8xye() {
        let mut chip8 = Chip8::new();
        chip8.v[0] = 0xF0;
        chip8.handle_opcode(0x801E);
        assert_eq!(chip8.v[0], 0xF0 << 1);
        assert_eq!(chip8.v[0x0F], 1);
        chip8.v[0] = 0x0F;
        chip8.handle_opcode(0x801E);
        assert_eq!(chip8.v[0], 0x0F << 1);
        assert_eq!(chip8.v[0x0F], 0);
    }

    #[test]
    fn op_9xy0() {
        let mut chip8 = Chip8::new();
        chip8.v[0] = 0x23;
        chip8.v[1] = 0x24;
        chip8.pc = 0;
        chip8.handle_opcode(0x9010);
        assert_eq!(chip8.pc, 4);
        chip8.v[1] = 0x23;
        chip8.handle_opcode(0x9010);
        assert_eq!(chip8.pc, 6);
    }

    #[test]
    fn op_annn() {
        let mut chip8 = Chip8::new();
        chip8.handle_opcode(0xA123);
        assert_eq!(chip8.i, 0x123);
    }

    #[test]
    fn op_bnnn() {
        let mut chip8 = Chip8::new();
        chip8.v[0] = 0x23;
        chip8.handle_opcode(0xB123);
        assert_eq!(chip8.pc, 0x23 + 0x123);
    }

    #[test]
    fn op_fx07() {
        let mut chip8 = Chip8::new();
        chip8.delay = 5;
        chip8.handle_opcode(0xf007);
        assert_eq!(chip8.v[0], 5);
    }

    #[test]
    fn op_fx15() {
        let mut chip8 = Chip8::new();
        chip8.v[0] = 5;
        chip8.handle_opcode(0xf015);
        assert_eq!(chip8.delay, 5);
    }

    #[test]
    fn op_fx18() {
        let mut chip8 = Chip8::new();
        chip8.v[0] = 5;
        chip8.handle_opcode(0xf018);
        assert_eq!(chip8.sound, 5);
    }

    #[test]
    fn op_fx1e() {
        let mut chip8 = Chip8::new();
        chip8.i = 0x0023;
        chip8.v[0] = 0x02;
        chip8.handle_opcode(0xF01E);
        assert_eq!(chip8.i, 0x25);
        chip8.i = 0xFFFF;
        chip8.handle_opcode(0xF01E);
        assert_eq!(chip8.i, 0x01);
    }

    #[test]
    fn op_fx29() {
        let mut chip8 = Chip8::new();
        chip8.v[0] = 0xA;
        chip8.handle_opcode(0xF029);
        assert_eq!(chip8.i, 0xA * 5 + 0x50);
    }

    #[test]
    fn op_fx33() {
        let mut chip8 = Chip8::new();
        chip8.v[0] = 123;
        chip8.handle_opcode(0xF033);
        assert_eq!(chip8.memory[0], 1);
        assert_eq!(chip8.memory[1], 2);
        assert_eq!(chip8.memory[2], 3);
    }

    #[test]
    fn op_fx55() {
        let mut chip8 = Chip8::new();
        chip8.v[0] = 0;
        chip8.v[1] = 1;
        chip8.v[2] = 2;
        chip8.i = 0x202;
        chip8.handle_opcode(0xF255);
        assert_eq!(chip8.memory[0x202], 0);
        assert_eq!(chip8.memory[0x203], 1);
        assert_eq!(chip8.memory[0x204], 2);
    }

    #[test]
    fn op_fx65() {
        let mut chip8 = Chip8::new();
        chip8.memory[0x202] = 0;
        chip8.memory[0x203] = 1;
        chip8.memory[0x204] = 2;
        chip8.i = 0x202;
        chip8.handle_opcode(0xF265);
        assert_eq!(chip8.v[0], 0);
        assert_eq!(chip8.v[1], 1);
        assert_eq!(chip8.v[2], 2);
    }

    // TODO: write tests for the following:
    // cxkk
    // dxyn
}
