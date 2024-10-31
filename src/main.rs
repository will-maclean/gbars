use gameboy::Gameboy;

pub mod cpu;
pub mod gameboy;
pub mod instructions;
pub mod registers;
pub mod memory;

fn main() {
    let mut gb = Gameboy::new_and_empty();
    gb.load_cartridge("resources/Pokemon Red (UE) [S][!].gb");
    gb.boot();

    for _ in 0..500 {
        gb.cpu.step();
    }
    println!("Hello, world!");
}
