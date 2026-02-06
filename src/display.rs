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
        let (rl, thread) = raylib::init().size(640, 480).title("GBARS").build();

        Ok(Self { rl, thread })
    }

    pub fn render(&mut self, bus: &MemoryBus, ppu: &PPU) -> bool {
        if bus.registers.LCDC.lcd_display_enable {
            let buffer = ppu.get_screen_buffer();

            self.rl.draw(&self.thread, |mut d| {
                d.clear_background(Color::GREEN);

                let screen_width = d.get_screen_width();
                let screen_height = d.get_screen_height();

                let px_width = screen_width / SCREEN_WIDTH_PIXELS as i32;
                let px_height = screen_height / SCREEN_HEIGHT_PIXELS as i32;

                for i in 0..buffer.len() {
                    for j in 0..buffer[0].len() {
                        d.draw_rectangle(
                            i as i32 * px_width,
                            j as i32 * px_height,
                            px_width,
                            px_height,
                            match buffer[i][j] {
                                crate::ppu::DrawColor::BLACK => Color::BLUE,
                                crate::ppu::DrawColor::DARKGREY => Color::GRAY,
                                crate::ppu::DrawColor::LIGHTGREY => Color::LIGHTGRAY,
                                crate::ppu::DrawColor::WHITE => Color::WHITE,
                            },
                        );
                    }
                }
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
