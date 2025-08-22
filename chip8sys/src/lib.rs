#![crate_name = "chip8sys"]
#![crate_type = "lib"]

extern crate getrandom;

pub mod chip8;
pub mod chip8error;
mod decode;
mod roms; // used for testing, may not be needed long term
