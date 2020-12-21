use std::fs::File;
use std::io::Read;
// use Instructions::*;
use std::{thread, time};

pub struct Cpu {
    mem: [u8; 4096],
    pc: usize,
    i: u16,
    sp: u8,
    stack: [u16; 16],
    opcode: u16,
    dt: u8,
    keyboard: [bool; 16],
    display: [u8; 64 * 32],
    draw: bool,
}

impl Cpu {
    fn init() -> Cpu {
        Cpu {
            mem: [0; 4096],
            pc: 0x200,
            i: 0,
            sp: 0,
            stack: [0; 16],
            opcode: 0,
            dt: 0,
            keyboard: [false; 16],
            display: [0; 64 * 32],
            draw: false
        }
    }
    
    
    fn load_rom(&mut self, game: &str) {
        println!("Loading Game...");
        let mut rom = File::open(game).unwrap(); // Open file from games dir
        // rom.read(&mut self.mem[0x200..]).unwrap();
        let mut buffer = Vec::<u8>::new(); // create buffer for rom
        rom.read_to_end(&mut buffer);
        // let buff_size = rom.read(&mut buffer[..]); // read bytes into buffer
        for i in 0..buffer.len() {
            self.mem[0x200 + i] = buffer[i];
        }
        // thread::sleep(time::Duration::from_secs(1));
    }

    fn load_fonts(&mut self) {
    // Functions for reading and writing bytes to memory
        let fonts = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        for i in 0..fonts.len() {
            self.mem[i] = fonts[i];
            println!("MemLoc {:#05X}, Font {:#05X}", i, fonts[i]);
            thread::sleep(time::Duration::from_micros(150000));
        }
    }

    fn get_opcode(&self) -> u16 {
        (self.mem[self.pc as usize] as u16) << 8 | (self.mem[(self.pc + 1) as usize] as u16)
    }

    fn do_opcode(&mut self, opcode: u16) {
        let most_sig = opcode & 0xF000;
        println!("pc: {:#05X}, {:04X} : ", self.pc, opcode);

    }

    fn run(&mut self) {
        let opcode = self.get_opcode();
        self.do_opcode(opcode);
        self.load_fonts();
    }

}



    


fn main() {
    let mut cpu = Cpu::init();
    cpu.load_rom("games/PONG");
    cpu.run();

    










    
}
