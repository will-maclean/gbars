#[derive(Debug, Copy, Clone)]
pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

#[derive(Debug)]
pub enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}
#[derive(Debug)]
pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}

#[derive(Debug)]
pub enum LoadWordTarget {
    BC,
    HL,
    SP,
}

#[derive(Debug)]
pub enum LoadWordSource {
    BC,
    D16,
}

#[derive(Debug, Clone, Copy)]
pub enum LdByteAddress{
	C, A8
}

#[derive(Debug)]
pub enum LoadType {
    // Load 8 bits from LoadByteSource into LoadByteTarget
    Byte(LoadByteTarget, LoadByteSource),

    // Load 16 bits from LoadWordSource into LoadWordTarget
    Word(LoadWordTarget, LoadWordSource),

    // load the A register with the contents from a value from a memory location whose address is stored in some location
    AFromIndirect(LdByteAddress),

    // load a memory location whose address is stored in some location with the contents of the A register
    IndirectFromA(LdByteAddress),

    // Just like AFromIndirect except the memory address is some address in the very last byte of memory
    // AFromByteAddress,

    // // Just like IndirectFromA except the memory address is some address in the very last byte of memory
    // ByteAddressFromA,

    // Store the contents of register A into the memory location specified by register pair HL, and simultaneously increment the contents of HL.
    AIntoHLInc,

    // Store the contents of register A into the memory location specified by register pair HL, and simultaneously decrement the contents of HL.
    AIntoHLDec,
}

#[derive(Debug)]
pub enum StackTarget {
    BC,
    DE,
    HL,
}

#[derive(Debug)]
pub enum ArithmeticTargetType {
    Byte(ArithmeticByteTarget),
    Word(ArithmeticWordTarget),
}

#[derive(Debug)]
pub enum BitPosition {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

#[derive(Debug)]
pub enum BitRegister {
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

#[derive(Debug)]
pub enum Instruction {
    ADD(ArithmeticTargetType),
    // No-op
    NOP,

    // Halt
    HALT,

    SUB(ArithmeticByteTarget),

    AND(ArithmeticByteTarget),

    OR(ArithmeticByteTarget),

    XOR(ArithmeticByteTarget),

    // Increment
    INC(ArithmeticTargetType),

    // Decrement
    DEC(ArithmeticTargetType),

    // Jump to a particular address dependent on one of the following
    // conditions: the zero flag is true, the zero flag is flase, the
    // carry flag is true, the carry flag is false, or always jump.
    JP(JumpTest),

	JR(JumpTest),

    // Load
    LD(LoadType),

    // Push
    PUSH(StackTarget),

    // Pop
    POP(StackTarget),

    // Call
    CALL(JumpTest),

    // Return
    RET(JumpTest),

    // Bit
    BIT(BitPosition, BitRegister),

