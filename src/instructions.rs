use crate::cpu::Cpu;


pub fn cls(cpu: &mut Cpu) {
    for i in 0..64*32 {
        cpu.display[i] = 0;
    }
    cpu.draw = true;
    cpu.pc += 2;
    println!("Display Cleared!")
}
pub fn return_sub(cpu: &mut Cpu) {
    cpu.pc = cpu.stack[cpu.sp] as usize;
    cpu.sp -= 1;
    
}
pub fn test_code(cpu: &mut Cpu) {
    println!("Test Matched!!");
}
pub fn jump_to(cpu: &mut Cpu, nnn: usize) {
    println!("JUMP_TO:: PC {:#05X} set to {:#05X}", cpu.pc, nnn);
    cpu.pc = nnn;
    
}
pub fn call_to(cpu: &mut Cpu, nnn: usize) {
    println!("CALL_TO::PRE: PC {:#05X}, SP {:#05X}, Stack {:#05X}", cpu.pc, cpu.sp, cpu.stack[cpu.sp]);
    cpu.stack[cpu.sp] = (cpu.pc +2) as u16;
    cpu.sp += 1;
    cpu.pc = nnn;
    println!("CALL_TO::POST: PC {:#05X}, SP {:#05X}, STACK {:#05X}", cpu.pc, cpu.sp, cpu.stack[cpu.sp]);
}
pub fn skip_e_vx_kk(cpu: &mut Cpu, vx: usize, kk: usize) {

}
pub fn skip_ne_vx_kk(cpu: &mut Cpu, vx: usize, kk: usize) {

}

