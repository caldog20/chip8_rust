mod cpu;
mod instructions;

use crate::cpu::Cpu;

pub fn main() {
    let mut cpu = Cpu::init();
    cpu.load_rom("games/PONG");
    cpu.run_cycle(); 
}