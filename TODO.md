# CHIP-8 TO DO

## Emulation Tasks

### Library Refactoring

Ordered by how likely they are to be done (or be able to be fixed).

- [x] Convert this to a library instead of a binary
- [x] Have Chip8Sys::run() return Result<(),Err>
- [ ] Improve error codes
  - Invalid first byte
  - Invalid op code
  - Maybe even imbbed a String explaination...
- [ ] Implement delay and sound timer on their own threads
- [ ] Fix Flashing from Clear Screen actions
  - Looking at the code this seems to use the delay timer to keep flashing down
  - And my delay timer is off so it's constantly redrawing.
- [ ] Get Controls to be more responsive
  - Watching it through eframe it's like the chip8sys library is overwriting
      what I'm sending it it at first.
- [ ] Get Serde working for Chip8Sys
- [ ] Move the About::chip_8_decode into the Chip8Sys library
  - and take into account current quirks.
  - Could be good to have a global command lookup impl without &self
  - And one impl with &self taking into account quirks

  ### egui Presenter

- [x] Get Screen to Appear
- [x] Display Inside Registers & such
- [x] Get sound to work
- [x] Get Button Input to work
- [x] Show/Hide Config Windows
- [x] Get config windows to show/hide
  - I think I should do this in a left pane like egui demo
- [x] Run, Pause, Step Buttons
- [x] Restart Button
- [ ] Configure Quirks
- [ ] Dynamically Upload Rom
  - [ ] Roms from Memory
  - [ ] Roms from User
  - chip8eframe/src/app.rs line 367 (reset code) needs to know which rom is being reloaded
- [x] Compile to WASM for Demo
- [ ] Implement Sound in WASM
- [ ] Implement Serde

#### About Page

Info that I need on the about page:

- [ ] What is the chip-8
- [ ] How to load a new rom
- [ ] How to control the Chip-8
- [ ] Where to learn more about me

#### Stretch Goals

- [x] Get Screen to Dynamically Scale
- [ ] Make Sound Frequency Configurable
- [x] Make Pixel Color Configurable
- [ ] Export Save States
- [ ] Make Keyboard keys Configurable
- [ ] Build a lookup table for commands to print on Control Flow Window
- [ ] Control flow: "Previous command" is not the previous command executed
  - It's actually the previous command in memory

## Completed

### Completed Tasks

- [x] Implement Draw (DXYN) instruction
- [x] Create Rust Unit Tests
- [x] Manually Read IBM Logo Rom
- [x] Implement Remaining Instructions
- [x] Remove Nibble and Sprite structs
- [x] Change Chip8Sys.frame_buffer to be a stream of u8
  - instead of the 2D array of bools it is now
- [x] Get drawing to work with the new u8 frame_buffer

- [x] Test and Debug Instructions
  - Using Timendus's chip-8 test suite
  - <https://github.com/Timendus/chip8-test-suite>
- [x] Implement Timer Setup
- [x] Test Quirks
- [x] Connect Keyboard Controls
- [x] Turn Load Rom into a Chip8Sys function
- [x] Get Display wrapping to work
- [x] Get Display clipping to work
- [x] Automate Timendus' tests
  - Basically compare the FB to a successful test and then you'll know if it fails
- [x] Understand and implement vblank after drawing sprites
  - This is causing me to fail the display wait quirk
  - I think the delay timer not being on its own thread also causes that problem
  - This is specific to advanced chip-8s that have scrolling.
      I'm focusing on just the regular chip-8 and so it's not a problem right now.
- [x] Resolve TODOs in code
- [x] Setup Sound
