# CHIP-8 TO DO

## Refactoring

- [ ] Fix the various TODO: left in the code
- [ ] Convert this to a library instead of a binary

## Emulation Tasks

- [ ] Manually Read IBM Logo Rom
- [ ] Implement Remaining Instructions
- [ ] Dynamically Upload Rom
- [ ] Connect Keyboard Controls
- [ ] Setup Sound
- [ ] Compile to WASM for Demo

## Completed

### Refactoring

- [x] Remove Nibble and Sprite structs
- [x] Change Chip8Sys.frame_buffer to be a stream of u8
  - instead of the 2D array of bools it is now
- [x] Get drawing to work with the new u8 frame_buffer

### Emulation Tasks

- [x] Implement Draw (DXYN) instruction
- [x] Create Rust Unit Tests
