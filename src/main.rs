use chip8::{Chip8Sys, Sprite};
use minifb::{Key, ScaleMode, Window, WindowOptions};
use std::{thread, time};

mod chip8;
mod decode;

const WIDTH: usize = 640 * 2;
const HEIGHT: usize = 320 * 2;

fn main() {
    // this is the built in Chip-8 font that Roms expect to access
    const FONT: [u8; 80] = [
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

    let mut buffer = vec![0; WIDTH * HEIGHT];
    let mut game = Chip8Sys::new();

    // load the font in memeory
    for i in 0x050..0x0A0 {
        game.memory[i] = FONT[i - 0x050];
    }

    // load the ROM from Disc
    load_rom(&mut game);

    // Test the Drawing of Sprite characters
    // /*
    draw_all_characters(&mut game, 0x050, 5, 0, 0);
    draw_all_characters(&mut game, 0x09B, -5, 0, 12);
    game.draw_sprite(
        25,
        18,
        Sprite::from_vec_u8(&game.memory[0x9B..0xA0].to_vec()),
    );
    // */
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
        buffer = game.display_buffer();
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        thread::sleep(time::Duration::from_millis(200));
        game.run();
    }
}

fn draw_all_characters(game: &mut Chip8Sys, sm: i128, increment: i128, x: u8, y: u8) {
    // println!();
    let mut start_memory = sm as usize;
    let mut count = 0;
    let mut x_counter = x as usize;
    let mut y_counter = y as usize;
    while count <= 0xF {
        let end_memory = (start_memory + 5) as usize;
        // println!("start mem: {:02x}", start_memory);
        // println!("end mem:   {:02x}", end_memory);
        game.draw_sprite(
            x_counter,
            y_counter,
            Sprite::from_vec_u8(&game.memory[start_memory..end_memory].to_vec()),
        );
        if start_memory as i128 + increment > 0 {
            start_memory = (start_memory as i128 + increment) as usize;
        } else {
            start_memory = 0;
        }
        x_counter += 5;
        if x_counter > 64 {
            y_counter += 6;
            x_counter = 0;
        }
        count += 1;
    }
}
fn load_rom(game: &mut Chip8Sys) {
    // clear screen
    game.memory[0x200] = 0x00;
    game.memory[0x201] = 0xE0;
    // draw whole screen
    game.memory[0x202] = 0xD1;
    game.memory[0x203] = 0x11;
    // fill screen
    game.memory[0x204] = 0x20;
    game.memory[0x205] = 0x00;
    // draw whole screen
    game.memory[0x206] = 0xDF;
    game.memory[0x207] = 0xFF;
    // jump to random ending memory
    game.memory[0x208] = 0x1F;
    game.memory[0x209] = 0x12;
    // set register 2 to 0xF8
    game.memory[0xF12] = 0x62;
    game.memory[0xF13] = 0xF8;
    // Add 2 to Reg 2
    game.memory[0xF14] = 0x72;
    game.memory[0xF15] = 0x02;
    // Set reg I to F1F
    game.memory[0xF16] = 0xAF;
    game.memory[0xF17] = 0x1F;
    // jump back to start
    game.memory[0xF18] = 0x12;
    game.memory[0xF19] = 0x00;
}
