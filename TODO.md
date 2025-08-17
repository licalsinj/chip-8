# CHIP-8 TO DO

## Emulation Tasks

### Library Refactoring

Ordered by how likely they are to be done (or be able to be fixed).

- [x] Convert this to a library instead of a binary
- [x] Have Chip8Sys::run() return Result<(),Err>
- [ ] Implement delay and sound timer on their own threads?
- [ ] Fix Flashing from Clear Screen actions
  - Looking at the code this seems to use the delay timer to keep flashing down
  - And my delay timer is off so it's constantly redrawing.
- [ ] Get Controls to be more responsive
  - Watching it through eframe it's like the chip8sys library is overwriting
      what I'm sending it it at first.

  ### egui Presenter

- [x] Get Screen to Appear
- [x] Display Inside Registers & such
- [x] Get sound to work
- [x] Get Button Input to work
- [x] Show/Hide Config Windows
- [ ] Get config windows to show/hide
  - I think I should do this in a left pane like egui demo
- [ ] Run, Pause, Step Buttons
- [ ] Configure Quirks
- [ ] Dynamically Upload Rom
  - [ ] Roms from Memory
  - [ ] Roms from User
- [ ] Compile to WASM for Demo

#### Stretch Goals

- [x] Get Screen to Dynamically Scale
- [ ] Make Sound Frequency Configurable
- [x] Make Pixel Color Configurable
- [ ] Export Save States
- [ ] Make Keyboard keys Configurable

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
