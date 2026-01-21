use crate::{
    cartridge::{Cartridge, RAM_BANK_SIZE},
    memory::MemoryRegion,
};

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
