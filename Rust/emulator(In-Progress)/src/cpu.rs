//cpu.rs
use crate::memory::Memory;
use crate::display::Display;
use crate::io::IOController;
use crate::instructions::Instruction;

pub struct CPU {
    pub pc: u16,
    pub sp: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub status: u8,
}

const FLAG_CARRY: u8 = 0b0000_0001;
const FLAG_ZERO: u8 = 0b0000_0010;
const FLAG_INTERRUPT: u8 = 0b0000_0100;
const FLAG_DECIMAL: u8 = 0b0000_1000;
const FLAG_BREAK: u8 = 0b0001_0000;
const FLAG_OVERFLOW: u8 = 0b0100_0000;
const FLAG_NEGATIVE: u8 = 0b1000_0000;

impl CPU {
    pub fn new() -> Self {
        CPU { pc: 0, sp: 0xFF, a: 0, x: 0, y: 0, status: 0x20 }
    }
    
    pub fn reset(&mut self, memory: &Memory) {
        self.pc = memory.read_word(0xFFFC);
        self.sp = 0xFF;
        self.status = 0x20;
    }
    
    pub fn step(&mut self, memory: &mut Memory, display: &mut Display, io: &mut IOController) -> Result<u8, String> {
        let opcode = self.fetch_byte(memory);
        let instruction = Instruction::decode(opcode)?;
        self.execute(instruction, memory, display, io)
    }
    
    fn fetch_byte(&mut self, memory: &Memory) -> u8 {
        let byte = memory.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }
    
    fn fetch_word(&mut self, memory: &Memory) -> u16 {
        let word = memory.read_word(self.pc);
        self.pc = self.pc.wrapping_add(2);
        word
    }
    
    pub fn set_flag(&mut self, flag: u8, value: bool) {
        if value { self.status |= flag; } else { self.status &= !flag; }
    }
    
    pub fn get_flag(&self, flag: u8) -> bool {
        (self.status & flag) != 0
    }
    
    pub fn update_zero_negative_flags(&mut self, value: u8) {
        self.set_flag(FLAG_ZERO, value == 0);
        self.set_flag(FLAG_NEGATIVE, (value & 0x80) != 0);
    }
    
    fn execute(&mut self, instruction: Instruction, memory: &mut Memory, display: &mut Display, io: &mut IOController) -> Result<u8, String> {
        match instruction {
            Instruction::LDA_IMM => {
                let value = self.fetch_byte(memory);
                self.a = value;
                self.update_zero_negative_flags(self.a);
                Ok(2)
            }
            Instruction::LDA_ZP => {
                let addr = self.fetch_byte(memory) as u16;
                self.a = memory.read_byte(addr);
                self.update_zero_negative_flags(self.a);
                Ok(3)
            }
            Instruction::STA_ZP => {
                let addr = self.fetch_byte(memory) as u16;
                memory.write_byte(addr, self.a);
                Ok(3)
            }
            Instruction::STA_ABS => {
                let addr = self.fetch_word(memory);
                if addr >= 0x4000 && addr < 0x5000 {
                    display.write(addr - 0x4000, self.a);
                } else if addr >= 0x5000 && addr < 0x6000 {
                    io.write_port((addr - 0x5000) as u8, self.a);
                } else {
                    memory.write_byte(addr, self.a);
                }
                Ok(4)
            }
            Instruction::ADC_IMM => {
                let value = self.fetch_byte(memory);
                let carry = if self.get_flag(FLAG_CARRY) { 1 } else { 0 };
                let sum = self.a as u16 + value as u16 + carry as u16;
                self.set_flag(FLAG_CARRY, sum > 0xFF);
                self.set_flag(FLAG_OVERFLOW, ((self.a ^ value) & 0x80) == 0 && ((self.a ^ sum as u8) & 0x80) != 0);
                self.a = sum as u8;
                self.update_zero_negative_flags(self.a);
                Ok(2)
            }
            Instruction::SBC_IMM => {
                let value = self.fetch_byte(memory);
                let carry = if self.get_flag(FLAG_CARRY) { 1 } else { 0 };
                let diff = self.a as i16 - value as i16 - (1 - carry) as i16;
                self.set_flag(FLAG_CARRY, diff >= 0);
                self.set_flag(FLAG_OVERFLOW, ((self.a ^ value) & 0x80) != 0 && ((self.a ^ diff as u8) & 0x80) != 0);
                self.a = diff as u8;
                self.update_zero_negative_flags(self.a);
                Ok(2)
            }
            Instruction::INC_ZP => {
                let addr = self.fetch_byte(memory) as u16;
                let value = memory.read_byte(addr).wrapping_add(1);
                memory.write_byte(addr, value);
                self.update_zero_negative_flags(value);
                Ok(5)
            }
            Instruction::CMP_IMM => {
                let value = self.fetch_byte(memory);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAG_CARRY, self.a >= value);
                self.update_zero_negative_flags(result);
                Ok(2)
            }
            Instruction::BNE => {
                let offset = self.fetch_byte(memory) as i8;
                if !self.get_flag(FLAG_ZERO) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    Ok(3)
                } else {
                    Ok(2)
                }
            }
            Instruction::SEC => {
                self.set_flag(FLAG_CARRY, true);
                Ok(2)
            }
            Instruction::HALT => Err("HALT instruction executed".to_string())
        }
    }
}

impl std::fmt::Debug for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CPU {{ PC: 0x{:04X}, SP: 0x{:02X}, A: 0x{:02X}, X: 0x{:02X}, Y: 0x{:02X}, Status: 0b{:08b} }}", self.pc, self.sp, self.a, self.x, self.y, self.status)
    }
}