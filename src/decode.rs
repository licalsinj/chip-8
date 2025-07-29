use crate::chip8::Chip8Sys;

impl Chip8Sys {
    pub fn run(&mut self) {
        // fetch the program counter's instruction, parse it, and increment it
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
                println!("Hit 0x0 - Clear Screen");
                self.frame_buffer = [[false; 64]; 32];
            }
            0x1 => {
                println!("Hit 0x1 - Jump");
                self.program_counter = Chip8Sys::nnn(b, c, d); //(b as u16) << 8 | (c << 4 | d) as u16;
            }
            0x2 => {
                // this is not the correct instruction to execute here.
                println!("Hit 0x2 - Fill Screen");
                self.frame_buffer = [[true; 64]; 32];
            }
            0x3 => println!("Hit 0x3"),
            0x4 => println!("Hit 0x4"),
            0x5 => println!("Hit 0x5"),
            0x6 => {
                println!("Hit 0x6 - Load VX with NN");
                self.register[b as usize] = Chip8Sys::nn(c, d); // c << 4 | d;
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
    fn draw(&mut self, x: u8, y: u8, n: u8) {
        // Get X & Y Cordinates from register[X] and register[Y]
        let mut x_loc = self.register[x as usize] as usize;
        let mut y_loc = self.register[y as usize] as usize;

        // Get starting memory location of register_i
        let mut starting_loc = self.register_i as usize;

        // read memory n times to get the full sized sprite
        for index in 0..n {
            let sprite_px = self.memory[starting_loc];

            // TODO: Figure out why you're indexing things this way
            // TODO: Convert frame_buffer to be u8 not bools
            // this is a temp process to convert the frame_buffer bool to a u8
            let temp_fb = ((self.frame_buffer[x_loc][y_loc] as u8) << 7)
                + ((self.frame_buffer[x_loc][y_loc + 1] as u8) << 6)
                + ((self.frame_buffer[x_loc][y_loc + 2] as u8) << 5)
                + ((self.frame_buffer[x_loc][y_loc + 3] as u8) << 4)
                + ((self.frame_buffer[x_loc + 1][y_loc] as u8) << 3)
                + ((self.frame_buffer[x_loc + 1][y_loc + 1] as u8) << 2)
                + ((self.frame_buffer[x_loc + 1][y_loc + 2] as u8) << 2)
                + (self.frame_buffer[x_loc + 1][y_loc + 3] as u8);

            // XOR temp_fb (temp frame buffer) with the sprite data
            let temp_fb = temp_fb ^ sprite_px;

            // deconstruct temp_fb back into self.frame_buffer[x][y]
            self.frame_buffer[x_loc][y_loc] = (temp_fb & 0b1000_0000) == 0b1000_0000;
            self.frame_buffer[x_loc][y_loc + 1] = (temp_fb & 0b0100_0000) == 0b0100_0000;
            self.frame_buffer[x_loc][y_loc + 2] = (temp_fb & 0b0010_0000) == 0b0010_0000;
            self.frame_buffer[x_loc][y_loc + 3] = (temp_fb & 0b0001_0000) == 0b0001_0000;
            self.frame_buffer[x_loc + 1][y_loc] = (temp_fb & 0b0000_1000) == 0b0000_1000;
            self.frame_buffer[x_loc + 1][y_loc + 1] = (temp_fb & 0b0000_0100) == 0b0000_0100;
            self.frame_buffer[x_loc + 1][y_loc + 2] = (temp_fb & 0b0000_0010) == 0b0000_0010;
            self.frame_buffer[x_loc + 1][y_loc + 3] = (temp_fb & 0b0000_0001) == 0b0000_0001;

            // TODO: if pixels were turned "off" set the register[F] to 1 otherwise set to 0

            // increment x for the next loop and move to the next pixel location in memory
            x_loc += 1;
            starting_loc += 1;
        }
    }
    fn nnn(b: u8, c: u8, d: u8) -> u16 {
        (b as u16) << 8 | (c << 4 | d) as u16
    }
    fn nn(c: u8, d: u8) -> u8 {
        c << 4 | d
    }
}
