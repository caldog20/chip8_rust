extern crate rand;
use crate::cpu::Cpu;
use rand::Rng;

pub fn pc_advance(cpu: &mut Cpu, px: usize) {
    cpu.pc += px * 2;
}
pub fn clear_screen(cpu: &mut Cpu) {
     // println!("CLEAR SCREEN")
    for i in 0..64 * 32 {
        cpu.display[i] = 0;
    }
    cpu.draw = true;
    pc_advance(cpu, 1);
}
pub fn return_sub(cpu: &mut Cpu) {
    // println!("RETURN");
    cpu.sp -= 1;
    cpu.pc = cpu.stack[cpu.sp] as usize;
}
pub fn jump_addr(cpu: &mut Cpu, nnn: usize) {
    // println!("JUMP_ADDR");
    // println!("JUMP_TO:: PC {:#05X} > {:#05X}", cpu.pc, nnn);
    cpu.pc = nnn;
}
pub fn call_addr(cpu: &mut Cpu, nnn: usize) {
    // println!("CALL_ADDR");
    // println!("CALL_TO::PRE: PC {:#05X}, SP {:#05X}, Stack {:#05X}", cpu.pc, cpu.sp, cpu.stack[cpu.sp]);
    cpu.stack[cpu.sp] = (cpu.pc + 2) as u16;
    cpu.sp += 1;
    cpu.pc = nnn;
    // println!("CALL_TO::POST: PC {:#05X}, SP {:#05X}, STACK {:#05X}", cpu.pc, cpu.sp, cpu.stack[cpu.sp]);
}
pub fn skip_e_vx_kk(cpu: &mut Cpu, nx: usize, kk: u8) {
    // println!("SKIP_E_VX_KK");
    let vx = cpu.v[nx as usize];
    if vx == kk {
        pc_advance(cpu, 2);
    }
    else {
        pc_advance(cpu, 1);
    }
}
pub fn skip_ne_vx_kk(cpu: &mut Cpu, nx: usize, kk: u8) {
    // println!("SKIP_NE_VX_KK");
    let vx = cpu.v[nx as usize];
    if vx != kk {
        pc_advance(cpu, 2);
    }
    else {
        pc_advance(cpu, 1);
    }
}
pub fn skip_e_vx_vy(cpu: &mut Cpu, nx:usize, ny:usize) {
    // println!("SKIP_E_VX_VY");
    if cpu.v[nx] == cpu.v[ny] {
        pc_advance(cpu, 2);
    }
    else {
        pc_advance(cpu, 1);
    }
}
pub fn skip_ne_vx_vy(cpu: &mut Cpu, nx:usize, ny:usize) {
    // println!("SKIP_NE_VX_VY");
    if cpu.v[nx] != cpu.v[ny] {
        pc_advance(cpu, 2);
    }
    else {
        pc_advance(cpu, 1);
    }
}
pub fn load_vx_kk(cpu: &mut Cpu, nx:usize, kk: u8) {
    cpu.v[nx] = kk as u8;
    // println!("LOAD_VX_KK");
    // println!("Vx {:#05X}", cpu.v[nx]);
    pc_advance(cpu, 1);
}
pub fn add_vx_kk(cpu: &mut Cpu, nx:usize, kk: u8) {
    // println!("ADD_VX_KK");
    // println!("ADD PRE: {:#05X}", cpu.v[nx]);
    let vx = cpu.v[nx as usize] as u16;
    let sum = vx + kk as u16;
    cpu.v[nx] = sum as u8;
    // println!("ADD POST: {:#05X}", cpu.v[nx]);
    pc_advance(cpu, 1);
}
pub fn load_vx_vy(cpu: &mut Cpu, nx:usize, ny: usize) {
     // println!("LOAD_VX_VY");
    let vy = cpu.v[ny];
    cpu.v[nx] = vy;
    pc_advance(cpu, 1);
}
pub fn or_vx_vy(cpu: &mut Cpu, nx:usize, ny: usize) {
    // println!("OR_VX_VY");
    cpu.v[nx] = cpu.v[nx] | cpu.v[ny];
    pc_advance(cpu, 1);
}
pub fn and_vx_vy(cpu: &mut Cpu, nx:usize, ny: usize) {
    // println!("AND_VX_VY");
    cpu.v[nx] = cpu.v[nx] & cpu.v[ny];
    pc_advance(cpu, 1);
}
pub fn xor_vx_vy(cpu: &mut Cpu, nx:usize, ny: usize) {
    // println!("XOR_VX_VY");
    cpu.v[nx] = cpu.v[nx] ^ cpu.v[ny];
    pc_advance(cpu, 1);
}
pub fn add_vx_vy(cpu: &mut Cpu, nx:usize, ny: usize) {
    // println!("ADD_VX_VY");
    let (sum, carry) = cpu.v[nx].overflowing_add(cpu.v[ny]);
    cpu.v[0xF] = carry as u8;
    cpu.v[nx] = sum as u8;
    pc_advance(cpu, 1);
}
pub fn sub_vx_vy(cpu: &mut Cpu, nx:usize, ny: usize) {
    // println!("SUB_VX_VY");
    let (diff, borrow) = cpu.v[nx].overflowing_sub(cpu.v[ny]);
    cpu.v[0xF] = !borrow as u8;
    cpu.v[nx] = diff as u8;
    pc_advance(cpu, 1);
}
pub fn shr_vx(cpu: &mut Cpu, nx:usize) {
     // println!("SHIFT_RIGHT_VX");
    cpu.v[0xF] = cpu.v[nx] & 0x01;
    cpu.v[nx] = cpu.v[nx] >> 1;
    pc_advance(cpu, 1);
}
pub fn subn_vx_vy(cpu: &mut Cpu, nx:usize, ny: usize) {
    // println!("SUBN_VX_VY");
    let (diff, borrow) = cpu.v[ny].overflowing_sub(cpu.v[nx]);
    cpu.v[0xF] = !borrow as u8;
    cpu.v[nx] = diff as u8;
    pc_advance(cpu, 1);
}
pub fn shl_vx(cpu: &mut Cpu, nx:usize) {
    // println!("SHIFT_LEFT_VX");
    cpu.v[0xF] = (cpu.v[nx] & 0x80) >> 7;
    cpu.v[nx]  <<= 1;
    pc_advance(cpu, 1);
}
pub fn load_i_addr(cpu: &mut Cpu, nnn:usize) {
    // println!("LOAD_I_ADDR");
    cpu.i = nnn as u16;
    pc_advance(cpu, 1);
}
pub fn jump_v0_addr(cpu: &mut Cpu, nnn:usize) {
     // println!("JUMP_V0_ADDR");
    cpu.pc = nnn + cpu.v[0] as usize;
    pc_advance(cpu, 1);
}
pub fn rand_vx_kk(cpu: &mut Cpu, nx:usize, kk: u8) {
    // println!("RAND_VX_KK");
    let mut rng = rand::thread_rng();
    let random: u8 = rng.gen_range(0..255);
    cpu.v[nx] = random & kk as u8;
    pc_advance(cpu, 1);
}
pub fn draw_vx_vy_n(cpu: &mut Cpu, nx:usize, ny: usize, n: usize) {
    //  // println!("DRAW_VX_VY_N");
    let xx = cpu.v[nx];
    let xy = cpu.v[ny];

    cpu.v[0xF] = 0;
    
    for y in 0..n {
        let pixel = cpu.mem[(cpu.i + y as u16) as usize];
        for x in 0..8 {
            if (pixel & 0x80 >> x) != 0 {
                let position = ((xx as u16 + x as u16) + ((xy as u16 + y as u16) * 64)) % (32 * 64);
                if cpu.display[position as usize] == 1 {
                    cpu.v[0xF] = 1;
                }
                cpu.display[position as usize] ^= 1;
        // println!("DRAW AT {:#05X}", cpu.display[position as usize]);
            }
        }
    }
    cpu.draw = true;
    pc_advance(cpu, 1);
}
pub fn skip_p_vx(cpu: &mut Cpu, nx:usize) {
     // println!("SKIP_PRESS_VX");
    let key = cpu.v[nx] as usize;
    if cpu.keyboard[key] == true {
        pc_advance(cpu, 2);
    }
    else {
        pc_advance(cpu, 1);
    }
}
pub fn skip_np_vx(cpu: &mut Cpu, nx:usize) {
    // println!("SKIP_NOPRESS_VX");
    let key = cpu.v[nx] as usize;
    if cpu.keyboard[key] == false {
        pc_advance(cpu, 2);
    }
    else {
        pc_advance(cpu, 1);
    }
}
pub fn load_vx_dt(cpu: &mut Cpu, nx:usize) {
    // println!("LOAD_VX_DT");
    cpu.v[nx] = cpu.dt;
    pc_advance(cpu, 1);
}
pub fn load_vx_p(cpu: &mut Cpu, nx:usize) {
    // println!("LOAD_VX_P");
    for i in 0..16 {
        if cpu.keyboard[i] == true {
            cpu.v[nx] = i as u8;
            pc_advance(cpu, 1);
            break;
        }
    }
}
pub fn load_dt_vx(cpu: &mut Cpu, nx:usize) {
    // println!("LOAD_DT_VX");
    cpu.dt = cpu.v[nx];
    pc_advance(cpu, 1);
}
pub fn load_st_vx(cpu: &mut Cpu, nx:usize) {
    // println!("LOAD_ST_VX");
    cpu.st = cpu.v[nx];
    pc_advance(cpu, 1);
}
pub fn add_i_vx(cpu: &mut Cpu, nx:usize) {
    // println!("ADD_I_VX");
    cpu.i += cpu.v[nx] as u16;
    pc_advance(cpu, 1);
}
pub fn load_f_vx(cpu: &mut Cpu, nx:usize) {
    // println!("LOAD_F_VX");
    cpu.i = (cpu.v[nx] * 0x5) as u16;
    pc_advance(cpu, 1);
}
pub fn load_b_vx(cpu: &mut Cpu, nx:usize) {
    // println!("LOAD_B_VX");
    // println!("LOAD_B_VX I {:#05X} nx {:#05X}", cpu.i, nx);
    cpu.mem[cpu.i as usize] = cpu.v[nx] / 100;
    cpu.mem[cpu.i as usize + 1] = (cpu.v[nx] / 10) as u8 % 10;
    cpu.mem[cpu.i as usize + 2] = (cpu.v[nx] % 100) as u8 % 10;
    pc_advance(cpu, 1);

}
pub fn load_i_vx(cpu: &mut Cpu, nx:usize) {
    // println!("LOAD_I_VX");
    for i in 0..nx + 1 {
        cpu.mem[(cpu.i + i as u16) as usize] = cpu.v[i as usize];
    }
    pc_advance(cpu, 1);
}
pub fn load_vx_i(cpu: &mut Cpu, nx:usize) {
    // println!("LOAD_VX_I");
    for i in 0..nx + 1 {
        cpu.v[i as usize] = cpu.mem[(cpu.i + i as u16) as usize];
    }
    pc_advance(cpu, 1);
}

