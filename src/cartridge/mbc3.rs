use log::debug;

use crate::{
    cartridge::{Cartridge, ROM_BANK_SIZE},
    gameboy::DUMP_INFO_TICK,
    memory::MemoryRegion,
};

#[derive(Debug, Default)]
pub struct RTCRegister {
    seconds: u8,
    minutes: u8,
    hours: u8,
    days: u16, // actually only 9 bytes
    halt: bool,
    day_carry: bool,
}

#[derive(Debug, Default)]
pub struct MBC3Registers {
    ram_timer_enable: bool,
    rom_bank_number: u8,
    ram_bank_number: u8,
    latch_clock: u8,
    clock_latched: bool,
    rtc: RTCRegister,
    ram_over_rtc: bool,
}

#[derive(Debug)]
pub struct MBC3Cartridge {
    rom: Vec<u8>,
    ram: Option<Vec<u8>>,
    registers: MBC3Registers,
    has_battery: bool,
    rom_banks: usize,
}

impl MBC3Cartridge {
    pub fn new(rom: Vec<u8>, ram_banks: Option<usize>, has_battery: bool) -> Self {
        Self {
            rom_banks: rom.len() / ROM_BANK_SIZE,
            rom,
            ram: if let Some(ram_banks) = ram_banks {
                Some(vec![0; ram_banks * 0x8000])
            } else {
                None
            },
            registers: Default::default(),
            has_battery,
        }
    }

    pub fn new_and_empty() -> Self {
        todo!()
    }
}

impl Cartridge for MBC3Cartridge {
    fn read_byte(&self, address: u16) -> u8 {
        match MemoryRegion::from_addr(address, false) {
            MemoryRegion::GameROMBank0 => {
                let rom_addr = address as usize & 0x1FFF;
                self.rom[rom_addr]
            }
            MemoryRegion::GameROMBankN => {
                let mut rom_addr = address as usize - 0x4000;
                rom_addr += (self.registers.rom_bank_number as usize) * ROM_BANK_SIZE;

                if let Some(val) = self.rom.get(rom_addr) {
                    *val
                } else {
                    panic!("Illegal read address in MBC3! Read address = 0x{:x}, rom bank number = 0x{:x}, translated to ROM address = 0x{:x}. ROM size = 0x{:x}", address, self.registers.rom_bank_number, rom_addr, self.rom.len())
                }
            }
            MemoryRegion::CartridgeRAM => {
                if address - 0xA000 <= 0x07 {
                    // ram bank access

                    if let Some(ram) = &self.ram {
                        let mut ram_addr = address & 0x1FFF;

                        ram_addr |= (self.registers.ram_bank_number as u16) << 13;

                        ram[ram_addr as usize]
                    } else {
                        0xFF
                    }
                } else {
                    // RTC register access
                    match address - 0xA000 {
                        0x08 => self.registers.rtc.seconds,
                        0x09 => self.registers.rtc.minutes,
                        0x0A => self.registers.rtc.hours,
                        0x0B => self.registers.rtc.days as u8,
                        0x0C => {
                            let halt_bit: u8 = if self.registers.rtc.halt { 0x40 } else { 0 };
                            let overflow_bit: u8 = if self.registers.rtc.day_carry {
                                0x80
                            } else {
                                0
                            };

                            (self.registers.rtc.days >> 8) as u8 | halt_bit | overflow_bit
                        }
                        _ => 0xFF,
                    }
                }
            }
            _ => panic!("Bad read address for a cartridge!"),
        }
    }

    fn write_byte(&mut self, address: u16, val: u8) {
        match address {
            0x0000..=0x1FFF => {
                // ram enable
                // set TRUE if lower 4 bits = 0xA, and FALSE otherwise
                self.registers.ram_timer_enable = val & 0b1111 == 0xa;
                debug!(
                    "RAM ENABLE write. Write addr=0x{:x}, val=0x{:x}",
                    address, val
                );

                // unsafe {
                //     DUMP_INFO_TICK = true;
                // }
            }
            0x2000..=0x3FFF => {
                // rom bank number
                let mut new_val = val;

                if new_val == 0 {
                    new_val = 1;
                }

                new_val &= 0x7F;

                new_val = (new_val as usize % self.rom_banks) as u8;

                debug!(
                    "updating ROM bank number. Write addr=0x{:x}, val=0x{:x}, new rom bank=0x{:x}",
                    address, val, new_val,
                );
                self.registers.rom_bank_number = new_val;
            }

            0x4000..=0x5FFF => {
                // secondary bank number
                if val <= 0x07 {
                    self.registers.ram_bank_number = val;
                    self.registers.ram_over_rtc = true;
                    debug!(
                        "RAM BANK write. Write addr=0x{:x}, val=0x{:x}",
                        address, val
                    );
                } else {
                    debug!("RTC write. Write addr=0x{:x}, val=0x{:x}", address, val);
                    self.registers.ram_over_rtc = false;
                }
            }

            0x6000..=0x7FFF => {
                if self.registers.latch_clock == 0 && val == 1 {
                    self.registers.clock_latched = !self.registers.clock_latched;
                }
                self.registers.latch_clock = val;
                debug!(
                    "LATCH CLOCK write. Write addr=0x{:x}, val=0x{:x}",
                    address, val
                );
            }

            0xA000..=0xBFFF => {
                if self.registers.ram_timer_enable {
                    if self.registers.ram_over_rtc {
                        if let Some(ram) = &mut self.ram {
                            let ram_addr = (address as usize - 0xA000)
                                + 0x2000 * self.registers.ram_bank_number as usize;
                            ram[ram_addr] = val;
                        }
                    }
                } else {
                    match val {
                        0x08 => self.registers.rtc.seconds = val % 60,
                        0x09 => self.registers.rtc.minutes = val % 60,
                        0x0A => self.registers.rtc.hours = val % 24,
                        0x0B => {
                            self.registers.rtc.days = (self.registers.rtc.days & 0x100) | val as u16
                        }
                        0x0C => {
                            self.registers.rtc.halt = val & 0x40 > 0;
                            self.registers.rtc.day_carry = val & 0x80 > 0;
                            self.registers.rtc.days =
                                (self.registers.rtc.days & 0xFF) | (val as u16 & 0b1) << 8;
                        }
                        _ => {}
                    }
                }
            }

            _ => panic!(
                "Writing to bad address on MBC1 chip! Address = 0x{:x}",
                address
            ),
        }
    }
}
