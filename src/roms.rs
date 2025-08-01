use crate::chip8::Chip8Sys;
// extending Chip8Sys to load roms
impl Chip8Sys {
    // A simple rom that prints the F sprite on the screen at (0,0)
    /*
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
    // */
    // A more advanced rom that prints F E F at (0,5)
    // it then and then prints E over the 2nd F
    // to test XOR functionality and VF flag
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
}
