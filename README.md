# Overview of CHIP8 emulator in Rust

## Code Layout
- Main - main.rs
  - Main Game Loop
  - Function Calls
- CPU - cpu.rs
  - CPU Initialize
  - Determine OPcode and execute instruction functions
  - Load inital ROM/GAME
  - Load Fonts
- Instructions - instructions.rs
  - functions for instruction execution used by CPU
- MEM - mem.rs
  - Will move from cpu.rs to mem.rs 
  - Functions to allocate, read, and write to memory






# Resources
- https://github.com/etscrivner/chipokto/blob/master/docs/Requirements%20-%20ChipAte%20-%202018-05-29.pdf
- http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.1
- http://mattmik.com/files/chip8/mastering/chip8.html