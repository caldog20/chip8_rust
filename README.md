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

Chip8 Key Original Layout      
| | | | |
|-|-|-|-|    
|1|2|3|C|       
|4|5|6|D|       
|7|8|9|E|       
|A|0|B|F|             
                      
Mapped Key Layout
| | | | |
|-|-|-|-|
|1|2|3|4|
|Q|W|E|R|
|A|S|D|F|
|Z|X|C|V|              
                      
                      
                      



## Code Layout
- Main - main.rs
  - Main Game Loop
- CPU - cpu.rs
  - CPU Initialization
  - Determine OPcode and execute instruction functions
  - Load ROM into memory
  - Load Fonts into memory
  - Emulator Timers
- Instructions - instructions.rs
  - functions for instruction execution used by CPU
- Subsystem - subsystem.rs
  - SDL
  - Display
  - Audio
  - Key input






# Resources
- https://github.com/etscrivner/chipokto/blob/master/docs/Requirements%20-%20ChipAte%20-%202018-05-29.pdf
- http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.1
- http://mattmik.com/files/chip8/mastering/chip8.html