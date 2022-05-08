use std::path::Path;

use gameboy_rust_webassembly_emulator::hardware::cartridge::Cartridge;

fn main() {
    let cartridge = Cartridge::load(Path::new("roms/tetris.gb"));
    println!("TITLE: {:?}", cartridge.get_title());
    println!("ROM_SIZE: {:?}", cartridge.get_header());
}