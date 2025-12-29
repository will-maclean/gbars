use ggez::{Context, GameError, GameResult};

use crate::{cpu::CPU, display::GbDisplay};

// const CLOCK_SPEED_MHz: f32 = 4.194304;
// const CPU_INSTRUCTION_MHz: f32 = CLOCK_SPEED_MHz / 4.0;
// const DESIRED_RENDER_FPS: f32 = 30.0; // Can't imagine we'd ever need more than this?

pub struct Gameboy {
    //TODO: set as private once proper control is implemented
    pub cpu: CPU,
    pub display: GbDisplay,

    dt: std::time::Duration,
    running: bool,
}

impl Gameboy {
    pub fn new_and_empty(debug: bool) -> Self {
        Self {
            cpu: CPU::new(debug),
            display: GbDisplay::new(),
            running: false,
            dt: std::time::Duration::new(0, 0),
        }
    }

    pub fn boot(&mut self) {
        self.running = true;
        self.cpu.reset();
        // self.run();
    }

    pub fn load_cartridge(&mut self, path: &str) {
        self.cpu.load_cartridge(path);
    }

    pub fn run(&mut self) {
        while self.running {
            self.cpu.step();
        }
    }
}

impl ggez::event::EventHandler<GameError> for Gameboy {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        println!(
            "Hello ggez! dt = {}ns, FPS={}",
            self.dt.as_nanos(),
            1.0 / self.dt.as_secs_f32()
        );
        println!("screen size: {:?}", ctx.gfx.size());

        let _ = self.display.render(ctx, &mut self.cpu);

        Ok(())
    }
}
