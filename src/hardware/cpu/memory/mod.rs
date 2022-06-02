pub const MAX_RAM: usize = u16::MAX as usize;

pub struct Memory {
    cells: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            cells: vec![0; MAX_RAM]
        }
    }

    pub fn length(&self) -> usize {
        self.cells.len()
    }
}

