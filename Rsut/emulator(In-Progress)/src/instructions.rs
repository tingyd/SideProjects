//instructions.rs
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    LDA_IMM, LDA_ZP, STA_ZP, STA_ABS,
    ADC_IMM, SBC_IMM, INC_ZP,
    CMP_IMM, BNE, SEC, HALT,
}

impl Instruction {
    pub fn decode(opcode: u8) -> Result<Self, String> {
        match opcode {
            0xA9 => Ok(Instruction::LDA_IMM),
            0xA5 => Ok(Instruction::LDA_ZP),
            0x85 => Ok(Instruction::STA_ZP),
            0x8D => Ok(Instruction::STA_ABS),
            0x69 => Ok(Instruction::ADC_IMM),
            0xE9 => Ok(Instruction::SBC_IMM),
            0xE6 => Ok(Instruction::INC_ZP),
            0xC9 => Ok(Instruction::CMP_IMM),
            0xD0 => Ok(Instruction::BNE),
            0x38 => Ok(Instruction::SEC),
            0xFF => Ok(Instruction::HALT),
            _ => Err(format!("Unknown opcode: 0x{:02X}", opcode)),
        }
    }
}