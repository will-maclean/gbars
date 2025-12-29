use ggez::{
    graphics::{self, Canvas, Color},
    Context, GameResult,
};

use crate::cpu::CPU;

pub const BACKGROUND_WIDTH_PIXELS: usize = 256;
pub const BACKGROUND_HEIGHT_PIXELS: usize = 256;
pub const BACKGROUND_TILES_PIXELS_WIDTH: usize = 8;
pub const BACKGROUND_TILES_PIXELS_HEIGHT: usize = 8;
pub const BACKGROUND_TILES_WIDTH_N: usize = 32;
pub const BACKGROUND_TILES_HEIGHT_N: usize = 32;
pub const SCREEN_WIDTH_PIXELS: usize = 166;
pub const SCREEN_HEIGHT_PIXELS: usize = 144;
pub const MAX_DISPLAY_SPRITES: usize = 40;
pub const MAX_DISPLAY_SPRITES_PER_SCAN_LINE: usize = 10;

enum DisplayRegisters {
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
    fn get_address(&self) -> usize {
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

#[derive(Copy, Debug, Clone)]
enum DrawColor {
    BLACK,
    DARKGREY,
    LIGHTGREY,
    WHITE,
}

impl DrawColor {
    fn to_colour(&self) -> graphics::Color {
        match self {
            DrawColor::BLACK => graphics::Color::BLACK,
            DrawColor::DARKGREY => graphics::Color::BLUE,
            DrawColor::LIGHTGREY => graphics::Color::YELLOW,
            DrawColor::WHITE => graphics::Color::WHITE,
        }
    }
}

struct LCDC {
    lcd_display_enable: bool,             // (0=Off, 1=On)
    window_tile_map_display_select: bool, // (0=9800-9BFF, 1=9C00-9FFF)
    window_display_enable: bool,          // (0=Off, 1=On)
    bg_window_tile_data_select: bool,     // (0=8800-97FF, 1=8000-8FFF)
    bg_tile_map_display_selct: bool,      // (0=9800-9BFF, 1=9C00-9FFF)
    obj_size: bool,                       // (0=8x8, 1=8x16)
    obj_display_enable: bool,             // (0=Off, 1=On)
    bg_dispaly: bool,                     // (0=Off, 1=On)
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

pub struct GbDisplay {}

impl GbDisplay {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&mut self) {}

    pub fn render(&mut self, ctx: &mut Context, cpu: &mut CPU) -> GameResult {
        let mut draw_pixels: [[DrawColor; SCREEN_WIDTH_PIXELS]; SCREEN_HEIGHT_PIXELS] =
            [[DrawColor::BLACK; SCREEN_WIDTH_PIXELS]; SCREEN_HEIGHT_PIXELS];

        let lcdc = LCDC::from(cpu.read_byte(DisplayRegisters::LCDC.get_address()));

        if !lcdc.lcd_display_enable {
            return self.draw(ctx, draw_pixels);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, pixels: [[DrawColor; 166]; 144]) -> GameResult {
        // simple naive idea, doesn't work. We seem to hit a max number of meshes to draw at
        // 5000. When we have one mesh per pixel we hit this unfortunately. I think we'll need
        // to be a bit smarter, and build a texture for 1) background 2) window and 3) any sprites,
        // then do a single call for each.
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        // for i in 0..SCREEN_HEIGHT_PIXELS {
        //     for j in 0..SCREEN_WIDTH_PIXELS {
        //         println!("rendering (i,j)=({},{})", i, j);

        //         canvas = self.draw_pixel(pixels[i][j], i, j, canvas, ctx);
        //     }
        // }

        canvas = self.draw_pixel(pixels[30][30], 30, 30, canvas, ctx);

        canvas.finish(ctx)?;

        Ok(())
    }

    fn draw_pixel(
        &mut self,
        val: DrawColor,
        i: usize,
        j: usize,
        mut canvas: Canvas,
        ctx: &mut Context,
    ) -> Canvas {
        let x = j as f32 * ctx.gfx.size().0 / SCREEN_WIDTH_PIXELS as f32;
        let y = i as f32 * ctx.gfx.size().1 / SCREEN_HEIGHT_PIXELS as f32;
        let w = ctx.gfx.size().0 / SCREEN_WIDTH_PIXELS as f32;
        let h = ctx.gfx.size().1 / SCREEN_HEIGHT_PIXELS as f32;

        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(x, y, w, h),
            val.to_colour(),
        )
        .unwrap();
        canvas.draw(&rect, graphics::DrawParam::default());

        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            graphics::Rect::new(x, y, w, h),
            Color::WHITE,
        )
        .unwrap();

        canvas.draw(&rect, graphics::DrawParam::default());

        canvas
    }
}
