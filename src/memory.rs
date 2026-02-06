use std::{cell::RefCell, fs, rc::Rc};

use crate::{
    cartridge::{basic::BasicCartridge, Cartridge},
    hardware_registers::{HardwareRegisters, RegisterAddresses, IE, LCDC},
    ppu::{PPUMode, PPU},
};

const BOOT_ROM_LOCK_REGISTER: u16 = 0xFF50;
const BOOT_ROM_BIN_PATH: &'static str = "resources/dmg_boot.bin";

#[derive(Debug, PartialEq, Eq)]
pub enum MemoryRegion {
    BootROM,
    GameROMBank0,
    GameROMBankN,
    TileRAM,
    BackgroundMap,
    CartridgeRAM,
    WorkingRAM,
    EchoRAM,
    OAM,
    Unused,
    IO,
    HighRAM,
    InterruptEnabledRegister,
}

impl MemoryRegion {
    pub fn from_addr(addr: u16, is_booting: bool) -> Self {
        if addr <= 0x00FF && is_booting {
            MemoryRegion::BootROM
        } else if addr <= 0x3FFF {
            MemoryRegion::GameROMBank0
        } else if addr <= 0x7FFF {
            MemoryRegion::GameROMBankN
        } else if addr <= 0x97FF {
            MemoryRegion::TileRAM
        } else if addr <= 0x9FFF {
            MemoryRegion::BackgroundMap
        } else if addr <= 0xBFFF {
            MemoryRegion::CartridgeRAM
        } else if addr <= 0xDFFF {
            MemoryRegion::WorkingRAM
        } else if addr <= 0xFDFF {
            MemoryRegion::EchoRAM
        } else if addr <= 0xFE9F {
            MemoryRegion::OAM
        } else if addr <= 0xFEFF {
            MemoryRegion::Unused
        } else if addr <= 0xFF7F {
            MemoryRegion::IO
        } else if addr <= 0xFFFE {
            MemoryRegion::HighRAM
        } else {
            MemoryRegion::InterruptEnabledRegister
        }
        //TODO: implement the rest of these
    }
}

#[derive(Debug)]
pub struct MemoryBus {
    // Memory Map
    //
    // 0x0000 - 0x00FF: Boot ROM
    //
    // 0x0000 - 0x3FFF: Game ROM Bank 0
    //
    // 0x4000 - 0x7FFF: Game ROM Bank N
    //
    // 0x8000 - 0x97FF: Tile RAM
    //
    // 0x9800 - 0x9FFF: Background Map
    //
    // 0xA000 - 0xBFFF: Cartridge RAM
    //
    // 0xC000 - 0xDFFF: Working RAM
    //
    // 0xE000 - 0xFDFF: Echo RAM
    //
    // 0xFE00 - 0xFE9F: Object Atribute Memory (OAM)
    //
    // 0xFEA0 - OXFEFF: Unusued
    //
    // 0xFF00 - OxFF7F: I/O Registers
    //
    // 0xFF80 - 0xFFFE: High RAM Area
    //
    // 0xFFFF: Interrupt Enabled Register
    boot_rom: [u8; 0x100],

    memory: [u8; 0x10000],

    cartridge: Box<dyn Cartridge>,

    ppu: Rc<RefCell<PPU>>,

    pub registers: HardwareRegisters,
}

impl MemoryBus {
    pub fn new_and_empty(cartridge: Option<Box<dyn Cartridge>>, ppu: Rc<RefCell<PPU>>) -> Self {
        let bus = Self {
            boot_rom: [0; 0x100],
            memory: [0; 0x10000],
            // gpu: GPU::new(),
            cartridge: cartridge.unwrap_or_else(|| Box::new(BasicCartridge::new())),
            ppu,
            registers: HardwareRegisters::from_zeros(),
        };

        bus
    }

