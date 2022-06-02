use memory::Memory;
use registers::flags::Flag;
use registers::Registers;

use crate::hardware::cartridge::Cartridge;
use crate::hardware::cpu::instructions::{Instruction, JRTarget, Target};
use crate::hardware::cpu::instructions::Target::{A, B, BC, C, D, DE, E, H, HL, I8, L, SP, U16, U8};
use crate::hardware::utils;
use crate::hardware::utils::concatenate_bytes;

pub mod instructions;
pub mod bus;
pub mod alu;
pub mod registers;
pub mod memory;

pub struct CPU {
    pub memory: Memory,
    pub registers: Registers,
    pub cartridge: Cartridge,
    pub is_running: bool,
    pub cycles: Cycles,
}

pub struct Cycles {
    machine: usize,
    clock: usize,
}

impl CPU {
    pub fn new(cartridge: Cartridge) -> Self {
        CPU {
            memory: Memory::new(),
            registers: Registers::new(),
            cartridge,
            is_running: true,
            cycles: Cycles {
                machine: 0,
                clock: 0,
            },
        }
    }


    pub fn fetch_and_increment_pc(&mut self) -> u16 {
        let pc = self.registers.pc;
        self.registers.pc += 1;
        pc
    }

    pub fn increment_pc(&mut self) {
        self.registers.pc += 1
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn start(&mut self) {
        self.is_running = true;
    }

    pub fn step(&mut self) {
        let pc = self.fetch_and_increment_pc();
        let instruction = Instruction::from_byte(self.bus_read(pc));
        self.execute(instruction);
    }

    pub fn execute(&mut self, instruction: Instruction) {
        print!("0x{:x} ", self.registers.pc - 1);
        match instruction {
            Instruction::NOP => println!("NOP"),
            Instruction::STOP => println!("STOP"),
            Instruction::LD(a, b) => {
                match (a, b) {
                    (A, U8) => {
                        let mut pc = self.fetch_and_increment_pc();
                        let value = self.bus_read(pc);
                        self.registers.a = value;

                        println!("LD A, {:X}", value);
                    }
                    (U16, A) => {
                        let mut pc = self.fetch_and_increment_pc();
                        let lower = self.bus_read(pc);
                        pc = self.fetch_and_increment_pc();
                        let higher = self.bus_read(pc);

                        let value = concatenate_bytes(lower, higher);
                        self.bus_write(value, self.registers.a);

                        println!("LD 0x{:X}, A", value);
                    }
                    (HL, U16) => {
                        let mut pc = self.fetch_and_increment_pc();
                        let lower = self.bus_read(pc);
                        pc = self.fetch_and_increment_pc();
                        let higher = self.bus_read(pc);

                        let value = concatenate_bytes(lower, higher);
                        self.registers.set_hl(value);

                        println!("LD HL, 0x{:X}", value);
                    }
                    (C, U8) => {
                        let mut pc = self.fetch_and_increment_pc();
                        let value = self.bus_read(pc);
                        self.registers.c = value;

                        println!("LD C, 0x{:X}", value);
                    }
                    (B, U8) => {
                        let mut pc = self.fetch_and_increment_pc();
                        let value = self.bus_read(pc);
                        self.registers.b = value;

                        println!("LD B, 0x{:X}", value);
                    }
                    _ => { unimplemented!("LD {:?}, {:?}", a, b) }
                }
            }
            Instruction::LDD(a, b) => {
                match (a, b) {
                    (HL, A) => {
                        let hl = self.registers.get_hl();
                        self.bus_write(hl, self.registers.a);
                        self.registers.set_hl(hl - 1);
                        println!("LDD (HL--), A");
                    }
                    _ => { unimplemented!("LDD {:?}, {:?}", a, b) }
                }
            }
            Instruction::ADD(a, b) => println!("ADD {:?}, {:?}", a, b),
            Instruction::INC(target) => println!("INC {:?}", target),
            Instruction::DEC(target) => {
                match target {
                    B => {
                        // let result = self.registers.b.wrapping_sub(1);
                        // self.registers.b = result;
                        // self.registers.set_flag(Flag::Zero, result == 0);
                        // self.registers.set_flag(Flag::Negative, true);
                        self.registers.b = self.alu_sub(self.registers.b, 1);
                        println!("DEC B");
                    }
                    C => {
                        // let result = self.registers.b.wrapping_sub(1);
                        // self.registers.b = result;
                        // self.registers.set_flag(Flag::Zero, result == 0);
                        // self.registers.set_flag(Flag::Negative, true);
                        self.registers.c = self.alu_sub(self.registers.c, 1);
                        println!("DEC C");
                    }
                    _ => { unimplemented!("DEC {:?}", target) }
                }
            }
            Instruction::RLCA => println!("RLCA"),
            Instruction::RRCA => println!("RRCA"),
            Instruction::CP(a, b) => {
                match (a, b) {
                    (A, U8) => {
                        let mut pc = self.fetch_and_increment_pc();
                        let value1 = self.registers.a;
                        let value2 = self.bus_read(pc);

                        let total = value1.wrapping_sub(value2);
                        // self.registers.set_flag(Flag::Zero, total == 0);
                        // self.registers.set_flag(Flag::Negative, true);
                        // self.registers.set_flag(Flag::HalfCarry, (value1 & 0x0F) > (value2 & 0x0F));
                        // self.registers.set_flag(Flag::Carry, value1 > value2);
                        self.alu_sub(value1, value2);
                        println!("CP A, {:x}", value2)
                    }
                    _ => { unimplemented!("CP {:?}, {:?}", a, b) }
                }
            }
            Instruction::JP(target) => {
                match target {
                    U16 => {
                        let mut pc = self.fetch_and_increment_pc();
                        let lower = self.bus_read(pc);
                        pc = self.fetch_and_increment_pc();
                        let higher = self.bus_read(pc);

                        let value = concatenate_bytes(lower, higher);
                        self.registers.pc = value;
                        println!("JP 0x{:x}", value)
                    }
                    _ => { unimplemented!("JP {:?}", target) }
                }
            }
            Instruction::XOR(a, b) => {
                match (a, b) {
                    (A, A) => {
                        let reg_a = self.registers.a;
                        let result = reg_a ^ reg_a;
                        self.registers.a = result;
                        self.registers.set_flag(Flag::Zero, result == 0);
                        self.registers.set_flag(Flag::Negative, false);
                        self.registers.set_flag(Flag::HalfCarry, false);
                        self.registers.set_flag(Flag::Carry, false);
                        println!("XOR A, A");
                    }
                    _ => { unimplemented!("XOR {:?}, {:?}", a, b) }
                }
            }
            Instruction::JR(target) => {
                match target {
                    JRTarget::Z => {
                        let pc = self.fetch_and_increment_pc();
                        let n = self.bus_read(pc) as i8;
                        let addr = self.registers.pc.wrapping_add(n as u16);
                        if self.registers.get_flag(Flag::Zero) {
                            self.registers.pc = addr;
                        }

                        println!("JR Z, 0x{:X}", addr)
                    }
                    JRTarget::I8 => {
                        let pc = self.fetch_and_increment_pc();
                        let n = self.bus_read(pc) as i8;
                        let addr = self.registers.pc.wrapping_add(n as u16);
                        self.registers.pc = addr;
                        println!("JR {:x}", addr)
                    }
                    JRTarget::NZ => {
                        let pc = self.fetch_and_increment_pc();

                        let n = self.bus_read(pc) as i8;
                        let addr = self.registers.pc.wrapping_add(n as u16);
                        if !self.registers.get_flag(Flag::Zero) {
                            self.registers.pc = addr;
                        }

                        println!("JR NZ, 0x{:X}", addr)
                    }
                    JRTarget::C => {
                        let pc = self.fetch_and_increment_pc();
                        let n = self.bus_read(pc) as i8;
                        let addr = self.registers.pc.wrapping_add(n as u16);
                        if self.registers.get_flag(Flag::Carry) {
                            self.registers.pc = addr;
                        }

                        println!("JR C, 0x{:X}", addr)
                    }

                    JRTarget::NC => {
                        let pc = self.fetch_and_increment_pc();
                        let n = self.bus_read(pc);
                        let addr = ((self.registers.pc as i16) + (n as i16)) as u16;
                        if !self.registers.get_flag(Flag::Carry) {
                            self.registers.pc = addr;
                        }

                        println!("JR NC, 0x{:X}", addr)
                    }
                    _ => { unimplemented!("JR {:?}", target) }
                }
            }
            Instruction::UNKNOWN(opcode) => println!("UNKNOWN OPCODE: 0x{:X}", opcode),
            _ => unimplemented!("{:?}", instruction)
        }
    }
}