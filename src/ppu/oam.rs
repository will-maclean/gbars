#[derive(Debug, Clone, Copy)]
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
            priority: value[3] & 0x7F == 1,
            y_flip: value[3] & 0x3F == 1,
            x_flip: value[3] & 0x1F == 1,
            palette: value[3] & 0xF,
        }
    }
}

impl OAMEntry {
    pub fn from_zeros() -> Self {
        Self::from([0; 4])
    }
    pub fn to_byte(&self, offset: usize) -> u8 {
        match offset {
            0 => self.y,
            1 => self.x,
            2 => self.tile_idx,
            3 => {
                ((self.priority as u8) << 6)
                    | ((self.y_flip as u8) << 5)
                    | ((self.x_flip as u8) << 4)
                    | ((self.palette as u8) << 3)

                //TODO: any weirdness from not capturing and then returning the unused bits??
            }
            _ => panic!("Invalid index into OAMEntry: {offset}"),
        }
    }

    pub fn write_byte(&mut self, offset: usize, val: u8) {
        match offset {
            0 => self.y = val,
            1 => self.x = val,
            2 => self.tile_idx = val,
            3 => {
                self.priority = val & 0x7F == 1;
                self.y_flip = val & 0x3F == 1;
                self.x_flip = val & 0x1F == 1;
                self.palette = val & 0xF;
            }
            _ => panic!("Invalid index into OAMEntry: {offset}"),
        }
    }
}
