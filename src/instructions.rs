#[derive(Debug, Copy, Clone)]
pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum LoadWordTarget {
    BC,
	DE,
    HL,
    SP,
}

#[derive(Debug, Copy, Clone)]
pub enum LoadWordSource {
    BC,
    D16,
}

#[derive(Debug, Clone, Copy)]
pub enum LdByteAddress{
	C, A8
}

#[derive(Debug, Clone, Copy)]
pub enum LdIndirectAddr {
	BC,
	DE,
	A16,
}

#[derive(Debug, Copy, Clone)]
pub enum LoadType {
    // Load 8 bits from LoadByteSource into LoadByteTarget
    Byte(LoadByteTarget, LoadByteSource),

    // Load 16 bits from LoadWordSource into LoadWordTarget
    Word(LoadWordTarget, LoadWordSource),

    // load the A register with the contents from a value from a memory location whose address is stored in some location
    AFromIndirect(LdIndirectAddr),

    // load a memory location whose address is stored in some location with the contents of the A register
    IndirectFromA(LdIndirectAddr),

    // Just like AFromIndirect except the memory address is some address in the very last byte of memory
    AFromByteAddress(LdByteAddress),

    // // Just like IndirectFromA except the memory address is some address in the very last byte of memory
    ByteAddressFromA(LdByteAddress),

    // Store the contents of register A into the memory location specified by register pair HL, and simultaneously increment the contents of HL.
    AIntoHLInc,

    // Store the contents of register A into the memory location specified by register pair HL, and simultaneously decrement the contents of HL.
    AIntoHLDec,
}

#[derive(Debug, Copy, Clone)]
pub enum StackTarget {
    BC,
    DE,
    HL,
	AF,
}

#[derive(Debug, Copy, Clone)]
pub enum ArithmeticTargetType {
    Byte(ArithmeticByteTarget),
    Word(ArithmeticWordTarget),
}

#[derive(Debug, Copy, Clone)]
pub enum AddTargetType {
    Byte(AddByteTarget),
    Word(ArithmeticWordTarget),
	SPS8,
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum BitRegister {
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
	A
}

#[derive(Debug, Copy, Clone)]
pub enum JpAddrLoc {
	A16,
	HL,
}

#[derive(Debug, Copy, Clone)]
pub enum AdcTargetType {
	A,
	B,
	C,
	D,
	E,
	H,
	L,
	HLI,
	D8,
}

#[derive(Debug)]
pub enum Instruction {
	ADC(AdcTargetType),
    ADD(AddTargetType),
    AND(ArithmeticByteTarget),

    // Bit
    BIT(BitPosition, BitRegister),

	// Call
	CALL(JumpTest),

	// Flip carry flag
	CCF,

	// Compare
	CP(CPByteTarget),

	// ones complement of A
	CPL,

	// Adjust the accumulator (register A) too a binary-coded 
	// decimal (BCD) number after BCD addition and subtraction operations.
	DAA,

	// Decrement
	DEC(ArithmeticTargetType),

	// Set the interrupt master enable (IME) flag and enable maskable interrupts. 
	// This instruction can be used in an interrupt routine to enable higher-order interrupts.
	// The IME flag is reset immediately after an interrupt occurs. The IME flag 
	// reset remains in effect if coontrol is returned from the interrupt routine by 
	// a RET instruction. However, if an EI instruction is executed in the interrupt 
	// routine, control is returned with IME = 1.
	EI,


	// Halt
	HALT,

	// Increment
	INC(ArithmeticTargetType),

	// Jump to a particular address dependent on one of the following
	// conditions: the zero flag is true, the zero flag is flase, the
	// carry flag is true, the carry flag is false, or always jump.
	JP(JumpTest, JpAddrLoc),
	
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

	// Set a bit in a register to 0
	RES(BitPosition, BitRegister),
	
    // Return
    RET(JumpTest),
	
	// Rotate left
	// Rotate the contents of register ArithmeticByteTarget to the left. That is, the contents 
	// of bit 0 are copied to bit 1, and the previous contents of bit 1 (before the copy 
	// operation) are copied to bit 2. The same operation is repeated in sequence for the rest 
	// of the register. The previous contents of the carry (CY) flag are copied to bit 0 of
	// register ArithmeticByteTarget.
	RL(ArithmeticByteTarget),
	RLA,
	RR(ArithmeticByteTarget),
	RRA,

	// Rotate left and copy to carry
	// Rotate the contents of register ArithmeticByteTarget to the left. That is, the contents of bit 
	// 0 are copied to bit 1, and the previous contents of bit 1 (before the copy 
	// operation) are copied to bit 2. The same operation is repeated in sequence 
	// for the rest of the register. The contents of bit 7 are placed in both the 
	// CY flag and bit 0 of register ArithmeticByteTarget.
	RLC(ArithmeticByteTarget),
	RRC(ArithmeticByteTarget),

	RST(BitPosition),
	
	// Subtract byte and carry
	SBC(ArithmeticByteTarget),
    // Set Carry flag
    SCF,

	// Set a bit in a register to 1
	SET(BitPosition, BitRegister),

