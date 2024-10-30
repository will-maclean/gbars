use crate::instructions::{
    ArithmeticByteTarget, ArithmeticTargetType, ArithmeticWordTarget, BitPosition, BitRegister, Instruction, JumpTest, LdByteAddress, LoadByteSource, LoadByteTarget, LoadType, LoadWordSource, LoadWordTarget, StackTarget
};
use crate::memory::MemoryBus;
use crate::registers::Registers;

const CLOCK_SPEED_MHz: f32 = 4.194304;

#[derive(Debug, Clone, Copy)]
pub struct CPU {
    registers: Registers,
    pc: u16,
    sp: u16,
    bus: MemoryBus,
    is_halted: bool,
    is_booting: bool,
    debug_view: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Default::default(),
            pc: 0,
            sp: 0xFFFF,
            bus: MemoryBus::new_and_load_bios(),
            is_halted: false,
            is_booting: true,
            debug_view: true,
        }
    }
    pub fn new_and_empty() -> Self {
        Self {
            registers: Default::default(),
            pc: 0,
            sp: 0xFFFF,
            bus: MemoryBus::new_and_empty(),
            is_halted: false,
            is_booting: true,
            debug_view: true,
        }
    }
    pub fn reset(&mut self) {
        self.sp = 0xFFFF;
        self.pc = 0;
        self.is_booting = true;
    }

    pub fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);

        let prefix = instruction_byte == 0xCB;
        if prefix {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        self.pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefix) {
            if self.debug_view {
                print!(
                    "Executing instruction: 0x{}{:x}. pc=0x{:x}\n",
                    if prefix { "cb" } else { "" },
                    instruction_byte,
                    self.pc
                )
            }
            self.execute(instruction)
        } else {
            let description = format!("0x{}{:x}", if prefix { "cb" } else { "" }, instruction_byte);
            panic!("Unkown instruction found for: {}", description)
        };
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        if self.is_halted {
            return self.pc;
        }

        match instruction {
            Instruction::ADD(arithmetic_type) => match arithmetic_type {
                ArithmeticTargetType::Byte(target) => match target {
                    ArithmeticByteTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.add_byte(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticByteTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.add_byte(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticByteTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add_byte(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticByteTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.add_byte(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticByteTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.add_byte(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticByteTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.add_byte(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticByteTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.add_byte(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticByteTarget::HLI => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.add_byte(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(2)
                    }
                },
                ArithmeticTargetType::Word(target) => {
                    let value = match target {
                        ArithmeticWordTarget::BC => self.registers.get_bc(),
                        ArithmeticWordTarget::DE => self.registers.get_de(),
                        ArithmeticWordTarget::HL => self.registers.get_hl(),
                        ArithmeticWordTarget::SP => self.sp,
                    };

                    let new_value = self.add_word(value);
                    self.registers.set_hl(new_value);

                    self.pc.wrapping_add(2)
                }
            },

            Instruction::AND(target) => match target {
                ArithmeticByteTarget::A => {
                    self.registers.a = self.and(self.registers.a);
                    self.pc.wrapping_add(1)
                }
                ArithmeticByteTarget::B => {
                    self.registers.a = self.and(self.registers.b);
                    self.pc.wrapping_add(1)
                }
                ArithmeticByteTarget::C => {
                    self.registers.a = self.and(self.registers.c);
                    self.pc.wrapping_add(1)
                }
                ArithmeticByteTarget::D => {
                    self.registers.a = self.and(self.registers.d);
                    self.pc.wrapping_add(1)
                }
                ArithmeticByteTarget::E => {
                    self.registers.a = self.and(self.registers.e);
                    self.pc.wrapping_add(1)
                }
                ArithmeticByteTarget::H => {
                    self.registers.a = self.and(self.registers.e);
                    self.pc.wrapping_add(1)
                }
                ArithmeticByteTarget::L => {
                    self.registers.a = self.and(self.registers.h);
                    self.pc.wrapping_add(1)
                }
                ArithmeticByteTarget::HLI => {
                    self.registers.a = self.and(self.bus.read_byte(self.registers.get_hl()));
                    self.pc.wrapping_add(2)
                }
            },

            Instruction::BIT(position, register) => {
                let and_val: u8 = match position {
                    BitPosition::Zero => 1,
                    BitPosition::One => 2,
                    BitPosition::Two => 4,
                    BitPosition::Three => 8,
                    BitPosition::Four => 16,
                    BitPosition::Five => 32,
                    BitPosition::Six => 64,
                    BitPosition::Seven => 128,
                };

                let (flag_val, pc_inc) = match register {
                    BitRegister::B => ((self.registers.b & and_val) > 0, 2),
                    BitRegister::C => ((self.registers.c & and_val) > 0, 2),
                    BitRegister::D => ((self.registers.d & and_val) > 0, 2),
                    BitRegister::E => ((self.registers.e & and_val) > 0, 2),
                    BitRegister::H => ((self.registers.h & and_val) > 0, 2),
                    BitRegister::L => ((self.registers.l & and_val) > 0, 2),
                    BitRegister::HLI => (
                        (self.bus.read_byte(self.registers.get_hl()) & and_val) > 0,
                        2,
                    ),
                };

                self.registers.f.zero = flag_val;

                self.pc.wrapping_add(pc_inc)
            }

            Instruction::CALL(test) => self.call(self.jmp_test(test)),
            Instruction::DEC(inc_type) => match inc_type {
                ArithmeticTargetType::Word(target) => {
                    match target {
                        ArithmeticWordTarget::BC => todo!(),
                        ArithmeticWordTarget::DE => todo!(),
                        ArithmeticWordTarget::HL => todo!(),
                        ArithmeticWordTarget::SP => todo!(),
                    };

                    self.pc.wrapping_add(2)
                }
                ArithmeticTargetType::Byte(arithmetic_byte_target) => {
                    match arithmetic_byte_target {
                        ArithmeticByteTarget::A => {
                            self.registers.a = self.dec_byte(self.registers.a)
                        }
                        ArithmeticByteTarget::B => {
                            self.registers.b = self.dec_byte(self.registers.b)
                        }
                        ArithmeticByteTarget::C => {
                            self.registers.c = self.dec_byte(self.registers.c)
                        }
                        ArithmeticByteTarget::D => {
                            self.registers.d = self.dec_byte(self.registers.d)
                        }
                        ArithmeticByteTarget::E => {
                            self.registers.e = self.dec_byte(self.registers.e)
                        }
                        ArithmeticByteTarget::H => {
                            self.registers.h = self.dec_byte(self.registers.h)
                        }
                        ArithmeticByteTarget::L => {
                            self.registers.l = self.dec_byte(self.registers.l)
                        }
                        ArithmeticByteTarget::HLI => todo!(),
                    }

                    self.pc.wrapping_add(1)
                }
            },
            Instruction::HALT => {
                self.is_halted = true;
                self.pc.wrapping_add(1)
            }

            Instruction::INC(inc_type) => {
                match inc_type {
                    ArithmeticTargetType::Word(target) => {
                        match target {
                            ArithmeticWordTarget::BC => todo!(),
                            ArithmeticWordTarget::DE => todo!(),
                            ArithmeticWordTarget::HL => todo!(),
                            ArithmeticWordTarget::SP => todo!(),
                        };

                        self.pc.wrapping_add(2)
                    }
                    ArithmeticTargetType::Byte(arithmetic_byte_target) => {
                        match arithmetic_byte_target {
                            ArithmeticByteTarget::A => {
                                self.registers.a = self.inc_byte(self.registers.a)
                            }
                            ArithmeticByteTarget::B => {
                                self.registers.b = self.inc_byte(self.registers.b)
                            }
                            ArithmeticByteTarget::C => {
                                self.registers.c = self.inc_byte(self.registers.c)
                            }
                            ArithmeticByteTarget::D => {
                                self.registers.d = self.inc_byte(self.registers.d)
                            }
                            ArithmeticByteTarget::E => {
                                self.registers.e = self.inc_byte(self.registers.e)
                            }
                            ArithmeticByteTarget::H => {
                                self.registers.h = self.inc_byte(self.registers.h)
                            }
                            ArithmeticByteTarget::L => {
                                self.registers.l = self.inc_byte(self.registers.l)
                            }
                            ArithmeticByteTarget::HLI => todo!(),
                        }

                        self.pc.wrapping_add(1)
                    }
                }
            }
            Instruction::JP(test) => {
                let jump_condition = self.jmp_test(test);
                self.jump(jump_condition)
            }

            Instruction::JR(test) => {
                if self.jmp_test(test.clone()){
                    if self.debug_view {
                        println!("JR ({:?}), test succesful, jumping 0x{:x}\n", test, self.read_next_byte());
                    }
                    self.pc.wrapping_add(self.read_next_byte() as u16)
                } else {
                    if self.debug_view {
                        println!("JR ({:?}), test failed, jumping 2", test);
                    }
                    self.pc.wrapping_add(2)
                }
            }

            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(target, source) => {
                        let source_value = match source {
                            LoadByteSource::A => self.registers.a,
                            LoadByteSource::D8 => self.read_next_byte(),
                            LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                            LoadByteSource::B => self.registers.b,
                            LoadByteSource::C => self.registers.c,
                            LoadByteSource::D => self.registers.d,
                            LoadByteSource::E => self.registers.e,
                            LoadByteSource::H => self.registers.h,
                            LoadByteSource::L => self.registers.l,
                        };

                        match target {
                            LoadByteTarget::A => self.registers.a = source_value,
                            LoadByteTarget::HLI => {
                                self.bus.write_byte(self.registers.get_hl(), source_value)
                            }
                            LoadByteTarget::B => self.registers.b = source_value,
                            LoadByteTarget::C => self.registers.c = source_value,
                            LoadByteTarget::D => self.registers.d = source_value,
                            LoadByteTarget::E => self.registers.e = source_value,
                            LoadByteTarget::H => self.registers.h = source_value,
                            LoadByteTarget::L => self.registers.l = source_value,
                        }

                        if self.debug_view {
                            println!(
                                "LD (byte). From source={:?},value=0x{:x} to target={:?}",
                                source, source_value, target
                            );
                        }

                        match source {
                            LoadByteSource::D8 => self.pc.wrapping_add(2),
                            _ => self.pc.wrapping_add(1),
                        }
                    }
                    LoadType::Word(load_word_target, load_word_source) => {
                        let source_value = match load_word_source {
                            LoadWordSource::BC => self.registers.get_bc(),
                            LoadWordSource::D16 => self.read_next_word(),
                        };

                        let pc_inc = match load_word_target {
                            LoadWordTarget::BC => {
                                self.registers.set_bc(source_value);
                                3
                            }
                            LoadWordTarget::SP => {
                                self.sp = source_value;
                                3
                            }
                            LoadWordTarget::HL => {
                                self.registers.set_hl(source_value);
                                3
                            }
                        };

                        if self.debug_view {
                            println!(
                                "LD (word). From source={:?},value=0x{:x} to target={:?}",
                                load_word_source, source_value, load_word_target
                            );
                        }

                        self.pc.wrapping_add(pc_inc)
                    }
                    // LoadType::AFromByteAddress => todo!(),
                    // LoadType::ByteAddressFromA => {
                    //     //TODO
                    //     let write_addr: u16 = 0xFF00 | (self.read_next_byte() as u16);
                    //     let write_val = self.registers.a;

                    //     self.bus.write_byte(write_addr, write_val);

                    //     if self.debug_view {
                    //         println!(
                    //             "LD (ByteAddressFromA). From source=A,value=0x{:x} to address={:x}",
                    //             write_val, write_addr
                    //         );
                    //     }

                    //     self.pc.wrapping_add(2)
                    // }
                    LoadType::AIntoHLInc => {
                        let write_addr = self.registers.get_hl();
                        self.bus.write_byte(write_addr, self.registers.a);
                        self.registers.set_hl(write_addr.wrapping_add(1));

                        if self.debug_view {
                            println!(
                                "LD (AIntoHLInc). From source=A,value=0x{:x} to address=0x{:x}",
                                self.registers.a, write_addr
                            );
                        }

                        self.pc.wrapping_add(1)
                    }
                    LoadType::AIntoHLDec => {
                        let write_addr = self.registers.get_hl();
                        self.bus.write_byte(write_addr, self.registers.a);
                        self.registers.set_hl(write_addr.wrapping_sub(1));

                        if self.debug_view {
                            println!(
                                "LD (AIntoHLDec). From source=A,value=0x{:x} to address=0x{:x}",
                                self.registers.a, write_addr
                            );
                        }

                        self.pc.wrapping_add(1)
                    }
                    LoadType::AFromIndirect(ld_byte_address) => {
                        let (addr_inc, pc_inc) = match ld_byte_address {
                            LdByteAddress::C => (self.registers.c, 2),
                            LdByteAddress::A8 => (self.read_next_byte(), 3),
                        };

                        let addr = 0xFF00 + addr_inc as u16;

                        self.registers.a = self.bus.read_byte(addr);

                        if self.debug_view {
                            println!(
                                "LD (AFromIndirect). From source=({:?}),value=0x{:x} to address=A",
                                ld_byte_address, self.bus.read_byte(addr)
                            );
                        }

                        self.pc.wrapping_add(pc_inc)
                    },
                    LoadType::IndirectFromA(ld_byte_address) => {
                        let (addr_inc, pc_inc) = match ld_byte_address {
                            LdByteAddress::C => (self.registers.c, 2),
                            LdByteAddress::A8 => (self.read_next_byte(), 3),
                        };

                        let addr = 0xFF00 + addr_inc as u16;

                        self.bus.write_byte(addr, self.registers.a);

                        if self.debug_view {
                            println!(
                                "LD (IndirectFromA). From source=A,value=0x{:x} to address=0x{:x}",
                                self.registers.a, addr
                            );
                        }

                        self.pc.wrapping_add(pc_inc)
                    },
                }
            }
            Instruction::NOP => self.pc.wrapping_add(1),

            Instruction::OR(target) => match target {
                ArithmeticByteTarget::A => {
                    self.registers.a = self.or(self.registers.a);
                    self.pc.wrapping_add(1)
                }
                ArithmeticByteTarget::B => {
                    self.registers.a = self.or(self.registers.b);
                    self.pc.wrapping_add(1)
                }
                ArithmeticByteTarget::C => {
                    self.registers.a = self.or(self.registers.c);
                    self.pc.wrapping_add(1)
                }
                ArithmeticByteTarget::D => {
                    self.registers.a = self.or(self.registers.d);
                    self.pc.wrapping_add(1)
                }
                ArithmeticByteTarget::E => {
                    self.registers.a = self.or(self.registers.e);
                    self.pc.wrapping_add(1)
                }
                ArithmeticByteTarget::H => {
                    self.registers.a = self.or(self.registers.e);
                    self.pc.wrapping_add(1)
                }
                ArithmeticByteTarget::L => {
                    self.registers.a = self.or(self.registers.h);
                    self.pc.wrapping_add(1)
                }
                ArithmeticByteTarget::HLI => {
                    self.registers.a = self.or(self.bus.read_byte(self.registers.get_hl()));
                    self.pc.wrapping_add(2)
                }
            },

            Instruction::POP(target) => {
                let result = self.pop();
                match target {
                    StackTarget::BC => self.registers.set_bc(result),
                    _ => {
                        panic!("TODO: support more targets")
                    }
                };
                self.pc.wrapping_add(1)
            }

            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::BC => self.registers.get_bc(),
                    _ => {
                        //TODO: implement
                        panic!("TODO")
                    }
                };

                self.push(value);

                self.pc.wrapping_add(1)
            }

            Instruction::RET(test) => self._return(self.jmp_test(test)),

            Instruction::SCF => {
                self.registers.f.carry = true;

                self.pc.wrapping_add(1)
            }

            Instruction::XOR(target) => match target {
                ArithmeticByteTarget::A => {
                    self.registers.a = self.xor(self.registers.a);
                    self.pc.wrapping_add(1)
                }
                ArithmeticByteTarget::B => {
                    self.registers.a = self.xor(self.registers.b);
                    self.pc.wrapping_add(1)
                }
                ArithmeticByteTarget::C => {
                    self.registers.a = self.xor(self.registers.c);
                    self.pc.wrapping_add(1)
                }
                ArithmeticByteTarget::D => {
                    self.registers.a = self.xor(self.registers.d);
                    self.pc.wrapping_add(1)
                }
                ArithmeticByteTarget::E => {
                    self.registers.a = self.xor(self.registers.e);
                    self.pc.wrapping_add(1)
                }
                ArithmeticByteTarget::H => {
                    self.registers.a = self.xor(self.registers.e);
                    self.pc.wrapping_add(1)
                }
                ArithmeticByteTarget::L => {
                    self.registers.a = self.xor(self.registers.h);
                    self.pc.wrapping_add(1)
                }
                ArithmeticByteTarget::HLI => {
                    self.registers.a = self.xor(self.bus.read_byte(self.registers.get_hl()));
                    self.pc.wrapping_add(2)
                }
            },
            _ => {
                //TODO: add error handling for unknown instructions
                panic!("Unknown instruction");
            }
        }
    }

    fn add_byte(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtraction = false;
        self.registers.f.carry = did_overflow;

        // Half Carry is set if adding the lower nibbles of the calue and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        new_value
    }

    fn add_word(&mut self, value: u16) -> u16 {
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtraction = false;
        self.registers.f.carry = did_overflow;

        //TODO: this seems very wrong
        self.registers.f.half_carry = (self.registers.a & 0x20) == 0x20;

        new_value
    }

    fn and(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a & value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtraction = false;
        self.registers.f.carry = true;
        self.registers.f.half_carry = false;

        new_value
    }

    fn or(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a | value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtraction = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;

        new_value
    }

    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a ^ value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtraction = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;

        new_value
    }

    fn inc_byte(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = value.overflowing_add(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtraction = false;
        self.registers.f.carry = did_overflow;

        // Half Carry is set if adding the lower nibbles of the calue and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        new_value
    }

    fn dec_byte(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = value.overflowing_sub(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtraction = true;
        self.registers.f.carry = did_overflow;

        // Half Carry is set if adding the lower nibbles of the calue and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        new_value
    }

    fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            // Gameboy is little endian so read pc + 2 as most significant bit
            // and pc + 1 as least significant bit
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else {
            // If we don't jump we need to still move the program
            // counter forward by 3 since the jump instruction is
            // 3 bytes wide (1 byte for tag and 2 bytes for jump address)
            self.pc.wrapping_add(3)
        }
    }

    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);
    }

    fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);

        (msb as u16) << 8 | (lsb as u16)
    }

    fn jmp_test(&self, test: JumpTest) -> bool {
        match test {
            JumpTest::NotZero => !self.registers.f.zero,
            JumpTest::NotCarry => !self.registers.f.carry,
            JumpTest::Zero => self.registers.f.zero,
            JumpTest::Carry => self.registers.f.carry,
            JumpTest::Always => true,
        }
    }

    fn call(&mut self, should_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            self.read_next_word()
        } else {
            next_pc
        }
    }

    fn _return(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }

    fn read_next_byte(&mut self) -> u8 {
        self.bus.read_byte(self.pc.wrapping_add(1))
    }

    fn read_next_word(&mut self) -> u16 {
        let msp = self.bus.read_byte(self.pc.wrapping_add(1)) as u16;
        let lsp = self.bus.read_byte(self.pc.wrapping_add(2)) as u16;

        msp << 8 | lsp
    }
}
