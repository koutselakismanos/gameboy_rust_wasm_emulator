use crate::hardware::cpu::CPU;
use crate::hardware::cpu::registers::flags::Flag;

impl CPU {
    pub fn alu_sub(&mut self, a: u8, b: u8) -> u8 {
        let result = a.wrapping_sub(b);
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Negative, true);
        self.registers.set_flag(Flag::HalfCarry, (a & 0x0F) > (b & 0x0F));
        self.registers.set_flag(Flag::Carry, a > b);
        result
    }
}