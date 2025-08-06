use crate::{INC_INDEX, VF_RESET, WRAP_DRAW};
use std::fs::File;
use std::io::Read;

const EMPTY_MEMORY: [u8; 4096] = [0; 4096];
const EMPTY_REGISTER: [u8; 16] = [0; 16];
const EMPTY_STACK: [u16; 16] = [0; 16];
const PIXEL_COLOR: u32 = 0x0000FF88;

// This is the built in Chip-8 font that Roms expect to access
const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0 loc 0x050
    0x20, 0x60, 0x20, 0x20, 0x70, // 1 loc 0x055
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2 loc 0x05A
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3 loc 0x05F
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4 loc 0x064
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5 loc 0x069
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6 loc 0x06E
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7 loc 0x073
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8 loc 0x078
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9 loc 0x07D
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A loc 0x082
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B loc 0x087
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C loc 0x08C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D loc 0x091
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E loc 0x096
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F loc 0x09B
];
const FONT_RANGE_MIN: u8 = 0x050;
const FONT_RANGE_MAX: u8 = 0x0A0;

pub struct Chip8Sys {
    pub memory: [u8; 4096],
    pub register: [u8; 16],
    pub register_i: u16,
    pub delay_timer: u8, // Will be used eventually
    pub sound_timer: u8, // Will be used eventually
    pub program_counter: u16,
    pub stack_pointer: u8, // Will be used eventually
    pub stack: [u16; 16],  // Will be used eventually
    pub frame_buffer: [u8; 256],
    // NOTE: The wait for key press code is dependent on the length of keys <= registers
    pub keys: [bool; 16], // represents the 16 keys of Chip-8. true = pressed
    wait_for_key_press: Option<u8>, // for instruction 0xFXA0
    pub is_playing_sound: bool,
    // handles if FX55 & FX65 increment I index register
    is_inc_index: bool,
    // quirk that resets reg[0xF] to 0 when AND, OR, and XOR are set (0x8XY1-3)
    is_register_f_reset: bool,
    is_wrap_draw: bool,
}

impl Chip8Sys {
    // Creates a new Chip8Sys with default settings
    pub fn new(is_inc_index: bool, is_register_f_reset: bool, is_wrap_draw: bool) -> Chip8Sys {
        let mut new_chip_8_sys = Chip8Sys {
            memory: EMPTY_MEMORY,
            register: EMPTY_REGISTER,
            register_i: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0x200, // initialize PC to start reading at 0x200
            stack_pointer: 0,
            stack: EMPTY_STACK,
            frame_buffer: [0x00; 256],
            keys: [false; 16],
            wait_for_key_press: None,
            is_playing_sound: false,
            is_inc_index,
            is_register_f_reset,
            is_wrap_draw,
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
    pub fn check_waiting(&self) -> bool {
        self.wait_for_key_press != None
    }
    pub fn get_key_press_reg(&self) -> Result<u8, &str> {
        match self.wait_for_key_press {
            Some(register) => match register {
                0..=0xF => Ok(register),
                // NOTE: This should never get hit because we're storing
                // register with Chip8Sys::wait()
                _ => panic!("Stored Register is bigger than 0xF (15)"),
            },
            None => Err("Wasn't waiting for key press"),
        }
    }
    pub fn wait(&mut self, register: u8) -> Result<(), &str> {
        if register > 0xF {
            return Err("Must store value less than 0xF (15)");
        }
        self.wait_for_key_press = Some(register);
        Ok(())
    }
    pub fn is_inc_index(&self) -> bool {
        self.is_inc_index
    }
    pub fn is_register_f_reset(&self) -> bool {
        self.is_register_f_reset
    }
    pub fn is_wrap_draw(&self) -> bool {
        self.is_wrap_draw
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
    pub fn load_rom(&mut self, file_path: String) -> &mut Self {
        let mut file = File::open(file_path).expect("should have been able to open the file");
        let mut rom = [0; 0x1000];
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
            if i + 0x200 > self.memory.len() - 1 {
                println!("Rom to long reading stopped");
                break;
            }
            self.memory[0x200 + i] = byte.to_owned();
        }
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // Tests to make sure that we can create a new Chip8Sys with the font in the right place;
    fn create_new_chip_8_sys() {
        let new_chip_8_sys = Chip8Sys::new(INC_INDEX, VF_RESET, WRAP_DRAW);
        assert_eq!(
            new_chip_8_sys.memory[(FONT_RANGE_MIN as usize)..(FONT_RANGE_MAX as usize)],
            FONT
        );
    }
}
#[test]
// Test that the lowest number key pressed is stored
fn test_wait_for_key_press_wait_access() {
    let reg_x = 0x2;
    // send clear screen to make sure that wait doesn't change
    let mut chip8 = crate::decode::test::single_instruction_chip_8(0x00E0);
    chip8.run();
    assert_eq!(
        chip8.wait_for_key_press, None,
        "Chip-8 wait_for_key_press should not have been set to anything."
    );
}
