use crate::chip8::Chip8Sys;
// extending Chip8Sys to load roms
impl Chip8Sys {
    /*
    pub fn new() -> Self {
        Roms {
            memory: [0; 4096 - 0x0A0],
        }
    }*/
    pub fn load_dxyn_rom_simple(&mut self) -> &mut Self {
        // clear screen
        self.memory[0x200] = 0x00;
        self.memory[0x201] = 0xE0;
        // load register V0 with x position
        self.memory[0x202] = 0x60;
        self.memory[0x203] = 0x00;

        // load register V1 with y position
        self.memory[0x204] = 0x61;
        self.memory[0x205] = 0x05;
        // load register I with sprite location
        self.memory[0x206] = 0xA0;
        self.memory[0x207] = 0x9B; // F Sprite is at 0x09B

        // draw sprite @I in position V0 and V1
        self.memory[0x208] = 0xD1;
        self.memory[0x209] = 0x15; // the default sprites are 5 px tall
        self
    }
    pub fn load_dxyn_rom_adv(&mut self) -> &mut Self {
        // clear screen
        self.memory[0x200] = 0x00;
        self.memory[0x201] = 0xE0;
        // load register V0 with x position
        self.memory[0x202] = 0x60;
        self.memory[0x203] = 0x00;
        // load register V1 with y position
        self.memory[0x204] = 0x61;
        self.memory[0x205] = 0x05;
        // load register I with sprite location
        self.memory[0x206] = 0xA0;
        self.memory[0x207] = 0x9B; // F Sprite is at 0x09B

        // draw sprite at I at position V0 and V1
        self.memory[0x208] = 0xD0;
        self.memory[0x209] = 0x15; // the default sprites are 5 px tall
                                   // */
                                   // update reg[0]'s location to move 10 X
        self.memory[0x20A] = 0x70;
        self.memory[0x20B] = 0x0A;
        // draw F again at new I location
        self.memory[0x20C] = 0xD0;
        self.memory[0x20D] = 0x15;
        /*
        // try to draw 0 to 2 vertically at (0, 15)
        // load V2 with x position (0)
        self.memory[0x20E] = 0x62;
        self.memory[0x20F] = 0x00;
        // load V3 with y position 0x0F
        self.memory[0x210] = 0x63;
        self.memory[0x211] = 0x00;
        // load register I with sprite location
        self.memory[0x212] = 0xA0;
        self.memory[0x213] = 0x50; // 0 Sprite is at 0x050

        // draw something 0xF (15) lines tall
        self.memory[0x214] = 0xD2;
        self.memory[0x215] = 0x3F;
        // */
        // reset Y to be next to the first F
        self.memory[0x216] = 0x60;
        self.memory[0x217] = 0x00;

        // Add 5 to X
        self.memory[0x218] = 0x70;
        self.memory[0x219] = 0x05;

        // load E sprite's location into Register I
        self.memory[0x21A] = 0xA0;
        self.memory[0x21B] = 0x96; // E sprite location: 0x96

        // draw sprite at I at position V0 and V1
        self.memory[0x21C] = 0xD0;
        self.memory[0x21D] = 0x15; // the default sprites are 5 px tall

        // update reg[1]'s location to move 5 in Y direction
        self.memory[0x21E] = 0x70;
        self.memory[0x21F] = 0x05;

        // draw sprite at I at position V0 and V1
        self.memory[0x220] = 0xD0;
        self.memory[0x221] = 0x15; // the default sprites are 5 px tall

        // jump to beginning memory
        self.memory[0x230] = 0x12;
        self.memory[0x231] = 0x00;
        self
    }
    // TODO: Delete this
    // I'm intentionally not refactoring this because I don't want to use it by accident. I'll
    // delete it after it's checked in for posterity.
    fn load_flashing_rom(self: &mut Chip8Sys) {
        panic!("This is based on a non functional DXYN");
        // This is based dxyn doing nothing
        // This also uses a fake command 0x2000 which fills the screen
        // clear screen
        self.memory[0x200] = 0x00;
        self.memory[0x201] = 0xE0;
        // draw whole screen
        self.memory[0x202] = 0xD1;
        self.memory[0x203] = 0x11;
        // fill screen
        self.memory[0x204] = 0x20;
        self.memory[0x205] = 0x00;
        // draw whole screen
        self.memory[0x206] = 0xDF;
        self.memory[0x207] = 0xFF;
        // jump to random ending memory
        self.memory[0x208] = 0x1F;
        self.memory[0x209] = 0x12;
        // set register 2 to 0xF8
        self.memory[0xF12] = 0x62;
        self.memory[0xF13] = 0xF8;
        // Add 2 to Reg 2
        self.memory[0xF14] = 0x72;
        self.memory[0xF15] = 0x02;
        // Set reg I to F1F
        self.memory[0xF16] = 0xAF;
        self.memory[0xF17] = 0x1F;
        // jump back to start
        self.memory[0xF18] = 0x12;
        self.memory[0xF19] = 0x00;
    }
}
