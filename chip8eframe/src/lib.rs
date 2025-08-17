#![warn(clippy::all, rust_2018_idioms)]

mod about;
mod app;
pub use app::Chip8App;
pub use chip8::Chip8Sys;
pub use chip8sys;
pub use chip8sys::chip8;
