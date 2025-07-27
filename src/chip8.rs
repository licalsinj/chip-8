use std::fmt::Display;

const EMPTY_MEMORY: [u8; 4096] = [0; 4096];
const EMPTY_REGISTER: [u8; 16] = [0; 16];
const EMPTY_STACK: [u16; 16] = [0; 16];
const PIXEL_COLOR: u32 = 0x0000FF88;

pub struct Chip8Sys {
    pub memory: [u8; 4096],
    pub register: [u8; 16],
    pub register_i: u8,
    pub register_delay: u8,
    pub register_sound: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub stack: [u16; 16],
    pub frame_buffer: [[bool; 64]; 32], // display is 64px by 32px minifb expects u32
}

impl Chip8Sys {
    pub fn new() -> Chip8Sys {
        Chip8Sys {
            memory: EMPTY_MEMORY,
            register: EMPTY_REGISTER,
            register_i: 0,
            register_delay: 0,
            register_sound: 0,
            program_counter: 0,
            stack_pointer: 0,
            stack: EMPTY_STACK,
            frame_buffer: [[false; 64]; 32],
        }
    }
}

impl Chip8Sys {
    pub fn display_buffer(&self) -> Vec<u32> {
        // Convert the 64x32 pixel frame_buffer to the 640x320 computer display
        /*
        println!("frame_buffer");
        for (i, pixel_col) in self.frame_buffer.iter().enumerate() {
            print!("{:2}: ", i);
            for pixel in pixel_col {
                if *pixel {
                    print!("1");
                } else {
                    print!("0");
                }
            }
            println!();
        }
        println!();
        */

        // build all the rows
        let mut result = Vec::new();
        for row in self.frame_buffer.iter() {
            let mut row_display = Vec::new();
            for col in row {
                if *col {
                    row_display.append(&mut vec![PIXEL_COLOR; 20]);
                } else {
                    row_display.append(&mut vec![0; 20]);
                }
            }
            result.append(&mut vec![row_display; 20].concat());
        }
        result
    }
    pub fn draw_nibble(&mut self, x: usize, y: usize, nibble: Nibble) {
        println!("frame_buffer x: {}", x);
        println!("frame_buffer y: {}", y);
        println!(
            "nibble 0-3: {}{}{}{}",
            nibble.0 as u8, nibble.1 as u8, nibble.2 as u8, nibble.3 as u8
        );
        self.frame_buffer[y][x] = nibble.0;
        self.frame_buffer[y][x + 1] = nibble.1;
        self.frame_buffer[y][x + 2] = nibble.2;
        self.frame_buffer[y][x + 3] = nibble.3;
    }
    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: Sprite) {
        self.draw_nibble(x, y, sprite.0);
        self.draw_nibble(x, y + 1, sprite.1);
        self.draw_nibble(x, y + 2, sprite.2);
        self.draw_nibble(x, y + 3, sprite.3);
        self.draw_nibble(x, y + 4, sprite.4);
    }
}

pub struct Nibble(bool, bool, bool, bool);
impl Nibble {
    fn from_u8(byte: u8) -> Nibble {
        println!("nibble byte: {:08b}", byte);
        Nibble(
            byte & 0b1000_0000 == 128,
            byte & 0b0100_0000 == 64,
            byte & 0b0010_0000 == 32,
            byte & 0b0001_0000 == 16,
        )
    }
}

pub struct Sprite(Nibble, Nibble, Nibble, Nibble, Nibble);
impl Sprite {
    pub fn from_vec_u8(input: &Vec<u8>) -> Sprite {
        let mut bytes = input.clone();
        if bytes.len() < 5 {
            bytes.append(&mut vec![0; (bytes.len()..5).len()]);
        }
        Sprite(
            Nibble::from_u8(bytes[0]),
            Nibble::from_u8(bytes[1]),
            Nibble::from_u8(bytes[2]),
            Nibble::from_u8(bytes[3]),
            Nibble::from_u8(bytes[4]),
        )
    }
}
