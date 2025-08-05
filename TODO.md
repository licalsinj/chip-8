# CHIP-8 TO DO

## Refactoring

- [ ] Fix the various TODO: left in the code
- [ ] Convert this to a library instead of a binary

## Emulation Tasks

- [ ] Implement Timer Setup
- [ ] Test Quirks
- [ ] Connect Keyboard Controls
- [ ] Setup Sound
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
