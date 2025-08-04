# CHIP-8 TO DO

## Refactoring

- [ ] Fix the various TODO: left in the code
- [ ] Convert this to a library instead of a binary

## Emulation Tasks

- [ ] Test and Debug Instructions
  - Using corax89's chip8-test-rom
- [ ] Dynamically Upload Rom
- [ ] Connect Keyboard Controls
- [ ] Setup Sound
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
