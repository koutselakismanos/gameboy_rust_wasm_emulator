use std::fmt;
use std::fmt::Formatter;

use crate::hardware::cpu::instructions::Target::{A, B, BC, C, D, DE, E, H, HL, I8, L, SP, U16, U8};

#[derive(Debug, Copy, Clone)]
pub enum Target {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    // Z,
    // NZ,
    // NC,
    // C,
    BC,
    DE,
    HL,
    SP,
    U8,
    U16,
    I8,
}

#[derive(Debug, Copy, Clone)]
pub enum JRTarget {
    N,
    Z,
    NZ,
    C,
    NC,
    I8,
}


#[derive(Debug)]
pub enum Instruction {
    UNKNOWN(u8),
    NOP,
    STOP,
    ADD(Target, Target),
    INC(Target),
    LD(Target, Target),
    LDI(Target, Target),
    LDD(Target, Target),
    DEC(Target),
    JP(Target),
    CP(Target, Target),
    JR(JRTarget),
    XOR(Target, Target),
    RLCA,
    RRCA,
    DI,
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Instruction {
        match byte {
            0x00 => Instruction::NOP,
            0x01 => Instruction::LD(BC, U16),

            0x02 => Instruction::LD(BC, A),
            0x03 => Instruction::INC(BC),
            0x04 => Instruction::INC(B),
            0x05 => Instruction::DEC(B),
            0x06 => Instruction::LD(B, U8),
            0x07 => Instruction::RLCA,
            0x08 => Instruction::LD(U16, SP),
            0x09 => Instruction::ADD(HL, BC),
            0x0A => Instruction::LD(A, BC),
            0x0B => Instruction::DEC(BC),
            0x0C => Instruction::INC(C),
            0x0D => Instruction::DEC(C),
            0x0E => Instruction::LD(C, U8),
            0x0F => Instruction::RRCA,
            0x10 => Instruction::STOP,
            0xC3 => Instruction::JP(U16),
            0xFE => Instruction::CP(A, U8),
            0x28 => Instruction::JR(JRTarget::Z),
            0xAF => Instruction::XOR(A, A),
            0x18 => Instruction::JR(JRTarget::I8),
            0xEA => Instruction::LD(U16, A),
            0x21 => Instruction::LD(HL, U16),
            0x32 => Instruction::LDD(HL, A),
            0x20 => Instruction::JR(JRTarget::NZ),
            0x3E => Instruction::LD(A, U8),
            0xF3 => Instruction::DI,
            0xE0 => Instruction::LD(U8, A),

            _ => Instruction::UNKNOWN(byte)
        }
    }
}
