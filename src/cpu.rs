use std::fs::File;
use std::io::Read;
use crate::instructions::*;
use crate::subsystem::Subsystem;

pub struct Cpu {
    pub mem: [u8; 4096],
    pub pc: usize,
    pub i: u16,
    pub sp: usize,
    pub stack: [u16; 16],
    pub v: [u8; 16],
    pub dt: u8,
    pub st: u8,
    pub keyboard: [bool; 16],
    pub display: [u8; 64 * 32],
    pub draw: bool,
    pub counter: u8,
    pub quit: bool,
    pub speed: u32
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
            dt: 0,
            st: 0,
            keyboard: [false; 16],
            display: [0; 64 * 32],
            draw: false,
            counter: 10,
            quit: false,
            speed: 2400
        }
    }
    pub fn load_rom(&mut self, game: &str) {
        // Load game rom into memory starting a 0x200
        println!("Loading Game...");
        let mut rom = File::open(game).unwrap(); // Open file from games dir
        let mut buffer = Vec::<u8>::new(); // create buffer for rom
        rom.read_to_end(&mut buffer).expect("Error reading rom to buffer");
        
        for i in 0..buffer.len() {
            self.mem[0x200 + i] = buffer[i];
        }
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
        }
    }

    pub fn get_opcode(&self) -> u16 {
        (self.mem[self.pc as usize] as u16) << 8 | (self.mem[(self.pc + 1) as usize] as u16)
    }

    pub fn do_opcode(&mut self, opcode: u16) {
        /*
        Program Counter = pc
        Stack Pointer = sp
        CPU Registers = v
        Stack = stack
        Index Register = i
        nnn = lower 12 bits of instruction
        kk = lowest 8 bits of instruction
        nx = lower 4 bits of high byte
        ny = lower 4 bits of low byte
        n = lower 4 bits if instruction
        */
        let ms_byte = opcode & 0xF000;

        let nibbles = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            (opcode & 0x000F)
        );
        let nnn = (opcode & 0x0FFF) as usize; // Address
        let kk = (opcode & 0x00FF) as u8; // OPCODE NN 8 bits
        let nx = nibbles.1 as usize; // lower 4 bits of high byte
        let ny = nibbles.2 as usize; // lower 4 bits of low byte
        let n = nibbles.3 as usize; // lowest 4 bits
        
            

        match ms_byte {
            0x0000 => match nnn {
                0x00E0 => clear_screen(self), // Clear Screen
                0x00EE => return_sub(self), // Return subroutine
                _ => panic!("NNN Opcode: {:#05X}", opcode) // Panic if no other match in lower 12 bits
            }
            0x1000 => jump_addr(self, nnn), // jump to address at nnn - set pc to nnn
            0x2000 => call_addr(self, nnn), // call to address at nnn - increment sp, put current pc at top of stack, set pc to nnn
            0x3000 => skip_e_vx_kk(self, nx, kk), // Skip next instruction if v[x] == kk
            0x4000 => skip_ne_vx_kk(self, nx, kk), // Skip next instruction if v[x] != kk
            0x5000 => skip_e_vx_vy(self, nx, ny), // Skip next instruction if v[x] == v[y]
            0x6000 => load_vx_kk(self, nx, kk), // load v[x] to kk
            0x7000 => add_vx_kk(self, nx, kk), // Add kk to v[x] then pushes result to v[x]
            0x8000 => match n { // Match most significant byte 8 against lowest 4 bits
                0x0 => load_vx_vy(self, nx, ny), // load v[y] into v[x] 
                0x1 => or_vx_vy(self, nx, ny), // v[x] = v[x] OR v[y]
                0x2 => and_vx_vy(self, nx, ny), // v[x] = v[x] AND v[y]
                0x3 => xor_vx_vy(self, nx, ny), // v[x] = v[x] XOR v[y]
                0x4 => add_vx_vy(self, nx, ny),
                0x5 => sub_vx_vy(self, nx, ny),
                0x6 => shr_vx(self, nx),
                0x7 => subn_vx_vy(self, nx, ny),
                0xE => shl_vx(self, nx),
                _ => panic!("N Opcode: {:#05X}", opcode)
            }
            0x9000 => skip_ne_vx_vy(self, nx, ny),
            0xA000 => load_i_addr(self, nnn),
            0xB000 => jump_v0_addr(self, nnn),
            0xC000 => rand_vx_kk(self, nx, kk),
            0xD000 => draw_vx_vy_n(self, nx, ny, n),
            0xE000 => match kk {
                0x009E => skip_p_vx(self, nx),
                0x00A1 => skip_np_vx(self, nx),
                _ => panic!("KK Opcode: {:#05X}", opcode)
            }
            0xF000 => match kk {
                0x0007 => load_vx_dt(self, nx),
                0x000A => load_vx_p(self, nx),
                0x0015 => load_dt_vx(self, nx),
                0x0018 => load_st_vx(self, nx),
                0x001E => add_i_vx(self, nx),
                0x0029 => load_f_vx(self, nx),
                0x0033 => load_b_vx(self, nx),
                0x0055 => load_i_vx(self, nx),
                0x0065 => load_vx_i(self, nx),
                _ => panic!("KK Opcode: {:#05X}", opcode)
            }

            _ => panic!("Invalid Opcode: {:#05X}", opcode)
            }    
        }
    
    pub fn handle_timers(&mut self, subsystem: &mut Subsystem) {
        if self.counter == 10 {
            if self.dt > 0 {
                self.dt -= 1;
            }
            if self.st > 0 {
                if self.st == 1 {
                    subsystem.start_beep();
                }
                self.st -= 1;
            } else if self.st == 0 {
                subsystem.stop_beep();
            }
            self.counter = 0;
        } else {
            self.counter += 1;
        }
    }


    pub fn run_cycle(&mut self, subsystem: &mut Subsystem) {
        let opcode = self.get_opcode();
        println!("Opcode {:#05X} PC {:#05X} SP {:#05X} DT {:#} ST {:#} I {:#05X} STACK {:#05X},
         Speed {:#}", opcode, self.pc, self.sp, self.dt, self.st, self.i, self.stack[self.sp],
          self.speed);
        self.do_opcode(opcode);
        self.handle_timers(subsystem);
    }

}
