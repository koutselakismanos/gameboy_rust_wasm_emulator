use std::borrow::{Borrow, Cow};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::iter::Copied;
use std::ops::{Range, RangeInclusive};
use std::path::Path;

pub struct Cartridge {
    pub filename: String,
    pub header: CartridgeHeader,
}

#[derive(Debug)]
pub struct CartridgeHeader {
    title: String,
    cartridge_type: u8,
    rom_size: u8,
    ram_size: u8,
    header_checksum: u8,
    global_checksum: u8,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CartridgeSection {
    EntryPoint,
    NintendoLogo,
    Title,
    ManufacturerCode,
    CGBFlag,
    NewLicenseeCode,
    SGBFlag,
    CartridgeType,
    ROMSize,
    RAMSize,
    DestinationCode,
    OldLicenseeCode,
    MaskROMVersionNumber,
    HeaderChecksum,
    GlobalChecksum,
}

impl Cartridge {
    pub fn range_map(section: CartridgeSection) -> RangeInclusive<usize> {
        let mut map: HashMap<CartridgeSection, RangeInclusive<usize>> = HashMap::new();
        map.insert(CartridgeSection::EntryPoint, 0x100..=0x103);
        map.insert(CartridgeSection::NintendoLogo, 0x104..=0x133);
        map.insert(CartridgeSection::Title, 0x134..=0x143);
        map.insert(CartridgeSection::ManufacturerCode, 0x013F..=0x142);
        map.insert(CartridgeSection::CGBFlag, 0x143..=0x143);
        map.insert(CartridgeSection::NewLicenseeCode, 0x144..=0x145);
        map.insert(CartridgeSection::SGBFlag, 0x146..=0x146);
        map.insert(CartridgeSection::CartridgeType, 0x147..=0x147);
        map.insert(CartridgeSection::ROMSize, 0x148..=0x148);
        map.insert(CartridgeSection::RAMSize, 0x149..=0x149);
        map.insert(CartridgeSection::DestinationCode, 0x14A..=0x14A);
        map.insert(CartridgeSection::OldLicenseeCode, 0x14B..=0x14B);
        map.insert(CartridgeSection::MaskROMVersionNumber, 0x14C..=0x14C);
        map.insert(CartridgeSection::HeaderChecksum, 0x14D..=0x14D);
        map.insert(CartridgeSection::GlobalChecksum, 0x14E..=0x14F);

        map.remove(&section).unwrap()
    }

    pub fn get_title(&self) -> &String {
        &self.header.title
    }
    pub fn get_header(&self) -> &CartridgeHeader { &self.header }

    #[cfg(target_family = "windows")]
    pub fn load(path: &Path) -> Self {
        let mut data = vec![];
        File::open(path).and_then(|mut f| f.read_to_end(&mut data)).map_err(|_| "Could not read ROM");

        let title_range: RangeInclusive<usize> = Cartridge::range_map(CartridgeSection::Title);
        let title: String =
            String::from_utf8_lossy(&data[title_range])
                .trim_matches(char::from(0))
                .to_string();

        let cartridge_type_range: RangeInclusive<usize> = Cartridge::range_map(CartridgeSection::CartridgeType);
        let cartridge_type: u8 = data[cartridge_type_range][0];

        let rom_size_range: RangeInclusive<usize> = Cartridge::range_map(CartridgeSection::ROMSize);
        let rom_size: u8 = data[rom_size_range][0];

        let ram_size_range: RangeInclusive<usize> = Cartridge::range_map(CartridgeSection::RAMSize);
        let ram_size: u8 = data[ram_size_range][0];

        let header_checksum_range: RangeInclusive<usize> = Cartridge::range_map(CartridgeSection::HeaderChecksum);
        let header_checksum: u8 = data[header_checksum_range][0];

        let global_checksum_range: RangeInclusive<usize> = Cartridge::range_map(CartridgeSection::GlobalChecksum);
        let global_checksum: u8 = data[global_checksum_range][0];

        Cartridge {
            filename: path.file_name().unwrap().to_str().unwrap().to_string(),
            header: CartridgeHeader {
                title,
                cartridge_type,
                rom_size,
                ram_size,
                header_checksum,
                global_checksum
            },
        }
    }

    #[cfg(target_family = "wasm")]
    pub fn load(path: &Path) -> Self {}
}