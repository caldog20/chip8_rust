pub const RAM_SIZE: usize = 4096;
pub const RAM_START: usize = 0x00;
pub const RAM_OFFSET: usize = 0x200
pub const RAM_END: usize = 0xFFF;
pub const FONT_OFFSET: usize = 0;


pub struct Mem {
    ram: Vec<u8>,
}
// Implementation of Mem struct with basic memory functions/methods
impl Mem {
    // Initialize new Mem array
    pub fn init() -> Mem {
        let mut ram = vec![0; RAM_SIZE];

    }

    pub fn read_mem(&self, addr: usize) -> u8 {
        self.ram[addr]
    }
    pub fn write_mem(&mut self, addr: usize, value: u8) {
        self.ram[addr] = value;
    }

    // Load rom at 0x200 (512) offset in memory
    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        for i in 0..rom.len() {
            self.ram.write_mem((RAM_OFFSET + i) = rom[i]);
        }
    }




    fn read_fonts(mem: &mut Vec<u8>)
    // Functions for reading and writing bytes to memory
        let fonts = vec![
            vec![0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
            vec![0x20, 0x60, 0x20, 0x20, 0x70], // 1
            vec![0xF0, 0x10, 0xf0, 0x80, 0xF0], // 2
            vec![0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
            vec![0x90, 0x90, 0xF0, 0x10, 0x10], // 4
            vec![0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
            vec![0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
            vec![0xF0, 0x10, 0x20, 0x40, 0x40], // 7
            vec![0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
            vec![0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
            vec![0xF0, 0x90, 0xF0, 0x90, 0x90], // A
            vec![0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
            vec![0xF0, 0x80, 0x80, 0x80, 0xF0], // C
            vec![0xE0, 0x90, 0x90, 0x90, 0xE0], // D
            vec![0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
            vec![0xF0, 0x80, 0xF0, 0x80, 0x80], // F
        ];

        for i in 0..FONT_OFFSET {
            self.ram[i] = fonts[i];
        }

    

}
