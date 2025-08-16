use core::panic;
use std::env;
use std::fs::File;
use std::io::Read;

use crate::chip8error::Chip8Error;

const EMPTY_MEMORY: [u8; 4096] = [0; 4096];
const EMPTY_REGISTER: [u8; 16] = [0; 16];
const EMPTY_STACK: [u16; 16] = [0; 16];

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
    // quirk that lets sprites drawn at the edge wrap to other side (or clip if false)
    is_wrap_draw: bool,
    // quirk that modifies vx in place and ignores vy for <<= and >>= 0x8XY6 & ..E
    is_mod_vx_in_place: bool,
}

impl Chip8Sys {
    // Creates a new Chip8Sys with default settings
    pub fn new_set_quirks(
        is_inc_index: bool,
        is_register_f_reset: bool,
        is_wrap_draw: bool,
        is_mod_vx_in_place: bool,
    ) -> Chip8Sys {
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
            is_mod_vx_in_place,
        };
        // load the font in memeory
        for i in FONT_RANGE_MIN..FONT_RANGE_MAX {
            new_chip_8_sys.memory[i as usize] = FONT[i as usize - FONT_RANGE_MIN as usize];
        }
        new_chip_8_sys
    }
    // sets up a new chip 8 with default quirks for the chip 8 system
    pub fn new_chip_8() -> Chip8Sys {
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
            is_inc_index: true,
            is_register_f_reset: true,
            is_wrap_draw: false,
            is_mod_vx_in_place: false,
        };
        // load the font in memeory
        for i in FONT_RANGE_MIN..FONT_RANGE_MAX {
            new_chip_8_sys.memory[i as usize] = FONT[i as usize - FONT_RANGE_MIN as usize];
        }
        new_chip_8_sys
    }
}

