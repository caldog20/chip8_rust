use crate::cpu::Cpu;


pub fn cls(cpu: &mut Cpu) {
    for i in 0..64*32 {
        cpu.display[i] = 0;
    }
    cpu.draw = true;
    cpu.pc += 2;
}

pub fn test(cpu: &mut Cpu) {
    println!("Opcode for CLS Aquired");
}

