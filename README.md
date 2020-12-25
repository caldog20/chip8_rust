# CHIP8 Emulator in Rust

## Test the Emulator!

The emulator requires SDL.



To run the emulator, clone the repo and inside the directory:
```
cargo run </path/to/game>
```

The speed the emulator runs can be increased/decreased with Up/Down arrow.
Escape will quit and Space will restart

Key mapping is as follows:

|Chip8 Key | Mapped Key |     

|1|2|3|C|  |1|2|3|4|     
|4|5|6|D|  |Q|W|E|R|     
|7|8|9|E|  |A|S|D|F|     
|A|0|B|F|  |Z|X|C|V|           
                      
                      
                      
                      
                      



## Code Layout
- Main - main.rs
  - Main Game Loop
- CPU - cpu.rs
  - CPU Initialize
  - Determine OPcode and execute instruction functions
  - Load inital ROM/GAME
  - Load Fonts
- Instructions - instructions.rs
  - functions for instruction execution used by CPU
- MEM - mem.rs
  - Will move from cpu.rs to mem.rs
  - Memory Initialize
  - Struct and impl 
  - Functions to allocate, read, and write to memory
- Subsystem - subsystem.rs
  - SDL
  - Display
  - Audio
  - Key input






# Resources
- https://github.com/etscrivner/chipokto/blob/master/docs/Requirements%20-%20ChipAte%20-%202018-05-29.pdf
- http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.1
- http://mattmik.com/files/chip8/mastering/chip8.html