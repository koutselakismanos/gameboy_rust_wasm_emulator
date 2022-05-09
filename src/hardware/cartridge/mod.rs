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
    entry_point: [u8; 4],
    nintendo_logo: [u8; 48],
    title: String,
    manufacturer_code: [u8; 4],
    cgb_flag: u8,
    new_licensee_code: [u8; 2],
    sgb_flag: u8,
    cartridge_type: u8,
    rom_size: u8,
    ram_size: u8,
    destination_code: u8,
    old_licensee_code: u8,
    mask_rom_version_number: u8,
    header_checksum: u8,
    global_checksum: [u8; 2],
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

        let entry_point_range = Cartridge::range_map(CartridgeSection::EntryPoint);
        let entry_point = data[entry_point_range].try_into().unwrap();

        let nintendo_logo_range = Cartridge::range_map(CartridgeSection::NintendoLogo);
        let nintendo_logo = data[nintendo_logo_range].try_into().unwrap();

        let title_range: RangeInclusive<usize> = Cartridge::range_map(CartridgeSection::Title);
        let title: String =
            String::from_utf8_lossy(&data[title_range])
                .trim_matches(char::from(0))
                .to_string();

        let manufacturer_code_range = Cartridge::range_map(CartridgeSection::ManufacturerCode);
        let manufacturer_code  = data[manufacturer_code_range].try_into().unwrap();

        let cgb_flag_range = Cartridge::range_map(CartridgeSection::CGBFlag);
        let cgb_flag = data[cgb_flag_range][0];

        let new_licensee_code_range = Cartridge::range_map(CartridgeSection::NewLicenseeCode);
        let new_licensee_code = data[new_licensee_code_range].try_into().unwrap();

        let sgb_flag_range = Cartridge::range_map(CartridgeSection::SGBFlag);
        let sgb_flag = data[sgb_flag_range][0];

        let cartridge_type_range = Cartridge::range_map(CartridgeSection::CartridgeType);
        let cartridge_type: u8 = data[cartridge_type_range][0];

        let rom_size_range = Cartridge::range_map(CartridgeSection::ROMSize);
        let rom_size: u8 = data[rom_size_range][0];

        let ram_size_range = Cartridge::range_map(CartridgeSection::RAMSize);
        let ram_size: u8 = data[ram_size_range][0];

        let destination_code_range = Cartridge::range_map(CartridgeSection::DestinationCode);
        let destination_code: u8 = data[destination_code_range][0];

        let old_licensee_code_range = Cartridge::range_map(CartridgeSection::OldLicenseeCode);
        let old_licensee_code: u8 = data[old_licensee_code_range][0];

        let mask_rom_version_number_range = Cartridge::range_map(CartridgeSection::MaskROMVersionNumber);
        let mask_rom_version_number: u8 = data[mask_rom_version_number_range][0];

        let header_checksum_range = Cartridge::range_map(CartridgeSection::HeaderChecksum);
        let header_checksum: u8 = data[header_checksum_range][0];

        let global_checksum_range = Cartridge::range_map(CartridgeSection::GlobalChecksum);
        let global_checksum = data[global_checksum_range].try_into().unwrap(); // TODO: should return a 16bit global_checksum

        Cartridge {
            filename: path.file_name().unwrap().to_str().unwrap().to_string(),
            header: CartridgeHeader {
                entry_point,
                nintendo_logo,
                title,
                manufacturer_code,
                cgb_flag,
                new_licensee_code,
                sgb_flag,
                cartridge_type,
                rom_size,
                ram_size,
                destination_code,
                old_licensee_code,
                mask_rom_version_number,
                header_checksum,
                global_checksum,
            },
        }
    }

    #[cfg(target_family = "wasm")]
    pub fn load(path: &Path) -> Self {}
}