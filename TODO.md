# CHIP-8 TO DO

## Refactoring

- [ ] Fix the various TODO: left in the code
- [ ] Convert this to a library instead of a binary
- [ ] Implement delay and sound timer on their own threads?
- [ ] Fix Flashing from Clear Screen actions

## Emulation Tasks

- [ ] Get Display clipping to work
- [ ] Automate Timendus' tests
  - Basically compare the FB to a successful test and then you'll know if it fails
- [ ] Setup Sound
- [ ] Write a dump rom function to get state of Chip-8
- [ ] Get Controls to be more responsive
- [ ] Understand and implement vblank after drawing sprites
  - This is causing me to fail the display wait quirk
  - I think the delay timer not being on its own thread also causes that problem

  ### Front End Wrapper

  A holding spot for things I want to handle in the front end wrapper long term.

- [ ] Dynamically Upload Rom
- [ ] Compile to WASM for Demo

## Completed

### Completed Refactoring Tasks

- [x] Remove Nibble and Sprite structs
- [x] Change Chip8Sys.frame_buffer to be a stream of u8
  - instead of the 2D array of bools it is now
- [x] Get drawing to work with the new u8 frame_buffer

### Completed Emulation Tasks

- [x] Implement Draw (DXYN) instruction
- [x] Create Rust Unit Tests
- [x] Manually Read IBM Logo Rom
- [x] Implement Remaining Instructions
- [x] Test and Debug Instructions
  - Using Timendus's chip-8 test suite
  - <https://github.com/Timendus/chip8-test-suite>
- [x] Implement Timer Setup
- [x] Test Quirks
- [x] Connect Keyboard Controls
- [x] Turn Load Rom into a Chip8Sys function
- [x] Get Display wrapping to work
