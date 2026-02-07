pub mod cartridge;
pub mod cpu;
pub mod display;
pub mod gameboy;
pub mod hardware_registers;
pub mod instructions;
pub mod memory;
pub mod ppu;
pub mod registers;

use std::{
    env, fs,
    path::{Path, PathBuf},
};

use gameboy::Gameboy;

use crate::{gameboy::GbOptions, instructions::Instruction};

pub fn main() {
    // decode_file("resources/game.gb");
    create_and_run(&PathBuf::from("resources/game.gb"));
}

pub fn create_and_run(cartridge_path: &Path) {
    configure_logger();
    let mut gb = Gameboy::new(
        false,
        Some(cartridge_path),
        Some(GbOptions {
            limit_speed: false,
            render: false,
        }),
    );
    gb.boot();
}

fn configure_logger() {
    if env::var("RUST_LOG").is_err() {
        unsafe { env::set_var("RUST_LOG", "error") }
    }
    env_logger::init();
}

pub fn decode_file<P: AsRef<Path> + std::fmt::Debug + Copy>(path: P) {
    let read_res = fs::read(path);

    match read_res {
        Ok(data) => {
            let mut pos = 0;

            while pos < data.len() {
                let read_pos = pos;
                let mut instruction_byte = data[pos];
                let prefix = instruction_byte == 0xCB;
                if prefix {
                    instruction_byte = data[pos + 1];
                    pos += 2;
                } else {
                    pos += 1;
                }

                if let Some(instruction) = Instruction::from_byte(instruction_byte, prefix) {
                    let description =
                        format!("0x{}{:x}", if prefix { "cb" } else { "" }, instruction_byte);
                    println!(
                        "Position 0x{:x}={}: {:?}",
                        read_pos, description, instruction
                    );
                } else {
                    let description =
                        format!("0x{}{:x}", if prefix { "cb" } else { "" }, instruction_byte);
                    println!("Position 0x{:x}={}: ???", read_pos, description);
                };
            }
        }
        Err(_) => panic!("Failed to load cartridge at {:?}", path),
    };
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::gameboy::{Gameboy, GbOptions};

    #[test]
    fn test_roms() {
        let mut gb = Gameboy::new(
            true,
            Some(&PathBuf::from("resources/cpu_instrs.gb")),
            Some(GbOptions {
                limit_speed: false,
                render: false,
            }),
        );
        gb.boot();
    }
}
