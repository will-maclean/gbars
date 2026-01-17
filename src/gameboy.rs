use std::path::Path;

use crate::{
    cartridge::{create_cartridge, CartridgeType},
    cpu::CPU,
    memory::MemoryBus,
    ppu::PPU,
};

// const CLOCK_SPEED_MHz: f32 = 4.194304;
// const CPU_INSTRUCTION_MHz: f32 = CLOCK_SPEED_MHz / 4.0;
// const DESIRED_RENDER_FPS: f32 = 30.0; // Can't imagine we'd ever need more than this?

pub struct Gameboy {
    bus: MemoryBus,
    cpu: CPU,
    // pub display: GbDisplay,
    ppu: PPU,

    // dt: std::time::Duration,
    running: bool,
}

impl Gameboy {
    pub fn new(debug_mode: bool, cartridge_path: &Path, cartridge_type: CartridgeType) -> Self {
        Self {
            bus: MemoryBus::new_and_load_bios(Some(create_cartridge(
                cartridge_path,
                cartridge_type,
            ))),
            cpu: CPU::new(debug_mode),
            // display: GbDisplay::new(),
            running: false,
            // dt: std::time::Duration::new(0, 0),
            ppu: PPU::new(),
        }
    }

    pub fn new_and_empty(debug_mode: bool) -> Self {
        Self {
            bus: MemoryBus::new_and_empty(None),
            cpu: CPU::new(debug_mode),
            // display: GbDisplay::new(),
            running: false,
            // dt: std::time::Duration::new(0, 0),
            ppu: PPU::new(),
        }
    }

    pub fn boot(&mut self) {
        self.running = true;
        self.cpu.reset();

        self.run();
    }

    fn run(&mut self) {
        let mut cpu_ticker = 0;
        let m_ticks_per_cpu_step = 4; // or 2 if in cpu double speed mode

        while self.running {
            // each loop represents a single tick of the Master clock, or M tick

            if cpu_ticker >= m_ticks_per_cpu_step {
                cpu_ticker = 0;
                self.running &= self.cpu.step(&mut self.bus);
            }
            cpu_ticker += 1;

            self.running &= self.ppu.step(&mut self.bus);
        }
    }
}
