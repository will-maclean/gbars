use std::{fmt::Debug, path::Path};

use crate::memory::MemoryRegion;

// const ROM_BANK_SIZE: usize = 0x4000; // 16 KiB
const RAM_BANK_SIZE: usize = 0x2000; // 8 KiB

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

    match cart_type {
        CartridgeType::MBC1 | CartridgeType::MBC1Ram => {
            Box::new(MBC1Cartridge::new(cart, ram_banks, rom_banks))
        }
        _ => panic!("Unimplemented cartridge selected! {:?}", cart_type),
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

    fn write_byte(&mut self, _address: u16, _val: u8) {
        // no RAM, just ROM
    }
}

#[derive(Debug, Default)]
struct MBC1Registers {
    // enables writing to RAM
    ram_enable: bool,

    // controls selected ROM bank
    // 5 bit register
    rom_bank_number: u8,

    // secondary bank
    secondary_bank: u8,

    banking_mode_select: u8,
}

#[derive(Debug)]
pub struct MBC1Cartridge {
    rom: Vec<u8>,
    ram: Vec<u8>,
    registers: MBC1Registers,
    rom_banks: usize,
    ram_banks: usize,
}

impl MBC1Cartridge {
    pub fn new(rom: Vec<u8>, ram_banks: usize, rom_banks: usize) -> Self {
        Self {
            rom,
            ram: vec![0; ram_banks * RAM_BANK_SIZE],
            registers: Default::default(),
            rom_banks,
            ram_banks,
        }
    }

    pub fn new_and_empty() -> Self {
        todo!()
    }
}

impl Cartridge for MBC1Cartridge {
    fn read_byte(&self, address: u16) -> u8 {
        match MemoryRegion::from_addr(address, false) {
            MemoryRegion::GameROMBank0 | MemoryRegion::GameROMBankN => {
                let mut rom_addr = address as usize & 0x1FFF;

                if address > 0x4000 || self.registers.banking_mode_select > 0 {
                    rom_addr |= (self.registers.secondary_bank as usize) << 19;
                }

                if address > 0x400 {
                    rom_addr |= (self.registers.rom_bank_number as usize) << 14;
                }

                self.rom[rom_addr]
            }
            MemoryRegion::CartridgeRAM => {
                if self.registers.ram_enable {
                    let mut ram_addr = address & 0x1FFF;

                    if self.registers.banking_mode_select > 0 {
                        ram_addr |= (self.registers.secondary_bank as u16) << 13;
                    }

                    self.ram[ram_addr as usize]
                } else {
                    0xFF
                }
            }
            _ => panic!("Bad read address for a cartridge!"),
        }
    }

    fn write_byte(&mut self, address: u16, val: u8) {
        match address {
            0x0000..=0x1FFF => {
                // ram enable
                // set TRUE if lower 4 bits = 0xA, and FALSE otherwise
                self.registers.ram_enable = val & 0b1111 == 0xa;
            }
            0x2000..=0x3FFF => {
                // rom bank number
                let mut new_val = val & 0b11111;

                if new_val == 0 {
                    new_val = 1;
                }

                // TODO: handle if new val > number of banks

                self.registers.rom_bank_number = new_val;
            }

            0x4000..=0x5FFF => {
                // secondary bank number
                self.registers.secondary_bank = val & 0b11;
            }

            0x6000..=0x7FFF => {
                self.registers.banking_mode_select = val & 0b1;
            }

            _ => panic!(
                "Writing to bad address on MBC1 chip! Address = 0x{:x}",
                address
            ),
        }
    }
}
