use crate::cpu::Cpu;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;

pub struct Subsystem {
    pub canvas: Canvas<Window>
}

impl Subsystem {
    pub fn init(context: &Sdl, scale: u32) -> Subsystem {
        let vid_subsystem = context.video().unwrap();
        let window = vid_subsystem.window("Chip8_Rust", 64 * scale, 32 * scale)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        canvas.present();

        Subsystem {
            canvas:canvas
        }
    }

    pub fn key_down(&mut self, cpu: &mut Cpu, keycode: Keycode) {
        match keycode {
            Keycode::Num0 => { cpu.keyboard[0x0] = true; },
            Keycode::Num1 => { cpu.keyboard[0x1] = true; },
            Keycode::Num2 => { cpu.keyboard[0x2] = true; },
            Keycode::Num3 => { cpu.keyboard[0x3] = true; },
            Keycode::Num4 => { cpu.keyboard[0x4] = true; },
            Keycode::Num5 => { cpu.keyboard[0x5] = true; },
            Keycode::Num6 => { cpu.keyboard[0x6] = true; },
            Keycode::Num7 => { cpu.keyboard[0x7] = true; },
            Keycode::Num8 => { cpu.keyboard[0x8] = true; },
            Keycode::Num9 => { cpu.keyboard[0x9] = true; },
            Keycode::A => { cpu.keyboard[0xA] = true; },
            Keycode::B => { cpu.keyboard[0xB] = true; },
            Keycode::C => { cpu.keyboard[0xC] = true; },
            Keycode::D => { cpu.keyboard[0xD] = true; },
            Keycode::E => { cpu.keyboard[0xE] = true; },
            Keycode::F => { cpu.keyboard[0xF] = true; },

        }
    }
    pub fn key_up(&mut self, cpu: &mut Cpu, keycode: Keycode) {
        match keycode {
            Keycode::Num0 => { cpu.keyboard[0x0] = false; },
            Keycode::Num1 => { cpu.keyboard[0x1] = false; },
            Keycode::Num2 => { cpu.keyboard[0x2] = false; },
            Keycode::Num3 => { cpu.keyboard[0x3] = false; },
            Keycode::Num4 => { cpu.keyboard[0x4] = false; },
            Keycode::Num5 => { cpu.keyboard[0x5] = false; },
            Keycode::Num6 => { cpu.keyboard[0x6] = false; },
            Keycode::Num7 => { cpu.keyboard[0x7] = false; },
            Keycode::Num8 => { cpu.keyboard[0x8] = false; },
            Keycode::Num9 => { cpu.keyboard[0x9] = false; },
            Keycode::A => { cpu.keyboard[0xA] = false; },
            Keycode::B => { cpu.keyboard[0xB] = false; },
            Keycode::C => { cpu.keyboard[0xC] = false; },
            Keycode::D => { cpu.keyboard[0xD] = false; },
            Keycode::E => { cpu.keyboard[0xE] = false; },
            Keycode::F => { cpu.keyboard[0xF] = false; },

        }
    }
    pub fn cpu_draw(&mut self, cpu: &mut Cpu, scale: u32) {
        for i in 0..64 * 32 {
            let pixel = cpu.display[i];
            let px = (i % 64) * scale as usize;
            let py = (i / 64) * scale as usize;
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            if pixel == 1{
                self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            }
            let _ = self.canvas.fill_rect(Rect::new(px as i32, py as i32, scale, scale));

        }
        self.canvas.present();
    }




}