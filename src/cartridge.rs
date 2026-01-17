use std::{fmt::Debug, path::Path};

use crate::memory::MemoryRegion;

pub enum CartridgeType {
    Basic,
    MBC1,
    // MBC1M,
    // MBC2,
    // MBC3,
    // MBC30,
    // MBC5,
    // MBC6,
    // MBC7,
    // HuC1,
    // HuC3,
}

pub fn create_cartridge(path: &Path, catridge_type: CartridgeType) -> Box<dyn Cartridge> {
    match catridge_type {
        CartridgeType::MBC1 => Box::new(MBC1Cartridge::new(path)),
        CartridgeType::Basic => todo!(),
    }
}

pub trait Cartridge: Debug {
    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, val: u8);
}

#[derive(Debug)]
pub struct BasicCartridge {
    data: [u8; 0x8000],
}

impl BasicCartridge {}

impl Cartridge for BasicCartridge {
    fn read_byte(&self, address: u16) -> u8 {
        let region = MemoryRegion::from_addr(address, false);

        match region {
            MemoryRegion::GameROMBank0 | MemoryRegion::GameROMBankN => self.data[address as usize],
            _ => 0xFF,
        }
    }

    fn write_byte(&mut self, address: u16, val: u8) {
        let region = MemoryRegion::from_addr(address, false);

        match region {
            MemoryRegion::GameROMBank0 | MemoryRegion::GameROMBankN => {
                self.data[address as usize] = val
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct MBC1Cartridge {}

impl MBC1Cartridge {
    pub fn new(path: &Path) -> Self {
        todo!()
    }

    pub fn new_and_empty() -> Self {
        todo!()
    }
}

impl Cartridge for MBC1Cartridge {
    fn read_byte(&self, address: u16) -> u8 {
        todo!()
    }

    fn write_byte(&mut self, address: u16, val: u8) {
        todo!()
    }
}

// pub struct MBC1MCartridge {}
// pub struct MBC2Cartridge {}
// pub struct MBC3Cartridge {}
// pub struct MBC30Cartridge {}
// pub struct MBC5Cartridge {}
// pub struct MBC6Cartridge {}
// pub struct MBC7Cartridge {}
// pub struct HuC1Cartridge {}
// pub struct HuC3Cartridge {}
