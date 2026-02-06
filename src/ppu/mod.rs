pub mod oam;
pub mod sprite;

use crate::{
    display::{SCREEN_HEIGHT_PIXELS, SCREEN_WIDTH_PIXELS},
    hardware_registers::RegisterAddresses,
    memory::MemoryBus,
    ppu::{oam::OAMEntry, sprite::Sprite},
};

#[derive(Copy, Debug, Clone)]
pub enum DrawColor {
    BLACK,
    DARKGREY,
    LIGHTGREY,
    WHITE,
}

#[derive(Debug, Copy, Clone)]
pub enum ColorIdx {
    Zero,
    One,
    Two,
    Three,
}

impl From<u8> for ColorIdx {
    fn from(value: u8) -> Self {
        match value {
            0 => ColorIdx::Zero,
            1 => ColorIdx::One,
            2 => ColorIdx::Two,
            3 => ColorIdx::Three,
            _ => panic!("Unknown colour index {value}"),
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub enum PPUMode {
    Mode0HorizontalBlank,
    Mode1VerticalBlank,
    Mode2OAMScan,
    Mode3DrawingPixels,
}

#[derive(Debug)]
pub struct PPU {
    lx: usize,
    mode: PPUMode,
    sprites: [Sprite; 384],
    oam_entries: [OAMEntry; 40],

    screen_buffer: [[DrawColor; SCREEN_WIDTH_PIXELS]; SCREEN_HEIGHT_PIXELS],

    tile_map_lower: [u8; 32 * 32],
    tile_map_upper: [u8; 32 * 32],
}

impl PPU {
    pub fn new() -> Self {
        Self {
            lx: 0,
            mode: PPUMode::Mode2OAMScan,
            sprites: [Sprite::from_zeros(); 384],
            oam_entries: [OAMEntry::from_zeros(); 40],
            screen_buffer: [[DrawColor::BLACK; SCREEN_WIDTH_PIXELS]; SCREEN_HEIGHT_PIXELS],
            tile_map_lower: [0; 32 * 32],
            tile_map_upper: [0; 32 * 32],
        }
    }

    // called once per "dot"
    //
    // A “dot” = one 222 Hz (≅ 4.194 MHz) time unit. Dots
    // remain the same regardless of whether the CPU is in
    // Double Speed mode, so there are 4 dots per Normal
    // Speed M-cycle, and 2 per Double Speed M-cycle.
    pub fn step(&mut self, memory: &mut MemoryBus) -> bool {
        if self.update_scan_registers(memory) {
            self.render_line(memory);
        }

        true
    }
    pub fn read_tile_map(&self, address: u16) -> u8 {
        if address < 0x9800 {
            panic!("Invalid vram tile read address: {address}");
        } else if address <= 0x9BFF {
            self.tile_map_lower[address as usize - 0x9800]
        } else if address <= 0x9FFF {
            self.tile_map_lower[address as usize - 0x9C00]
        } else {
            panic!("Invalid vram tile read address: {address}");
        }
    }

    pub fn write_tile_map(&mut self, address: u16, value: u8) {
        if address < 0x9800 {
            panic!("Invalid vram tile read address: {address}");
        } else if address <= 0x9BFF {
            self.tile_map_lower[address as usize - 0x9800] = value;
        } else if address <= 0x9FFF {
            self.tile_map_lower[address as usize - 0x9C00] = value;
        } else {
            panic!("Invalid vram tile read address: {address}");
        }
    }

    pub fn read_oam(&self, address: u16) -> u8 {
        if address < 0xFE00 || address > 0xFE9f {
            panic!("Attempting OAM read from invalid OAM address: {address}");
        }
        let offset_address = address as usize - 0xFE00;

        let entry = self.oam_entries[offset_address / 4];

        entry.to_byte(offset_address % 4)
    }

    pub fn write_oam(&mut self, address: u16, val: u8) {
        if address < 0xFE00 || address > 0xFE9f {
            panic!("Attempting OAM write from invalid OAM address: {address}");
        }

        let offset_address = address as usize - 0xFE00;

        self.oam_entries
            .get_mut(offset_address / 4)
            .expect(&format!(
                "Bad OAM entry access! Trying to get entry {}",
                offset_address / 4
            ))
            .write_byte(offset_address % 4, val);
    }

    pub fn read_vram(&self, address: u16) -> u8 {
        if address < 0x8000 || address > 0x97FF {
            panic!("Attempting vram read from invalid OAM address: {address}");
        }

        let offset_address = address as usize - 0x8000;

        self.sprites
            .get(offset_address / 16)
            .unwrap()
            .read_byte(offset_address % 16)
    }

    pub fn write_vram(&mut self, address: u16, val: u8) {
        if address < 0x8000 || address > 0x97FF {
            panic!("Attempting vram write from invalid OAM address: {address}");
        }

        let offset_address = address as usize - 0x8000;

        self.sprites
            .get_mut(offset_address / 16)
            .unwrap()
            .write_byte(offset_address % 16, val)
    }

    fn render_line(&mut self, memory: &mut MemoryBus) {
        for x in 0..SCREEN_WIDTH_PIXELS {
            self.screen_buffer[memory.registers.LY as usize][x] =
                self.get_pixel(memory.registers.LY, x as u8, memory);
        }
    }

    fn get_pixel(&self, y: u8, x: u8, memory: &mut MemoryBus) -> DrawColor {
        //TODO: OAM pixel

        let pixel = self.get_bg_pixel(y, x, memory);

        //TODO: palettes
        match pixel {
            ColorIdx::Zero => DrawColor::WHITE,
            ColorIdx::One => DrawColor::LIGHTGREY,
            ColorIdx::Two => DrawColor::DARKGREY,
            ColorIdx::Three => DrawColor::BLACK,
        }
    }

    fn get_bg_pixel(&self, y: u8, x: u8, memory: &mut MemoryBus) -> ColorIdx {
        let bg_x = x + memory.registers.SCX;
        let bg_y = y + memory.registers.SCY;

        let bg_x_idx = bg_x / 8;
        let bg_y_idx = bg_y / 8;

        let bg_tile_idx: u8 = self.tile_map_upper[bg_x_idx as usize * 32 + bg_y_idx as usize];

        let bg_tile: Sprite = self.sprites[bg_tile_idx as usize];

        bg_tile.pixel_at(bg_x % 8, bg_y % 8)
    }
    fn update_scan_registers(&mut self, memory: &mut MemoryBus) -> bool {
        let (ly, render_line) = if self.lx >= 456 {
            self.lx = 0;

            (
                memory.wrapping_inc_byte(RegisterAddresses::LY.address(), 153),
                true,
            )
        } else {
            self.lx += 1;

            (memory.read_byte(RegisterAddresses::LY.address()), false)
        };

        self.update_mode(ly);
        memory.update_ppu_lock(self.mode);

        render_line
    }

    fn update_mode(&mut self, ly: u8) {
        //TODO: flick from Mode 3 to Mode 0 may not be purely
        // based on number of dots spent in Mode 3... Need to investigate
        if ly >= 144 {
            self.mode = PPUMode::Mode1VerticalBlank;
        } else if self.lx <= 80 {
            self.mode = PPUMode::Mode2OAMScan;
        } else if self.lx <= 289 {
            self.mode = PPUMode::Mode3DrawingPixels;
        } else {
            self.mode = PPUMode::Mode0HorizontalBlank;
        }
    }

    pub fn get_screen_buffer(&self) -> [[DrawColor; SCREEN_WIDTH_PIXELS]; SCREEN_HEIGHT_PIXELS] {
        self.screen_buffer
    }
}
