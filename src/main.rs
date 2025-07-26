use chip8::{Chip8Sys, Nibble, Sprite};
use minifb::{Key, ScaleMode, Window, WindowOptions};

mod chip8;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;

fn main() {
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

    // let mut noise;
    // let mut carry;
    // let mut seed = 0xbeefu32;

    let mut buffer = vec![0; WIDTH * HEIGHT];
    let mut game = Chip8Sys::new();

    // load the font in memeory
    for i in 0x050..0x09F {
        game.memory[i] = FONT[i - 0x050];
    }
    game.frame_buffer[0][0] = true;
    game.frame_buffer[1][1] = true;
    /*
    game.draw_sprite(
        1,
        1,
        Sprite::from_vec_u8(&vec![0xF0; 5]), //Sprite::from_vec_u8(&game.memory[0x050..0x055].to_vec()),
    );
    game.draw_sprite(
        10,
        10,
        Sprite::from_vec_u8(&game.memory[0x055..0x60].to_vec()),
    );
    */

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
    }
}

/*
fn print_byte_stream(b_s: Vec<char>, col: u8, div: u8) {
    let mut counter: u8 = 0;
    let mut div_counter: i8 = 0;

    for c in b_s.iter() {
        if counter % col == 0 {
            print!("{:0>2} - ", counter / col);
        }
        print!("{}", c.to_string());
        if div != 0 && (div_counter % (div as i8)) == (div as i8 - 1) {
            print!(" ");
            div_counter = -1;
        }
        if counter % col == col - 1 {
            println!("");
            div_counter = -1;
        }
        // This will cause the row counter to reset.
        // I could add a special row counter var for that
        // but I don't think that's important right now
        if counter == u8::MAX {
            counter = 0;
        } else {
            counter += 1;
        }
        div_counter += 1;
    }
    println!();
}
*/
