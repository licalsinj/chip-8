use crate::{HEIGHT, WIDTH};

const EMPTY_MEMORY: [u8; 4096] = [0; 4096];
const EMPTY_REGISTER: [u8; 16] = [0; 16];
// const EMPTY_STACK: [u16; 16] = [0; 16];
const PIXEL_COLOR: u32 = 0x0000FF88;

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
        Chip8Sys {
            memory: EMPTY_MEMORY,
            register: EMPTY_REGISTER,
            register_i: 0,
            // register_delay: 0,
            // register_sound: 0,
            program_counter: 0x200, // initialize PC to start reading at 0x200
            // stack_pointer: 0,
            // stack: EMPTY_STACK,
            frame_buffer: [0x00; 256],
        }
    }
}

impl Chip8Sys {
    // converts the Chip8Sys frame_buffer to the 1280x640 display I'm using
    // TODO: Make this actually use the WIDTH and HEIGHT constants I define in main.rs
    pub fn display_buffer(&self) -> Vec<u32> {
        // NOTE: u32 is 4x as big as u8
        // Multiply frame_buffer length by 8 for u32 into u8 conversion
        // then by 20 for the WIDTH * HEIGHT scaling (which is still a magic number...)
        let scaler = (WIDTH * HEIGHT) / (self.frame_buffer.len() * 8 * 20);
        println!("scaler: {scaler}");

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
        for (i, pixel_col) in self.frame_buffer.iter().enumerate() {
            print!("{:2}: ", i);
            println!("{:08b}", pixel_col);
        }
    }
    */
}
