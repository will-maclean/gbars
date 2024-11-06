pub mod cpu;
pub mod gameboy;
pub mod instructions;
pub mod registers;
pub mod memory;
pub mod display;

use gameboy::Gameboy;
use ggez::{conf, event, ContextBuilder};

pub fn main() {
    let mut gb = Gameboy::new_and_empty();
    gb.load_cartridge("resources/Pokemon Red (UE) [S][!].gb");
    gb.boot();

    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .default_conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, gb);
}
