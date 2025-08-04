use chip8::Chip8Sys;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use std::{fs::File, io::Read, thread, time};

mod bitwise;
mod chip8;
mod decode;
// mod roms; // used for testing, may not be needed long term

pub const WIDTH: usize = 640 * 2;
pub const HEIGHT: usize = 320 * 2;

fn main() {
    let mut game = Chip8Sys::new();

    // load the ROM from Disc
    // let file_path = "roms/1-chip8-logo.ch8";
    // let file_path = "roms/2-ibm-logo.ch8";
    let file_path = "roms/4-flags.ch8";
    let mut file = File::open(file_path).expect("should have been able to open the file");
    let mut rom = [0; 256];
    file.read(&mut rom[..])
        .expect("Should have been able to read the rom file");
    println!("rom to bytes:");
    for (i, byte) in rom.iter().enumerate() {
        print!("{:02X} ", byte);
        if (i + 1) % 16 == 0 {
            println!("");
        }
        game.memory[0x200 + i] = byte.to_owned();
        if i + 0x200 > game.memory.len() {
            println!("Rom to long reading stopped");
            break;
        }
    }

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
        thread::sleep(time::Duration::from_millis(50));
    }
}
