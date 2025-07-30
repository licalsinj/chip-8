use chip8::Chip8Sys;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use std::{thread, time};

mod chip8;
mod decode;

const WIDTH: usize = 640 * 2;
const HEIGHT: usize = 320 * 2;

fn main() {
    println!(" u8::MAX: {:04X}", u8::MAX);
    println!("u16::MAX: {:04X}", u16::MAX);
    println!(" u8::MAX: {}", u8::MAX);
    println!("u16::MAX: {}", u16::MAX);
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

    let mut buffer = vec![0; WIDTH * HEIGHT];
    let mut game = Chip8Sys::new();

    // load the font in memeory
    for i in 0x050..0x0A0 {
        game.memory[i] = FONT[i - 0x050];
    }

    // load the ROM from Disc
    load_dxyn_rom(&mut game);

    // Test the Drawing of Sprite characters
    // /*
    draw_all_characters(&mut game, 0x050, 5, 0, 0);
    draw_all_characters(&mut game, 0x09B, -5, 0, 12);
    /*
    game.draw_sprite(
        25,
        18,
        Sprite::from_vec_u8(&game.memory[0x9B..0xA0].to_vec()),
    );
    */
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
        /*
        game.draw_sprite(
            x_counter,
            y_counter,
            Sprite::from_vec_u8(&game.memory[start_memory..end_memory].to_vec()),
        );
        */
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
fn load_dxyn_rom(game: &mut Chip8Sys) {
    // clear screen
    game.memory[0x200] = 0x00;
    game.memory[0x201] = 0xE0;
    // load register V0 with x position
    game.memory[0x202] = 0x60;
    game.memory[0x203] = 0x00;
    // load register V1 with y position
    game.memory[0x204] = 0x61;
    game.memory[0x205] = 0x05;
    // load register I with sprite location
    game.memory[0x206] = 0xA0;
    game.memory[0x207] = 0x9B; // F Sprite is at 0x09B

    /*
    // draw sprite at I at position V0 and V1
    game.memory[0x208] = 0xD0;
    game.memory[0x209] = 0x15; // the default sprites are 5 px tall
    // */

    // update reg[1]'s location to move 10 Y
    game.memory[0x20A] = 0x71;
    game.memory[0x20B] = 0x0A;
    // draw F again at new I location
    game.memory[0x20C] = 0xD0;
    game.memory[0x20D] = 0x15;
    /*
    // try to draw 0 to 2 vertically at (0, 15)
    // load V2 with x position (0)
    game.memory[0x20E] = 0x62;
    game.memory[0x20F] = 0x00;
    // load V3 with y position 0x0F
    game.memory[0x210] = 0x63;
    game.memory[0x211] = 0x00;
    // load register I with sprite location
    game.memory[0x212] = 0xA0;
    game.memory[0x213] = 0x50; // 0 Sprite is at 0x050

    // draw something 0xF (15) lines tall
    game.memory[0x214] = 0xD2;
    game.memory[0x215] = 0x3F;
    // */
    // reset Y to be next to the first F
    game.memory[0x216] = 0x61;
    game.memory[0x217] = 0x05;

    // Add 5 to Y
    game.memory[0x218] = 0x71;
    game.memory[0x219] = 0x05;

    // load E sprite's location into Register I
    game.memory[0x21A] = 0xA0;
    game.memory[0x21B] = 0x00; // E sprite location: 0x96

    // draw sprite at I at position V0 and V1
    game.memory[0x21C] = 0xD0;
    game.memory[0x21D] = 0x15; // the default sprites are 5 px tall

    // update reg[1]'s location to move 5 in Y direction
    game.memory[0x21E] = 0x71;
    game.memory[0x21F] = 0x05;

    // draw sprite at I at position V0 and V1
    game.memory[0x220] = 0xD0;
    game.memory[0x221] = 0x15; // the default sprites are 5 px tall

    // jump to beginning memory
    // game.memory[0x230] = 0x12;
    // game.memory[0x231] = 0x00;
}
fn load_flashing_rom(game: &mut Chip8Sys) {
    // This is based dxyn doing nothing
    // This also uses a fake commande 0x2000 which fills the screen
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
