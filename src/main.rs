use cpu::CPU;
use gameboy::Gameboy;

pub mod instructions;
pub mod registers;
pub mod cpu;
pub mod gameboy;

fn main() {
    let mut gb = Gameboy::new_and_empty();
    gb.boot();

    gb.cpu.step();
    println!("Hello, world!");
}
