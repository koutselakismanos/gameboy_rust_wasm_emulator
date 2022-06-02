pub fn concatenate_bytes(lower: u8, higher: u8) -> u16 {
    ((higher as u16) << 8) | lower as u16
}
