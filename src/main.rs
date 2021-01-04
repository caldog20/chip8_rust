extern crate rand;
extern crate sdl2;
mod cpu;
mod instructions;
mod subsystem;
use crate::cpu::Cpu;
use subsystem::Subsystem;
use std::env;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
static SCALE: u32 = 16;


pub fn main() {
    game_loop();
}
fn game_loop() {
    loop {
        let mut cpu = Cpu::init();

        let sdl = sdl2::init().unwrap();
        let mut subsystem = Subsystem::init(&sdl, SCALE);
        let args: Vec<String> = env::args().collect();
        cpu.load_fonts();
        cpu.load_rom(&args[1]);
        let mut event_pump = sdl.event_pump().unwrap();
        'running: loop {
            let mut pause: bool;
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        cpu.quit = true;
                        println!("Quitting...");
                        break 'running
                    },
                    Event::KeyDown { keycode: Some(Keycode::Space), .. } => { 
                        break 'running 
                    },
                    Event::KeyDown { keycode: Some(Keycode::Tab), .. } => { 
                        pause = !pause;
                    },
                    Event::KeyDown { keycode: Some(Keycode::Up), .. } => cpu.speed += 600,
                    Event::KeyDown { keycode: Some(Keycode::Down), .. } => cpu.speed -= 600,
                    Event::KeyDown { keycode: Some(keycode), .. } => subsystem.key_down(&mut cpu, keycode),
                    Event::KeyUp { keycode: Some(keycode), .. } => subsystem.key_up(&mut cpu, keycode),
                    _ => {}
                }
                if cpu.speed <= 600 {
                    cpu.speed = 600;
                }
            }
            ::std::thread::sleep(Duration::new(0, 1_000_000_000 as u32 / cpu.speed));
            if pause {
                print!(".");
            }
            else {
                cpu.run_cycle(&mut subsystem);
            }
            if cpu.draw {
                subsystem.cpu_draw(&mut cpu, SCALE);
            }
            }
        if cpu.quit == true {
            break;
        }   
    }
}