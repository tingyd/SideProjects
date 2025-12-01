//memory.rs
pub struct Memory {
    ram: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory { ram: vec![0; 0x10000] }
    }
    
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }
    
    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value;
    }
    
    pub fn read_word(&self, addr: u16) -> u16 {
        let lo = self.read_byte(addr) as u16;
        let hi = self.read_byte(addr.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }
    
    pub fn write_word(&mut self, addr: u16, value: u16) {
        self.write_byte(addr, (value & 0xFF) as u8);
        self.write_byte(addr.wrapping_add(1), (value >> 8) as u8);
    }
    
    pub fn load_program(&mut self, program: &[u8], start_addr: u16) {
        let start = start_addr as usize;
        let end = start + program.len();
        if end <= self.ram.len() {
            self.ram[start..end].copy_from_slice(program);
        }
    }
}