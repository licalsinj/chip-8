# CHIP-8 TO DO

## Refactoring

- [x] Remove Nibble and Sprite structs
- [x] Change Chip8Sys.frame_buffer to be a stream of u8
  - instead of the 2D array of bools it is now
- [x] Get drawing to work with the new u8 frame_buffer

## Emulation Tasks

- [x] Implement Draw (DXYN) instruction
- [ ] Manually Read IBM Logo Rom
- [ ] Create Rust Unit Tests
- [ ] Implement Remaining Instructions
- [ ] Dynamically Upload Rom
- [ ] Connect Keyboard Controls
- [ ] Setup Sound
- [ ] Compile to WASM for Demo
