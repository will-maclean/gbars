pub struct OAMEntry {
    pub y: u8,
    pub x: u8,
    pub tile_idx: u8,

    // flags
    pub priority: bool,
    pub y_flip: bool,
    pub x_flip: bool,
    pub palette: u8, // can only be 0 or 1
}

impl From<[u8; 4]> for OAMEntry {
    fn from(value: [u8; 4]) -> Self {
        Self {
            y: value[0],
            x: value[1],
            tile_idx: value[2],
            priority: value[3] & 0x7F,
            y_flip: value[3] & 0x3F,
            x_flip: value[3] & 0x1F,
            palette: value[3] & 0xF,
        }
    }
}
