use gameboy::Gameboy;

pub mod cpu;
pub mod gameboy;
pub mod instructions;
pub mod registers;
pub mod memory;

fn main() {
    let mut gb = Gameboy::new_and_empty();
    gb.boot();

    for _ in 0..50 {
        gb.cpu.step();
    }
    println!("Hello, world!");
}
