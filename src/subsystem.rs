use crate::cpu::Cpu;
use crate::sdl2::pixels::Color;
use crate::sdl2::rect::Rect;
use crate::sdl2::video::Window;
use crate::sdl2::render::Canvas;
use crate::sdl2::keyboard::Keycode;
use crate::sdl2::audio::{AudioCallback, AudioSpecDesired, AudioDevice};
use crate::sdl2::Sdl;

pub struct Subsystem {
    pub canvas: Canvas<Window>,
    audio_device: AudioDevice<SquareWave>
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

        let audio_subsystem = context.audio().unwrap();
        let audio_spec = AudioSpecDesired {
            freq: Some(44000),
            channels: Some(1),
            samples: None

        };
        
        let audio_device = audio_subsystem.open_playback(None, &audio_spec, |spec| {
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25
            }
        }).unwrap();

        Subsystem {
            canvas: canvas,
            audio_device:  audio_device
        }
    }

    pub fn key_down(&mut self, cpu: &mut Cpu, keycode: Keycode) {
        // println!("KEY DETECTED DOWN");
        match keycode {
            Keycode::Num1 => { cpu.keyboard[0x1] = true; },
            Keycode::Num2 => { cpu.keyboard[0x2] = true; },
            Keycode::Num3 => { cpu.keyboard[0x3] = true; },
            Keycode::Num4 => { cpu.keyboard[0xC] = true; },
            Keycode::Q => { cpu.keyboard[0x4] = true; },
            Keycode::W => { cpu.keyboard[0x5] = true; },
            Keycode::E => { cpu.keyboard[0x6] = true; },
            Keycode::R => { cpu.keyboard[0xD] = true; },
            Keycode::A => { cpu.keyboard[0x7] = true; },
            Keycode::S => { cpu.keyboard[0x8] = true; },
            Keycode::D => { cpu.keyboard[0x9] = true; },
            Keycode::F => { cpu.keyboard[0xE] = true; },
            Keycode::Z => { cpu.keyboard[0xA] = true; },
            Keycode::X => { cpu.keyboard[0x0] = true; },
            Keycode::C => { cpu.keyboard[0xB] = true; },
            Keycode::V => { cpu.keyboard[0xF] = true; },
            Keycode::P => {cpu.restart = true; },
            _ => {}

        }
    }
    pub fn key_up(&mut self, cpu: &mut Cpu, keycode: Keycode) {
        // println!("KEY DETECTED UP");
        match keycode {
            Keycode::Num1 => { cpu.keyboard[0x1] = false; },
            Keycode::Num2 => { cpu.keyboard[0x2] = false; },
            Keycode::Num3 => { cpu.keyboard[0x3] = false; },
            Keycode::Num4 => { cpu.keyboard[0xC] = false; },
            Keycode::Q => { cpu.keyboard[0x4] = false; },
            Keycode::W => { cpu.keyboard[0x5] = false; },
            Keycode::E => { cpu.keyboard[0x6] = false; },
            Keycode::R => { cpu.keyboard[0xD] = false; },
            Keycode::A => { cpu.keyboard[0x7] = false; },
            Keycode::S => { cpu.keyboard[0x8] = false; },
            Keycode::D => { cpu.keyboard[0x9] = false; },
            Keycode::F => { cpu.keyboard[0xE] = false; },
            Keycode::Z => { cpu.keyboard[0xA] = false; },
            Keycode::X => { cpu.keyboard[0x0] = false; },
            Keycode::C => { cpu.keyboard[0xB] = false; },
            Keycode::V => { cpu.keyboard[0xF] = false; },
            // Keycode::Space => {cpu.restart = false; }
            _ => {}

        }
    }

    pub fn start_beep(&mut self) {
        self.audio_device.resume();
    }

    pub fn stop_beep(&mut self) {
        self.audio_device.pause();
    }

    pub fn cpu_draw(&mut self, cpu: &mut Cpu, scale: u32) {
        for i in 0..64 * 32 {
            let pixel = cpu.display[i];
            let px = (i % 64) * scale as usize;
            let py = (i / 64) * scale as usize;
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            if pixel == 1 {
                self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            }
            let _ = self.canvas.fill_rect(Rect::new(px as i32, py as i32, scale, scale));

        }
        self.canvas.present();
    }
}
struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}
impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}