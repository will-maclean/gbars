use std::{
    path::Path,
    thread::sleep,
    time::{Duration, Instant},
};

use log::debug;

use crate::{
    cartridge::create_cartridge, cpu::CPU, display::GbDisplay, memory::MemoryBus, ppu::PPU,
};

pub static mut DUMP_INFO_TICK: bool = false;

const CLOCK_SPEED_HZ: f32 = 4.194304e6;
const DESIRED_RENDER_FPS: f32 = 30.0;

pub struct Gameboy {
    bus: MemoryBus,
    cpu: CPU,
    display: GbDisplay,
    ppu: PPU,
    running: bool,
}

impl Gameboy {
    pub fn new(debug_mode: bool, cartridge_path: &Path) -> Self {
        let display = GbDisplay::start();

        if let Err(_) = display {
            panic!("Failed to start display!");
        }

        Self {
            bus: MemoryBus::new_and_load_bios(Some(create_cartridge(cartridge_path))),
            cpu: CPU::new(debug_mode),
            running: false,
            ppu: PPU::new(),
            display: display.unwrap(),
        }
    }

    pub fn new_and_empty(debug_mode: bool) -> Self {
        let display = GbDisplay::start();

        if let Err(_) = display {
            panic!("Failed to start display!");
        }

        Self {
            bus: MemoryBus::new_and_empty(None),
            cpu: CPU::new(debug_mode),
            // display: GbDisplay::new(),
            running: false,
            // dt: std::time::Duration::new(0, 0),
            ppu: PPU::new(),
            display: display.unwrap(),
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

        let mut last_m_tick = Instant::now();
        let mut last_render = Instant::now();

        let m_tick_duration = Duration::from_secs_f32(1.0 / CLOCK_SPEED_HZ);
        let render_tick_duration = Duration::from_secs_f32(1.0 / DESIRED_RENDER_FPS);

        while self.running {
            // each loop represents a single tick of the Master clock, or M tick
            debug!("M tick");

            if cpu_ticker >= m_ticks_per_cpu_step {
                cpu_ticker = 0;
                self.running &= self.cpu.step(&mut self.bus);
            }
            cpu_ticker += 1;

            self.running &= self.ppu.step(&mut self.bus);

            if Instant::now() - m_tick_duration < last_m_tick {
                sleep(m_tick_duration - (Instant::now() - last_m_tick));
                last_m_tick = Instant::now();
            }

            if Instant::now() - render_tick_duration > last_render {
                self.running &= self.display.render(&self.bus);
                last_render = Instant::now();
                debug!("Render");
            }

            unsafe { DUMP_INFO_TICK = false };
        }
    }
}