impl Chip8Sys {
    pub fn check_waiting(&mut self) -> bool {
        match self.wait_for_key_press {
            Some(r) => match r {
                0..0xF => {
                    for (n, p) in self.keys.iter().enumerate() {
                        if *p {
                            self.register[r as usize] = n as u8;
                            return false;
                        }
                    }
                }
                // We should never get here because wait_for_key_press is private and only set
                // by the 0xFX0A op code.
                r => panic!(
                    "Register {:02X} does not exist. It should be in 0..0xF and only set by 0xFX0A op code.",
                    r
                ),
            },
            None => return false,
        }
        true
    }
    pub fn wait(&mut self, register: u8) -> Result<(), Chip8Error> {
        if register > 0xF {
            return Err(Chip8Error::InvalidWaitRegister(register));
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
    pub fn is_mod_vx_in_place(&self) -> bool {
        self.is_mod_vx_in_place
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
        let path = env::current_dir().unwrap();
        println!("Path is: {}", path.display());
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
                // println!("Rom to long reading stopped");
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
        let new_chip_8_sys = Chip8Sys::new_chip_8();
        assert_eq!(
            new_chip_8_sys.memory[(FONT_RANGE_MIN as usize)..(FONT_RANGE_MAX as usize)],
            FONT
        );
    }

    #[test]
    // Test that the lowest number key pressed is stored
    fn test_wait_for_key_press_wait_access() {
        // send clear screen to make sure that wait doesn't change
        let mut chip8 = crate::decode::test::single_instruction_chip_8(0x00E0);
        let _ = chip8.run().unwrap();
        assert_eq!(
            chip8.wait_for_key_press, None,
            "Chip-8 wait_for_key_press should not have been set to anything."
        );
    }

    #[test]
    #[ignore]
    // Tests whether or not the Corax+ test rom passes
    fn run_corax_plus_test_rom() {
        let mut chip8 = Chip8Sys::new_chip_8();
        let file_path = "../roms/3-corax+.ch8";
        chip8.load_rom(String::from(file_path));

        let pass_fb: [u8; 256] = [
            0, 0, 0, 0, 0, 0, 0, 0, 58, 128, 58, 128, 58, 128, 59, 128, 25, 20, 9, 20, 59, 148, 35,
            20, 10, 152, 50, 152, 40, 152, 48, 152, 58, 144, 58, 144, 56, 144, 35, 16, 0, 0, 0, 0,
            0, 0, 0, 0, 42, 128, 59, 128, 59, 128, 59, 128, 57, 20, 43, 20, 59, 20, 33, 148, 10,
            152, 42, 24, 40, 152, 48, 152, 10, 144, 59, 144, 59, 16, 35, 144, 0, 0, 0, 0, 0, 0, 0,
            0, 58, 128, 59, 128, 59, 128, 59, 128, 49, 20, 58, 148, 56, 148, 35, 20, 10, 152, 42,
            152, 41, 24, 50, 24, 50, 144, 59, 144, 57, 16, 35, 144, 0, 0, 0, 0, 0, 0, 0, 0, 58,
            128, 59, 0, 57, 128, 2, 128, 9, 20, 57, 20, 58, 20, 41, 20, 18, 152, 41, 24, 43, 152,
            42, 152, 18, 144, 59, 144, 59, 144, 18, 144, 0, 0, 0, 0, 0, 0, 0, 0, 58, 128, 59, 128,
            59, 128, 0, 0, 57, 20, 56, 148, 59, 20, 0, 0, 10, 152, 43, 24, 42, 24, 0, 0, 50, 144,
            59, 144, 59, 144, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 50, 128, 59, 128, 57, 128, 2, 142, 17,
            20, 57, 148, 34, 20, 43, 130, 18, 152, 40, 152, 51, 152, 40, 140, 58, 144, 59, 144, 35,
            144, 16, 174, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        for _ in 0..500 {
            let _ = chip8.run().unwrap();
        }
        assert_eq!(
            pass_fb, chip8.frame_buffer,
            "Frame Buffer should match the one from a passing run after 140 cycles."
        );
    }
    #[test]
    #[ignore]
    fn run_flags_test_rom() {
        let mut chip8 = Chip8Sys::new_chip_8();
        let file_path = "../roms/4-flags.ch8";
        chip8.load_rom(String::from(file_path));
        chip8.memory[0x1FF] = 1;

        // This has "disp. wait slow x" in the output
        // Needs to be updated if/when I fix that.
        let pass_fb: [u8; 256] = [
            164, 204, 163, 0, 0, 14, 0, 0, 234, 170, 161, 21, 84, 2, 85, 80, 174, 204, 65, 25, 152,
            12, 102, 96, 170, 136, 67, 145, 16, 14, 68, 64, 0, 0, 0, 0, 0, 0, 0, 0, 224, 0, 2, 128,
            0, 14, 0, 0, 101, 85, 3, 149, 85, 76, 85, 85, 38, 102, 0, 153, 153, 130, 102, 102, 228,
            68, 0, 145, 17, 12, 68, 68, 0, 0, 0, 0, 0, 0, 0, 0, 224, 0, 3, 128, 0, 14, 0, 0, 133,
            85, 0, 149, 85, 76, 85, 80, 230, 102, 0, 153, 153, 136, 102, 96, 228, 68, 0, 145, 17,
            14, 68, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 228, 204, 162, 128, 0, 14,
            0, 0, 138, 170, 163, 149, 85, 76, 85, 85, 142, 204, 64, 153, 153, 130, 102, 102, 234,
            170, 64, 145, 17, 12, 68, 68, 0, 0, 0, 0, 0, 0, 0, 0, 224, 0, 3, 128, 0, 14, 0, 0, 133,
            85, 0, 149, 85, 76, 85, 80, 230, 102, 0, 153, 153, 136, 102, 96, 228, 68, 0, 145, 17,
            14, 68, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 238, 174, 195, 184, 0, 0,
            2, 142, 164, 236, 162, 49, 84, 0, 43, 130, 164, 168, 195, 33, 152, 0, 40, 140, 228,
            174, 162, 57, 16, 0, 16, 174, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        for _ in 0..1100 {
            let _ = chip8.run().unwrap();
        }
        assert_eq!(
            pass_fb, chip8.frame_buffer,
            "Frame Buffer should match the one from a passing run after 5,000 cycles."
        );
    }
    #[test]
    #[ignore]
    fn run_quirks_test_rom() {
        let mut chip8 = Chip8Sys::new_chip_8();
        let file_path = "../roms/5-quirks.ch8";
        chip8.load_rom(String::from(file_path));
        chip8.memory[0x1FF] = 1;

        // This has "disp. wait slow x" in the output
        // Needs to be updated if/when I fix that.
        let pass_fb: [u8; 256] = [
            0, 0, 0, 0, 0, 0, 0, 0, 87, 6, 115, 119, 0, 59, 0, 0, 84, 5, 102, 98, 0, 42, 128, 20,
            86, 6, 65, 66, 0, 42, 128, 24, 36, 5, 118, 114, 0, 58, 128, 16, 0, 0, 0, 0, 0, 0, 0, 0,
            119, 119, 101, 0, 0, 59, 0, 0, 118, 117, 85, 0, 0, 42, 128, 20, 84, 85, 98, 0, 0, 42,
            128, 24, 87, 87, 82, 0, 0, 58, 128, 16, 0, 0, 0, 0, 0, 0, 0, 0, 103, 54, 5, 39, 112,
            26, 58, 128, 82, 101, 5, 82, 32, 50, 42, 148, 82, 22, 7, 114, 32, 10, 43, 136, 103,
            100, 39, 87, 32, 51, 187, 148, 0, 0, 0, 0, 0, 0, 0, 0, 116, 118, 103, 99, 0, 59, 0, 0,
            68, 37, 82, 84, 0, 42, 128, 20, 68, 38, 98, 85, 0, 42, 128, 24, 119, 116, 71, 83, 0,
            58, 128, 16, 0, 0, 0, 0, 0, 0, 0, 0, 53, 119, 119, 99, 0, 59, 184, 0, 103, 36, 34, 84,
            0, 42, 32, 20, 21, 38, 34, 85, 0, 43, 48, 24, 101, 116, 39, 83, 0, 58, 32, 16, 0, 0, 0,
            0, 0, 0, 0, 0, 53, 118, 118, 48, 0, 59, 184, 0, 21, 117, 37, 64, 0, 42, 32, 20, 21, 86,
            37, 80, 0, 43, 48, 24, 99, 84, 117, 48, 0, 58, 32, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ];

        for _ in 0..5000 {
            let _ = chip8.run().unwrap();
        }
        assert_eq!(
            pass_fb, chip8.frame_buffer,
            "Frame Buffer should match the one from a passing run after 5,000 cycles."
        );
    }
}
