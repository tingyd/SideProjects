//io.rs
use std::collections::HashMap;

pub struct IOController {
    ports: HashMap<u8, u8>,
    input_buffer: Vec<u8>,
    output_log: Vec<(u8, u8)>,
}

impl IOController {
    pub fn new() -> Self {
        IOController { ports: HashMap::new(), input_buffer: Vec::new(), output_log: Vec::new() }
    }
    
    pub fn write_port(&mut self, port: u8, value: u8) {
        self.ports.insert(port, value);
        self.output_log.push((port, value));
        match port {
            0x00 => println!("I/O: Output to port 0 - value: 0x{:02X} ({})", value, value),
            0x01 => println!("I/O: Serial output - char: '{}'", value as char),
            _ => {}
        }
    }
    
    pub fn read_port(&mut self, port: u8) -> u8 {
        if !self.input_buffer.is_empty() {
            self.input_buffer.remove(0)
        } else {
            *self.ports.get(&port).unwrap_or(&0)
        }
    }
    
    pub fn update(&mut self) {}
    
    pub fn print_stats(&self) {
        println!("Total I/O operations: {}", self.output_log.len());
        println!("Active ports: {}", self.ports.len());
        for (port, value) in &self.ports {
            println!("  Port 0x{:02X}: 0x{:02X}", port, value);
        }
    }
}