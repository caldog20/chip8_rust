use mem::Mem;
use super::mem::RAM_OFFSET;

pub struct Cpu {
    mem: Mem,
    pc: u16,
    i: u16.
    sp: u8;
    stack: [u16; 16],
    opcode: u16,
    dt: u8,
    keyboard: Keyboard,
    display: Display,
}

impl Cpu {
    pub fn init() -> Cpu {
        Cpu {
            ram: Mem::init(),
            sp: 0,
            pc: RAM_OFFSET; // Replace with constant later !!!
            i: 0;
            v: [0; u16];
            dt: 0,

        }
    }
    
}