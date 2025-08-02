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
    let mut game = Chip8Sys::new();

    // load the ROM from Disc
    game.load_dxyn_rom_adv();

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
