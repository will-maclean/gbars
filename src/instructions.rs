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
	ADC(ArithmeticByteTarget),
    ADD(ArithmeticTargetType),
    AND(ArithmeticByteTarget),

    // Bit
    BIT(BitPosition, BitRegister),

	// Call
	CALL(JumpTest),

	// Decrement
	DEC(ArithmeticTargetType),

	// Halt
	HALT,

	// Increment
	INC(ArithmeticTargetType),

	// Jump to a particular address dependent on one of the following
	// conditions: the zero flag is true, the zero flag is flase, the
	// carry flag is true, the carry flag is false, or always jump.
	JP(JumpTest),
	
	JR(JumpTest),
	
    // Load
    LD(LoadType),

    // No-op
    NOP,

    OR(ArithmeticByteTarget),
	
    // Pop
    POP(StackTarget),
	
    // Push
    PUSH(StackTarget),
	
    // Return
    RET(JumpTest),
	
	// Rotate left and copy to carry
	// Rotate the contents of register ArithmeticByteTarget to the left. That is, the contents of bit 
	// 0 are copied to bit 1, and the previous contents of bit 1 (before the copy 
	// operation) are copied to bit 2. The same operation is repeated in sequence 
	// for the rest of the register. The contents of bit 7 are placed in both the 
	// CY flag and bit 0 of register ArithmeticByteTarget.
	RLC(ArithmeticByteTarget),
	RRC(ArithmeticByteTarget),
	
    // Set Carry flag
    SCF,

    SUB(ArithmeticByteTarget),

	SWAP(ArithmeticByteTarget),

    XOR(ArithmeticByteTarget),
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
			// 0x02 => Some(Instruction::LD(LoadType::IndirectFromA(Ld))),
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
			0x07 => Some(Instruction::RLC(ArithmeticByteTarget::A)),
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
            0x3E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::D8))),
            0x40 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B,LoadByteSource::B))),
            0x41 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B,LoadByteSource::C))),
            0x42 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B,LoadByteSource::D))),
            0x43 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B,LoadByteSource::E))),
            0x44 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B,LoadByteSource::H))),
            0x45 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B,LoadByteSource::L))),
            0x46 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B,LoadByteSource::HLI))),
            0x47 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B,LoadByteSource::A))),
			0x48 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C,LoadByteSource::B))),
            0x49 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C,LoadByteSource::C))),
            0x4A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C,LoadByteSource::D))),
            0x4B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C,LoadByteSource::E))),
            0x4C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C,LoadByteSource::H))),
            0x4D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C,LoadByteSource::L))),
            0x4E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C,LoadByteSource::HLI))),
            0x4F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C,LoadByteSource::A))),
			0x50 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::B))),
            0x51 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::C))),
            0x52 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::D))),
            0x53 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::E))),
            0x54 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::H))),
            0x55 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::L))),
            0x56 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::HLI))),
            0x57 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D,LoadByteSource::A))),
			0x58 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::B))),
            0x59 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::C))),
            0x5A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::D))),
            0x5B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::E))),
            0x5C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::H))),
            0x5D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::L))),
            0x5E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::HLI))),
            0x5F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E,LoadByteSource::A))),
			0x60 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::B))),
            0x61 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::C))),
            0x62 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::D))),
            0x63 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::E))),
            0x64 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::H))),
            0x65 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::L))),
            0x66 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::HLI))),
            0x67 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H,LoadByteSource::A))),
			0x68 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::B))),
            0x69 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::C))),
            0x6A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::D))),
            0x6B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::E))),
            0x6C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::H))),
            0x6D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::L))),
            0x6E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::HLI))),
            0x6F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L,LoadByteSource::A))),
			0x70 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI,LoadByteSource::B))),
            0x71 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI,LoadByteSource::C))),
            0x72 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI,LoadByteSource::D))),
            0x73 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI,LoadByteSource::E))),
            0x74 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI,LoadByteSource::H))),
            0x75 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI,LoadByteSource::L))),
			0x76 => Some(Instruction::HALT),
            0x77 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI,LoadByteSource::A))),
			0x78 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::B))),
            0x79 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::C))),
            0x7A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::D))),
            0x7B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::E))),
            0x7C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::H))),
            0x7D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::L))),
            0x7E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::HLI))),
            0x7F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::A))),
            0x80 => Some(Instruction::ADD(ArithmeticTargetType::Byte(ArithmeticByteTarget::B))),
            0x81 => Some(Instruction::ADD(ArithmeticTargetType::Byte(ArithmeticByteTarget::C))),
            0x82 => Some(Instruction::ADD(ArithmeticTargetType::Byte(ArithmeticByteTarget::D))),
            0x83 => Some(Instruction::ADD(ArithmeticTargetType::Byte(ArithmeticByteTarget::E))),
            0x84 => Some(Instruction::ADD(ArithmeticTargetType::Byte(ArithmeticByteTarget::H))),
            0x85 => Some(Instruction::ADD(ArithmeticTargetType::Byte(ArithmeticByteTarget::L))),
            0x86 => Some(Instruction::ADD(ArithmeticTargetType::Byte(ArithmeticByteTarget::HLI))),
            0x87 => Some(Instruction::ADD(ArithmeticTargetType::Byte(ArithmeticByteTarget::A))),
			0x88 => Some(Instruction::ADC(ArithmeticByteTarget::B)),
			0x89 => Some(Instruction::ADC(ArithmeticByteTarget::C)),
			0x8A => Some(Instruction::ADC(ArithmeticByteTarget::D)),
			0x8B => Some(Instruction::ADC(ArithmeticByteTarget::E)),
			0x8C => Some(Instruction::ADC(ArithmeticByteTarget::H)),
			0x8D => Some(Instruction::ADC(ArithmeticByteTarget::L)),
			0x8E => Some(Instruction::ADC(ArithmeticByteTarget::HLI)),
			0x8F => Some(Instruction::ADC(ArithmeticByteTarget::A)),
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
			0x00 => Some(Instruction::RLC(ArithmeticByteTarget::B)),
			0x01 => Some(Instruction::RLC(ArithmeticByteTarget::C)),
			0x02 => Some(Instruction::RLC(ArithmeticByteTarget::D)),
			0x03 => Some(Instruction::RLC(ArithmeticByteTarget::E)),
			0x04 => Some(Instruction::RLC(ArithmeticByteTarget::H)),
			0x05 => Some(Instruction::RLC(ArithmeticByteTarget::L)),
			0x06 => Some(Instruction::RLC(ArithmeticByteTarget::HLI)),
			0x07 => Some(Instruction::RLC(ArithmeticByteTarget::A)),
			0x08 => Some(Instruction::RLC(ArithmeticByteTarget::B)),
			0x09 => Some(Instruction::RRC(ArithmeticByteTarget::C)),
			0x0A => Some(Instruction::RRC(ArithmeticByteTarget::D)),
			0x0B => Some(Instruction::RRC(ArithmeticByteTarget::E)),
			0x0C => Some(Instruction::RRC(ArithmeticByteTarget::H)),
			0x0D => Some(Instruction::RRC(ArithmeticByteTarget::L)),
			0x0E => Some(Instruction::RRC(ArithmeticByteTarget::HLI)),
			0x0F => Some(Instruction::RRC(ArithmeticByteTarget::A)),
            0x7C => Some(Instruction::BIT(BitPosition::Seven, BitRegister::H)),
			0x30 => Some(Instruction::SWAP(ArithmeticByteTarget::B)),
			0x31 => Some(Instruction::SWAP(ArithmeticByteTarget::C)),
			0x32 => Some(Instruction::SWAP(ArithmeticByteTarget::D)),
			0x33 => Some(Instruction::SWAP(ArithmeticByteTarget::E)),
			0x34 => Some(Instruction::SWAP(ArithmeticByteTarget::H)),
			0x35 => Some(Instruction::SWAP(ArithmeticByteTarget::L)),
			0x36 => Some(Instruction::SWAP(ArithmeticByteTarget::HLI)),
			0x37 => Some(Instruction::SWAP(ArithmeticByteTarget::A)),
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
    use test_case::test_matrix;

    use super::Instruction;

	#[test_matrix(
        0x00_u8..=0xFF_u8,
        [true, false]
    )]
	fn test_all_decodes(opcode: u8, prefixed: bool){
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

		if prefixed {
			assert!(!Instruction::from_byte(opcode, true).is_none(), "failed prefixed opcode: 0x{:x}", opcode);
		} else {
			if !ndef.contains(&opcode){
				assert!(!Instruction::from_byte(opcode, false).is_none(), "failed non_prefixed opcode: 0x{:x}", opcode);
			}
		}
	}
}