use std::fs;

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
        if addr < 0x00FF && is_booting {
            MemoryRegion::BootROM
        } else if addr < 0x00FF {
            MemoryRegion::GameROMBank0
        } else if addr < 0x7FFF {
            MemoryRegion::GameROMBankN
        } else {
            MemoryRegion::TileRAM
        }
        //TODO: implement the rest of these
    }
}

#[derive(Debug, Clone)]
pub struct MemoryBus {
    // Memory Map
    //
    // 0x0000 - 0x00FF: Boot ROM
    //
    // 0x0000 - 0x3FFF: Game ROM Bank 0
    //
    // 0x400 - 0x7FFF: Game ROM Bank N
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

    cartridge: Vec<u8>,
}

impl MemoryBus {
    pub fn new_and_empty() -> Self {
        Self {
            boot_rom: [0; 0x100],
            memory: [0; 0x10000],
            cartridge: Default::default(),
        }
    }

    pub fn new_and_load_bios() -> Self {
        let mut bus = Self::new_and_empty();
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

    pub fn load_cartridge(&mut self, load_pth: &str) {
        let read_res = fs::read(load_pth);

        match read_res {
            Ok(data) => {
                self.cartridge = data;

                // load bank 0
                for i in 0..0x4000 {
                    self.memory[i] = self.cartridge[i];
                }
            }
            Err(_) => panic!("Failed to load cartridge at {}", load_pth),
        };
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        let booting = self.memory[BOOT_ROM_LOCK_REGISTER as usize] & 1 == 0;

        let region = MemoryRegion::from_addr(address, booting);
        //TODO: set region access behaviour

        if booting && (region == MemoryRegion::BootROM) {
            return self.boot_rom[address as usize];
        } else {
            return self.memory[address as usize];
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        // let booting = self.memory[BOOT_ROM_LOCK_REGISTER as usize] & 1 == 1;

        // let region = MemoryRegion::from_addr(address, booting);
        //TODO: set region access behaviour

        self.memory[address as usize] = value;
    }

    pub fn set_done_booting(&mut self) {
        self.memory[BOOT_ROM_LOCK_REGISTER as usize] = 1;
    }

    pub fn reset(&mut self) {
        self.memory[BOOT_ROM_LOCK_REGISTER as usize] = 0;
    }
}