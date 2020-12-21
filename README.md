# Overview of code factorization and flow

## Code Factorization 

- CPU - cpu.rs
  - Instructions from memory are executed
- MEM - mem.rs
  - Functions to allocate, read, and write to memory
- Main - main.rs
  - Main Game Loop
  - Loads ROM
  - Display Output
  - Sound output 
  - Input from Keyboard (SDL?)
- Fonts
  - Load fonts into app