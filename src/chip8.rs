// use crate::{HEIGHT, WIDTH};

const EMPTY_MEMORY: [u8; 4096] = [0; 4096];
const EMPTY_REGISTER: [u8; 16] = [0; 16];
// const EMPTY_STACK: [u16; 16] = [0; 16];
const PIXEL_COLOR: u32 = 0x0000FF88;
// This is the built in Chip-8 font that Roms expect to access
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
const FONT_RANGE_MIN: u8 = 0x050;
const FONT_RANGE_MAX: u8 = 0x0A0;

pub struct Chip8Sys {
    pub memory: [u8; 4096],
    pub register: [u8; 16],
    pub register_i: u16,
    // pub register_delay: u8, // Will be used eventually
    // pub register_sound: u8,// Will be used eventually
    pub program_counter: u16,
    // pub stack_pointer: u8,// Will be used eventually
    // pub stack: [u16; 16],// Will be used eventually
    pub frame_buffer: [u8; 256],
}

impl Chip8Sys {
    // Creates a new Chip8Sys with default settings
    pub fn new() -> Chip8Sys {
        let mut new_chip_8_sys = Chip8Sys {
            memory: EMPTY_MEMORY,
            register: EMPTY_REGISTER,
            register_i: 0,
            // register_delay: 0,
            // register_sound: 0,
            program_counter: 0x200, // initialize PC to start reading at 0x200
            // stack_pointer: 0,
            // stack: EMPTY_STACK,
            frame_buffer: [0x00; 256],
        };
        // load the font in memeory
        for i in FONT_RANGE_MIN..FONT_RANGE_MAX {
            new_chip_8_sys.memory[i as usize] = FONT[i as usize - FONT_RANGE_MIN as usize];
        }
        new_chip_8_sys
    }
}

impl Chip8Sys {
    // converts the Chip8Sys frame_buffer to the 1280x640 display I'm using
    // TODO: Make this actually use the WIDTH and HEIGHT constants I define in main.rs
    pub fn display_buffer(&self) -> Vec<u32> {
        // NOTE: u32 is 4x as big as u8
        // Multiply frame_buffer length by 8 for u32 into u8 conversion
        // then by 20 for the WIDTH * HEIGHT scaling (which is still a magic number...)
        // let scaler = (WIDTH * HEIGHT) / (self.frame_buffer.len() * 8 * 20);
        // println!("scaler: {scaler}");
        let scaler = 20;

        // Prints debug of the frame buffer to the console
        // self.debug_print_frame_buffer();

        let mut results = Vec::new();
        let mut result: Vec<u32> = Vec::new();
        for (i, pixel) in self.frame_buffer.iter().enumerate() {
            let mut power_2 = 0b1000_0000;
            for _ in 0..8 {
                if pixel & power_2 == power_2 {
                    result.append(&mut vec![PIXEL_COLOR; scaler]);
                } else {
                    result.append(&mut vec![0; scaler]);
                }
                // reduce power_2 to check the next bit to the right
                power_2 /= 2;
            }
            // every 8 bytes (64 bits) add scaler number of rows to results
            // this adds vertical thickness to the screen
            if (i + 1) % 8 == 0 {
                results.append(&mut vec![result; scaler].concat());
                result = Vec::new();
            }
        }
        results
    }
    /*
    // This will print the frame_buffer to the console
    fn debug_print_frame_buffer(&self) {
        println!("frame_buffer");
        // Prints debug frame buffer values as bits
        for (i, px) in self.frame_buffer.iter().enumerate() {
            print!("{:08b}", px);
            if (i + 1) % 8 == 0 {
                println!("!");
            }
        }
    }
    // */
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // Tests to make sure that we can create a new Chip8Sys with the font in the right place;
    fn create_new_chip_8_sys() {
        let new_chip_8_sys = Chip8Sys::new();
        assert_eq!(
            new_chip_8_sys.memory[(FONT_RANGE_MIN as usize)..(FONT_RANGE_MAX as usize)],
            FONT
        );
    }
}
