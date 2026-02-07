use crate::{
    memory::MemoryBus,
    ppu::{DrawColor, PPU},
};
use minifb::{Key, Window, WindowOptions};

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

pub const WINDOW_PX_WIDTH: usize = 144;
pub const WINDOW_PX_HEIHGT: usize = 160;

pub struct GbDisplay {
    window: Window,
}

impl GbDisplay {
    pub fn start() -> Result<Self, ()> {
        let mut window = Window::new(
            "Test - ESC to exit",
            WINDOW_PX_WIDTH,
            WINDOW_PX_HEIHGT,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
        window.set_target_fps(30);

        Ok(Self { window })
    }

    pub fn render(&mut self, bus: &MemoryBus, ppu: &PPU) -> bool {
        let buffer = ppu.get_screen_buffer();
        let mut draw_buffer: Vec<u32> =
            Vec::with_capacity(SCREEN_WIDTH_PIXELS * SCREEN_HEIGHT_PIXELS);

        if bus.registers.LCDC.lcd_display_enable {
            for i in 0..buffer.len() {
                for j in 0..buffer[0].len() {
                    draw_buffer.push(match buffer[i][j] {
                        DrawColor::BLACK => 0,
                        DrawColor::DARKGREY => 0x525252,
                        DrawColor::LIGHTGREY => 0xb0b0b0,
                        DrawColor::WHITE => 0xffffff,
                    });
                }
            }
        } else {
            draw_buffer = vec![0; SCREEN_WIDTH_PIXELS * SCREEN_HEIGHT_PIXELS];
        }

        self.window
            .update_with_buffer(&draw_buffer, SCREEN_WIDTH_PIXELS, SCREEN_HEIGHT_PIXELS)
            .unwrap();
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }
}
// pub struct GbDisplay {
//     rl: RaylibHandle,
//     thread: RaylibThread,
// }
//
// impl GbDisplay {
//     pub fn start() -> Result<Self, ()> {
//         let (rl, thread) = raylib::init().size(640, 480).title("GBARS").build();
//
//         Ok(Self { rl, thread })
//     }
//
//     pub fn render(&mut self, bus: &MemoryBus, ppu: &PPU) -> bool {
//         self.rl.draw(&self.thread, |mut d| {
//             let buffer = ppu.get_screen_buffer();
//             d.clear_background(Color::BLACK);
//
//             if bus.registers.LCDC.lcd_display_enable {
//                 let screen_width = d.get_screen_width();
//                 let screen_height = d.get_screen_height();
//
//                 let px_width = screen_width / SCREEN_WIDTH_PIXELS as i32;
//                 let px_height = screen_height / SCREEN_HEIGHT_PIXELS as i32;
//
//                 for i in 0..buffer.len() {
//                     for j in 0..buffer[0].len() {
//                         d.draw_rectangle(
//                             i as i32 * px_width,
//                             j as i32 * px_height,
//                             px_width,
//                             px_height,
//                             match buffer[i][j] {
//                                 DrawColor::BLACK => Color::BLACK,
//                                 DrawColor::DARKGREY => Color::GRAY,
//                                 DrawColor::LIGHTGREY => Color::LIGHTGRAY,
//                                 DrawColor::WHITE => Color::WHITE,
//                             },
//                         );
//
//                         log::error!(
//                             "drawing rect({}, {}, {}, {}, {:?})",
//                             i as i32 * px_width,
//                             j as i32 * px_height,
//                             px_width,
//                             px_height,
//                             match buffer[i][j] {
//                                 DrawColor::BLACK => Color::BLACK,
//                                 DrawColor::DARKGREY => Color::GRAY,
//                                 DrawColor::LIGHTGREY => Color::LIGHTGRAY,
//                                 DrawColor::WHITE => Color::WHITE,
//                             },
//                         );
//                     }
//                 }
//             }
//         });
//
//         !self.rl.window_should_close()
//     }
// }