	SLA(ArithmeticByteTarget),
	SRA(ArithmeticByteTarget),

	SRL(ArithmeticByteTarget),

	STOP,

    SUB(SubByteTarget),

	SWAP(ArithmeticByteTarget),

    XOR(ArithmeticByteTarget),
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
            0x01 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::BC,LoadWordSource::D16))),
			0x02 => Some(Instruction::LD(LoadType::IndirectFromA(LdIndirectAddr::BC))),
            0x03 => Some(Instruction::INC(ArithmeticTargetType::Word(ArithmeticWordTarget::BC))),
            0x04 => Some(Instruction::INC(ArithmeticTargetType::Byte(ArithmeticByteTarget::B))),
            0x05 => Some(Instruction::DEC(ArithmeticTargetType::Byte(ArithmeticByteTarget::B))),
            0x06 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B,LoadByteSource::D8))),
			0x07 => Some(Instruction::RLC(ArithmeticByteTarget::A)),
			0x08 => Some(Instruction::DEC(ArithmeticTargetType::Word(ArithmeticWordTarget::BC))),
			0x09 => Some(Instruction::ADD(AddTargetType::Word(ArithmeticWordTarget::BC))),
			0x0A => Some(Instruction::LD(LoadType::AFromIndirect(LdIndirectAddr::BC))),
			0x0B => Some(Instruction::DEC(ArithmeticTargetType::Word(ArithmeticWordTarget::BC))),
			0x0C => Some(Instruction::INC(ArithmeticTargetType::Byte(ArithmeticByteTarget::C))),
			0x0D => Some(Instruction::DEC(ArithmeticTargetType::Byte(ArithmeticByteTarget::C))),
			0x0E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D8))),
			0x0F => Some(Instruction::RRC(ArithmeticByteTarget::A)),
			0x10 => Some(Instruction::STOP),
			0x11 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::DE, LoadWordSource::D16))),
			0x12 => Some(Instruction::LD(LoadType::IndirectFromA(LdIndirectAddr::DE))),
			0x13 => Some(Instruction::INC(ArithmeticTargetType::Word(ArithmeticWordTarget::DE))),
			0x14 => Some(Instruction::INC(ArithmeticTargetType::Byte(ArithmeticByteTarget::D))),
			0x15 => Some(Instruction::DEC(ArithmeticTargetType::Byte(ArithmeticByteTarget::D))),
			0x16 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D8))),
			0x17 => Some(Instruction::RLA),
			0x18 => Some(Instruction::JR(JumpTest::Always)),
			0x19 => Some(Instruction::ADD(AddTargetType::Word(ArithmeticWordTarget::DE))),
			0x1A => Some(Instruction::LD(LoadType::AFromIndirect(LdIndirectAddr::DE))),
			0x1B => Some(Instruction::DEC(ArithmeticTargetType::Word(ArithmeticWordTarget::DE))),
			0x1C => Some(Instruction::INC(ArithmeticTargetType::Byte(ArithmeticByteTarget::C))),
			0x1D => Some(Instruction::DEC(ArithmeticTargetType::Byte(ArithmeticByteTarget::E))),
			0x1E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D8))),
			0x1F => Some(Instruction::RRA),
			0x20 => Some(Instruction::JR(JumpTest::NotZero)),
            0x21 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::HL,LoadWordSource::D16))),
            0x22 => Some(Instruction::LD(LoadType::AIntoHLInc)),
			0x23 => Some(Instruction::INC(ArithmeticTargetType::Word(ArithmeticWordTarget::HL))),
			0x24 => Some(Instruction::INC(ArithmeticTargetType::Byte(ArithmeticByteTarget::H))),
			0x25 => Some(Instruction::DEC(ArithmeticTargetType::Byte(ArithmeticByteTarget::H))),
			0x26 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D8))),
			0x27 => Some(Instruction::DAA),
			0x28 => Some(Instruction::JR(JumpTest::Zero)),
			0x29 => Some(Instruction::ADD(AddTargetType::Word(ArithmeticWordTarget::HL))),
			0x2A => None,
			0x2B => Some(Instruction::DEC(ArithmeticTargetType::Word(ArithmeticWordTarget::HL))),
			0x2C => Some(Instruction::INC(ArithmeticTargetType::Byte(ArithmeticByteTarget::L))),
			0x2D => Some(Instruction::DEC(ArithmeticTargetType::Byte(ArithmeticByteTarget::L))),
			0x2E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D8))),
			0x2F => Some(Instruction::CPL),
			0x30 => Some(Instruction::JR(JumpTest::NotCarry)),
            0x31 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::SP,LoadWordSource::D16))),
            0x32 => Some(Instruction::LD(LoadType::AIntoHLDec)),
			0x33 => Some(Instruction::INC(ArithmeticTargetType::Word(ArithmeticWordTarget::SP))),
			0x34 => Some(Instruction::INC(ArithmeticTargetType::Byte(ArithmeticByteTarget::HLI))),
			0x35 => Some(Instruction::DEC(ArithmeticTargetType::Byte(ArithmeticByteTarget::HLI))),
			0x36 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::D8))),
            0x37 => Some(Instruction::SCF),
			0x38 => Some(Instruction::JR(JumpTest::Carry)),
			0x39 => Some(Instruction::ADD(AddTargetType::Word(ArithmeticWordTarget::SP))),
			0x3A => None,
			0x3B => Some(Instruction::DEC(ArithmeticTargetType::Word(ArithmeticWordTarget::SP))),
			0x3C => Some(Instruction::INC(ArithmeticTargetType::Byte(ArithmeticByteTarget::A))),
			0x3D => Some(Instruction::DEC(ArithmeticTargetType::Byte(ArithmeticByteTarget::A))),
            0x3E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A,LoadByteSource::D8))),
			0x3F => Some(Instruction::CCF),
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
            0x80 => Some(Instruction::ADD(AddTargetType::Byte(AddByteTarget::B))),
            0x81 => Some(Instruction::ADD(AddTargetType::Byte(AddByteTarget::C))),
            0x82 => Some(Instruction::ADD(AddTargetType::Byte(AddByteTarget::D))),
            0x83 => Some(Instruction::ADD(AddTargetType::Byte(AddByteTarget::E))),
            0x84 => Some(Instruction::ADD(AddTargetType::Byte(AddByteTarget::H))),
            0x85 => Some(Instruction::ADD(AddTargetType::Byte(AddByteTarget::L))),
            0x86 => Some(Instruction::ADD(AddTargetType::Byte(AddByteTarget::HLI))),
            0x87 => Some(Instruction::ADD(AddTargetType::Byte(AddByteTarget::A))),
			0x88 => Some(Instruction::ADC(AdcTargetType::B)),
			0x89 => Some(Instruction::ADC(AdcTargetType::C)),
			0x8A => Some(Instruction::ADC(AdcTargetType::D)),
			0x8B => Some(Instruction::ADC(AdcTargetType::E)),
			0x8C => Some(Instruction::ADC(AdcTargetType::H)),
			0x8D => Some(Instruction::ADC(AdcTargetType::L)),
			0x8E => Some(Instruction::ADC(AdcTargetType::HLI)),
			0x8F => Some(Instruction::ADC(AdcTargetType::A)),
            0x90 => Some(Instruction::SUB(SubByteTarget::B)),
            0x91 => Some(Instruction::SUB(SubByteTarget::C)),
            0x92 => Some(Instruction::SUB(SubByteTarget::D)),
            0x93 => Some(Instruction::SUB(SubByteTarget::E)),
            0x94 => Some(Instruction::SUB(SubByteTarget::H)),
            0x95 => Some(Instruction::SUB(SubByteTarget::L)),
            0x96 => Some(Instruction::SUB(SubByteTarget::HLI)),
            0x97 => Some(Instruction::SUB(SubByteTarget::A)),
			0x98 => Some(Instruction::SBC(ArithmeticByteTarget::B)),
            0x99 => Some(Instruction::SBC(ArithmeticByteTarget::C)),
            0x9A => Some(Instruction::SBC(ArithmeticByteTarget::D)),
            0x9B => Some(Instruction::SBC(ArithmeticByteTarget::E)),
            0x9C => Some(Instruction::SBC(ArithmeticByteTarget::H)),
            0x9D => Some(Instruction::SBC(ArithmeticByteTarget::L)),
            0x9E => Some(Instruction::SBC(ArithmeticByteTarget::HLI)),
            0x9F => Some(Instruction::SBC(ArithmeticByteTarget::A)),
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
			0xB8 => Some(Instruction::CP(CPByteTarget::B)),
            0xB9 => Some(Instruction::CP(CPByteTarget::C)),
            0xBA => Some(Instruction::CP(CPByteTarget::D)),
            0xBB => Some(Instruction::CP(CPByteTarget::E)),
            0xBC => Some(Instruction::CP(CPByteTarget::H)),
            0xBD => Some(Instruction::CP(CPByteTarget::L)),
            0xBE => Some(Instruction::CP(CPByteTarget::HLI)),
            0xBF => Some(Instruction::CP(CPByteTarget::A)),
			0xC0 => Some(Instruction::RET(JumpTest::NotZero)),
			0xC1 => Some(Instruction::POP(StackTarget::BC)),
			0xC2 => Some(Instruction::JP(JumpTest::NotZero, JpAddrLoc::A16)),
			0xC3 => Some(Instruction::JP(JumpTest::Always, JpAddrLoc::A16)),
			0xC4 => Some(Instruction::CALL(JumpTest::NotZero)),
			0xC5 => Some(Instruction::PUSH(StackTarget::BC)),
			0xC6 => Some(Instruction::ADD(AddTargetType::Byte(AddByteTarget::D8))),
			0xC7 => Some(Instruction::RST(BitPosition::Zero)),
			0xC8 => Some(Instruction::RET(JumpTest::Zero)),
			0xC9 => Some(Instruction::RET(JumpTest::Always)),
			0xCA => Some(Instruction::JP(JumpTest::Zero, JpAddrLoc::A16)),
			0xCB => None,
			0xCC => Some(Instruction::CALL(JumpTest::Zero)),
			0xCD => Some(Instruction::CALL(JumpTest::Always)),
			0xCE => Some(Instruction::ADC(AdcTargetType::D8)),
			0xCF => Some(Instruction::RST(BitPosition::One)),
			0xD0 => Some(Instruction::RET(JumpTest::NotCarry)),
			0xD1 => Some(Instruction::POP(StackTarget::DE)),
			0xD2 => Some(Instruction::JP(JumpTest::NotCarry, JpAddrLoc::A16)),
			0xD3 => None,
			0xD4 => Some(Instruction::CALL(JumpTest::NotCarry)),
			0xD5 => Some(Instruction::PUSH(StackTarget::DE)),
			0xD6 => Some(Instruction::SUB(SubByteTarget::D8)),
			0xD7 => Some(Instruction::RST(BitPosition::Two)),
			0xD8 => Some(Instruction::RET(JumpTest::Carry)),
			0xD9 => None,
			0xDA => Some(Instruction::JP(JumpTest::Carry, JpAddrLoc::A16)),
			0xDB => None,
			0xDC => Some(Instruction::CALL(JumpTest::Carry)),
			0xDD => None,
			0xDE => None,
			0xDF => Some(Instruction::RST(BitPosition::Three)),
            0xE0 => Some(Instruction::LD(LoadType::ByteAddressFromA(LdByteAddress::A8))),
			0xE1 => Some(Instruction::POP(StackTarget::HL)),
			0xE2 => Some(Instruction::LD(LoadType::ByteAddressFromA(LdByteAddress::C))),
			0xE3 => None,
			0xE4 => None,
			0xE5 => Some(Instruction::PUSH(StackTarget::HL)),
			0xE6 => None,
			0xE7 => Some(Instruction::RST(BitPosition::Four)),
			0xE8 => Some(Instruction::ADD(AddTargetType::SPS8)),
			0xE9 => Some(Instruction::JP(JumpTest::Always, JpAddrLoc::HL)),
			0xEA => Some(Instruction::LD(LoadType::IndirectFromA(LdIndirectAddr::A16))),
			0xEB => None,
			0xEC => None,
			0xED => None,
			0xEE => None,
			0xEF => Some(Instruction::RST(BitPosition::Five)),
			0xF0 => Some(Instruction::LD(LoadType::AFromByteAddress(LdByteAddress::A8))),
			0xF1 => Some(Instruction::POP(StackTarget::AF)),
			0xF2 => Some(Instruction::LD(LoadType::AFromByteAddress(LdByteAddress::C))),
			0xF3 => None,
			0xF4 => None,
			0xF5 => Some(Instruction::PUSH(StackTarget::AF)),
			0xF6 => None,
			0xF7 => Some(Instruction::RST(BitPosition::Six)),
			0xF8 => None,
			0xF9 => None,
			0xFA => Some(Instruction::LD(LoadType::AFromIndirect(LdIndirectAddr::A16))),
			0xFB => Some(Instruction::EI),
			0xFC => None,
			0xFD => None,
			0xFE => Some(Instruction::CP(CPByteTarget::D8)),
			0xFF => Some(Instruction::RST(BitPosition::Seven)),
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
			0x10 => Some(Instruction::RL(ArithmeticByteTarget::B)),
			0x11 => Some(Instruction::RL(ArithmeticByteTarget::C)),
			0x12 => Some(Instruction::RL(ArithmeticByteTarget::D)),
			0x13 => Some(Instruction::RL(ArithmeticByteTarget::E)),
			0x14 => Some(Instruction::RL(ArithmeticByteTarget::H)),
			0x15 => Some(Instruction::RL(ArithmeticByteTarget::L)),
			0x16 => Some(Instruction::RL(ArithmeticByteTarget::HLI)),
			0x17 => Some(Instruction::RL(ArithmeticByteTarget::A)),
			0x18 => Some(Instruction::RL(ArithmeticByteTarget::B)),
			0x19 => Some(Instruction::RR(ArithmeticByteTarget::C)),
			0x1A => Some(Instruction::RR(ArithmeticByteTarget::D)),
			0x1B => Some(Instruction::RR(ArithmeticByteTarget::E)),
			0x1C => Some(Instruction::RR(ArithmeticByteTarget::H)),
			0x1D => Some(Instruction::RR(ArithmeticByteTarget::L)),
			0x1E => Some(Instruction::RR(ArithmeticByteTarget::HLI)),
			0x1F => Some(Instruction::RR(ArithmeticByteTarget::A)),
			0x20 => Some(Instruction::SLA(ArithmeticByteTarget::B)),
			0x21 => Some(Instruction::SLA(ArithmeticByteTarget::C)),
			0x22 => Some(Instruction::SLA(ArithmeticByteTarget::D)),
			0x23 => Some(Instruction::SLA(ArithmeticByteTarget::E)),
			0x24 => Some(Instruction::SLA(ArithmeticByteTarget::H)),
			0x25 => Some(Instruction::SLA(ArithmeticByteTarget::L)),
			0x26 => Some(Instruction::SLA(ArithmeticByteTarget::HLI)),
			0x27 => Some(Instruction::SLA(ArithmeticByteTarget::A)),
			0x28 => Some(Instruction::SRA(ArithmeticByteTarget::B)),
			0x29 => Some(Instruction::SRA(ArithmeticByteTarget::C)),
			0x2A => Some(Instruction::SRA(ArithmeticByteTarget::D)),
			0x2B => Some(Instruction::SRA(ArithmeticByteTarget::E)),
			0x2C => Some(Instruction::SRA(ArithmeticByteTarget::H)),
			0x2D => Some(Instruction::SRA(ArithmeticByteTarget::L)),
			0x2E => Some(Instruction::SRA(ArithmeticByteTarget::HLI)),
			0x2F => Some(Instruction::SRA(ArithmeticByteTarget::A)),
			0x30 => Some(Instruction::SWAP(ArithmeticByteTarget::B)),
			0x31 => Some(Instruction::SWAP(ArithmeticByteTarget::C)),
			0x32 => Some(Instruction::SWAP(ArithmeticByteTarget::D)),
			0x33 => Some(Instruction::SWAP(ArithmeticByteTarget::E)),
			0x34 => Some(Instruction::SWAP(ArithmeticByteTarget::H)),
			0x35 => Some(Instruction::SWAP(ArithmeticByteTarget::L)),
			0x36 => Some(Instruction::SWAP(ArithmeticByteTarget::HLI)),
			0x37 => Some(Instruction::SWAP(ArithmeticByteTarget::A)),
			0x38 => Some(Instruction::SRL(ArithmeticByteTarget::B)),
			0x39 => Some(Instruction::SRL(ArithmeticByteTarget::C)),
			0x3A => Some(Instruction::SRL(ArithmeticByteTarget::D)),
			0x3B => Some(Instruction::SRL(ArithmeticByteTarget::E)),
			0x3C => Some(Instruction::SRL(ArithmeticByteTarget::H)),
			0x3D => Some(Instruction::SRL(ArithmeticByteTarget::L)),
			0x3E => Some(Instruction::SRL(ArithmeticByteTarget::HLI)),
			0x3F => Some(Instruction::SRL(ArithmeticByteTarget::A)),
			0x40 => Some(Instruction::BIT(BitPosition::Zero, BitRegister::B)),
			0x41 => Some(Instruction::BIT(BitPosition::Zero, BitRegister::C)),
			0x42 => Some(Instruction::BIT(BitPosition::Zero, BitRegister::D)),
			0x43 => Some(Instruction::BIT(BitPosition::Zero, BitRegister::E)),
			0x44 => Some(Instruction::BIT(BitPosition::Zero, BitRegister::H)),
			0x45 => Some(Instruction::BIT(BitPosition::Zero, BitRegister::L)),
			0x46 => Some(Instruction::BIT(BitPosition::Zero, BitRegister::HLI)),
			0x47 => Some(Instruction::BIT(BitPosition::Zero, BitRegister::A)),
			0x48 => Some(Instruction::BIT(BitPosition::One, BitRegister::B)),
			0x49 => Some(Instruction::BIT(BitPosition::One, BitRegister::C)),
			0x4A => Some(Instruction::BIT(BitPosition::One, BitRegister::D)),
			0x4B => Some(Instruction::BIT(BitPosition::One, BitRegister::E)),
			0x4C => Some(Instruction::BIT(BitPosition::One, BitRegister::H)),
			0x4D => Some(Instruction::BIT(BitPosition::One, BitRegister::L)),
			0x4E => Some(Instruction::BIT(BitPosition::One, BitRegister::HLI)),
			0x4F => Some(Instruction::BIT(BitPosition::One, BitRegister::A)),
            0x50 => Some(Instruction::BIT(BitPosition::Two, BitRegister::B)),
			0x51 => Some(Instruction::BIT(BitPosition::Two, BitRegister::C)),
			0x52 => Some(Instruction::BIT(BitPosition::Two, BitRegister::D)),
			0x53 => Some(Instruction::BIT(BitPosition::Two, BitRegister::E)),
			0x54 => Some(Instruction::BIT(BitPosition::Two, BitRegister::H)),
			0x55 => Some(Instruction::BIT(BitPosition::Two, BitRegister::L)),
			0x56 => Some(Instruction::BIT(BitPosition::Two, BitRegister::HLI)),
			0x57 => Some(Instruction::BIT(BitPosition::Two, BitRegister::A)),
			0x58 => Some(Instruction::BIT(BitPosition::Three, BitRegister::B)),
			0x59 => Some(Instruction::BIT(BitPosition::Three, BitRegister::C)),
			0x5A => Some(Instruction::BIT(BitPosition::Three, BitRegister::D)),
			0x5B => Some(Instruction::BIT(BitPosition::Three, BitRegister::E)),
			0x5C => Some(Instruction::BIT(BitPosition::Three, BitRegister::H)),
			0x5D => Some(Instruction::BIT(BitPosition::Three, BitRegister::L)),
			0x5E => Some(Instruction::BIT(BitPosition::Three, BitRegister::HLI)),
			0x5F => Some(Instruction::BIT(BitPosition::Three, BitRegister::A)),
			0x60 => Some(Instruction::BIT(BitPosition::Four, BitRegister::B)),
			0x61 => Some(Instruction::BIT(BitPosition::Four, BitRegister::C)),
			0x62 => Some(Instruction::BIT(BitPosition::Four, BitRegister::D)),
			0x63 => Some(Instruction::BIT(BitPosition::Four, BitRegister::E)),
			0x64 => Some(Instruction::BIT(BitPosition::Four, BitRegister::H)),
			0x65 => Some(Instruction::BIT(BitPosition::Four, BitRegister::L)),
			0x66 => Some(Instruction::BIT(BitPosition::Four, BitRegister::HLI)),
			0x67 => Some(Instruction::BIT(BitPosition::Four, BitRegister::A)),
			0x68 => Some(Instruction::BIT(BitPosition::Five, BitRegister::B)),
			0x69 => Some(Instruction::BIT(BitPosition::Five, BitRegister::C)),
			0x6A => Some(Instruction::BIT(BitPosition::Five, BitRegister::D)),
			0x6B => Some(Instruction::BIT(BitPosition::Five, BitRegister::E)),
			0x6C => Some(Instruction::BIT(BitPosition::Five, BitRegister::H)),
			0x6D => Some(Instruction::BIT(BitPosition::Five, BitRegister::L)),
			0x6E => Some(Instruction::BIT(BitPosition::Five, BitRegister::HLI)),
			0x6F => Some(Instruction::BIT(BitPosition::Five, BitRegister::A)),
			0x70 => Some(Instruction::BIT(BitPosition::Six, BitRegister::B)),
			0x71 => Some(Instruction::BIT(BitPosition::Six, BitRegister::C)),
			0x72 => Some(Instruction::BIT(BitPosition::Six, BitRegister::D)),
			0x73 => Some(Instruction::BIT(BitPosition::Six, BitRegister::E)),
			0x74 => Some(Instruction::BIT(BitPosition::Six, BitRegister::H)),
			0x75 => Some(Instruction::BIT(BitPosition::Six, BitRegister::L)),
			0x76 => Some(Instruction::BIT(BitPosition::Six, BitRegister::HLI)),
			0x77 => Some(Instruction::BIT(BitPosition::Six, BitRegister::A)),
			0x78 => Some(Instruction::BIT(BitPosition::Seven, BitRegister::B)),
			0x79 => Some(Instruction::BIT(BitPosition::Seven, BitRegister::C)),
			0x7A => Some(Instruction::BIT(BitPosition::Seven, BitRegister::D)),
			0x7B => Some(Instruction::BIT(BitPosition::Seven, BitRegister::E)),
			0x7C => Some(Instruction::BIT(BitPosition::Seven, BitRegister::H)),
			0x7D => Some(Instruction::BIT(BitPosition::Seven, BitRegister::L)),
			0x7E => Some(Instruction::BIT(BitPosition::Seven, BitRegister::HLI)),
			0x7F => Some(Instruction::BIT(BitPosition::Seven, BitRegister::A)),
			0x80 => Some(Instruction::RES(BitPosition::Zero, BitRegister::B)),
			0x81 => Some(Instruction::RES(BitPosition::Zero, BitRegister::C)),
			0x82 => Some(Instruction::RES(BitPosition::Zero, BitRegister::D)),
			0x83 => Some(Instruction::RES(BitPosition::Zero, BitRegister::E)),
			0x84 => Some(Instruction::RES(BitPosition::Zero, BitRegister::H)),
			0x85 => Some(Instruction::RES(BitPosition::Zero, BitRegister::L)),
			0x86 => Some(Instruction::RES(BitPosition::Zero, BitRegister::HLI)),
			0x87 => Some(Instruction::RES(BitPosition::Zero, BitRegister::A)),
			0x88 => Some(Instruction::RES(BitPosition::One, BitRegister::B)),
			0x89 => Some(Instruction::RES(BitPosition::One, BitRegister::C)),
			0x8A => Some(Instruction::RES(BitPosition::One, BitRegister::D)),
			0x8B => Some(Instruction::RES(BitPosition::One, BitRegister::E)),
			0x8C => Some(Instruction::RES(BitPosition::One, BitRegister::H)),
			0x8D => Some(Instruction::RES(BitPosition::One, BitRegister::L)),
			0x8E => Some(Instruction::RES(BitPosition::One, BitRegister::HLI)),
			0x8F => Some(Instruction::RES(BitPosition::One, BitRegister::A)),
            0x90 => Some(Instruction::RES(BitPosition::Two, BitRegister::B)),
			0x91 => Some(Instruction::RES(BitPosition::Two, BitRegister::C)),
			0x92 => Some(Instruction::RES(BitPosition::Two, BitRegister::D)),
			0x93 => Some(Instruction::RES(BitPosition::Two, BitRegister::E)),
			0x94 => Some(Instruction::RES(BitPosition::Two, BitRegister::H)),
			0x95 => Some(Instruction::RES(BitPosition::Two, BitRegister::L)),
			0x96 => Some(Instruction::RES(BitPosition::Two, BitRegister::HLI)),
			0x97 => Some(Instruction::RES(BitPosition::Two, BitRegister::A)),
			0x98 => Some(Instruction::RES(BitPosition::Three, BitRegister::B)),
			0x99 => Some(Instruction::RES(BitPosition::Three, BitRegister::C)),
			0x9A => Some(Instruction::RES(BitPosition::Three, BitRegister::D)),
			0x9B => Some(Instruction::RES(BitPosition::Three, BitRegister::E)),
			0x9C => Some(Instruction::RES(BitPosition::Three, BitRegister::H)),
			0x9D => Some(Instruction::RES(BitPosition::Three, BitRegister::L)),
			0x9E => Some(Instruction::RES(BitPosition::Three, BitRegister::HLI)),
			0x9F => Some(Instruction::RES(BitPosition::Three, BitRegister::A)),
			0xA0 => Some(Instruction::RES(BitPosition::Four, BitRegister::B)),
			0xA1 => Some(Instruction::RES(BitPosition::Four, BitRegister::C)),
			0xA2 => Some(Instruction::RES(BitPosition::Four, BitRegister::D)),
			0xA3 => Some(Instruction::RES(BitPosition::Four, BitRegister::E)),
			0xA4 => Some(Instruction::RES(BitPosition::Four, BitRegister::H)),
			0xA5 => Some(Instruction::RES(BitPosition::Four, BitRegister::L)),
			0xA6 => Some(Instruction::RES(BitPosition::Four, BitRegister::HLI)),
			0xA7 => Some(Instruction::RES(BitPosition::Four, BitRegister::A)),
			0xA8 => Some(Instruction::RES(BitPosition::Five, BitRegister::B)),
			0xA9 => Some(Instruction::RES(BitPosition::Five, BitRegister::C)),
			0xAA => Some(Instruction::RES(BitPosition::Five, BitRegister::D)),
			0xAB => Some(Instruction::RES(BitPosition::Five, BitRegister::E)),
			0xAC => Some(Instruction::RES(BitPosition::Five, BitRegister::H)),
			0xAD => Some(Instruction::RES(BitPosition::Five, BitRegister::L)),
			0xAE => Some(Instruction::RES(BitPosition::Five, BitRegister::HLI)),
			0xAF => Some(Instruction::RES(BitPosition::Five, BitRegister::A)),
			0xB0 => Some(Instruction::RES(BitPosition::Six, BitRegister::B)),
			0xB1 => Some(Instruction::RES(BitPosition::Six, BitRegister::C)),
			0xB2 => Some(Instruction::RES(BitPosition::Six, BitRegister::D)),
			0xB3 => Some(Instruction::RES(BitPosition::Six, BitRegister::E)),
			0xB4 => Some(Instruction::RES(BitPosition::Six, BitRegister::H)),
			0xB5 => Some(Instruction::RES(BitPosition::Six, BitRegister::L)),
			0xB6 => Some(Instruction::RES(BitPosition::Six, BitRegister::HLI)),
			0xB7 => Some(Instruction::RES(BitPosition::Six, BitRegister::A)),
			0xB8 => Some(Instruction::RES(BitPosition::Seven, BitRegister::B)),
			0xB9 => Some(Instruction::RES(BitPosition::Seven, BitRegister::C)),
			0xBA => Some(Instruction::RES(BitPosition::Seven, BitRegister::D)),
			0xBB => Some(Instruction::RES(BitPosition::Seven, BitRegister::E)),
			0xBC => Some(Instruction::RES(BitPosition::Seven, BitRegister::H)),
			0xBD => Some(Instruction::RES(BitPosition::Seven, BitRegister::L)),
			0xBE => Some(Instruction::RES(BitPosition::Seven, BitRegister::HLI)),
			0xBF => Some(Instruction::RES(BitPosition::Seven, BitRegister::A)),
			0xC0 => Some(Instruction::SET(BitPosition::Zero, BitRegister::B)),
			0xC1 => Some(Instruction::SET(BitPosition::Zero, BitRegister::C)),
			0xC2 => Some(Instruction::SET(BitPosition::Zero, BitRegister::D)),
			0xC3 => Some(Instruction::SET(BitPosition::Zero, BitRegister::E)),
			0xC4 => Some(Instruction::SET(BitPosition::Zero, BitRegister::H)),
			0xC5 => Some(Instruction::SET(BitPosition::Zero, BitRegister::L)),
			0xC6 => Some(Instruction::SET(BitPosition::Zero, BitRegister::HLI)),
			0xC7 => Some(Instruction::SET(BitPosition::Zero, BitRegister::A)),
			0xC8 => Some(Instruction::SET(BitPosition::One, BitRegister::B)),
			0xC9 => Some(Instruction::SET(BitPosition::One, BitRegister::C)),
			0xCA => Some(Instruction::SET(BitPosition::One, BitRegister::D)),
			0xCB => Some(Instruction::SET(BitPosition::One, BitRegister::E)),
			0xCC => Some(Instruction::SET(BitPosition::One, BitRegister::H)),
			0xCD => Some(Instruction::SET(BitPosition::One, BitRegister::L)),
			0xCE => Some(Instruction::SET(BitPosition::One, BitRegister::HLI)),
			0xCF => Some(Instruction::SET(BitPosition::One, BitRegister::A)),
            0xD0 => Some(Instruction::SET(BitPosition::Two, BitRegister::B)),
			0xD1 => Some(Instruction::SET(BitPosition::Two, BitRegister::C)),
			0xD2 => Some(Instruction::SET(BitPosition::Two, BitRegister::D)),
			0xD3 => Some(Instruction::SET(BitPosition::Two, BitRegister::E)),
			0xD4 => Some(Instruction::SET(BitPosition::Two, BitRegister::H)),
			0xD5 => Some(Instruction::SET(BitPosition::Two, BitRegister::L)),
			0xD6 => Some(Instruction::SET(BitPosition::Two, BitRegister::HLI)),
			0xD7 => Some(Instruction::SET(BitPosition::Two, BitRegister::A)),
			0xD8 => Some(Instruction::SET(BitPosition::Three, BitRegister::B)),
			0xD9 => Some(Instruction::SET(BitPosition::Three, BitRegister::C)),
			0xDA => Some(Instruction::SET(BitPosition::Three, BitRegister::D)),
			0xDB => Some(Instruction::SET(BitPosition::Three, BitRegister::E)),
			0xDC => Some(Instruction::SET(BitPosition::Three, BitRegister::H)),
			0xDD => Some(Instruction::SET(BitPosition::Three, BitRegister::L)),
			0xDE => Some(Instruction::SET(BitPosition::Three, BitRegister::HLI)),
			0xDF => Some(Instruction::SET(BitPosition::Three, BitRegister::A)),
			0xE0 => Some(Instruction::SET(BitPosition::Four, BitRegister::B)),
			0xE1 => Some(Instruction::SET(BitPosition::Four, BitRegister::C)),
			0xE2 => Some(Instruction::SET(BitPosition::Four, BitRegister::D)),
			0xE3 => Some(Instruction::SET(BitPosition::Four, BitRegister::E)),
			0xE4 => Some(Instruction::SET(BitPosition::Four, BitRegister::H)),
			0xE5 => Some(Instruction::SET(BitPosition::Four, BitRegister::L)),
			0xE6 => Some(Instruction::SET(BitPosition::Four, BitRegister::HLI)),
			0xE7 => Some(Instruction::SET(BitPosition::Four, BitRegister::A)),
			0xE8 => Some(Instruction::SET(BitPosition::Five, BitRegister::B)),
			0xE9 => Some(Instruction::SET(BitPosition::Five, BitRegister::C)),
			0xEA => Some(Instruction::SET(BitPosition::Five, BitRegister::D)),
			0xEB => Some(Instruction::SET(BitPosition::Five, BitRegister::E)),
			0xEC => Some(Instruction::SET(BitPosition::Five, BitRegister::H)),
			0xED => Some(Instruction::SET(BitPosition::Five, BitRegister::L)),
			0xEE => Some(Instruction::SET(BitPosition::Five, BitRegister::HLI)),
			0xEF => Some(Instruction::SET(BitPosition::Five, BitRegister::A)),
			0xF0 => Some(Instruction::SET(BitPosition::Six, BitRegister::B)),
			0xF1 => Some(Instruction::SET(BitPosition::Six, BitRegister::C)),
			0xF2 => Some(Instruction::SET(BitPosition::Six, BitRegister::D)),
			0xF3 => Some(Instruction::SET(BitPosition::Six, BitRegister::E)),
			0xF4 => Some(Instruction::SET(BitPosition::Six, BitRegister::H)),
			0xF5 => Some(Instruction::SET(BitPosition::Six, BitRegister::L)),
			0xF6 => Some(Instruction::SET(BitPosition::Six, BitRegister::HLI)),
			0xF7 => Some(Instruction::SET(BitPosition::Six, BitRegister::A)),
			0xF8 => Some(Instruction::SET(BitPosition::Seven, BitRegister::B)),
			0xF9 => Some(Instruction::SET(BitPosition::Seven, BitRegister::C)),
			0xFA => Some(Instruction::SET(BitPosition::Seven, BitRegister::D)),
			0xFB => Some(Instruction::SET(BitPosition::Seven, BitRegister::E)),
			0xFC => Some(Instruction::SET(BitPosition::Seven, BitRegister::H)),
			0xFD => Some(Instruction::SET(BitPosition::Seven, BitRegister::L)),
			0xFE => Some(Instruction::SET(BitPosition::Seven, BitRegister::HLI)),
			0xFF => Some(Instruction::SET(BitPosition::Seven, BitRegister::A)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum CPByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
	D8,
}

#[derive(Debug, Copy, Clone)]
pub enum AddByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
	D8,
}

#[derive(Debug, Copy, Clone)]
pub enum SubByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
	D8,
}

#[derive(Debug, Copy, Clone)]
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