    pub fn new_and_load_bios(cartridge: Option<Box<dyn Cartridge>>, ppu: Rc<RefCell<PPU>>) -> Self {
        let mut bus = Self::new_and_empty(cartridge, ppu);
        let read_res = fs::read(BOOT_ROM_BIN_PATH);

        match read_res {
            Ok(data) => {
                let n_data = data.len();

                for i in 0..n_data {
                    bus.boot_rom[i] = data[i];
                }
            }
            Err(_) => panic!("Failed to load boot rom"),
        };

        bus
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        let booting = self.memory[BOOT_ROM_LOCK_REGISTER as usize] & 1 == 0;

        let region = MemoryRegion::from_addr(address, booting);

        match region {
            MemoryRegion::BootROM => {
                if booting {
                    self.boot_rom[address as usize]
                } else {
                    self.memory[address as usize]
                }
            }
            MemoryRegion::TileRAM => self.ppu.borrow().read_vram(address),
            MemoryRegion::BackgroundMap => self.ppu.borrow().read_tile_map(address),
            MemoryRegion::OAM => self.ppu.borrow().read_oam(address),

            // Handle sections that go to the cartridge
            MemoryRegion::GameROMBank0
            | MemoryRegion::GameROMBankN
            | MemoryRegion::CartridgeRAM => self.cartridge.read_byte(address),

            MemoryRegion::IO => match RegisterAddresses::from_address(address) {
                Some(reg) => match reg {
                    RegisterAddresses::LCDC => self.registers.LCDC.to_byte(),
                    RegisterAddresses::LY => self.registers.LY,
                    RegisterAddresses::IE => self.registers.IE.to_byte(),
                    RegisterAddresses::SCY => self.registers.SCY,
                    RegisterAddresses::SCX => self.registers.SCX,
                    RegisterAddresses::WX => self.registers.WX,
                    RegisterAddresses::WY => self.registers.WY,
                    RegisterAddresses::STAT => self.registers.STAT.to_byte(),
                },
                None => self.memory[address as usize],
            },

            _ => self.memory[address as usize],
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        let booting = self.memory[BOOT_ROM_LOCK_REGISTER as usize] & 1 == 1;

        let region = MemoryRegion::from_addr(address, booting);

        match region {
            // boot rom cannot be written to
            MemoryRegion::BootROM => {}

            // graphics RAM should be handled by the GPU
            MemoryRegion::TileRAM => self.ppu.borrow_mut().write_vram(address, value),
            MemoryRegion::OAM => self.ppu.borrow_mut().write_oam(address, value),
            MemoryRegion::BackgroundMap => self.ppu.borrow_mut().write_tile_map(address, value),

            // Handle sections that go to the cartridge
            MemoryRegion::GameROMBank0
            | MemoryRegion::GameROMBankN
            | MemoryRegion::CartridgeRAM => self.cartridge.write_byte(address, value),

            MemoryRegion::IO => match RegisterAddresses::from_address(address) {
                Some(reg) => match reg {
                    RegisterAddresses::LCDC => self.registers.LCDC = LCDC::from(value),
                    RegisterAddresses::LY => self.registers.LY = value,
                    RegisterAddresses::IE => self.registers.IE = IE::from(value),
                    RegisterAddresses::SCY => self.registers.SCY = value,
                    RegisterAddresses::SCX => self.registers.SCX = value,
                    RegisterAddresses::WX => self.registers.WX = value,
                    RegisterAddresses::WY => self.registers.WY = value,
                    RegisterAddresses::STAT => self.registers.STAT.write_byte(value),
                },
                None => self.memory[address as usize] = value,
            },
            // everything else can be written as usual
            _ => self.memory[address as usize] = value,
        }
    }

    pub fn reset(&mut self) {
        self.memory[BOOT_ROM_LOCK_REGISTER as usize] = 0;
    }

    pub fn boot_mode_active(&self) -> bool {
        self.memory[BOOT_ROM_LOCK_REGISTER as usize] == 0
    }

    pub fn wrapping_inc_byte(&mut self, address: u16, wrapping_val: u8) -> u8 {
        let curr_val = self.read_byte(address);
        let new_val = (curr_val + 1) % wrapping_val;
        self.write_byte(address, new_val);

        self.read_byte(address)
    }

    pub fn update_ppu_lock(&mut self, _ppu_mode: PPUMode) {}
}
