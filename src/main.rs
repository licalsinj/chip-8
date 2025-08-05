use chip8::Chip8Sys;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use std::{fs::File, io::Read, thread, time};

mod bitwise;
mod chip8;
mod decode;
// mod roms; // used for testing, may not be needed long term

pub const WIDTH: usize = 640 * 2;
pub const HEIGHT: usize = 320 * 2;
// passed when creating all new Chip8Sys
// handles if FX55 & FX65 increment I index register
pub const INC_INDEX: bool = true;

fn main() {
    let mut game = Chip8Sys::new(INC_INDEX);

    // load the ROM from Disc
    // let file_path = "roms/1-chip8-logo.ch8";
    // let file_path = "roms/2-ibm-logo.ch8";
    // let file_path = "roms/3-corax+.ch8";
    // let file_path = "roms/4-flags.ch8";
    let file_path = "roms/5-quirks.ch8";
    let mut file = File::open(file_path).expect("should have been able to open the file");
    let mut rom = [0; 0xFFF];
    file.read(&mut rom[..])
        .expect("Should have been able to read the rom file");
    /*
    println!(
        "Game memory length: {}, {:X}",
        game.memory.len(),
        game.memory.len()
    );
    // */
    // Manually prints the rom instructions to the screen
    // println!("rom to bytes:");
    for (i, byte) in rom.iter().enumerate() {
        /*
        // Manually prints the rom instructions to the screen
        print!("{:02X} ", byte);
        if (i + 1) % 16 == 0 {
            println!("");
        }
        // prints what i'm loading into where in memory
        println!(
            "{:02x}: load {:02X} in memory location {:02X}",
            i,
            byte,
            0x200 + i
        );
        // */
        if i + 0x200 > game.memory.len() - 1 {
            println!("Rom to long reading stopped");
            break;
        }
        game.memory[0x200 + i] = byte.to_owned();
    }
    // in the case of quirks hardcode this memory spot to auto run Chip-8
    game.memory[0x1FF] = 1;

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

    window.set_target_fps(65);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&game.display_buffer(), WIDTH, HEIGHT)
            .unwrap();
        game.run();
        // thread::sleep(time::Duration::from_millis(50));
    }
}
