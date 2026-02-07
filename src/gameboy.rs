use std::{
    cell::RefCell,
    path::Path,
    rc::Rc,
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

pub struct GbOptions {
    pub limit_speed: bool,
    pub render: bool,
}

impl Default for GbOptions {
    fn default() -> Self {
        Self {
            limit_speed: true,
            render: true,
        }
    }
}

pub struct Gameboy {
    bus: MemoryBus,
    cpu: CPU,
    display: Option<GbDisplay>,
    ppu: Rc<RefCell<PPU>>,
    running: bool,
    options: GbOptions,
}

impl Gameboy {
    pub fn new(
        debug_mode: bool,
        cartridge_path: Option<&Path>,
        options: Option<GbOptions>,
    ) -> Self {
        let options = options.unwrap_or(GbOptions::default());
        let display = if options.render {
            let d = GbDisplay::start();

            if d.is_err() {
                panic!("Failed to start display!");
            }

            Some(d.unwrap())
        } else {
            None
        };

        let ppu = Rc::new(RefCell::new(PPU::new()));

        let bus = if let Some(cp) = cartridge_path {
            MemoryBus::new_and_load_bios(Some(create_cartridge(cp)), ppu.clone())
        } else {
            MemoryBus::new_and_empty(None, ppu.clone())
        };

        Self {
            bus,
            cpu: CPU::new(debug_mode),
            running: false,
            ppu,
            display,
            options,
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

            self.running &= self.ppu.borrow_mut().step(&mut self.bus);

            if self.options.limit_speed && Instant::now() - m_tick_duration < last_m_tick {
                sleep(m_tick_duration - (Instant::now() - last_m_tick));
                last_m_tick = Instant::now();
            }

            if self.options.render && Instant::now() - render_tick_duration > last_render {
                self.running &= self
                    .display
                    .as_mut()
                    .unwrap()
                    .render(&self.bus, &self.ppu.borrow_mut());
                last_render = Instant::now();
                debug!("Render");
            }

            unsafe { DUMP_INFO_TICK = false };
        }
    }
}
