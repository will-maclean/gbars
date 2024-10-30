use crate::instructions::{ArithmeticByteTarget, ArithmeticWordTarget, ArithmeticTargetType, Instruction, JumpTest, LoadByteSource, LoadByteTarget, LoadType, StackTarget};
use crate::registers::Registers;

const CLOCK_SPEED_MHz: f32 = 4.194304;

#[derive(Debug, Clone, Copy)]
struct MemoryBus{
    memory: [u8; 0xFFFF]
}

impl MemoryBus{
    fn new_and_empty() -> Self {
        Self {
            memory: [0; 0xFFFF]
        }
    }
    fn read_byte(&self, address: u16) -> u8 {
        return self.memory[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        //TODO: invalid write regions?
        self.memory[address as usize] = value;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CPU{
    registers: Registers,
    pc: u16,
    sp: u16,
    bus: MemoryBus,
    is_halted: bool,
}

impl CPU {
    pub fn new_and_empty() -> Self {
        Self {
            registers: Default::default(),
            pc: 0,
            sp: 0xFFFF,
            bus: MemoryBus::new_and_empty(),
            is_halted: false,
        }
    }
    pub fn reset(&mut self){
        self.sp = 0xFFFF;
        self.pc = 0;
    }

    pub fn step(&mut self){
        let mut instruction_byte = self.bus.read_byte(self.pc);

        let prefix = instruction_byte == 0xCB;
        if prefix {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        self.pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefix) {
            self.execute(instruction)
        } else {
            let description = format!("0x{}{:x}", if prefix { "cb" } else { "" }, instruction_byte);
            panic!("Unkown instruction found for: {}", description)
        };
    }

    fn execute(&mut self, instruction: Instruction) -> u16{
        if self.is_halted {
            return self.pc;
        }
        match instruction {
            Instruction::ADD(arithmetic_type) => {
                match arithmetic_type {
                    ArithmeticTargetType::Byte(target) => {
                        let value = match target {
                            ArithmeticByteTarget::A => self.registers.a,
                            ArithmeticByteTarget::B => self.registers.b,
                            ArithmeticByteTarget::C => self.registers.c,
                            ArithmeticByteTarget::D => self.registers.d,
                            ArithmeticByteTarget::E => self.registers.e,
                            ArithmeticByteTarget::H => self.registers.h,
                            ArithmeticByteTarget::L => self.registers.l,
                            ArithmeticByteTarget::HLI => todo!(),
                        };
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTargetType::Word(target) => {
                        match target {
                            ArithmeticWordTarget::BC => todo!(),
                            ArithmeticWordTarget::DE => todo!(),
                            ArithmeticWordTarget::HL => todo!(),
                        }
                        
                        self.pc.wrapping_add(2)
                    }
                }
            },

            Instruction::AND(target) => {
                match target {
                    ArithmeticByteTarget::A => {
                        self.registers.a = self.and(self.registers.a);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticByteTarget::B => {
                        self.registers.a = self.and(self.registers.b);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticByteTarget::C => {
                        self.registers.a = self.and(self.registers.c);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticByteTarget::D => {
                        self.registers.a = self.and(self.registers.d);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticByteTarget::E => {
                        self.registers.a = self.and(self.registers.e);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticByteTarget::H => {
                        self.registers.a = self.and(self.registers.e);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticByteTarget::L => {
                        self.registers.a = self.and(self.registers.h);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticByteTarget::HLI => {
                        self.registers.a = self.and(self.bus.read_byte(self.registers.get_hl()));
                        self.pc.wrapping_add(2)
                    },
                }
            },

            Instruction::CALL(test) => self.call(self.jmp_test(test)),
            Instruction::DEC(inc_type) => {
                match inc_type {
                    ArithmeticTargetType::Word(target) => {
                        match target {
                            // TODO: no way this works
                            ArithmeticWordTarget::BC => todo!(),
                            ArithmeticWordTarget::DE => todo!(),
                            ArithmeticWordTarget::HL => todo!(),
                        };
                        
                        self.pc.wrapping_add(2)
                    }
                    ArithmeticTargetType::Byte(arithmetic_byte_target) =>{
                        match arithmetic_byte_target {
                            ArithmeticByteTarget::A => self.registers.a = self.dec_byte(self.registers.a),
                            ArithmeticByteTarget::B => self.registers.b = self.dec_byte(self.registers.b),
                            ArithmeticByteTarget::C => self.registers.c = self.dec_byte(self.registers.c),
                            ArithmeticByteTarget::D => self.registers.d = self.dec_byte(self.registers.d),
                            ArithmeticByteTarget::E => self.registers.e = self.dec_byte(self.registers.e),
                            ArithmeticByteTarget::H => self.registers.h = self.dec_byte(self.registers.h),
                            ArithmeticByteTarget::L => self.registers.l = self.dec_byte(self.registers.l),
                            ArithmeticByteTarget::HLI => todo!(),
                        }

                        self.pc.wrapping_add(1)
                    }
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
                            // TODO: no way this works
                            ArithmeticWordTarget::BC => todo!(),
                            ArithmeticWordTarget::DE => todo!(),
                            ArithmeticWordTarget::HL => todo!(),
                        };
                        
                        self.pc.wrapping_add(2)
                    }
                    ArithmeticTargetType::Byte(arithmetic_byte_target) =>{
                        match arithmetic_byte_target {
                            ArithmeticByteTarget::A => self.registers.a = self.inc_byte(self.registers.a),
                            ArithmeticByteTarget::B => self.registers.b = self.inc_byte(self.registers.b),
                            ArithmeticByteTarget::C => self.registers.c = self.inc_byte(self.registers.c),
                            ArithmeticByteTarget::D => self.registers.d = self.inc_byte(self.registers.d),
                            ArithmeticByteTarget::E => self.registers.e = self.inc_byte(self.registers.e),
                            ArithmeticByteTarget::H => self.registers.h = self.inc_byte(self.registers.h),
                            ArithmeticByteTarget::L => self.registers.l = self.inc_byte(self.registers.l),
                            ArithmeticByteTarget::HLI => todo!(),
                        }
                        
                        self.pc.wrapping_add(1)
                    }
                }
            },
            Instruction::JP(test) => {
                let jump_condition = self.jmp_test(test);
                self.jump(jump_condition)
            }
            
            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(target, source) => {
                        let source_value = match source {
                            LoadByteSource::A => {self.registers.a}
                            LoadByteSource::D8 => {self.read_next_byte()}
                            LoadByteSource::HLI => {self.bus.read_byte(self.registers.get_hl())}
                            LoadByteSource::B => todo!(),
                            LoadByteSource::C => todo!(),
                            LoadByteSource::D => todo!(),
                            LoadByteSource::E => todo!(),
                            LoadByteSource::H => todo!(),
                            LoadByteSource::L => todo!(),
                        };

                        match target {
                            LoadByteTarget::A => self.registers.a = source_value,
                            LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), source_value),
                            _ => { panic!("TODO: implement other targets") }
                        }

                        match source {
                            LoadByteSource::D8  => self.pc.wrapping_add(2),
                            _                   => self.pc.wrapping_add(1),
                        }
                    }
                    LoadType::Word(load_word_target, load_word_source) => todo!(),
                    LoadType::AFromIndirect => todo!(),
                    LoadType::IndirectFromA => todo!(),
                    LoadType::AFromByteAddress => todo!(),
                    LoadType::ByteAddressFromA => todo!(),
                }
            }
            Instruction::NOP => self.pc.wrapping_add(1),

            Instruction::OR(target) => {
                match target {
                    ArithmeticByteTarget::A => {
                        self.registers.a = self.or(self.registers.a);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticByteTarget::B => {
                        self.registers.a = self.or(self.registers.b);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticByteTarget::C => {
                        self.registers.a = self.or(self.registers.c);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticByteTarget::D => {
                        self.registers.a = self.or(self.registers.d);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticByteTarget::E => {
                        self.registers.a = self.or(self.registers.e);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticByteTarget::H => {
                        self.registers.a = self.or(self.registers.e);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticByteTarget::L => {
                        self.registers.a = self.or(self.registers.h);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticByteTarget::HLI => {
                        self.registers.a = self.or(self.bus.read_byte(self.registers.get_hl()));
                        self.pc.wrapping_add(2)
                    },
                }
            },
            
            Instruction::POP(target) => {
                let result = self.pop();
                match target {
                    StackTarget::BC => self.registers.set_bc(result),
                    _ => { panic!("TODO: support more targets") }
                };
                self.pc.wrapping_add(1)

            },

            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::BC => {
                        self.registers.get_bc()
                    },
                    _ => {
                        //TODO: implement
                        panic!("TODO")
                    }
                };

                self.push(value);

                self.pc.wrapping_add(1)
            },


            Instruction::RET(test) => self._return(self.jmp_test(test)),

            Instruction::XOR(target) => {
                match target {
                    ArithmeticByteTarget::A => {
                        self.registers.a = self.xor(self.registers.a);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticByteTarget::B => {
                        self.registers.a = self.xor(self.registers.b);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticByteTarget::C => {
                        self.registers.a = self.xor(self.registers.c);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticByteTarget::D => {
                        self.registers.a = self.xor(self.registers.d);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticByteTarget::E => {
                        self.registers.a = self.xor(self.registers.e);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticByteTarget::H => {
                        self.registers.a = self.xor(self.registers.e);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticByteTarget::L => {
                        self.registers.a = self.xor(self.registers.h);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticByteTarget::HLI => {
                        self.registers.a = self.xor(self.bus.read_byte(self.registers.get_hl()));
                        self.pc.wrapping_add(2)
                    },
                }
            },
            _ => {
                //TODO: add error handling for unknown instructions
                panic!("Unknown instruction");
            }
        }
    }

    fn add(&mut self, value: u8) -> u8{
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

      fn push(&mut self, value: u16){
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
            JumpTest::Always => true
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
        todo!()
    }

    fn read_next_word(&mut self) -> u16 {
        todo!()
    }
}