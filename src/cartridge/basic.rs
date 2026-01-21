use crate::{cartridge::Cartridge, memory::MemoryRegion};

#[derive(Debug)]
pub struct BasicCartridge {
    data: [u8; 0x8000],
}

impl BasicCartridge {
    pub fn new() -> Self {
        Self { data: [0; 0x8000] }
    }
}

impl Cartridge for BasicCartridge {
    fn read_byte(&self, address: u16) -> u8 {
        let region = MemoryRegion::from_addr(address, false);

        match region {
            MemoryRegion::GameROMBank0 | MemoryRegion::GameROMBankN => self.data[address as usize],
            _ => 0xFF,
        }
    }

    fn write_byte(&mut self, _address: u16, _val: u8) {
        // no RAM, just ROM
    }
}
