pub mod cpu;
pub mod display;
pub mod gameboy;
pub mod instructions;
pub mod memory;
pub mod registers;

use gameboy::Gameboy;
// use ggez::{conf, event, ContextBuilder};

pub fn main() {
    let mut gb = Gameboy::new_and_empty(true);
    gb.load_cartridge("resources/game.gb");
    gb.boot();
    gb.run();

    // let c = conf::Conf::new();
    // let (ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
    //     .default_conf(c)
    //     .build()
    //     .unwrap();
    //
    // event::run(ctx, event_loop, gb);
}

#[cfg(test)]
mod tests {
    use crate::gameboy::Gameboy;

    #[test]
    fn test_roms() {
        let mut gb = Gameboy::new_and_empty(true);
        gb.load_cartridge("resources/cpu_instrs.gb");
        gb.boot();
        gb.run();
    }
}
