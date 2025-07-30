# CHIP-8 TO DO

## Refactoring

- [ ] Remove Nibble and Sprite structs
- [ ] Change Chip8Sys.frame_buffer to be a stream of u8
  - instead of the 2D array of bools it is now
- [ ] Get drawing to work with the new u8 frame_buffer

## Emulation Tasks

- [ ] Implement Draw (DXYN) instruction
  - I've got basic drawing working but need to include the collision logic
- [ ] Manually Read IBM Logo Rom
- [ ] Create Rust Unit Tests
- [ ] Implement Remaining Instructions
- [ ] Dynamically Upload Rom
- [ ] Connect Keyboard Controls
- [ ] Setup Sound
- [ ] Compile to WASM for Demo
