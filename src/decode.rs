use crate::{bitwise::Bitwise, chip8::Chip8Sys};

impl Chip8Sys {
    // This will run the next command in program_counter is pointing to in Chip8Sys.memory
    pub fn run(&mut self) {
        // println!();
        // println!("PC: {:x}", self.program_counter);
        let instruction = self.memory[self.program_counter as usize];
        // println!("instruction 1: {:x}", instruction);
        let a: u8 = (0xF0 & instruction) >> 0x4;
        let b: u8 = 0x0F & instruction;
        let instruction = self.memory[(self.program_counter + 1) as usize];
        // println!("instruction 2: {:x}", instruction);
        let c: u8 = (0xF0 & instruction) >> 0x4;
        let d: u8 = 0x0F & instruction;
        self.program_counter += 2;
        // Prints debug what instruction values I'm sending in
        /*
        println!("a: {:x}", a);
        println!("b: {:x}", b);
        println!("c: {:x}", c);
        println!("d: {:x}", d);
        println!("PC inc: {:x}", self.program_counter);
        // */
        // Implement the Instructions for the Chip-8
        match a {
            0x0 => {
                println!("Hit 0x0");
                match instruction {
                    0x00E0 => self.frame_buffer = [0x00; 256],
                    0x00EE => return, // TODO: Return from Subroutine
                    _ => return,      // TODO: SYS Addr
                }
            }
            0x1 => {
                println!("Hit 0x1 - Jump");
                self.program_counter = Chip8Sys::nnn(b, c, d);
            }
            0x2 => {
                // this is not the correct instruction to execute here.
                println!("Hit 0x2 - Fill Screen");
                self.frame_buffer = [0xAA; 256];
            }
            0x3 => println!("Hit 0x3"),
            0x4 => println!("Hit 0x4"),
            0x5 => println!("Hit 0x5"),
            0x6 => {
                println!("Hit 0x6 - Load VX with NN");
                self.register[b as usize] = Chip8Sys::nn(c, d);
                println!("register[{:02X}] = {:02X}", b, self.register[b as usize]);
            }
            0x7 => {
                println!("Hit 0x7");
                self.register[b as usize] += Chip8Sys::nn(c, d);
                println!(
                    "register[{:X}] + {:02X} = {:02X}",
                    b,
                    Chip8Sys::nn(c, d),
                    self.register[b as usize]
                );
            }
            0x8 => println!("Hit 0x8"),
            0x9 => println!("Hit 0x9"),
            0xA => {
                println!("Hit 0xA");
                self.register_i = Chip8Sys::nnn(b, c, d);
                println!("reg I = {:02X}", self.register_i);
            }
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
    // Helper function to handle the Draw command logic 0xDXYN
    // TODO: Expand this to handle printing at the edge correctly. Right now it wraps.
    fn draw(&mut self, x: u8, y: u8, n: u8) {
        // Prints debug values being sent to DXYN
        // println!("x: {x} y:{y} n:{n}");

        // Get X & Y Cordinates from register[X] and register[Y]
        // This requires me to convert u8 to bits
        // First figure out which u8 they're in
        let x_loc = (self.register[x as usize] as f32 / 8.).floor() as u8;
        let mut y_loc = ((self.register[y as usize] as u32 * 64) as f32 / 8.).floor() as u8;
        // Then figure out which bit of the u8 is being referenced
        let x_bit = (self.register[x as usize] % 8) as usize;
        // let y_bit = self.register[y as usize] % 8;

        // Get starting memory location of register_i
        // This is where the rom will store the sprite it wants drawn
        let mut starting_loc = self.register_i as usize;
        // Prints debug frame buffer values as bits
        /*
        for (i, px) in self.frame_buffer.iter().enumerate() {
            print!("{:08b}", px);
            print!("_");
            if i % 64 == 0 {
                println!("");
            }
        }
        println!();
        */

        // read memory n times to get the full sized sprite
        for _ in 0..n {
            let sprite_pxs = self.memory[starting_loc].bit_vec();
            // Prints debug sprite pixel info and x,y location including bits
            /*
            print_vec(&sprite_pxs, "sprite_pxs");
            println!("x_loc: {x_loc} x_bit: {x_bit}");
            println!("y_loc: {y_loc} y_bit: {y_bit}");
            // */

            // Need to get the u8 at x_loc y_loc
            let mut fb_start = self.frame_buffer[(x_loc + y_loc) as usize].bit_vec();
            // In the case I start in the middle of a u8 in frame_buffer I get the overflow u8
            // I shouldn't ever get more than 2 u8s because sprites can only be 8 px wide
            let mut fb_overflow = self.frame_buffer[(x_loc + y_loc + 1) as usize].bit_vec();
            fb_start.append(&mut fb_overflow);

            // I need to build a result vec.
            // It's going to be filled with what's already on the screen from 0 to the starting x
            // positon
            let mut result_vec = fb_start[..x_bit as usize].to_vec();
            // then I'll add the sprite data to the vec
            for (loc, b) in fb_start[x_bit as usize..x_bit as usize + 8]
                .iter()
                .enumerate()
            {
                // XOR the value on the screen with the value that should be written to the screen
                let xor_res = b ^ sprite_pxs[loc];
                // in the event of turning off a pixel update reg[F] aka VF
                if (!(b & xor_res) & b) != false {
                    self.register[0xF] = 1;
                    // println!("Register F is: {}", self.register[0xF]);
                }
                result_vec.push(xor_res);
            }
            // finally I'll add the leftover bits that were already on the screen and I don't want
            // affected by the sprite drawing
            result_vec.append(&mut fb_start[x_bit as usize + 8..].to_vec());

            // TODO: Fix the expect here
            self.frame_buffer[(x_loc + y_loc) as usize] =
                u8::from_bit_vec(result_vec[..8].to_vec())
                    .expect("provided vector should be correct number of bits long.");
            self.frame_buffer[(x_loc + y_loc + 1) as usize] =
                u8::from_bit_vec(result_vec[8..].to_vec())
                    .expect("provided vector should be correct number of bits long.");

            // increment y by 8 bytes (64 bits) to get to the next row
            y_loc += 8;
            // also increment the memory location we're reading to find the next row of the sprite
            starting_loc += 1;
        }
    }
    // helper function to get the last 3 nibbles of a command
    // commands coming in as 0x?NNN will use this
    fn nnn(b: u8, c: u8, d: u8) -> u16 {
        (b as u16) << 8 | (c << 4 | d) as u16
    }
    // helper function to get the last 2 nibbles of a command
    // commands coming in as 0x??NN will use this
    fn nn(c: u8, d: u8) -> u8 {
        c << 4 | d
    }
}

// TODO: Remove this. It's a temporary
// helper function to print a bool vector
/*
fn print_vec(v: &Vec<bool>, vec_name: &str) {
    print!("{vec_name}: ");
    for b in v.iter() {
        print!("{}", (b == &true) as u8);
    }
    println!("");
}
// */

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // Test that the nibbles going to nn() build a byte of NN
    fn test_nn() {
        assert_eq!(0x45, Chip8Sys::nn(0x4, 0x5));
    }
    #[test]
    // Test that the nibbles going to nnn() build a byte of NNN
    fn test_nnn() {
        assert_eq!(0x456, Chip8Sys::nnn(0x4, 0x5, 0x6));
    }

    // NOTE: Section where I test all the Chip-8 instructions

    #[test]
    // Tests clear screen; 0x00E0
    fn test_clear_screen() {
        let mut chip8 = single_instruction_chip_8(0x00E0);
        chip8.frame_buffer = [0xAA; 256];
        chip8.run();
        assert_eq!([0x00; 256], chip8.frame_buffer);
    }

    #[test]
    // Tests Jump to memory location NNN; 0x1NNN
    fn test_jump() {
        let mut chip8 = single_instruction_chip_8(0x1556);
        chip8.run();
        assert_eq!(chip8.program_counter, 0x556);
    }

    #[test]
    // Tests Load Register X with NN; 0x6XNN
    fn test_load_register() {
        // set register 0xA to be 0x88
        let mut chip8 = single_instruction_chip_8(0x6A88);
        chip8.run();
        assert_eq!(0x88, chip8.register[0xA]);
    }

    #[test]
    // Tests Add Value NN to Register X; 0x7XNN
    fn test_add_register() {
        // add 0x0A to Register A
        let mut chip8 = single_instruction_chip_8(0x7A0B);
        // directly access the register for testing purposes
        println!("sum: {:02X}", 0x04 + 0x0B);
        chip8.register[0xA] = 0x04;
        chip8.run();
        // 0x0B + 0x04 = 0x10
        assert_eq!(0x0F, chip8.register[0xA]);
    }

    #[test]
    // Tests Setting Index Register I to NNN; 0xANNN
    fn test_set_register_i() {
        // Set register I to 0x9A9
        let mut chip8 = single_instruction_chip_8(0xA9A9);
        chip8.run();
        assert_eq!(chip8.register_i, 0x9A9);
    }
    #[test]
    // Tests Draw sprite in reg_i that's N pixels tall
    // in the frame_buffer at location stored in register X, and register Y;
    // 0xDXYN
    fn test_draw() {
        let mut chip8 = single_instruction_chip_8(0xD125);
        // load register 1 with X location
        chip8.register[0x1] = 0xF;
        // load register 2 with Y location
        chip8.register[0x2] = 0x8;
        // set register I to reference the sprite for 0 in memory 0x050
        chip8.register_i = 0x050;
        chip8.run();
        // make the expected frame empty
        let mut expected_frame_buffer = [0; 256];
        // manually load the 0 sprite into the right spots
        expected_frame_buffer[65] = 0b0000_0001;
        expected_frame_buffer[66] = 0b1110_0000;
        expected_frame_buffer[72] = 0b0000_0001;
        expected_frame_buffer[73] = 0b0010_0000;
        expected_frame_buffer[80] = 0b0000_0001;
        expected_frame_buffer[81] = 0b0010_0000;
        expected_frame_buffer[88] = 0b0000_0001;
        expected_frame_buffer[89] = 0b0010_0000;
        expected_frame_buffer[96] = 0b0000_0001;
        expected_frame_buffer[97] = 0b1110_0000;

        assert_eq!(chip8.frame_buffer, expected_frame_buffer);
    }
    // NOTE: This is the format I find myself using for these tests
    // with other code sprinkled in
    #[test]
    // Tests TEMPLATE
    fn test_chip8_command() {
        let mut chip8 = single_instruction_chip_8(0x0000);
        chip8.run();
        assert_eq!(1, 1);
    }

    // NOTE: Helper functions for testing
    // Helper function to build a Chip8Sys easily with 1 instruction at 200
    fn single_instruction_chip_8(instruction: u16) -> Chip8Sys {
        let mut chip8 = Chip8Sys::new();
        chip8.memory[0x200] = ((instruction & 0xFF00) >> 8) as u8;
        chip8.memory[0x201] = (instruction & 0xFF) as u8;
        chip8
    }
}
