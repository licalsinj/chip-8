use crate::{chip8::Nibble, Chip8Sys};

impl Chip8Sys {
    pub fn run(&mut self) {
        // fetch the program counter's instruction, parse it, and increment it
        println!();
        println!("PC: {:x}", self.program_counter);
        let instruction = self.memory[self.program_counter as usize];
        println!("instruction 1: {:x}", instruction);
        let a: u8 = (0xF0 & instruction) >> 0x4;
        let b: u8 = 0x0F & instruction;
        let instruction = self.memory[(self.program_counter + 1) as usize];
        println!("instruction 2: {:x}", instruction);
        let c: u8 = (0xF0 & instruction) >> 0x4;
        let d: u8 = 0x0F & instruction;
        self.program_counter += 2;
        println!("a: {:x}", a);
        println!("b: {:x}", b);
        println!("c: {:x}", c);
        println!("d: {:x}", d);
        println!("PC inc: {:x}", self.program_counter);
        // Implement the Instructions for the Chip-8
        match a {
            0x0 => {
                println!("Hit 0x0 - Clear Screen");
                self.frame_buffer = [[false; 64]; 32];
            }
            0x1 => {
                println!("Hit 0x1 - Jump");
                self.program_counter = 0x200;
            }
            0x2 => {
                println!("Hit 0x2 - Fill Screen");
                self.frame_buffer = [[true; 64]; 32];
            }
            0x3 => println!("Hit 0x3"),
            0x4 => println!("Hit 0x4"),
            0x5 => println!("Hit 0x5"),
            0x6 => println!("Hit 0x6"),
            0x7 => println!("Hit 0x7"),
            0x8 => println!("Hit 0x8"),
            0x9 => println!("Hit 0x9"),
            0xA => println!("Hit 0xA"),
            0xB => println!("Hit 0xB"),
            0xC => println!("Hit 0xC"),
            0xD => {
                println!("Hit 0xD - Draw");
                self.draw(b, c, d);
            }
            0xE => println!("Hit 0xE"),
            0xF => println!("Hit 0xF"),
            _ => return,
        }
    }
    fn draw(&self, x: u8, y: u8, n: u8) {
        println!("Hit 0xD");
        println!("x: {x}, y: {y}, n: {n}");
    }
}