    // Set Carry flag
    SCF,
    // TODO: implement the other instruction types
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::NOP),
            0x01 => Some(Instruction::LD(LoadType::Word(
                LoadWordTarget::BC,
                LoadWordSource::D16,
            ))),
            0x03 => Some(Instruction::INC(ArithmeticTargetType::Word(
                ArithmeticWordTarget::BC,
            ))),
            0x04 => Some(Instruction::INC(ArithmeticTargetType::Byte(
                ArithmeticByteTarget::B,
            ))),
            0x05 => Some(Instruction::DEC(ArithmeticTargetType::Byte(
                ArithmeticByteTarget::B,
            ))),
            0x06 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::D8,
            ))),
			0x0D => Some(Instruction::DEC(ArithmeticTargetType::Byte(ArithmeticByteTarget::C))),
			0x0E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D8))),
			0x18 => Some(Instruction::JR(JumpTest::Always)),
			0x20 => Some(Instruction::JR(JumpTest::NotZero)),
            0x21 => Some(Instruction::LD(LoadType::Word(
                LoadWordTarget::HL,
                LoadWordSource::D16,
            ))),
            0x22 => Some(Instruction::LD(LoadType::AIntoHLInc)),
			0x28 => Some(Instruction::JR(JumpTest::Zero)),
			0x30 => Some(Instruction::JR(JumpTest::NotCarry)),
            0x31 => Some(Instruction::LD(LoadType::Word(
                LoadWordTarget::SP,
                LoadWordSource::D16,
            ))),
            0x32 => Some(Instruction::LD(LoadType::AIntoHLDec)),
            0x37 => Some(Instruction::SCF),
			0x20 => Some(Instruction::JR(JumpTest::Carry)),
            0x3E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::D8,
            ))),
            0x40 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::B,
            ))),
            0x41 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::C,
            ))),
            0x42 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::D,
            ))),
            0x43 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::E,
            ))),
            0x44 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::H,
            ))),
            0x45 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::L,
            ))),
            0x46 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::HLI,
            ))),
            0x47 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::A,
            ))),
			0x5F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::A))),
			0x77 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLI))),
            0x80 => Some(Instruction::ADD(ArithmeticTargetType::Byte(
                ArithmeticByteTarget::B,
            ))),
            0x81 => Some(Instruction::ADD(ArithmeticTargetType::Byte(
                ArithmeticByteTarget::C,
            ))),
            0x82 => Some(Instruction::ADD(ArithmeticTargetType::Byte(
                ArithmeticByteTarget::D,
            ))),
            0x83 => Some(Instruction::ADD(ArithmeticTargetType::Byte(
                ArithmeticByteTarget::E,
            ))),
            0x84 => Some(Instruction::ADD(ArithmeticTargetType::Byte(
                ArithmeticByteTarget::H,
            ))),
            0x85 => Some(Instruction::ADD(ArithmeticTargetType::Byte(
                ArithmeticByteTarget::L,
            ))),
            0x86 => Some(Instruction::ADD(ArithmeticTargetType::Byte(
                ArithmeticByteTarget::HLI,
            ))),
            0x87 => Some(Instruction::ADD(ArithmeticTargetType::Byte(
                ArithmeticByteTarget::A,
            ))),
            0x90 => Some(Instruction::SUB(ArithmeticByteTarget::B)),
            0x91 => Some(Instruction::SUB(ArithmeticByteTarget::C)),
            0x92 => Some(Instruction::SUB(ArithmeticByteTarget::D)),
            0x93 => Some(Instruction::SUB(ArithmeticByteTarget::E)),
            0x94 => Some(Instruction::SUB(ArithmeticByteTarget::H)),
            0x95 => Some(Instruction::SUB(ArithmeticByteTarget::L)),
            0x96 => Some(Instruction::SUB(ArithmeticByteTarget::HLI)),
            0x97 => Some(Instruction::SUB(ArithmeticByteTarget::A)),
            0xA0 => Some(Instruction::AND(ArithmeticByteTarget::B)),
            0xA1 => Some(Instruction::AND(ArithmeticByteTarget::C)),
            0xA2 => Some(Instruction::AND(ArithmeticByteTarget::D)),
            0xA3 => Some(Instruction::AND(ArithmeticByteTarget::E)),
            0xA4 => Some(Instruction::AND(ArithmeticByteTarget::H)),
            0xA5 => Some(Instruction::AND(ArithmeticByteTarget::L)),
            0xA6 => Some(Instruction::AND(ArithmeticByteTarget::HLI)),
            0xA7 => Some(Instruction::AND(ArithmeticByteTarget::A)),
            0xA8 => Some(Instruction::XOR(ArithmeticByteTarget::B)),
            0xA9 => Some(Instruction::XOR(ArithmeticByteTarget::C)),
            0xAA => Some(Instruction::XOR(ArithmeticByteTarget::D)),
            0xAB => Some(Instruction::XOR(ArithmeticByteTarget::E)),
            0xAC => Some(Instruction::XOR(ArithmeticByteTarget::H)),
            0xAD => Some(Instruction::XOR(ArithmeticByteTarget::L)),
            0xAE => Some(Instruction::XOR(ArithmeticByteTarget::HLI)),
            0xAF => Some(Instruction::XOR(ArithmeticByteTarget::A)),
            0xB0 => Some(Instruction::OR(ArithmeticByteTarget::B)),
            0xB1 => Some(Instruction::OR(ArithmeticByteTarget::C)),
            0xB2 => Some(Instruction::OR(ArithmeticByteTarget::D)),
            0xB3 => Some(Instruction::OR(ArithmeticByteTarget::E)),
            0xB4 => Some(Instruction::OR(ArithmeticByteTarget::H)),
            0xB5 => Some(Instruction::OR(ArithmeticByteTarget::L)),
            0xB6 => Some(Instruction::OR(ArithmeticByteTarget::HLI)),
            0xB7 => Some(Instruction::OR(ArithmeticByteTarget::A)),
			0xC0 => Some(Instruction::RET(JumpTest::NotZero)),
            0xE0 => Some(Instruction::LD(LoadType::IndirectFromA(LdByteAddress::A8))),
			0xE2 => Some(Instruction::LD(LoadType::IndirectFromA(LdByteAddress::C))),
			0xF0 => Some(Instruction::LD(LoadType::AFromIndirect(LdByteAddress::A8))),
			0xF2 => Some(Instruction::LD(LoadType::AFromIndirect(LdByteAddress::C))),
            _ => None,
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x7C => Some(Instruction::BIT(BitPosition::Seven, BitRegister::H)),
            _ =>
            /* TODO: Add mapping for rest of instructions */
            {
                None
            }
        }
    }
}

#[derive(Debug)]
pub enum ArithmeticByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

#[derive(Debug)]
pub enum ArithmeticWordTarget {
    BC,
    DE,
    HL,
    SP,
}


#[cfg(test)]
mod tests {
    use super::Instruction;

	#[test]
	fn test_all_decodes(){
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

		for opcode in 0..0xFF {
			if !ndef.contains(&opcode){
				assert!(!Instruction::from_byte(opcode, false).is_none(), "failed non_prefixed opcode: 0x{:x}", opcode);
			}
			assert!(!Instruction::from_byte(opcode, true).is_none(), "failed prefixed opcode: 0x{:x}", opcode);
		}
	}
}