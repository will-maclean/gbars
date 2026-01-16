use crate::memory::MemoryBus;

pub enum DisplayRegisters {
    SCROLLX,
    SCROLLY,
    WX,
    WY,
    LCDC,
    STAT,
    LY,
    LYC,
    BGP,
    OBP0,
    OBP1,
    DMA,
}

impl DisplayRegisters {
    pub fn get_address(&self) -> usize {
        match self {
            DisplayRegisters::LCDC => 0xFF40,
            DisplayRegisters::STAT => 0xFF41,
            DisplayRegisters::SCROLLY => 0xFF42,
            DisplayRegisters::SCROLLX => 0xFF43,
            DisplayRegisters::LY => 0xFF44,
            DisplayRegisters::LYC => 0xFF45,
            DisplayRegisters::BGP => 0xFF47,
            DisplayRegisters::WY => 0xFF4A,
            DisplayRegisters::WX => 0xFF4B,
            DisplayRegisters::OBP0 => 0xFF48,
            DisplayRegisters::OBP1 => 0xFF49,
            DisplayRegisters::DMA => 0xFF46,
        }
    }
}

pub struct LCDC {
    pub lcd_display_enable: bool,             // (0=Off, 1=On)
    pub window_tile_map_display_select: bool, // (0=9800-9BFF, 1=9C00-9FFF)
    pub window_display_enable: bool,          // (0=Off, 1=On)
    pub bg_window_tile_data_select: bool,     // (0=8800-97FF, 1=8000-8FFF)
    pub bg_tile_map_display_selct: bool,      // (0=9800-9BFF, 1=9C00-9FFF)
    pub obj_size: bool,                       // (0=8x8, 1=8x16)
    pub obj_display_enable: bool,             // (0=Off, 1=On)
    pub bg_dispaly: bool,                     // (0=Off, 1=On)
}

impl std::convert::From<LCDC> for u8 {
    fn from(flag: LCDC) -> u8 {
        (if flag.lcd_display_enable { 1 } else { 0 }) << 7
            | (if flag.window_tile_map_display_select {
                1
            } else {
                0
            }) << 6
            | (if flag.window_display_enable { 1 } else { 0 }) << 5
            | (if flag.bg_window_tile_data_select {
                1
            } else {
                0
            }) << 4
            | (if flag.bg_tile_map_display_selct { 1 } else { 0 }) << 3
            | (if flag.obj_size { 1 } else { 0 }) << 2
            | (if flag.obj_display_enable { 1 } else { 0 }) << 1
            | (if flag.bg_dispaly { 1 } else { 0 }) << 0
    }
}

impl std::convert::From<u8> for LCDC {
    fn from(byte: u8) -> Self {
        LCDC {
            lcd_display_enable: ((byte >> 7) & 0b1) != 0,
            window_tile_map_display_select: ((byte >> 6) & 0b1) != 0,
            window_display_enable: ((byte >> 5) & 0b1) != 0,
            bg_window_tile_data_select: ((byte >> 4) & 0b1) != 0,
            bg_tile_map_display_selct: ((byte >> 3) & 0b1) != 0,
            obj_size: ((byte >> 2) & 0b1) != 0,
            obj_display_enable: ((byte >> 1) & 0b1) != 0,
            bg_dispaly: ((byte >> 0) & 0b1) != 0,
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
}

impl PPU {
    pub fn new() -> Self {
        Self {
            lx: 0,
            mode: PPUMode::Mode2OAMScan,
        }
    }
    // called once per "dot"
    //
    // A “dot” = one 222 Hz (≅ 4.194 MHz) time unit. Dots
    // remain the same regardless of whether the CPU is in
    // Double Speed mode, so there are 4 dots per Normal
    // Speed M-cycle, and 2 per Double Speed M-cycle.
    pub fn step(&mut self, memory: &mut MemoryBus) -> bool {
        self.update_scan_registers(memory);
        true
    }

    fn update_scan_registers(&mut self, memory: &mut MemoryBus) {
        let ly = if self.lx >= 456 {
            self.lx = 0;

            memory.wrapping_inc_byte(DisplayRegisters::LY.get_address() as u16, 153)
        } else {
            self.lx += 1;

            memory.read_byte(DisplayRegisters::LY.get_address() as u16)
        };

        self.update_mode(ly);
        memory.update_ppu_lock(self.mode);
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
}
