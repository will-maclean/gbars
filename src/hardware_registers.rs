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

#[derive(Debug)]
pub struct LCDC {
    pub lcd_display_enable: bool,             // (0=Off, 1=On)
    pub window_tile_map_display_select: bool, // (0=9800-9BFF, 1=9C00-9FFF)
    pub window_display_enable: bool,          // (0=Off, 1=On)
    pub bg_window_tile_data_select: bool,     // (0=8800-97FF, 1=8000-8FFF)
    pub bg_tile_map_display_selct: bool,      // (0=9800-9BFF, 1=9C00-9FFF)
    pub obj_size: bool,                       // (0=8x8, 1=8x16)
    pub obj_display_enable: bool,             // (0=Off, 1=On)
    pub bg_display: bool,                     // (0=Off, 1=On)
}

impl LCDC {
    pub fn to_byte(&self) -> u8 {
        (if self.lcd_display_enable { 1 } else { 0 }) << 7
            | (if self.window_tile_map_display_select {
                1
            } else {
                0
            }) << 6
            | (if self.window_display_enable { 1 } else { 0 }) << 5
            | (if self.bg_window_tile_data_select {
                1
            } else {
                0
            }) << 4
            | (if self.bg_tile_map_display_selct { 1 } else { 0 }) << 3
            | (if self.obj_size { 1 } else { 0 }) << 2
            | (if self.obj_display_enable { 1 } else { 0 }) << 1
            | (if self.bg_display { 1 } else { 0 }) << 0
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
            bg_display: ((byte >> 0) & 0b1) != 0,
        }
    }
}

#[derive(Debug)]
pub struct IE {}

impl IE {
    pub fn to_byte(&self) -> u8 {
        todo!()
    }
}

impl std::convert::From<u8> for IE {
    fn from(byte: u8) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct STAT {
    pub lyc_int_select: bool,
    pub mode_2_int_select: bool,
    pub mode_1_int_select: bool,
    pub mode_0_int_select: bool,
    pub lyc_eq_ly: bool,
    pub ppu_mode: bool,
}

impl STAT {
    pub fn zero_init() -> Self {
        Self {
            lyc_int_select: false,
            mode_2_int_select: false,
            mode_1_int_select: false,
            mode_0_int_select: false,
            lyc_eq_ly: false,
            ppu_mode: false,
        }
    }
    pub fn to_byte(&self) -> u8 {
        todo!()
    }

    pub fn write_byte(&mut self, val: u8) {
        todo!()
    }
}

#[derive(Debug)]
pub struct HardwareRegisters {
    pub IE: IE,
    pub LCDC: LCDC,
    pub SCY: u8,
    pub SCX: u8,
    pub LY: u8,
    pub WX: u8,
    pub WY: u8,
    pub STAT: STAT,
}

impl HardwareRegisters {
    pub fn from_zeros() -> Self {
        Self {
            IE: IE::from(0),
            LCDC: LCDC::from(0),
            STAT: STAT::zero_init(),
            SCY: 0,
            SCX: 0,
            LY: 0,
            WX: 0,
            WY: 0,
        }
    }
}

pub enum RegisterAddresses {
    LCDC,
    WX,
    WY,
    LY,
    IE,
    SCY,
    SCX,
    STAT,
}

impl RegisterAddresses {
    pub fn address(&self) -> u16 {
        match self {
            RegisterAddresses::LCDC => 0xFF40,
            RegisterAddresses::LY => 0xFF44,
            RegisterAddresses::IE => 0xFFFF,
            RegisterAddresses::SCY => 0xFF42,
            RegisterAddresses::SCX => 0xFF43,
            RegisterAddresses::WX => 0xFF4B,
            RegisterAddresses::WY => 0xFF4A,
            RegisterAddresses::STAT => 0xFF41,
        }
    }

    pub fn from_address(address: u16) -> Option<Self> {
        match address {
            0xFF40 => Some(RegisterAddresses::LCDC),
            0xFF41 => Some(RegisterAddresses::STAT),
            0xFF42 => Some(RegisterAddresses::SCY),
            0xFF43 => Some(RegisterAddresses::SCX),
            0xFF44 => Some(RegisterAddresses::LY),
            0xFFFF => Some(RegisterAddresses::IE),
            0xFF4B => Some(RegisterAddresses::WX),
            0xFF4A => Some(RegisterAddresses::WY),
            _ => None,
        }
    }
}
