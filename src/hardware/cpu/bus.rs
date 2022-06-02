use std::ops::RangeInclusive;
use crate::hardware::cpu::CPU;

const FIXED_ROM_BANK: RangeInclusive<u16> = 0x0000..=0x3FFF;

impl CPU {
    pub fn bus_read(&self, address: u16) -> u8 {
        if FIXED_ROM_BANK.contains(&address){
            return self.cartridge.read(address);
        }

        unimplemented!("bus read");
    }

    pub fn bus_write(&mut self, address: u16, value: u8) {
        if FIXED_ROM_BANK.contains(&address){
            self.cartridge.write(address, value);
        }
    }
}