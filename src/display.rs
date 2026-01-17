use ggez::graphics;

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

#[derive(Copy, Debug, Clone)]
pub enum DrawColor {
    BLACK,
    DARKGREY,
    LIGHTGREY,
    WHITE,
}

impl DrawColor {
    pub fn to_colour(&self) -> graphics::Color {
        match self {
            DrawColor::BLACK => graphics::Color::BLACK,
            DrawColor::DARKGREY => graphics::Color::BLUE,
            DrawColor::LIGHTGREY => graphics::Color::YELLOW,
            DrawColor::WHITE => graphics::Color::WHITE,
        }
    }
}

// pub struct GbDisplay {}
//
// impl GbDisplay {
//     pub fn new() -> Self {
//         Self {}
//     }
//
//     pub fn start(&mut self) {}
//
//     pub fn render(&mut self, ctx: &mut Context, cpu: &mut CPU) -> GameResult {
//         let mut draw_pixels: [[DrawColor; SCREEN_WIDTH_PIXELS]; SCREEN_HEIGHT_PIXELS] =
//         let draw_pixels: [[DrawColor; SCREEN_WIDTH_PIXELS]; SCREEN_HEIGHT_PIXELS] =
//             [[DrawColor::BLACK; SCREEN_WIDTH_PIXELS]; SCREEN_HEIGHT_PIXELS];
//
//         let lcdc = LCDC::from(cpu.read_byte(DisplayRegisters::LCDC.get_address()));
//
//         if !lcdc.lcd_display_enable {
//             return self.draw(ctx, draw_pixels);
//         }
//
//         Ok(())
//     }
//
//     fn draw(&mut self, ctx: &mut Context, pixels: [[DrawColor; 166]; 144]) -> GameResult {
//         // simple naive idea, doesn't work. We seem to hit a max number of meshes to draw at
//         // 5000. When we have one mesh per pixel we hit this unfortunately. I think we'll need
//         // to be a bit smarter, and build a texture for 1) background 2) window and 3) any sprites,
//         // then do a single call for each.
//         let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
//
//         // for i in 0..SCREEN_HEIGHT_PIXELS {
//         //     for j in 0..SCREEN_WIDTH_PIXELS {
//         //         println!("rendering (i,j)=({},{})", i, j);
//
//         //         canvas = self.draw_pixel(pixels[i][j], i, j, canvas, ctx);
//         //     }
//         // }
//
//         canvas = self.draw_pixel(pixels[30][30], 30, 30, canvas, ctx);
//
//         canvas.finish(ctx)?;
//
//         Ok(())
//     }
//
//     fn draw_pixel(
//         &mut self,
//         val: DrawColor,
//         i: usize,
//         j: usize,
//         mut canvas: Canvas,
//         ctx: &mut Context,
//     ) -> Canvas {
//         let x = j as f32 * ctx.gfx.size().0 / SCREEN_WIDTH_PIXELS as f32;
//         let y = i as f32 * ctx.gfx.size().1 / SCREEN_HEIGHT_PIXELS as f32;
//         let w = ctx.gfx.size().0 / SCREEN_WIDTH_PIXELS as f32;
//         let h = ctx.gfx.size().1 / SCREEN_HEIGHT_PIXELS as f32;
//
//         let rect = graphics::Mesh::new_rectangle(
//             ctx,
//             graphics::DrawMode::fill(),
//             graphics::Rect::new(x, y, w, h),
//             val.to_colour(),
//         )
//         .unwrap();
//         canvas.draw(&rect, graphics::DrawParam::default());
//
//         let rect = graphics::Mesh::new_rectangle(
//             ctx,
//             graphics::DrawMode::stroke(1.0),
//             graphics::Rect::new(x, y, w, h),
//             Color::WHITE,
//         )
//         .unwrap();
//
//         canvas.draw(&rect, graphics::DrawParam::default());
//
//         canvas
//     }
// }
