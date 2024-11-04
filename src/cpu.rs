use crate::instructions::{AdcTargetType, AddByteTarget, AddTargetType, AndTargetType, ArithmeticByteTarget, ArithmeticTargetType, ArithmeticWordTarget, BitPosition, BitRegister, Instruction, JpAddrLoc, JumpTest, LdByteAddress, LdIndirectAddr, LoadByteSource, LoadByteTarget, LoadType, LoadWordSource, LoadWordTarget, ORTargetType, SBCByteTarget, StackTarget, SubByteTarget, XORTargetType};
use crate::memory::MemoryBus;
use crate::registers::Registers;

// const CLOCK_SPEED_MHz: f32 = 4.194304;

#[derive(Debug, Clone)]
pub struct CPU {
    registers: Registers,
    pc: u16,
    sp: u16,
    bus: MemoryBus,
    is_halted: bool,
    debug_view: bool,
    instruction_counter: i64,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Default::default(),
            pc: 0,
            sp: 0xFFFF,
            bus: MemoryBus::new_and_load_bios(),
            is_halted: false,
            debug_view: true,
            instruction_counter: 0,
        }
    }
    pub fn new_and_empty() -> Self {
        Self {
            registers: Default::default(),
            pc: 0,
            sp: 0xFFFF,
            bus: MemoryBus::new_and_empty(),
            is_halted: false,
            debug_view: true,
            instruction_counter: 0,
        }
    }
    pub fn reset(&mut self) {
        self.sp = 0xFFFF;
        self.pc = 0;
    }

    pub fn load_cartridge(&mut self, path: &str) {
        self.bus.load_cartridge(path);
    }

    pub fn step(&mut self) {
        if !self.bus.boot_mode_active() {
            panic!("ROM finished")
        }
        let mut instruction_byte = self.bus.read_byte(self.pc);

        let prefix = instruction_byte == 0xCB;
        if prefix {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        self.pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefix) {
            if self.debug_view {
                println!(
                    "\n\n{}: Executing instruction: 0x{}{:x}. pc=0x{:x} (boot mode active: {})\n{:?}\nsp=0x{:x}",
                    self.instruction_counter,
                    if prefix { "cb" } else { "" },
                    instruction_byte,
                    self.pc,
                    self.bus.boot_mode_active(),
                    self.registers,
                    self.sp
                )
            }
            self.execute(instruction)
        } else {
            let description = format!("0x{}{:x}", if prefix { "cb" } else { "" }, instruction_byte);
            panic!("Unkown instruction found for: {}", description)
        };

        self.instruction_counter += 1;
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        if self.is_halted {
            return self.pc;
        }

        match instruction {
            Instruction::ADC(target) => {
                let (val, pc_inc) = match target {
                    AdcTargetType::A => (self.registers.a, 1),
                    AdcTargetType::B => (self.registers.b, 1),
                    AdcTargetType::C => (self.registers.c, 1),
                    AdcTargetType::D => (self.registers.d, 1),
                    AdcTargetType::E => (self.registers.e, 1),
                    AdcTargetType::H => (self.registers.h, 1),
                    AdcTargetType::L => (self.registers.l, 1),
                    AdcTargetType::HLI => (self.bus.read_byte(self.registers.get_hl()), 1),
                    AdcTargetType::D8 => (self.read_next_byte(), 2),
                };

                self.registers.a = self.add_byte(val + (self.registers.f.carry as u8));

                self.pc.wrapping_add(pc_inc)
            }

            Instruction::ADD(arithmetic_type) => match arithmetic_type {
                AddTargetType::Byte(target) => match target {
                    AddByteTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.add_byte(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    AddByteTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.add_byte(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    AddByteTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add_byte(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    AddByteTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.add_byte(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    AddByteTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.add_byte(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    AddByteTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.add_byte(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    AddByteTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.add_byte(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    AddByteTarget::HLI => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.add_byte(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(2)
                    }
                    AddByteTarget::D8 => {
                        let d8 = self.read_next_byte();
                        let new_value = self.add_byte(d8);
                        self.registers.a = new_value;

                        self.pc.wrapping_add(2)
                    },
                },
                AddTargetType::Word(target) => {
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
                AddTargetType::SPS8 => {
                    let value = self.read_next_byte();
                    let (new_value, overflow) = self.sp.overflowing_add_signed(value.into());
                    self.sp = new_value as u16;

                    self.registers.f.zero = false;
                    self.registers.f.subtraction = false;
                    self.registers.f.carry = overflow;
                    self.registers.f.half_carry = (new_value & 0xF) + (new_value & 0xF) > 0xF;

                    self.pc.wrapping_add(2)
                },
            },

            Instruction::AND(target) => match target {
                AndTargetType::A => {
                    self.registers.a = self.and(self.registers.a);
                    self.pc.wrapping_add(1)
                }
                AndTargetType::B => {
                    self.registers.a = self.and(self.registers.b);
                    self.pc.wrapping_add(1)
                }
                AndTargetType::C => {
                    self.registers.a = self.and(self.registers.c);
                    self.pc.wrapping_add(1)
                }
                AndTargetType::D => {
                    self.registers.a = self.and(self.registers.d);
                    self.pc.wrapping_add(1)
                }
                AndTargetType::E => {
                    self.registers.a = self.and(self.registers.e);
                    self.pc.wrapping_add(1)
                }
                AndTargetType::H => {
                    self.registers.a = self.and(self.registers.e);
                    self.pc.wrapping_add(1)
                }
                AndTargetType::L => {
                    self.registers.a = self.and(self.registers.h);
                    self.pc.wrapping_add(1)
                }
                AndTargetType::HLI => {
                    self.registers.a = self.and(self.bus.read_byte(self.registers.get_hl()));
                    self.pc.wrapping_add(2)
                }
                AndTargetType::D8 => {
                    let d8 = self.read_next_byte();
                    self.registers.a = self.and(d8);
                    self.pc.wrapping_add(2)
                },
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
                    BitRegister::A => ((self.registers.a & and_val) > 0, 2),
                };

                self.registers.f.zero = flag_val;

                self.pc.wrapping_add(pc_inc)
            }

            Instruction::CALL(test) => {
                let should_jump = self.jmp_test(test);
                let new_pc = self.call(should_jump);

                if self.debug_view {
                    println!("CALL. test={:?},res={},curr_pc=0x{:x},new_pc=0x{:x}", test, should_jump, self.pc,new_pc);
                }

                new_pc
            },

            Instruction::CCF => {
                self.registers.f.carry = !self.registers.f.carry;
                self.registers.f.subtraction = false;
                self.registers.f.half_carry = false;

                self.pc.wrapping_add(1)
            },

            Instruction::CP(target) => {
                let (value, pc_inc) = match target {
                    crate::instructions::CPByteTarget::A => {
                        (self.registers.a, 1)
                    },
                    crate::instructions::CPByteTarget::B => {
                        (self.registers.b, 1)
                    },
                    crate::instructions::CPByteTarget::C => {
                        (self.registers.c, 1)
                    },
                    crate::instructions::CPByteTarget::D => {
                        (self.registers.d, 1)
                    },
                    crate::instructions::CPByteTarget::E => {
                        (self.registers.e, 1)
                    },
                    crate::instructions::CPByteTarget::H => {
                        (self.registers.h, 1)
                    },
                    crate::instructions::CPByteTarget::L => {
                        (self.registers.l, 1)
                    },
                    crate::instructions::CPByteTarget::HLI => {
                        (self.bus.read_byte(self.registers.get_hl()), 1)
                    },
                    crate::instructions::CPByteTarget::D8 => {
                        (self.read_next_byte(), 2)
                    },
                };

                // we don't actually care about the result, so we can discard it
                self.sub(value);

                self.pc.wrapping_add(pc_inc)
            },

            Instruction::CPL => {
                self.registers.a = !self.registers.a;

                self.registers.f.half_carry = true;
                self.registers.f.subtraction = true;

                self.pc.wrapping_add(1)
            },
            Instruction::DEC(inc_type) => match inc_type {
                ArithmeticTargetType::Word(target) => {
                    let val = match target {
                        ArithmeticWordTarget::BC => self.registers.get_bc(),
                        ArithmeticWordTarget::DE => self.registers.get_de(),
                        ArithmeticWordTarget::HL => self.registers.get_hl(),
                        ArithmeticWordTarget::SP => self.sp,
                    };

                    let new_val = self.add_word(val);
                    self.registers.set_hl(new_val);

                    self.pc.wrapping_add(2)
                }
                ArithmeticTargetType::Byte(arithmetic_byte_target) => {
                    let pc_inc = match arithmetic_byte_target {
                        ArithmeticByteTarget::A => {
                            self.registers.a = self.dec_byte(self.registers.a);
                            1
                        }
                        ArithmeticByteTarget::B => {
                            self.registers.b = self.dec_byte(self.registers.b);
                            1
                        }
                        ArithmeticByteTarget::C => {
                            self.registers.c = self.dec_byte(self.registers.c);
                            1
                        }
                        ArithmeticByteTarget::D => {
                            self.registers.d = self.dec_byte(self.registers.d);
                            1
                        }
                        ArithmeticByteTarget::E => {
                            self.registers.e = self.dec_byte(self.registers.e);
                            1
                        }
                        ArithmeticByteTarget::H => {
                            self.registers.h = self.dec_byte(self.registers.h);
                            1
                        }
                        ArithmeticByteTarget::L => {
                            self.registers.l = self.dec_byte(self.registers.l);
                            1
                        }
                        ArithmeticByteTarget::HLI => {
                            let addr = self.registers.get_hl();
                            let value = self.bus.read_byte(addr);
                            let new_value = self.dec_byte(value);
                            self.bus.write_byte(self.registers.get_hl(), new_value);

                            3
                        },
                    };

                    self.pc.wrapping_add(pc_inc)
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
                            ArithmeticWordTarget::BC => self.registers.set_bc(self.registers.get_bc().wrapping_add(1)),
                            ArithmeticWordTarget::DE => self.registers.set_de(self.registers.get_de().wrapping_add(1)),
                            ArithmeticWordTarget::HL => self.registers.set_hl(self.registers.get_hl().wrapping_add(1)),
                            ArithmeticWordTarget::SP => self.sp = self.sp.wrapping_add(1),
                        };

                        if self.debug_view {
                            println!("INC (word) target=({:?})", target)
                        };

                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTargetType::Byte(arithmetic_byte_target) => {
                        let pc_inc = match arithmetic_byte_target {
                            ArithmeticByteTarget::A => {
                                self.registers.a = self.inc_byte(self.registers.a);
                                1
                            }
                            ArithmeticByteTarget::B => {
                                self.registers.b = self.inc_byte(self.registers.b);
                                1
                            }
                            ArithmeticByteTarget::C => {
                                self.registers.c = self.inc_byte(self.registers.c);
                                1
                            }
                            ArithmeticByteTarget::D => {
                                self.registers.d = self.inc_byte(self.registers.d);
                                1
                            }
                            ArithmeticByteTarget::E => {
                                self.registers.e = self.inc_byte(self.registers.e);
                                1
                            }
                            ArithmeticByteTarget::H => {
                                self.registers.h = self.inc_byte(self.registers.h);
                                1
                            }
                            ArithmeticByteTarget::L => {
                                self.registers.l = self.inc_byte(self.registers.l);
                                1
                            }
                            ArithmeticByteTarget::HLI =>{
                                let addr = self.registers.get_hl();
                                let value = self.bus.read_byte(addr);
                                let new_value = self.inc_byte(value);
                                self.bus.write_byte(self.registers.get_hl(), new_value);

                                3
                            }
                        };

                        self.pc.wrapping_add(pc_inc)
                    }
                }
            }
            Instruction::JP(test, target) => {
                let jump_condition = self.jmp_test(test);

                match target {
                    JpAddrLoc::A16 => self.jump(jump_condition),
                    JpAddrLoc::HL => self.registers.get_hl(),
                }
            }

            Instruction::JR(test) => {
                if self.jmp_test(test.clone()){
                    let offset = (self.read_next_byte() as i8).wrapping_add(2).into();
                    let new_pc = self.pc.wrapping_add_signed(offset);

                    if self.debug_view {
                        println!("JR ({:?}), test succesful, jumping {}(unsigned=0x{:x}) + pc=0x{:x} = 0x{:x}", 
                        test, offset, self.read_next_byte(), self.pc, new_pc);
                    }

                    new_pc
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
                            LoadWordSource::HL => self.registers.get_hl(),
                            LoadWordSource::SPPlusS8 => {
                                let s8 = self.read_next_byte() as i8;
                                self.sp.wrapping_add_signed(s8.into())
                            },
                        };

                        match load_word_target {
                            LoadWordTarget::BC => self.registers.set_bc(source_value),
                            LoadWordTarget::SP => self.sp = source_value,
                            LoadWordTarget::HL => self.registers.set_hl(source_value),
                            LoadWordTarget::DE => self.registers.set_de(source_value),
                        };

                        if self.debug_view {
                            println!(
                                "LD (word). From source={:?},value=0x{:x} to target={:?}",
                                load_word_source, source_value, load_word_target
                            );
                        }

                        let pc_inc = match (load_word_target, load_word_source) {
                            (LoadWordTarget::HL, LoadWordSource::SPPlusS8) => 2,
                            (LoadWordTarget::SP, LoadWordSource::HL) => 1,
                            _ => 3,
                        };
                        self.pc.wrapping_add(pc_inc)
                    }
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
                    LoadType::AFromByteAddress(ld_byte_address) => {
                        let (addr_inc, pc_inc) = match ld_byte_address {
                            LdByteAddress::C => (self.registers.c, 2),
                            LdByteAddress::A8 => (self.read_next_byte(), 3),
                        };

                        let addr = 0xFF00 + addr_inc as u16;

                        self.registers.a = self.bus.read_byte(addr);

                        if self.debug_view {
                            println!(
                                "LD (AFromByteAddress). From source=({:?}),value=0x{:x} to address=A",
                                ld_byte_address, self.bus.read_byte(addr)
                            );
                        }

                        self.pc.wrapping_add(pc_inc)
                    },
                    LoadType::ByteAddressFromA(ld_byte_address) => {
                        let (addr_inc, pc_inc) = match ld_byte_address {
                            LdByteAddress::C => (self.registers.c, 1),
                            LdByteAddress::A8 => (self.read_next_byte(), 2),
                        };

                        let addr = 0xFF00 + addr_inc as u16;

                        self.bus.write_byte(addr, self.registers.a);

                        if self.debug_view {
                            println!(
                                "LD (ByteAddressFromA). From source=A,value=0x{:x} to address=0x{:x}",
                                self.registers.a, addr
                            );
                        }

                        self.pc.wrapping_add(pc_inc)
                    },
                    LoadType::AFromIndirect(addr_source) => {
                        let (addr, pc_inc) = match addr_source {
                            LdIndirectAddr::BC => {
                                (self.registers.get_bc(), 1)
                            },
                            LdIndirectAddr::DE => {
                                (self.registers.get_de(), 1)
                            },
                            LdIndirectAddr::A16 => {
                                (self.read_next_word(), 3)
                            },
                        };
                        
                        self.registers.a = self.bus.read_byte(addr);

                        if self.debug_view {
                            println!(
                                "LD (AFromIndirect). From source={:?},memory=0x{:x},value=0x{:x} to A",
                                addr_source, addr, self.registers.a
                            );
                        }

                        self.pc.wrapping_add(pc_inc)
                    },
                    LoadType::IndirectFromA(addr_target) => {
                        let addr = self.read_next_word();
                        self.bus.write_byte(addr, self.registers.a);

                        let pc_inc = match addr_target {
                            LdIndirectAddr::BC => {
                                self.bus.write_byte(self.registers.get_bc(), self.registers.a);

                                1
                            },
                            LdIndirectAddr::DE => {
                                self.bus.write_byte(self.registers.get_de(), self.registers.a);

                                1
                            },
                            LdIndirectAddr::A16 => {
                                let write_addr = self.read_next_word();
                                self.bus.write_byte(write_addr, self.registers.a);

                                3
                            },
                        };

                        if self.debug_view {
                            println!(
                                "LD (IndirectFromA). From source=A,value=0x{:x} to address=0x{:x}",
                                self.registers.a, addr
                            );
                        }
                        
                        self.pc.wrapping_add(pc_inc)
                    },
                    LoadType::HLIncIntoA => todo!(),
                    LoadType::HLDecIntoA => todo!(),
                }
            }
            Instruction::NOP => self.pc.wrapping_add(1),

            Instruction::OR(target) => match target {
                ORTargetType::A => {
                    self.registers.a = self.or(self.registers.a);
                    self.pc.wrapping_add(1)
                }
                ORTargetType::B => {
                    self.registers.a = self.or(self.registers.b);
                    self.pc.wrapping_add(1)
                }
                ORTargetType::C => {
                    self.registers.a = self.or(self.registers.c);
                    self.pc.wrapping_add(1)
                }
                ORTargetType::D => {
                    self.registers.a = self.or(self.registers.d);
                    self.pc.wrapping_add(1)
                }
                ORTargetType::E => {
                    self.registers.a = self.or(self.registers.e);
                    self.pc.wrapping_add(1)
                }
                ORTargetType::H => {
                    self.registers.a = self.or(self.registers.e);
                    self.pc.wrapping_add(1)
                }
                ORTargetType::L => {
                    self.registers.a = self.or(self.registers.h);
                    self.pc.wrapping_add(1)
                }
                ORTargetType::HLI => {
                    self.registers.a = self.or(self.bus.read_byte(self.registers.get_hl()));
                    self.pc.wrapping_add(2)
                }
                ORTargetType::D8 => {
                    let d8 = self.read_next_byte();
                    self.registers.a = self.or(d8);

                    self.pc.wrapping_add(2)
                },
            },

            Instruction::POP(target) => {
                let result = self.pop();
                match target {
                    StackTarget::BC => self.registers.set_bc(result),
                    StackTarget::DE => self.registers.set_de(result),
                    StackTarget::HL => self.registers.set_hl(result),
                    StackTarget::AF => self.registers.set_af(result),
                };
                self.pc.wrapping_add(1)
            }

            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::BC => self.registers.get_bc(),
                    StackTarget::DE => self.registers.get_de(),
                    StackTarget::HL => self.registers.get_hl(),
                    StackTarget::AF => self.registers.get_af(),
                };

                self.push(value);

                if self.debug_view {
                    println!("PUSH. target={:?},value=0x{:x} (sp=0x{:x})", target, value, self.sp);
                }

                self.pc.wrapping_add(1)
            }

            Instruction::RES(pos, register) => {
                let and_val = match pos {
                    BitPosition::Zero => 1,
                    BitPosition::One => 2,
                    BitPosition::Two => 4,
                    BitPosition::Three => 8,
                    BitPosition::Four => 16,
                    BitPosition::Five => 32,
                    BitPosition::Six => 64,
                    BitPosition::Seven => 128,
                };

                let pc_inc = match register {
                    BitRegister::B => {
                        self.registers.b = !and_val & self.registers.b;
                        2
                    },
                    BitRegister::C => {
                        self.registers.c = !and_val & self.registers.c;
                        2
                    },
                    BitRegister::D => {
                        self.registers.d = !and_val & self.registers.d;
                        2
                    },
                    BitRegister::E => {
                        self.registers.e = !and_val & self.registers.e;
                        2
                    },
                    BitRegister::H => {
                        self.registers.h = !and_val & self.registers.h;
                        2
                    },
                    BitRegister::L => {
                        self.registers.l = !and_val & self.registers.l;
                        2
                    },
                    BitRegister::HLI => {
                        self.bus.write_byte(self.registers.get_hl(), !and_val & self.bus.read_byte(self.registers.get_hl()));
                        
                        4
                    },
                    BitRegister::A => {
                        self.registers.a = !and_val & self.registers.a;
                        2
                    },
                };

                self.pc.wrapping_add(pc_inc)
            }

            Instruction::RET(test) => self._return(self.jmp_test(test)),

            Instruction::RLA => {
                let old_carry = self.registers.f.carry;
                self.registers.f.carry = (self.registers.a & 128) > 0;
                self.registers.a = self.registers.a.rotate_left(1) | (old_carry as u8);
                self.pc.wrapping_add(1)
            }

            Instruction::RLC(target) => {
                let (value, pc_inc) = match target {
                    ArithmeticByteTarget::A => (self.registers.a, 2),
                    ArithmeticByteTarget::B => (self.registers.b, 2),
                    ArithmeticByteTarget::C => (self.registers.c, 2),
                    ArithmeticByteTarget::D => (self.registers.d, 2),
                    ArithmeticByteTarget::E => (self.registers.e, 2),
                    ArithmeticByteTarget::H => (self.registers.h, 2),
                    ArithmeticByteTarget::L => (self.registers.l, 2),
                    ArithmeticByteTarget::HLI => (self.bus.read_byte(self.registers.get_hl()), 4),
                };

               self.registers.f.carry = value & 1 > 0;
               self.registers.f.zero = value == 0;

                let new_value = value.rotate_left(1);

                if self.debug_view {
                    println!("RLC. target={:?}. orig_value=0x{:x}, new_value=0x{:x}, carry={}", target, value, new_value, self.registers.f.carry);
                }

                match target {
                    ArithmeticByteTarget::A => self.registers.a = new_value,
                    ArithmeticByteTarget::B => self.registers.b = new_value,
                    ArithmeticByteTarget::C => self.registers.c = new_value,
                    ArithmeticByteTarget::D => self.registers.d = new_value,
                    ArithmeticByteTarget::E => self.registers.e = new_value,
                    ArithmeticByteTarget::H => self.registers.h = new_value,
                    ArithmeticByteTarget::L => self.registers.l = new_value,
                    ArithmeticByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), new_value),
                }

                self.pc.wrapping_add(pc_inc)
            }

            Instruction::RL(target) => {
                let pc_inc = match target{
                    ArithmeticByteTarget::A => {
                        self.registers.a = self.rl(self.registers.a);

                        2
                    },
                    ArithmeticByteTarget::B => {
                        self.registers.b = self.rl(self.registers.b);

                        2
                    },
                    ArithmeticByteTarget::C => {
                        self.registers.c = self.rl(self.registers.c);

                        2
                    },
                    ArithmeticByteTarget::D => {
                        self.registers.d = self.rl(self.registers.d);

                        2
                    },
                    ArithmeticByteTarget::E => {
                        self.registers.e = self.rl(self.registers.e);

                        2
                    },
                    ArithmeticByteTarget::H => {
                        self.registers.h = self.rl(self.registers.h);

                        2
                    },
                    ArithmeticByteTarget::L => {
                        self.registers.l = self.rl(self.registers.l);

                        2
                    },
                    ArithmeticByteTarget::HLI => {
                        let address = self.registers.get_hl();
                        let new_value = self.rl(self.bus.read_byte(address));
                        self.bus.write_byte(address, new_value);

                        4
                    },
                };

                self.pc.wrapping_add(pc_inc)
            }

            Instruction::RR(target) => {
                let pc_inc = match target{
                    ArithmeticByteTarget::A => {
                        self.registers.a = self.rr(self.registers.a);

                        2
                    },
                    ArithmeticByteTarget::B => {
                        self.registers.b = self.rr(self.registers.b);

                        2
                    },
                    ArithmeticByteTarget::C => {
                        self.registers.c = self.rr(self.registers.c);

                        2
                    },
                    ArithmeticByteTarget::D => {
                        self.registers.d = self.rr(self.registers.d);

                        2
                    },
                    ArithmeticByteTarget::E => {
                        self.registers.e = self.rr(self.registers.e);

                        2
                    },
                    ArithmeticByteTarget::H => {
                        self.registers.h = self.rr(self.registers.h);

                        2
                    },
                    ArithmeticByteTarget::L => {
                        self.registers.l = self.rr(self.registers.l);

                        2
                    },
                    ArithmeticByteTarget::HLI => {
                        let address = self.registers.get_hl();
                        let new_value = self.rr(self.bus.read_byte(address));
                        self.bus.write_byte(address, new_value);

                        4
                    },
                };

                self.pc.wrapping_add(pc_inc)
            }

            Instruction::RRA => {
                let old_carry = self.registers.f.carry;

                self.registers.f.carry = (self.registers.a & 1) > 0;
                self.registers.a = self.registers.a.rotate_right(1) | (128 * (old_carry as u8));
                self.pc.wrapping_add(1)
            }

            Instruction::RRC(target) => {
                let (value, pc_inc) = match target {
                    ArithmeticByteTarget::A => (self.registers.a, 2),
                    ArithmeticByteTarget::B => (self.registers.b, 2),
                    ArithmeticByteTarget::C => (self.registers.c, 2),
                    ArithmeticByteTarget::D => (self.registers.d, 2),
                    ArithmeticByteTarget::E => (self.registers.e, 2),
                    ArithmeticByteTarget::H => (self.registers.h, 2),
                    ArithmeticByteTarget::L => (self.registers.l, 2),
                    ArithmeticByteTarget::HLI => (self.bus.read_byte(self.registers.get_hl()), 4),
                };

               self.registers.f.carry = value & 1 > 0;
               self.registers.f.zero = value == 0;

                let new_value = value.rotate_right(1);

                if self.debug_view {
                    println!("RLC. target={:?}. orig_value=0x{:x}, new_value=0x{:x}, carry={}", target, value, new_value, self.registers.f.carry);
                }

                match target {
                    ArithmeticByteTarget::A => self.registers.a = new_value,
                    ArithmeticByteTarget::B => self.registers.b = new_value,
                    ArithmeticByteTarget::C => self.registers.c = new_value,
                    ArithmeticByteTarget::D => self.registers.d = new_value,
                    ArithmeticByteTarget::E => self.registers.e = new_value,
                    ArithmeticByteTarget::H => self.registers.h = new_value,
                    ArithmeticByteTarget::L => self.registers.l = new_value,
                    ArithmeticByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), new_value),
                }

                self.pc.wrapping_add(pc_inc)
            }

            Instruction::RST(position) => {
                let and_offset = match position {
                    BitPosition::Zero => 1,
                    BitPosition::One => 2,
                    BitPosition::Two => 4,
                    BitPosition::Three => 8,
                    BitPosition::Four => 16,
                    BitPosition::Five => 32,
                    BitPosition::Six => 64,
                    BitPosition::Seven => 128,
                };

                self.push(self.pc);

                let msb = self.bus.read_byte(and_offset) as u16;
                let lsb = self.bus.read_byte(and_offset.wrapping_add(1)) as u16;
                self.pc = msb << 8 | lsb;

                self.pc
            }

            Instruction::SBC(target) => {
                let (val, pc_inc) = match target {
                    SBCByteTarget::A => (self.registers.a, 1),
                    SBCByteTarget::B => (self.registers.b, 1),
                    SBCByteTarget::C => (self.registers.c, 1),
                    SBCByteTarget::D => (self.registers.d, 1),
                    SBCByteTarget::E => (self.registers.e, 1),
                    SBCByteTarget::H => (self.registers.h, 1),
                    SBCByteTarget::L => (self.registers.l, 1),
                    SBCByteTarget::HLI => (self.bus.read_byte(self.registers.get_hl()), 1),
                    SBCByteTarget::D8 => (self.read_next_byte(), 2),
                };

                let new_val = self.sub(val.wrapping_add(self.registers.f.carry as u8));
                self.registers.a = new_val;

                if self.debug_view {
                    println!("SBC. target={:?}, orig_val=0x{:x},new_val=0x{:x}", target, val, new_val);
                }

                self.pc.wrapping_add(pc_inc)
            }

            Instruction::SET(pos, register) => {
                let or_val = match pos {
                    BitPosition::Zero => 1,
                    BitPosition::One => 2,
                    BitPosition::Two => 4,
                    BitPosition::Three => 8,
                    BitPosition::Four => 16,
                    BitPosition::Five => 32,
                    BitPosition::Six => 64,
                    BitPosition::Seven => 128,
                };

                let pc_inc = match register {
                    BitRegister::B => {
                        self.registers.b = or_val | self.registers.b;
                        2
                    },
                    BitRegister::C => {
                        self.registers.c = or_val | self.registers.c;
                        2
                    },
                    BitRegister::D => {
                        self.registers.d = or_val | self.registers.d;
                        2
                    },
                    BitRegister::E => {
                        self.registers.e = or_val | self.registers.e;
                        2
                    },
                    BitRegister::H => {
                        self.registers.h = or_val | self.registers.h;
                        2
                    },
                    BitRegister::L => {
                        self.registers.l = or_val | self.registers.l;
                        2
                    },
                    BitRegister::HLI => {
                        self.bus.write_byte(self.registers.get_hl(), or_val | self.bus.read_byte(self.registers.get_hl()));
                        4
                    },
                    BitRegister::A => {
                        self.registers.a = or_val | self.registers.a;
                        2
                    },
                };

                self.pc.wrapping_add(pc_inc)
            }

            Instruction::SCF => {
                self.registers.f.carry = true;
                self.registers.f.half_carry = false;
                self.registers.f.subtraction = false;

                self.pc.wrapping_add(1)
            },

            Instruction::SUB(target) => {
                let (val, pc_inc) = match target {
                    SubByteTarget::A => (self.registers.a, 1),
                    SubByteTarget::B => (self.registers.b, 1),
                    SubByteTarget::C => (self.registers.c, 1),
                    SubByteTarget::D => (self.registers.d, 1),
                    SubByteTarget::E => (self.registers.e, 1),
                    SubByteTarget::H => (self.registers.h, 1),
                    SubByteTarget::L => (self.registers.l, 1),
                    SubByteTarget::HLI => (self.bus.read_byte(self.registers.get_hl()), 1),
                    SubByteTarget::D8 => (self.read_next_byte(), 2),
                };

                let new_val = self.sub(val);
                self.registers.a = new_val;

                if self.debug_view {
                    println!("SUB. target={:?}, orig_val=0x{:x},new_val=0x{:x}", target, val, new_val);
                }

                self.pc.wrapping_add(pc_inc)
            }

            Instruction::SWAP(target) => {
                let (value, pc_inc) = match target {
                    ArithmeticByteTarget::A => (self.registers.a, 2),
                    ArithmeticByteTarget::B => (self.registers.b, 2),
                    ArithmeticByteTarget::C => (self.registers.c, 2),
                    ArithmeticByteTarget::D => (self.registers.d, 2),
                    ArithmeticByteTarget::E => (self.registers.e, 2),
                    ArithmeticByteTarget::H => (self.registers.h, 2),
                    ArithmeticByteTarget::L => (self.registers.l, 2),
                    ArithmeticByteTarget::HLI => (self.bus.read_byte(self.registers.get_hl()), 4),
                };

               self.registers.f.zero = value == 0;

                let new_value = value.rotate_left(4);

                if self.debug_view {
                    println!("SWAP. target={:?}. orig_value=0x{:x}, new_value=0x{:x}, zero={}", target, value, new_value, self.registers.f.zero);
                }

                match target {
                    ArithmeticByteTarget::A => self.registers.a = new_value,
                    ArithmeticByteTarget::B => self.registers.b = new_value,
                    ArithmeticByteTarget::C => self.registers.c = new_value,
                    ArithmeticByteTarget::D => self.registers.d = new_value,
                    ArithmeticByteTarget::E => self.registers.e = new_value,
                    ArithmeticByteTarget::H => self.registers.h = new_value,
                    ArithmeticByteTarget::L => self.registers.l = new_value,
                    ArithmeticByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), new_value),
                }

                self.pc.wrapping_add(pc_inc)
            }

            Instruction::XOR(target) => match target {
                XORTargetType::A => {
                    self.registers.a = self.xor(self.registers.a);
                    self.pc.wrapping_add(1)
                }
                XORTargetType::B => {
                    self.registers.a = self.xor(self.registers.b);
                    self.pc.wrapping_add(1)
                }
                XORTargetType::C => {
                    self.registers.a = self.xor(self.registers.c);
                    self.pc.wrapping_add(1)
                }
                XORTargetType::D => {
                    self.registers.a = self.xor(self.registers.d);
                    self.pc.wrapping_add(1)
                }
                XORTargetType::E => {
                    self.registers.a = self.xor(self.registers.e);
                    self.pc.wrapping_add(1)
                }
                XORTargetType::H => {
                    self.registers.a = self.xor(self.registers.e);
                    self.pc.wrapping_add(1)
                }
                XORTargetType::L => {
                    self.registers.a = self.xor(self.registers.h);
                    self.pc.wrapping_add(1)
                }
                XORTargetType::HLI => {
                    self.registers.a = self.xor(self.bus.read_byte(self.registers.get_hl()));
                    self.pc.wrapping_add(2)
                }
                XORTargetType::D8 => {
                    let d8 = self.read_next_byte();
                    self.registers.a = self.xor(d8);
                    self.pc.wrapping_add(2)
                },
            },
            _ => {
                panic!("Unknown instruction: {:?}", instruction);
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
        self.registers.f.half_carry = (self.registers.a & 0x20) > 0;

        new_value
    }

    fn rl(&mut self, value: u8) -> u8 {
        let new_val = value.rotate_left(1) & (self.registers.f.carry as u8);

        self.registers.f.zero = new_val == 0;
        self.registers.f.carry = (value & 128) > 0;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = false;

        new_val
    }

    fn rr(&mut self, value: u8) -> u8 {
        let new_val = value.rotate_right(1) & (128 * (self.registers.f.carry as u8));

        self.registers.f.zero = new_val == 0;
        self.registers.f.carry = (value & 1) > 0;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = false;

        new_val
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

        if self.debug_view {
            println!("XOR. value=0x{:x},new_value=0x{:x}", value, new_value);
        }

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
        if should_jump {
            self.push(self.pc.wrapping_add(1));

            self.read_next_word()
        } else {
            self.pc.wrapping_add(3)
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
        let lsp = self.bus.read_byte(self.pc.wrapping_add(1)) as u16;
        let msp = self.bus.read_byte(self.pc.wrapping_add(2)) as u16;

        msp << 8 | lsp
    }

    fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtraction = true;
        self.registers.f.carry = did_overflow;

        // Half Carry is set if adding the lower nibbles of the calue and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        new_value
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_matrix;

    use super::{Instruction, CPU};

	#[test_matrix(
        0x00_u8..=0xFF_u8,
        [true, false]
    )]
	fn test_all_instructions_empty_cpu(opcode: u8, prefixed: bool){
		let ndef: [u8; 12] = [
			0xCB,
			0xD3,
			0xDB,
			0xDD,
			0xE3,
			0xE4,
			0xEB,
			0xEC,
			0xED,
			0xF4,
			0xFC,
			0xFD,

		];

		if !prefixed && ndef.contains(&opcode) {
            return;
        }

        let instruction = Instruction::from_byte(opcode, prefixed).unwrap();

        let mut cpu = CPU::new_and_empty();

        cpu.execute(instruction);
	}
}