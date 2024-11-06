use std::time::Instant;

use ggez::{Context, GameError, GameResult};

use crate::{cpu::CPU, display::GbDisplay};

const CLOCK_SPEED_MHz: f32 = 4.194304;
const CPU_INSTRUCTION_MHz: f32 = CLOCK_SPEED_MHz / 4.0;
const DESIRED_RENDER_FPS: f32 = 30.0;   // Can't imagine we'd ever need more than this?

pub struct Gameboy {
    //TODO: set as private once proper control is implemented
    pub cpu: CPU,
    pub display: GbDisplay,

    running: bool,
}

impl Gameboy {
    pub fn new_and_empty() -> Self {
        Self { cpu: CPU::new(), display: GbDisplay::new(), running: false }
    }

    pub fn boot(&mut self) {
        self.running = true;
        self.cpu.reset();
        self.run();
    }

    pub fn load_cartridge(&mut self, path: &str){
        self.cpu.load_cartridge(path);
    }

    fn run(&mut self) {
        let no_render_duration = 1.0 / DESIRED_RENDER_FPS;
        let mut last_render = Instant::now();

        while self.running {
            self.running = self.cpu.step();

            if last_render.elapsed().as_secs_f32() > no_render_duration {
                last_render = Instant::now();   // investigate: should this go before or after the actual render call?
                self.display.render();
            }
        }
    }
}

impl ggez::event::EventHandler<GameError> for Gameboy {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
  }