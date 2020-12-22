use std::fs::File;
use std::io::*;
use crate::instructions::*;
// use std::{thread, time};

pub struct Cpu {
    pub mem: [u8; 4096],
    pub pc: usize,
    pub i: u16,
    pub sp: usize,
    pub stack: [u16; 16],
    pub v: [u8; 16],
    pub opcode: u16,
    pub dt: u8,
    pub keyboard: [bool; 16],
    pub display: [u8; 64 * 32],
    pub draw: bool,
}

impl Cpu {
    pub fn init() -> Cpu {
        Cpu {
            mem: [0; 4096],
            pc: 0x200,
            i: 0,
            sp: 0,
            stack: [0; 16],
            v: [0; 16],
            opcode: 0,
            dt: 0,
            keyboard: [false; 16],
            display: [0; 64 * 32],
            draw: false
        }
    }
    
    
    pub fn load_rom(&mut self, game: &str) {
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

    pub fn load_fonts(&mut self) {
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
            // println!("MemLoc {:#05X}, Font {:#05X}", i, fonts[i]);
            // thread::sleep(time::Duration::from_micros(150000));
        }
    }

    pub fn get_opcode(&self) -> u16 {
        (self.mem[self.pc as usize] as u16) << 8 | (self.mem[(self.pc + 1) as usize] as u16)
    }

    pub fn do_opcode(&mut self, opcode: u16) {
        let ms_byte = opcode & 0xF000;
        println!("OPCODE FROM MEMORY = {:#06X}", opcode);
        println!("MSBYTE = {:#X}", ms_byte);
        // println!("{:#05X}, {:#05X} : ", opcode, most_sig_byte);
        
        let nibbles = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            (opcode & 0x000F) >> 0,
        );
        let nnn = (opcode & 0x0FFF) as usize; // Address
        let kk = (opcode & 0x00FF) as u8; // OPCODE NN
        let nx = nibbles.1 as usize;
        let ny = nibbles.2 as usize;
        let n = nibbles.3 as usize; // Lowest 4 bits
            

        match ms_byte {

            0x0000 => match nnn {
                0x00E0 => test_code(self), // Clear Screen
                0x00EE => return_sub(self),
                _ => panic!("NNN Opcode: {:#05X}", opcode)
            }
            0x1000 => jump_to(self, nnn),
            0x2000 => call_to(self, nnn),
            0x3000 => skip_e_vx_kk(self, nx, kk),
            0x4000 => skip_ne_vx_kk(self, nx, kk),
            0x5000 => skip_e_vx_vy(self, nx, ny),
            0x6000 => set_vx_kk(self, nx, kk),
            _ => panic!("Invalid Opcode: {:#05X}", opcode)
            }    
        }
        
    pub fn run_cycle(&mut self) {
        // let opcode = self.get_opcode();
        let opcode = 0x2333; 
        self.do_opcode(opcode);
        self.load_fonts();
    }

}
