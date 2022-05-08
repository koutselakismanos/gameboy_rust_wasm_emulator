use std::path::Path;
use gameboy_rust_webassembly_emulator::hardware::cartridge::Cartridge;
use gameboy_rust_webassembly_emulator::hardware::memory::Memory;

fn main() {
    // let memory = Memory::new();
    // println!("{}", memory.length());
    Cartridge::load(Path::new("hehe"))
}