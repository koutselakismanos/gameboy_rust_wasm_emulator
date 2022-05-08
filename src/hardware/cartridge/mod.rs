use std::fs::File;
use std::path::Path;
use stdweb::Value::String;
use stdweb::web::FormDataEntry::File;

pub struct Cartridge {
    filename: String,
    rom_size: u32,
    rom_data: *u8,
    rom_header: String,
}

impl Cartridge {
    pub fn load(path: &Path) {
        let mut file = File::open(path);
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        assert_eq!(contents, "meow");
        Ok(());
    }
}