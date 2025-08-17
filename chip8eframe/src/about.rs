pub struct About {}
impl About {
    pub fn about() -> String {
        "The Chip-8 is an emulator. This one is written in rust and presented with egui."
            .to_string()
    }
}

impl About {
    // TODO: move this to the core library so it can take into account current quirks
    pub fn chip_8_decode(ab: u8, cd: u8) -> String {
        // Decodes the chip 8 instruction into a human readable format
        let a = (ab & 0xF0) >> 4;
        let b = ab & 0xF;
        let c = (cd & 0xF0) >> 4;
        let d = cd & 0xF;

        match a {
            0 => match cd {
                0xE0 => String::from("Clear Screen"),
                0xEE => String::from("Return from Subroutine"),
                _ => String::from("Unknown Command"),
            },
            1 => String::from(format!("Jump to address 0x{:X}{:02X}", b, cd)),
            2 => String::from(format!(
                "Execute subroutine at address: 0x{:X}{:02X}",
                b, cd
            )),
            3 => String::from(format!(
                "Skip next instruction if register {:X} is 0x{:02X}",
                b, cd
            )),
            4 => String::from(format!(
                "Skip next instruction if register {:X} is not 0x{:02X}",
                b, cd
            )),
            5 => String::from(format!(
                "Skip next instruction if register {:X} is equal to register {:X}",
                b, c
            )),
            6 => String::from(format!("Store value 0x{:02X} into register {:X}", cd, b)),
            7 => String::from(format!("Add value 0x{:02X} to register{:X}", cd, b)),
            8 => match d {
                0 => String::from(format!(
                    "Store value of register {:X} into register{:X}",
                    c, b
                )),
                // TODO: Take into account quirks
                1 => String::from(format!(
                    "Set register {:X} to register {:X} OR register {:X}",
                    b, b, c
                )),
                2 => String::from(format!(
                    "Set register {:X} to register {:X} AND register {:X}",
                    b, b, c
                )),
                3 => String::from(format!(
                    "Set register {:X} to register {:X} XOR register {:X}",
                    b, b, c
                )),
                4 => String::from(format!(
                    "Set register {:X} to register {:X} PLUS register {:X}",
                    b, b, c
                )),
                5 => String::from(format!(
                    "Set register {:X} to register {:X} MINUS register {:X}",
                    b, b, c
                )),
                6 => String::from(format!(
                    "Set register {:X} to register {:X} shifted RIGHT one bit",
                    b, c
                )),
                7 => String::from(format!(
                    "Set register {:X} to register {:X} MINUS register {:X}",
                    b, c, b
                )),
                0xE => String::from(format!(
                    "Set register {:X} to register {:X} shifted LEFT one bit",
                    b, c
                )),
                _ => String::from("Unknown Command"),
            },
            9 => String::from(format!(
                "Skip next instruction if register {:X} is not equal to register {:X}",
                b, c
            )),
            0xA => String::from(format!(
                "Store memory address 0x{:X}{:02X} into Register I",
                b, cd
            )),
            0xB => String::from(format!(
                "Jump to address 0x{:X}{:02X} PLUS register 0",
                b, cd
            )),
            0xC => String::from(format!(
                "Set register {:X} to a random number masked by {:02X}",
                b, cd
            )),
            0xD => String::from(format!(
                "Draw sprite located in memory at register I at X location in register {:X} and Y location in register {:X} and make it 0x{:X} lines tall",
                b, c, d
            )),
            0xE => match cd {
                0x9E => String::from(format!(
                    "Skip next instruction if key stored in register {:X} is pressed.",
                    b
                )),
                0xA1 => String::from(format!(
                    "Skip next instruction if key stored in register {:X} is not pressed.",
                    b
                )),
                _ => String::from("Unknown Command"),
            },
            0xF => match cd {
                0x07 => String::from(format!(
                    "Store the current value of the delay timer into register {:X}",
                    b
                )),
                0x0A => String::from(format!(
                    "Wait for key press and store it in register {:X}",
                    b
                )),
                0x15 => String::from(format!("Set delay timer to value in register {:X}", b)),
                0x18 => String::from(format!("Set sound timer to value in register {:X}", b)),
                0x1E => String::from(format!("Add value in register {:X} to Register I", b)),
                0x29 => String::from(format!(
                    "Set Register I to the memory address of the sprite data corresponding to the hexadecimal digit stored in register {:X}",
                    b
                )),
                0x33 => String::from(format!(
                    "Store the binary coded decimal equivalent of value in register {:X} to register I, register I+1, and register I+2",
                    b
                )),
                0x55 => String::from(format!(
                    "Store regsiter 0 to register {:X} to memory starting at location stored in register I.",
                    b
                )),
                0x65 => String::from(format!(
                    "Retrieve regsiter 0 to register {:X} to memory starting at location stored in register I.",
                    b
                )),
                _ => String::from("Unknown Command"),
            },
            _ => String::from("Unknown Command"),
        }
    }
}
