use raylib::prelude::*;

use crate::{memory::MemoryBus, ppu::PPU};

pub const BACKGROUND_WIDTH_PIXELS: usize = 256;
pub const BACKGROUND_HEIGHT_PIXELS: usize = 256;
pub const BACKGROUND_TILES_PIXELS_WIDTH: usize = 8;
pub const BACKGROUND_TILES_PIXELS_HEIGHT: usize = 8;
pub const BACKGROUND_TILES_WIDTH_N: usize = 32;
pub const BACKGROUND_TILES_HEIGHT_N: usize = 32;
pub const SCREEN_WIDTH_PIXELS: usize = 144;
pub const SCREEN_HEIGHT_PIXELS: usize = 160;
pub const MAX_DISPLAY_SPRITES: usize = 40;
pub const MAX_DISPLAY_SPRITES_PER_SCAN_LINE: usize = 10;

pub struct GbDisplay {
    rl: RaylibHandle,
    thread: RaylibThread,
}

impl GbDisplay {
    pub fn start() -> Result<Self, ()> {
        let (mut rl, thread) = raylib::init().size(640, 480).title("GBARS").build();
        rl.set_target_fps(30);

        Ok(Self { rl, thread })
    }

    pub fn render(&mut self, bus: &MemoryBus, ppu: &PPU) -> bool {
        if bus.registers.LCDC.lcd_display_enable {
            let buffer = ppu.get_screen_buffer();

            //TODO: draw the buffer

            self.rl.draw(&self.thread, |mut d| {
                d.clear_background(Color::WHITE);
                d.draw_text("Screen enabled :)", 12, 12, 20, Color::BLACK);
            });
        } else {
            self.rl.draw(&self.thread, |mut d| {
                d.clear_background(Color::BLACK);
                d.draw_text("Screen disabled :(", 12, 12, 20, Color::WHITE);
            });
        }

        !self.rl.window_should_close()
    }
}
