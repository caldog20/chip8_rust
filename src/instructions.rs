use crate::cpu::Cpu;
use rand::Rng;

pub fn pc_advance(cpu: &mut Cpu, px: usize) {
    cpu.pc += px * 2;
}

pub fn clear_screen(cpu: &mut Cpu) {
    for i in 0..64*32 {
        cpu.display[i] = 0;
    }
    cpu.draw = true;
    pc_advance(cpu, 1);
    println!("Display Cleared!")
}
pub fn return_sub(cpu: &mut Cpu) {
    cpu.pc = cpu.stack[cpu.sp] as usize;
    cpu.sp -= 1;
    cpu.sp += 2;
    
}
pub fn test_code(cpu: &mut Cpu) {
    println!("Test Matched!!");
}
pub fn jump_addr(cpu: &mut Cpu, nnn: usize) {
    println!("JUMP_TO:: PC {:#05X} set to {:#05X}", cpu.pc, nnn);
    cpu.pc = nnn;
    
}
pub fn call_addr(cpu: &mut Cpu, nnn: usize) {
    println!("CALL_TO::PRE: PC {:#05X}, SP {:#05X}, Stack {:#05X}", cpu.pc, cpu.sp, cpu.stack[cpu.sp]);
    cpu.stack[cpu.sp] = (cpu.pc +2) as u16;
    cpu.sp += 1;
    cpu.pc = nnn;
    println!("CALL_TO::POST: PC {:#05X}, SP {:#05X}, STACK {:#05X}", cpu.pc, cpu.sp, cpu.stack[cpu.sp]);
}
pub fn skip_e_vx_kk(cpu: &mut Cpu, nx: usize, kk: u8) {
    let vx = cpu.v[nx as usize];
    if vx == kk {
        pc_advance(cpu, 1);
    }
    else {
        pc_advance(cpu, 1);
    }
}
pub fn skip_ne_vx_kk(cpu: &mut Cpu, nx: usize, kk: u8) {
    let vx = cpu.v[nx as usize];
    if vx != kk {
        pc_advance(cpu, 2);
    }
    else {
        pc_advance(cpu, 1);
    }
}
pub fn skip_e_vx_vy(cpu: &mut Cpu, nx:usize, ny:usize) {
    if cpu.v[nx] == cpu.v[ny] {
        pc_advance(cpu, 2);
    }
    else {
        pc_advance(cpu, 1);
    }
}
pub fn skip_ne_vx_vy(cpu: &mut Cpu, nx:usize, ny:usize) {
    if cpu.v[nx] != cpu.v[ny] {
        pc_advance(cpu, 2);
    }
    else {
        pc_advance(cpu, 1);
    }
}
pub fn load_vx_kk(cpu: &mut Cpu, nx:usize, kk: u8) {
    cpu.v[nx] = kk;
    pc_advance(cpu, 1);
}
pub fn add_vx_kk(cpu: &mut Cpu, nx:usize, kk: u8) {
    cpu.v[nx] = cpu.v[nx] + kk;
    pc_advance(cpu, 1);
}
pub fn load_vx_vy(cpu: &mut Cpu, nx:usize, ny: usize) {
    cpu.v[nx] = cpu.v[ny];
    pc_advance(cpu, 1);
}
pub fn or_vx_vy(cpu: &mut Cpu, nx:usize, ny: usize) {
    cpu.v[nx] = cpu.v[nx] | cpu.v[ny];
    pc_advance(cpu, 1);
}
pub fn and_vx_vy(cpu: &mut Cpu, nx:usize, ny: usize) {
    cpu.v[nx] = cpu.v[nx] & cpu.v[ny];
    pc_advance(cpu, 1);
}
pub fn xor_vx_vy(cpu: &mut Cpu, nx:usize, ny: usize) {
    cpu.v[nx] = cpu.v[nx] ^ cpu.v[ny];
    pc_advance(cpu, 1);
}
pub fn add_vx_vy(cpu: &mut Cpu, nx:usize, ny: usize) {
    let (sum, carry) = cpu.v[nx].overflowing_add(cpu.v[ny]);
    cpu.v[0xF] = carry as u8;
    cpu.v[nx] = sum as u8;
    pc_advance(cpu, 1);
}
pub fn sub_vx_vy(cpu: &mut Cpu, nx:usize, ny: usize) {
    let (diff, borrow) = cpu.v[nx].overflowing_sub(cpu.v[ny]);
    cpu.v[0xF] = !borrow as u8;
    cpu.v[nx] = diff as u8;
    pc_advance(cpu, 1);
}
pub fn shr_vx(cpu: &mut Cpu, nx:usize) {
    cpu.v[0xF] = cpu.v[nx] & 0x1;
    cpu.v[nx] = cpu.v[nx] >> 1;
    pc_advance(cpu, 1);
}
pub fn subn_vx_vy(cpu: &mut Cpu, nx:usize, ny: usize) {
    let (diff, borrow) = cpu.v[ny].overflowing_sub(cpu.v[nx]);
    cpu.v[0xF] = !borrow as u8;
    cpu.v[nx] = diff as u8;
    pc_advance(cpu, 1);
}
pub fn shl_vx(cpu: &mut Cpu, nx:usize) {
    cpu.v[0xF] = cpu.v[nx] & 0x1;
    cpu.v[nx] = cpu.v[nx] << 1;
    pc_advance(cpu, 1);
}
pub fn load_i_addr(cpu: &mut Cpu, nnn:usize) {
    cpu.i = nnn as u16;
    pc_advance(cpu, 1);
}
pub fn jump_v0_addr(cpu: &mut Cpu, nnn:usize) {
    cpu.pc = nnn + cpu.v[0] as usize;
    pc_advance(cpu, 1);
}
pub fn rand_vx_kk(cpu: &mut Cpu, nx:usize, kk: u8) {
    let mut rng = rand::thread_rng();
    let random: u8 = rng.gen_range(0..255);
    cpu.v[nx] = random & kk as u8;
    pc_advance(cpu, 1);
}
pub fn draw_vx_vy_n(cpu: &mut Cpu, nx:usize, ny: usize, n: usize) {

}
pub fn skip_p_vx(cpu: &mut Cpu, nx:usize) {
    if cpu.keyboard[nx] == true {
        pc_advance(cpu, 2);
    }
    else {
        pc_advance(cpu, 1);
    }
}
pub fn skip_np_vx(cpu: &mut Cpu, nx:usize) {
    if cpu.keyboard[nx] == false {
        pc_advance(cpu, 2);
    }
    else {
        pc_advance(cpu, 1);
    }
}
pub fn load_vx_dt(cpu: &mut Cpu, nx:usize) {
    cpu.v[nx] = cpu.dt;
    pc_advance(cpu, 1);
}
pub fn load_vx_p(cpu: &mut Cpu, nx:usize) {
    for i in 0..16 {
        if cpu.keyboard[i] == true {
            cpu.v[nx] = i as u8;
            pc_advance(cpu, 1);
            break;
        }
    }
}
pub fn load_dt_vx(cpu: &mut Cpu, nx:usize) {
    cpu.dt = cpu.v[nx];
    pc_advance(cpu, 1);
}
pub fn load_st_vx(cpu: &mut Cpu, nx:usize) {
    cpu.st = cpu.v[nx];
    pc_advance(cpu, 1);
}
pub fn add_i_vx(cpu: &mut Cpu, nx:usize) {
    cpu.i += cpu.v[nx] as u16;
    pc_advance(cpu, 1);
}
pub fn load_f_vx(cpu: &mut Cpu, nx:usize) {

}
pub fn load_b_vx(cpu: &mut Cpu, nx:usize) {

}
pub fn load_i_vx(cpu: &mut Cpu, nx:usize) {

}
pub fn load_vx_i(cpu: &mut Cpu, nx:usize) {

}