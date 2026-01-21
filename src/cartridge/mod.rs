use std::{fmt::Debug, path::Path};

use crate::cartridge::{mbc1::MBC1Cartridge, mbc3::MBC3Cartridge};

pub mod basic;
pub mod mbc1;
pub mod mbc3;

const RAM_BANK_SIZE: usize = 0x2000;
const ROM_BANK_SIZE: usize = 0x4000;

#[derive(Debug)]
enum CartridgeType {
    ROMOnly,
    MBC1,
    MBC1Ram,
    MBC1RamBat,
    MBC2,
    MBC2Bat,
    RomRam,
    RomRamBat,
    MBC3TimerBat,
    MBC3RamTimerBat,
    MBC3,
    MBC3Ram,
    MBC3RamBat,
    MBC4,
    MBC4Ram,
    MBC4RamBat,
    MBC5,
    MBC5Ram,
    MBC5RamBat,
}

impl CartridgeType {
    fn has_battery(&self) -> bool {
        match self {
            CartridgeType::MBC1Ram
            | CartridgeType::ROMOnly
            | CartridgeType::MBC1
            | CartridgeType::MBC2
            | CartridgeType::MBC3
            | CartridgeType::MBC4
            | CartridgeType::MBC5Ram
            | CartridgeType::MBC4Ram
            | CartridgeType::MBC3Ram
            | CartridgeType::MBC5 => false,

            CartridgeType::MBC1RamBat
            | CartridgeType::RomRam
            | CartridgeType::RomRamBat
            | CartridgeType::MBC2Bat
            | CartridgeType::MBC3TimerBat
            | CartridgeType::MBC3RamTimerBat
            | CartridgeType::MBC3RamBat
            | CartridgeType::MBC4RamBat
            | CartridgeType::MBC5RamBat => true,
        }
    }
    fn has_ram(&self) -> bool {
        match self {
            CartridgeType::ROMOnly
            | CartridgeType::MBC1
            | CartridgeType::MBC2
            | CartridgeType::MBC2Bat
            | CartridgeType::MBC3TimerBat
            | CartridgeType::MBC3
            | CartridgeType::MBC4
            | CartridgeType::MBC5 => false,

            CartridgeType::MBC1Ram
            | CartridgeType::MBC1RamBat
            | CartridgeType::RomRam
            | CartridgeType::RomRamBat
            | CartridgeType::MBC3RamTimerBat
            | CartridgeType::MBC3Ram
            | CartridgeType::MBC3RamBat
            | CartridgeType::MBC4Ram
            | CartridgeType::MBC4RamBat
            | CartridgeType::MBC5Ram
            | CartridgeType::MBC5RamBat => true,
        }
    }
}

impl From<u8> for CartridgeType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::ROMOnly,
            0x01 => Self::MBC1,
            0x02 => Self::MBC1Ram,
            0x03 => Self::MBC1RamBat,
            0x05 => Self::MBC2,
            0x06 => Self::MBC2Bat,
            0x08 => Self::RomRam,
            0x09 => Self::RomRamBat,
            0x0F => Self::MBC3TimerBat,
            0x10 => Self::MBC3RamTimerBat,
            0x11 => Self::MBC3,
            0x12 => Self::MBC3Ram,
            0x13 => Self::MBC3RamBat,
            0x15 => Self::MBC4,
            0x16 => Self::MBC4Ram,
            0x17 => Self::MBC4RamBat,
            0x19 => Self::MBC5,
            0x1A => Self::MBC5Ram,
            0x1B => Self::MBC5RamBat,
            _ => panic!("Unknown Cartridge Type value: 0x{:x}", value),
        }
    }
}

enum CartridgeHeaderConstants {
    RAMSize,
    ROMSize,
    CartridgeType,
}

impl CartridgeHeaderConstants {
    fn get_address(&self) -> usize {
        match self {
            CartridgeHeaderConstants::RAMSize => 0x149,
            CartridgeHeaderConstants::ROMSize => 0x148,
            CartridgeHeaderConstants::CartridgeType => 0x147,
        }
    }
}

fn get_ram_banks(ram_constant: u8) -> usize {
    match ram_constant {
        0 => 0,
        2 => 1,
        3 => 4,
        4 => 16,
        5 => 8,
        _ => panic!("Unknown Cartridge RAM size value: 0x{:x}", ram_constant),
    }
}

fn get_rom_banks(ram_constant: u8) -> usize {
    match ram_constant {
        0 => 2,
        1 => 4,
        2 => 8,
        3 => 16,
        4 => 32,
        5 => 64,
        6 => 128,
        7 => 256,
        8 => 512,
        _ => panic!("Unknown Cartridge RAM size value: 0x{:x}", ram_constant),
    }
}

pub fn create_cartridge(path: &Path) -> Box<dyn Cartridge> {
    let cart = std::fs::read(path).unwrap();

    let cart_type =
        CartridgeType::from(cart[CartridgeHeaderConstants::CartridgeType.get_address()]);
    let ram_banks = get_ram_banks(cart[CartridgeHeaderConstants::RAMSize.get_address()]);
    let rom_banks = get_ram_banks(cart[CartridgeHeaderConstants::ROMSize.get_address()]);
    let battery = cart_type.has_battery();
    let has_ram = cart_type.has_ram();

    match cart_type {
        CartridgeType::MBC1 | CartridgeType::MBC1Ram => {
            Box::new(MBC1Cartridge::new(cart, ram_banks, rom_banks))
        }
        CartridgeType::MBC3
        | CartridgeType::MBC3Ram
        | CartridgeType::MBC3RamBat
        | CartridgeType::MBC3RamTimerBat => Box::new(MBC3Cartridge::new(
            cart,
            if has_ram { Some(ram_banks) } else { None },
            battery,
        )),
        _ => panic!("Unimplemented cartridge selected! {:?}", cart_type),
    }
}

pub trait Cartridge: Debug {
    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, val: u8);

    // let the cartridge have a tick every M cycle.
    // Not necessary in most cases, but useful for
    // when cartridges have onboard clocks
    fn tick(&mut self) {}
}
