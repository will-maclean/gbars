pub struct Sprite {
    raw: [[ColorIdx; 8]; 8],
}

enum ColorIdx {
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

impl Sprite {
    pub fn new(raw: [u8; 16]) -> Self {
        let mut spr = Sprite {
            raw: [[ColorIdx::Zero; 8]; 8],
        };

        // top to bottom
        for i in 0..8 {
            // left to right
            for j in 0..8 {
                let b1 = raw[2 * i];
                let b2 = raw[2 * i + 1];

                let retrieve_bit = 7 - j;
                let b1_retrieve = (b1 >> retrieve_bit) & 1;
                let b2_retrieve = (b2 >> retrieve_bit) & 1;
                let val = (b2_retrieve << 1) | b1_retrieve;

                spr.raw[i][j] = ColorIdx::from(val);
            }
        }

        spr
    }
}
