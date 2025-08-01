pub const EMPTY_MEMORY: [u8; 4096] = [0; 4096];
const EMPTY_REGISTER: [u8; 16] = [0; 16];
const EMPTY_STACK: [u16; 16] = [0; 16];
const PIXEL_COLOR: u32 = 0x0000FF88;

pub struct Chip8Sys {
    pub memory: [u8; 4096],
    pub register: [u8; 16],
    pub register_i: u16,
    pub register_delay: u8,
    pub register_sound: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub stack: [u16; 16],
    pub frame_buffer: [u8; 256],
}

impl Chip8Sys {
    pub fn new() -> Chip8Sys {
        Chip8Sys {
            memory: EMPTY_MEMORY,
            register: EMPTY_REGISTER,
            register_i: 0,
            register_delay: 0,
            register_sound: 0,
            program_counter: 0x200, // initialize PC to start reading at 0x200
            stack_pointer: 0,
            stack: EMPTY_STACK,
            frame_buffer: [0x00; 256],
        }
    }
}

impl Chip8Sys {
    pub fn display_buffer(&self) -> Vec<u32> {
        // self.debug_print_frame_buffer();

        // Convert the 64x32 pixel frame_buffer to the 640x320 computer display
        let mut results = Vec::new();
        let mut result: Vec<u32> = Vec::new();
        for (i, pixel) in self.frame_buffer.iter().enumerate() {
            // Convert frame_buffer to the display buffer which is 10x bigger and u32
            // u32 is 4x as big as u8
            // TODO: There has got to be a way to do this in a loop!
            if pixel & 128 == 128 {
                result.append(&mut vec![PIXEL_COLOR; 20]);
            } else {
                result.append(&mut vec![0; 20]);
            }
            if pixel & 64 == 64 {
                result.append(&mut vec![PIXEL_COLOR; 20]);
            } else {
                result.append(&mut vec![0; 20]);
            }
            if pixel & 32 == 32 {
                result.append(&mut vec![PIXEL_COLOR; 20]);
            } else {
                result.append(&mut vec![0; 20]);
            }

            if pixel & 16 == 16 {
                result.append(&mut vec![PIXEL_COLOR; 20]);
            } else {
                result.append(&mut vec![0; 20]);
            }

            if pixel & 8 == 8 {
                result.append(&mut vec![PIXEL_COLOR; 20]);
            } else {
                result.append(&mut vec![0; 20]);
            }

            if pixel & 4 == 4 {
                result.append(&mut vec![PIXEL_COLOR; 20]);
            } else {
                result.append(&mut vec![0; 20]);
            }
            if pixel & 2 == 2 {
                result.append(&mut vec![PIXEL_COLOR; 20]);
            } else {
                result.append(&mut vec![0; 20]);
            }
            if pixel & 1 == 1 {
                result.append(&mut vec![PIXEL_COLOR; 20]);
            } else {
                result.append(&mut vec![0; 20]);
            }
            if (i + 1) % 8 == 0 {
                results.append(&mut vec![result; 20].concat());
                result = Vec::new();
            }
        }
        results
    }
    fn debug_print_frame_buffer(&self) {
        println!("frame_buffer");
        for (i, pixel_col) in self.frame_buffer.iter().enumerate() {
            print!("{:2}: ", i);
            println!("{:08b}", pixel_col);
        }
    }
}
