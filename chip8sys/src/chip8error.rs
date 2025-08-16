#[derive(Debug)]
pub enum Chip8Error {
    InvalidFirstByte(u8), // if the N of 0xN___ is invalid it will return this and the N provided
    InvalidRegisterX(u8), // If the X register should be <= 0xF
    Invalid0x8XYN(u8),    // if the N in 0x8XYN is invalid it will return this and the N provided
    Invalid0xENNN(u8, u8), // if the N in 0x8XYN is invalid it will return this and the N provided
    Invalid0xFNNN(u8, u8), // if the N in 0x8XYN is invalid it will return this and the N provided
    InvalidWaitRegister(u8), // If the register we're waiting for is somehow > 0xF
}
