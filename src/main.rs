use chip8::Chip8Sys;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use rodio::source::{SineWave, Source};

mod chip8;
mod decode;
// mod roms; // used for testing, may not be needed long term

const PIXEL_COLOR_OFF: u32 = 0x3D521E;
const PIXEL_COLOR_ON: u32 = 0x80B039;
pub const WIDTH: usize = 640 * 3;
pub const HEIGHT: usize = 320 * 3;
// passed when creating all new Chip8Sys
// handles if FX55 & FX65 increment I index register
pub const INC_INDEX: bool = true;
pub const VF_RESET: bool = true;
pub const WRAP_DRAW: bool = false;
pub const MOD_VX_IN_PLACE: bool = false;

fn main() {
    let mut game = Chip8Sys::new_chip_8();

    // load the ROM from Disc
    // let file_path = "roms/1-chip8-logo.ch8";
    // let file_path = "roms/2-ibm-logo.ch8";
    // let file_path = "roms/3-corax+.ch8";
    // let file_path = "roms/4-flags.ch8";
    // let file_path = "roms/5-quirks.ch8";
    let file_path = "roms/7-beep.ch8";
    // When running quirks rom hardcode this memory spot to auto run Chip-8
    // game.memory[0x1FF] = 1;
    // let file_path = "roms/6-keypad.ch8";
    // let file_path = "roms/walking_man.ch8";

    game.load_rom(String::from(file_path));

    // game.memory = [0; 0x1000];
    // Old way
    // game.load_dxyn_rom_adv();

    // Setup Sound
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());
    // sink.append(SineWave::new(440.0).repeat_infinite());

    // Setup Window
    let mut window = Window::new(
        "Chip 8 - Press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale_mode: ScaleMode::UpperLeft,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create the window");

    window.set_target_fps(240);

    let mut buffer;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        check_key_input(&mut game, &window);
        buffer = display_buffer(&mut game);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        game.run();
        if game.is_playing_sound {
            sink.append(SineWave::new(440.0).repeat_infinite());
        } else {
            sink.stop();
        }
    }
}
fn check_key_input(chip8: &mut Chip8Sys, window: &Window) {
    chip8.keys[0] = window.is_key_down(Key::X);
    chip8.keys[1] = window.is_key_down(Key::Key1);
    chip8.keys[2] = window.is_key_down(Key::Key2);
    chip8.keys[3] = window.is_key_down(Key::Key3);
    chip8.keys[4] = window.is_key_down(Key::Q);
    chip8.keys[5] = window.is_key_down(Key::W);
    chip8.keys[6] = window.is_key_down(Key::E);
    chip8.keys[7] = window.is_key_down(Key::A);
    chip8.keys[8] = window.is_key_down(Key::S);
    chip8.keys[9] = window.is_key_down(Key::D);
    chip8.keys[0xA] = window.is_key_down(Key::Z);
    chip8.keys[0xB] = window.is_key_down(Key::C);
    chip8.keys[0xC] = window.is_key_down(Key::Key4);
    chip8.keys[0xD] = window.is_key_down(Key::R);
    chip8.keys[0xE] = window.is_key_down(Key::F);
    chip8.keys[0xF] = window.is_key_down(Key::V);
}
// converts the Chip8Sys frame_buffer to the 1280x640 display I'm using
// This belongs here instead of on Chip8Sys because it's specific to how I'm displaying the screen
pub fn display_buffer(chip8: &mut Chip8Sys) -> Vec<u32> {
    // NOTE: Multiply by 8 b/c there are 8 bits (px) in a u8
    // Then square root because we reinsert the result vec into results scalar times
    let scaler = ((WIDTH * HEIGHT) as f64 / (chip8.frame_buffer.len() * 8) as f64)
        .sqrt()
        .floor() as usize;
    // println!("scaler: {scaler}");
    // let scaler = 20;

    // Prints debug of the frame buffer to the console
    // self.debug_print_frame_buffer();

    let mut results = Vec::new();
    let mut result: Vec<u32> = Vec::new();
    for (i, pixel) in chip8.frame_buffer.iter().enumerate() {
        let mut power_2 = 0b1000_0000;
        for _ in 0..8 {
            if pixel & power_2 == power_2 {
                result.append(&mut vec![PIXEL_COLOR_ON; scaler]);
            } else {
                result.append(&mut vec![PIXEL_COLOR_OFF; scaler]);
            }
            // reduce power_2 to check the next bit to the right
            power_2 /= 2;
        }
        // every 8 bytes (64 bits) add scaler number of rows to results
        // this adds vertical thickness to the screen
        if (i + 1) % 8 == 0 {
            results.append(&mut vec![result; scaler].concat());
            result = Vec::new();
        }
    }
    results
}
