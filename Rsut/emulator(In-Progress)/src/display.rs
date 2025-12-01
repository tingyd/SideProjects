//display.rs
pub struct Display {
    width: usize,
    height: usize,
    buffer: Vec<u8>,
    dirty: bool,
}

impl Display {
    pub fn new(width: usize, height: usize) -> Self {
        Display { width, height, buffer: vec![0x20; width * height], dirty: false }
    }
    
    pub fn write(&mut self, offset: u16, value: u8) {
        let index = offset as usize;
        if index < self.buffer.len() {
            self.buffer[index] = value;
            self.dirty = true;
        }
    }
    
    pub fn render(&self) {
        println!("┌{}┐", "─".repeat(self.width));
        for y in 0..self.height {
            print!("│");
            for x in 0..self.width {
                let index = y * self.width + x;
                let ch = self.buffer[index];
                if ch >= 0x20 && ch < 0x7F {
                    print!("{}", ch as char);
                } else {
                    print!(".");
                }
            }
            println!("│");
        }
        println!("└{}┘", "─".repeat(self.width));
    }
}