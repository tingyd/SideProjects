//main.rs
mod cpu;
mod memory;
mod display;
mod io;
mod instructions;

use cpu::CPU;
use memory::Memory;
use display::Display;
use io::IOController;

fn main() {
    println!("=== 8-Bit Emulator System ===\n");
    let mut memory = Memory::new();
    let mut display = Display::new(32, 16);
    let mut io = IOController::new();
    let mut cpu = CPU::new();
    
    load_demo_program(&mut memory);
    memory.write_word(0xFFFC, 0x8000);
    cpu.reset(&memory);
    
    println!("Running emulator...\n");
    let mut cycles = 0;
    let max_cycles = 10000;
    
    while cycles < max_cycles {
        match cpu.step(&mut memory, &mut display, &mut io) {
            Ok(cycle_count) => {
                cycles += cycle_count as u32;
                if cycles % 100 == 0 {
                    io.update();
                }
            }
            Err(e) => {
                if e.contains("HALT") {
                    println!("\nProgram halted after {} cycles", cycles);
                    break;
                }
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
    
    println!("\n{:?}", cpu);
    println!("\nDisplay output:");
    display.render();
    println!("\nI/O Statistics:");
    io.print_stats();
}

fn load_demo_program(memory: &mut Memory) {
    let program = vec![
        0xA9, 0x00, 0x85, 0x00, 0x85, 0x01,
        0xA9, 0x2A, 0x8D, 0x00, 0x40,
        0xE6, 0x00, 0xA5, 0x00, 0xC9, 0x10, 0xD0, 0xF3,
        0xA9, 0x05, 0x69, 0x03, 0x85, 0x10,
        0xA9, 0x0A, 0x38, 0xE9, 0x04, 0x85, 0x11,
        0xA5, 0x10, 0x8D, 0x00, 0x50,
        0xFF,
    ];
    memory.load_program(&program, 0x8000);
}