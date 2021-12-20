use chip8::{Chip8, HEIGHT, WIDTH};
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use std::env;
use std::fs;

use std::time::{Duration, Instant};

mod chip8;
mod disassembler;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let binary = fs::read(file_name).expect("Error reading file");

    // use disassembler::*;
    // let start = 0x200;
    // for (pc, chunk) in binary.chunks(2).enumerate() {
    //     let opcode = ((chunk[0] as u16) << 8) | chunk[1] as u16;
    //     println!("{}", disassemble_opcode(start + pc * 2, opcode));
    // }
    // return;

    let mut chip = Chip8::new();
    chip.load_binary(binary);

    let opts = WindowOptions {
        borderless: false,
        title: true,
        resize: false,
        scale: Scale::X8,
        scale_mode: ScaleMode::Stretch,
        topmost: false,
        transparency: false,
        none: false,
    };
    let mut window =
        Window::new("Chip 8 Emulator", WIDTH, HEIGHT, opts).unwrap_or_else(|e| panic!("{}", e));

    // TODO: Calibrate
    window.limit_update_rate(Some(std::time::Duration::from_micros(1500)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Keypad layout:
        // 1 2 3 C
        // 4 5 6 D
        // 7 8 9 E
        // A 0 B F
        chip.keypad.iter_mut().for_each(|key| *key = false);
        window.get_keys().iter().for_each(|key| match key {
            Key::Key1 => chip.keypad[0x1] = true,
            Key::Key2 => chip.keypad[0x2] = true,
            Key::Key3 => chip.keypad[0x3] = true,
            Key::Key4 => chip.keypad[0xc] = true,
            Key::Q => chip.keypad[0x4] = true,
            Key::W => chip.keypad[0x5] = true,
            Key::E => chip.keypad[0x6] = true,
            Key::R => chip.keypad[0xD] = true,
            Key::A => chip.keypad[0x7] = true,
            Key::S => chip.keypad[0x8] = true,
            Key::D => chip.keypad[0x9] = true,
            Key::F => chip.keypad[0xE] = true,
            Key::Z => chip.keypad[0xA] = true,
            Key::X => chip.keypad[0x0] = true,
            Key::C => chip.keypad[0xB] = true,
            Key::V => chip.keypad[0xF] = true,
            _ => (),
        });

        for _ in 0..10 {
            chip.step();

            if chip.draw_flag == true {
                window
                    .update_with_buffer(&chip.display, WIDTH, HEIGHT)
                    .unwrap();
            } else {
                window.update();
            }
        }
        chip.decrement_timers();
    }

    println!("Escape was pressed!");
}
