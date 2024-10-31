use crate::cpu::CPU;

pub struct Gameboy {
    //TODO: set as private once proper control is implemented
    pub cpu: CPU,
}

impl Gameboy {
    pub fn new_and_empty() -> Self {
        Self { cpu: CPU::new() }
    }

    pub fn boot(&mut self) {
        self.cpu.reset();
    }

    pub fn load_cartridge(&mut self, path: &str){
        self.cpu.load_cartridge(path);
    }
}
