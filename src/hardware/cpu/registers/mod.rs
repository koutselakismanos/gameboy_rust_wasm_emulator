use crate::hardware::cpu::registers::flags::Flag;
use crate::hardware::utils::concatenate_bytes;

pub mod flags;

#[derive(Copy, Clone)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0x100, // start from address 0x100
        }
    }

    pub fn set_flag(&mut self, flag: Flag, set: bool) {
        let mask = flag as u8;
        match set {
            true => self.f |= mask,
            false => self.f &= !mask,
        }
    }
    pub fn get_flag(self, flag: Flag) -> bool {
        let mask = flag as u8;
        self.f & mask == mask
    }

    pub fn get_af(&self) -> u16 {
        concatenate_bytes(self.f, self.a)
    }

    pub fn get_bc(&self) -> u16 {
        concatenate_bytes(self.c, self.b)
    }

    pub fn get_de(&self) -> u16 {
        concatenate_bytes(self.e, self.d)
    }

    pub fn get_hl(&self) -> u16 {
        concatenate_bytes(self.l, self.h)
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0xff) as u8
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xff) as u8
    }

}

#[cfg(test)]
mod tests {
    use crate::hardware::cpu::registers::flags::Flag;
    use crate::hardware::cpu::registers::Registers;

    #[test]
    fn set_flag() {
        let mut registers = Registers::new();

        registers.set_flag(Flag::Zero, true);
        assert_eq!(registers.f, 0b0001_0000);
        registers.set_flag(Flag::HalfCarry, true);
        assert_eq!(registers.f, 0b0101_0000);

        registers.set_flag(Flag::HalfCarry, false);
        assert_eq!(registers.f, 0b0001_0000);
    }
}