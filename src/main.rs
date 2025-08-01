use crate::bitwise::Bitwise;
use chip8::Chip8Sys;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use std::{thread, time};

mod bitwise;
mod chip8;
mod decode;
mod roms;

pub const WIDTH: usize = 640 * 2;
pub const HEIGHT: usize = 320 * 2;

fn main() {
    let temp_vec_bool = vec![true, false, true, false, false, true, false, true];
    println!(
        "{:08b}",
        u8::from_bit_vec(temp_vec_bool[..8].to_vec())
            .expect("provided vector should be correct number of bits long.")
    );

    // fetch the program counter's instruction, parse it, and increment it
    // this is the built in Chip-8 font that Roms expect to access
    const FONT: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0 loc 0x050
        0x20, 0x60, 0x20, 0x20, 0x70, // 1 loc 0x055
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2 loc 0x05A
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3 loc 0x05F
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4 loc 0x064
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5 loc 0x069
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6 loc 0x06D
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7 loc 0x072
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8 loc 0x077
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9 loc 0x07C
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A loc 0x081
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B loc 0x086
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C loc 0x08B
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D loc 0x090
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E loc 0x095
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F loc 0x09A
    ];

    let mut game = Chip8Sys::new();

    // load the font in memeory
    for i in 0x050..0x0A0 {
        game.memory[i] = FONT[i - 0x050];
    }

    // load the ROM from Disc
    game.load_dxyn_rom_adv();
    // load_dxyn_rom_adv(&mut game); // Old way of doing this

    let mut window = Window::new(
        "Chip 8 - Press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale_mode: ScaleMode::UpperLeft,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create the window");

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&game.display_buffer(), WIDTH, HEIGHT)
            .unwrap();
        game.run();
        thread::sleep(time::Duration::from_millis(200));
    }
}
