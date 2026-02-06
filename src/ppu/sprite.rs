use crate::ppu::ColorIdx;

#[derive(Debug, Copy, Clone)]
pub struct Sprite {
    raw: [u8; 16],
    colour: [[ColorIdx; 8]; 8],
}

impl Sprite {
    pub fn new(raw: [u8; 16]) -> Self {
        let mut spr = Sprite {
            raw,
            colour: [[ColorIdx::Zero; 8]; 8],
        };

        spr.update_colour();

        spr
    }

    fn update_colour(&mut self) {
        // top to bottom
        for i in 0..8 {
            // left to right
            for j in 0..8 {
                let b1 = self.raw[2 * i];
                let b2 = self.raw[2 * i + 1];

                let retrieve_bit = 7 - j;
                let b1_retrieve = (b1 >> retrieve_bit) & 1;
                let b2_retrieve = (b2 >> retrieve_bit) & 1;
                let val = (b2_retrieve << 1) | b1_retrieve;

                self.colour[i][j] = ColorIdx::from(val);
            }
        }
    }

    pub fn from_zeros() -> Self {
        Self::new([0; 16])
    }

    pub fn read_byte(&self, offset: usize) -> u8 {
        self.raw[offset]
    }

    pub fn write_byte(&mut self, offset: usize, val: u8) {
        self.raw[offset] = val;

        self.update_colour();
    }

    pub fn pixel_at(&self, x: u8, y: u8) -> ColorIdx {
        self.colour[x as usize][y as usize]
    }
}
