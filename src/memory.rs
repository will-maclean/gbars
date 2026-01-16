use std::fs;

use crate::ppu::PPUMode;

const BOOT_ROM_LOCK_REGISTER: u16 = 0xFF50;
const BOOT_ROM_BIN_PATH: &'static str = "resources/dmg_boot.bin";

const VRAM_BEGIN: u16 = 0x8000;
const VRAM_END: u16 = 0x9FFF;
const VRAM_SIZE: u16 = VRAM_END - VRAM_BEGIN + 1;

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
        } else if addr < 0x97FF {
            MemoryRegion::TileRAM
        } else if addr < 0x9FFF {
            MemoryRegion::BackgroundMap
        } else if addr < 0xBFFF {
            MemoryRegion::CartridgeRAM
        } else if addr < 0xDFFF {
            MemoryRegion::WorkingRAM
        } else if addr < 0xFDFF {
            MemoryRegion::EchoRAM
        } else if addr < 0xFE9F {
            MemoryRegion::OAM
        } else if addr < 0xFEFF {
            MemoryRegion::Unused
        } else if addr < 0xFF7F {
            MemoryRegion::IO
        } else if addr < 0xFFFE {
            MemoryRegion::HighRAM
        } else {
            MemoryRegion::InterruptEnabledRegister
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

    gpu: GPU,

    cartridge: Vec<u8>,
}

impl MemoryBus {
    pub fn new_and_empty() -> Self {
        Self {
            boot_rom: [0; 0x100],
            memory: [0; 0x10000],
            cartridge: Default::default(),
            gpu: GPU::new(),
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

        match region {
            MemoryRegion::BootROM => {
                if booting {
                    self.boot_rom[address as usize]
                } else {
                    self.memory[address as usize]
                }
            }
            MemoryRegion::TileRAM => self.gpu.read_vram((address - VRAM_BEGIN) as usize),
            _ => self.memory[address as usize],
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        let booting = self.memory[BOOT_ROM_LOCK_REGISTER as usize] & 1 == 1;

        let region = MemoryRegion::from_addr(address, booting);
        //TODO: set region access behaviour

        match region {
            // boot rom cannot be written to
            MemoryRegion::BootROM | MemoryRegion::GameROMBank0 | MemoryRegion::GameROMBankN => {}

            // graphics RAM should be handled by the PGU
            MemoryRegion::TileRAM => self.gpu.write_vram(address - VRAM_BEGIN, value),

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

        new_val
    }

    pub fn update_ppu_lock(&mut self, ppu_mode: PPUMode) {
        todo!();
    }
}

// #[derive(Debug, Copy, Clone)]
// pub enum TilePixelValue {
//     Zero,
//     One,
//     Two,
//     Three,
// }
//
// pub type Tile = [[TilePixelValue; 8]; 8];
// pub fn empty_tile() -> Tile {
//     [[TilePixelValue::Zero; 8]; 8]
// }
//
// #[derive(Debug, Clone)]
// pub struct GPU {
//     vram: [u8; VRAM_SIZE as usize],
//     tile_set: [Tile; 384],
// }
//
// impl GPU {
//     pub fn new() -> Self {
//         Self {
//             vram: [0; VRAM_SIZE as usize],
//             tile_set: [empty_tile(); 384],
//         }
//     }
//     fn read_vram(&self, address: usize) -> u8 {
//         self.vram[address]
//     }
//
//     fn write_vram(&mut self, index: u16, value: u8) {
//         let index = index as usize;
//
//         self.vram[index] = value;
//         // If our index is greater than 0x1800, we're not writing to the tile set storage
//         // so we can just return.
//         if index >= 0x1800 {
//             return;
//         }
//
//         // Tiles rows are encoded in two bytes with the first byte always
//         // on an even address. Bitwise ANDing the address with 0xffe
//         // gives us the address of the first byte.
//         // For example: `12 & 0xFFFE == 12` and `13 & 0xFFFE == 12`
//         let normalized_index = index & 0xFFFE;
//
//         // First we need to get the two bytes that encode the tile row.
//         let byte1 = self.vram[normalized_index];
//         let byte2 = self.vram[normalized_index + 1];
//
//         // A tiles is 8 rows tall. Since each row is encoded with two bytes a tile
//         // is therefore 16 bytes in total.
//         let tile_index = index / 16;
//         // Every two bytes is a new row
//         let row_index = (index % 16) / 2;
//
//         // Now we're going to loop 8 times to get the 8 pixels that make up a given row.
//         for pixel_index in 0..8 {
//             // To determine a pixel's value we must first find the corresponding bit that encodes
//             // that pixels value:
//             // 1111_1111
//             // 0123 4567
//             //
//             // As you can see the bit that corresponds to the nth pixel is the bit in the nth
//             // position *from the left*. Bits are normally indexed from the right.
//             //
//             // To find the first pixel (a.k.a pixel 0) we find the left most bit (a.k.a bit 7). For
//             // the second pixel (a.k.a pixel 1) we first the second most left bit (a.k.a bit 6) and
//             // so on.
//             //
//             // We then create a mask with a 1 at that position and 0s everywhere else.
//             //
//             // Bitwise ANDing this mask with our bytes will leave that particular bit with its
//             // original value and every other bit with a 0.
//             let mask = 1 << (7 - pixel_index);
//             let lsb = byte1 & mask;
//             let msb = byte2 & mask;
//
//             // If the masked values are not 0 the masked bit must be 1. If they are 0, the masked
//             // bit must be 0.
//             //
//             // Finally we can tell which of the four tile values the pixel is. For example, if the least
//             // significant byte's bit is 1 and the most significant byte's bit is also 1, then we
//             // have tile value `Three`.
//             let value = match (lsb != 0, msb != 0) {
//                 (true, true) => TilePixelValue::Three,
//                 (false, true) => TilePixelValue::Two,
//                 (true, false) => TilePixelValue::One,
//                 (false, false) => TilePixelValue::Zero,
//             };
//
//             self.tile_set[tile_index][row_index][pixel_index] = value;
//         }
//     }
// }
