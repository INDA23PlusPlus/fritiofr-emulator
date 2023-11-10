use super::addressing_mode::*;

/// ADC - Add with Carry
pub const ADC_IMMEDIATE: u8 = 0x69;
pub const ADC_ZERO_PAGE: u8 = 0x65;
pub const ADC_ZERO_PAGE_X: u8 = 0x75;
pub const ADC_ABSOLUTE: u8 = 0x6D;
pub const ADC_ABSOLUTE_X: u8 = 0x7D;
pub const ADC_ABSOLUTE_Y: u8 = 0x79;
pub const ADC_INDIRECT_X: u8 = 0x61;
pub const ADC_INDIRECT_Y: u8 = 0x71;

/// AND - Logical AND
pub const AND_IMMEDIATE: u8 = 0x29;
pub const AND_ZERO_PAGE: u8 = 0x25;
pub const AND_ZERO_PAGE_X: u8 = 0x35;
pub const AND_ABSOLUTE: u8 = 0x2D;
pub const AND_ABSOLUTE_X: u8 = 0x3D;
pub const AND_ABSOLUTE_Y: u8 = 0x39;
pub const AND_INDIRECT_X: u8 = 0x21;
pub const AND_INDIRECT_Y: u8 = 0x31;

/// ASL - Arithmetic Shift Left
pub const ASL_ACCUMULATOR: u8 = 0x0A;
pub const ASL_ZERO_PAGE: u8 = 0x06;
pub const ASL_ZERO_PAGE_X: u8 = 0x16;
pub const ASL_ABSOLUTE: u8 = 0x0E;
pub const ASL_ABSOLUTE_X: u8 = 0x1E;

/// BCC - Branch if Carry Clear
pub const BCC: u8 = 0x90;

/// BCS - Branch if Carry Set
pub const BCS: u8 = 0xB0;

/// BEQ - Branch if Equal
pub const BEQ: u8 = 0xF0;

/// BIT - Bit Test
pub const BIT_ZERO_PAGE: u8 = 0x24;
pub const BIT_ABSOLUTE: u8 = 0x2C;

/// BMI - Branch if Minus
pub const BMI: u8 = 0x30;

/// BNE - Branch if Not Equal
pub const BNE: u8 = 0xD0;

/// BPL - Branch if Positive
pub const BPL: u8 = 0x10;

/// BRK - Force Interrupt
pub const BRK: u8 = 0x00;

/// BVC - Branch if Overflow Clear
pub const BVC: u8 = 0x50;

/// BVS - Branch if Overflow Set
pub const BVS: u8 = 0x70;

/// CLC - Clear Carry Flag
pub const CLC: u8 = 0x18;

/// CLD - Clear Decimal Mode
pub const CLD: u8 = 0xD8;

/// CLI - Clear Interrupt Disable
pub const CLI: u8 = 0x58;

/// CLV - Clear Overflow Flag
pub const CLV: u8 = 0xB8;

/// CMP - Compare
pub const CMP_IMMEDIATE: u8 = 0xC9;
pub const CMP_ZERO_PAGE: u8 = 0xC5;
pub const CMP_ZERO_PAGE_X: u8 = 0xD5;
pub const CMP_ABSOLUTE: u8 = 0xCD;
pub const CMP_ABSOLUTE_X: u8 = 0xDD;
pub const CMP_ABSOLUTE_Y: u8 = 0xD9;
pub const CMP_INDIRECT_X: u8 = 0xC1;
pub const CMP_INDIRECT_Y: u8 = 0xD1;

/// CPX - Compare X Register
pub const CPX_IMMEDIATE: u8 = 0xE0;
pub const CPX_ZERO_PAGE: u8 = 0xE4;
pub const CPX_ABSOLUTE: u8 = 0xEC;

/// CPY - Compare Y Register
pub const CPY_IMMEDIATE: u8 = 0xC0;
pub const CPY_ZERO_PAGE: u8 = 0xC4;
pub const CPY_ABSOLUTE: u8 = 0xCC;

/// DEC - Decrement Memory
pub const DEC_ZERO_PAGE: u8 = 0xC6;
pub const DEC_ZERO_PAGE_X: u8 = 0xD6;
pub const DEC_ABSOLUTE: u8 = 0xCE;
pub const DEC_ABSOLUTE_X: u8 = 0xDE;

/// DEX - Decrement X Register
pub const DEX: u8 = 0xCA;

/// DEY - Decrement Y Register
pub const DEY: u8 = 0x88;

/// EOR - Exclusive OR
pub const EOR_IMMEDIATE: u8 = 0x49;
pub const EOR_ZERO_PAGE: u8 = 0x45;
pub const EOR_ZERO_PAGE_X: u8 = 0x55;
pub const EOR_ABSOLUTE: u8 = 0x4D;
pub const EOR_ABSOLUTE_X: u8 = 0x5D;
pub const EOR_ABSOLUTE_Y: u8 = 0x59;
pub const EOR_INDIRECT_X: u8 = 0x41;
pub const EOR_INDIRECT_Y: u8 = 0x51;

/// INC - Increment Memory
pub const INC_ZERO_PAGE: u8 = 0xE6;
pub const INC_ZERO_PAGE_X: u8 = 0xF6;
pub const INC_ABSOLUTE: u8 = 0xEE;
pub const INC_ABSOLUTE_X: u8 = 0xFE;

/// INX - Increment X
pub const INX: u8 = 0xE8;

/// INY - Increment Y
pub const INY: u8 = 0xC8;

/// JMP - Jump
pub const JMP_ABSOLUTE: u8 = 0x4C;
pub const JMP_INDIRECT: u8 = 0x6C;

/// JSR - Jump to Subroutine
pub const JSR: u8 = 0x20;

/// LDA - Load Accumulator
pub const LDA_IMMEDIATE: u8 = 0xA9;
pub const LDA_ZERO_PAGE: u8 = 0xA5;
pub const LDA_ZERO_PAGE_X: u8 = 0xB5;
pub const LDA_ABSOLUTE: u8 = 0xAD;
pub const LDA_ABSOLUTE_X: u8 = 0xBD;
pub const LDA_ABSOLUTE_Y: u8 = 0xB9;
pub const LDA_INDIRECT_X: u8 = 0xA1;
pub const LDA_INDIRECT_Y: u8 = 0xB1;

/// LDX - Load X Register
pub const LDX_IMMEDIATE: u8 = 0xA2;
pub const LDX_ZERO_PAGE: u8 = 0xA6;
pub const LDX_ZERO_PAGE_Y: u8 = 0xB6;
pub const LDX_ABSOLUTE: u8 = 0xAE;
pub const LDX_ABSOLUTE_Y: u8 = 0xBE;

/// LDY - Load Y Register
pub const LDY_IMMEDIATE: u8 = 0xA0;
pub const LDY_ZERO_PAGE: u8 = 0xA4;
pub const LDY_ZERO_PAGE_X: u8 = 0xB4;
pub const LDY_ABSOLUTE: u8 = 0xAC;
pub const LDY_ABSOLUTE_X: u8 = 0xBC;

/// LSR - Logical Shift Right
pub const LSR_ACCUMULATOR: u8 = 0x4A;
pub const LSR_ZERO_PAGE: u8 = 0x46;
pub const LSR_ZERO_PAGE_X: u8 = 0x56;
pub const LSR_ABSOLUTE: u8 = 0x4E;
pub const LSR_ABSOLUTE_X: u8 = 0x5E;

/// NOP - No Operation
pub const NOP: u8 = 0xEA;

/// ORA - Logical Inclusive OR
pub const ORA_IMMEDIATE: u8 = 0x09;
pub const ORA_ZERO_PAGE: u8 = 0x05;
pub const ORA_ZERO_PAGE_X: u8 = 0x15;
pub const ORA_ABSOLUTE: u8 = 0x0D;
pub const ORA_ABSOLUTE_X: u8 = 0x1D;
pub const ORA_ABSOLUTE_Y: u8 = 0x19;
pub const ORA_INDIRECT_X: u8 = 0x01;
pub const ORA_INDIRECT_Y: u8 = 0x11;

/// PHA - Push Accumulator
pub const PHA: u8 = 0x48;

/// PHP - Push Processor Status
pub const PHP: u8 = 0x08;

/// PLA - Pull Accumulator
pub const PLA: u8 = 0x68;

/// PLP - Pull Processor Status
pub const PLP: u8 = 0x28;

/// ROL - Rotate Left
pub const ROL_ACCUMULATOR: u8 = 0x2A;
pub const ROL_ZERO_PAGE: u8 = 0x26;
pub const ROL_ZERO_PAGE_X: u8 = 0x36;
pub const ROL_ABSOLUTE: u8 = 0x2E;
pub const ROL_ABSOLUTE_X: u8 = 0x3E;

/// ROR - Rotate Right
pub const ROR_ACCUMULATOR: u8 = 0x6A;
pub const ROR_ZERO_PAGE: u8 = 0x66;
pub const ROR_ZERO_PAGE_X: u8 = 0x76;
pub const ROR_ABSOLUTE: u8 = 0x6E;
pub const ROR_ABSOLUTE_X: u8 = 0x7E;

/// RTI - Return from Interrupt
pub const RTI: u8 = 0x40;

/// RTS - Return from Subroutine
pub const RTS: u8 = 0x60;

/// SBC - Subtract with Carry
pub const SBC_IMMEDIATE: u8 = 0xE9;
pub const SBC_ZERO_PAGE: u8 = 0xE5;
pub const SBC_ZERO_PAGE_X: u8 = 0xF5;
pub const SBC_ABSOLUTE: u8 = 0xED;
pub const SBC_ABSOLUTE_X: u8 = 0xFD;
pub const SBC_ABSOLUTE_Y: u8 = 0xF9;
pub const SBC_INDIRECT_X: u8 = 0xE1;
pub const SBC_INDIRECT_Y: u8 = 0xF1;

/// SEC - Set Carry Flag
pub const SEC: u8 = 0x38;

/// SED - Set Decimal Flag
pub const SED: u8 = 0xF8;

/// SEI - Set Interrupt Disable
pub const SEI: u8 = 0x78;

/// STA - Store Accumulator
pub const STA_ZERO_PAGE: u8 = 0x85;
pub const STA_ZERO_PAGE_X: u8 = 0x95;
pub const STA_ABSOLUTE: u8 = 0x8D;
pub const STA_ABSOLUTE_X: u8 = 0x9D;
pub const STA_ABSOLUTE_Y: u8 = 0x99;
pub const STA_INDIRECT_X: u8 = 0x81;
pub const STA_INDIRECT_Y: u8 = 0x91;

/// STX - Store X Register
pub const STX_ZERO_PAGE: u8 = 0x86;
pub const STX_ZERO_PAGE_Y: u8 = 0x96;
pub const STX_ABSOLUTE: u8 = 0x8E;

/// STY - Store Y Register
pub const STY_ZERO_PAGE: u8 = 0x84;
pub const STY_ZERO_PAGE_X: u8 = 0x94;
pub const STY_ABSOLUTE: u8 = 0x8C;

/// TAX - Transfer Accumulator to X
pub const TAX: u8 = 0xAA;

/// TAY - Transfer Accumulator to Y
pub const TAY: u8 = 0xA8;

/// TSX - Transfer Stack Pointer to X
pub const TSX: u8 = 0xBA;

/// TXA - Transfer X to Accumulator
pub const TXA: u8 = 0x8A;

/// TXS - Transfer X to Stack Pointer
pub const TXS: u8 = 0x9A;

/// TYA - Transfer Y to Accumulator
pub const TYA: u8 = 0x98;

#[derive(Debug, Clone, Copy)]
pub struct OpCode {
    pub code: u8,
    pub name: &'static str,
    pub size: u16,
    pub cycles: u8,
    pub addr_mode: AddressingMode,
}

impl OpCode {
    pub const fn new(
        code: u8,
        name: &'static str,
        size: u16,
        cycles: u8,
        addr_mode: AddressingMode,
    ) -> Self {
        OpCode {
            code,
            name,
            size,
            cycles,
            addr_mode,
        }
    }
}

pub const OP_CODES: [Option<OpCode>; 256] = generate_op_codes();

const fn generate_op_codes() -> [Option<OpCode>; 256] {
    let op_codes: &[OpCode] = &[
        // ADC
        OpCode::new(ADC_IMMEDIATE, "ADC", 2, 2, AddressingMode::Immediate),
        OpCode::new(ADC_ZERO_PAGE, "ADC", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(ADC_ZERO_PAGE_X, "ADC", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(ADC_ABSOLUTE, "ADC", 3, 4, AddressingMode::Absolute),
        OpCode::new(ADC_ABSOLUTE_X, "ADC", 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(ADC_ABSOLUTE_Y, "ADC", 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(ADC_INDIRECT_X, "ADC", 2, 6, AddressingMode::IndirectX),
        OpCode::new(ADC_INDIRECT_Y, "ADC", 2, 5, AddressingMode::IndirectY),
        // AND
        OpCode::new(AND_IMMEDIATE, "AND", 2, 2, AddressingMode::Immediate),
        OpCode::new(AND_ZERO_PAGE, "AND", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(AND_ZERO_PAGE_X, "AND", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(AND_ABSOLUTE, "AND", 3, 4, AddressingMode::Absolute),
        OpCode::new(AND_ABSOLUTE_X, "AND", 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(AND_ABSOLUTE_Y, "AND", 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(AND_INDIRECT_X, "AND", 2, 6, AddressingMode::IndirectX),
        OpCode::new(AND_INDIRECT_Y, "AND", 2, 5, AddressingMode::IndirectY),
        // ASL
        OpCode::new(ASL_ACCUMULATOR, "ASL", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(ASL_ZERO_PAGE, "ASL", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(ASL_ZERO_PAGE_X, "ASL", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(ASL_ABSOLUTE, "ASL", 3, 6, AddressingMode::Absolute),
        OpCode::new(ASL_ABSOLUTE_X, "ASL", 3, 7, AddressingMode::AbsoluteX),
        // BCC
        OpCode::new(BCC, "BCC", 2, 2, AddressingMode::NoneAddressing),
        // BCS
        OpCode::new(BCS, "BCS", 2, 2, AddressingMode::NoneAddressing),
        // BEQ
        OpCode::new(BEQ, "BEQ", 2, 2, AddressingMode::NoneAddressing),
        // BIT
        OpCode::new(BIT_ZERO_PAGE, "BIT", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(BIT_ABSOLUTE, "BIT", 3, 4, AddressingMode::Absolute),
        // BMI
        OpCode::new(BMI, "BMI", 2, 2, AddressingMode::NoneAddressing),
        // BNE
        OpCode::new(BNE, "BNE", 2, 2, AddressingMode::NoneAddressing),
        // BPL
        OpCode::new(BPL, "BPL", 2, 2, AddressingMode::NoneAddressing),
        // BRK
        OpCode::new(BRK, "BRK", 1, 7, AddressingMode::NoneAddressing),
        // BVC
        OpCode::new(BVC, "BVC", 2, 2, AddressingMode::NoneAddressing),
        // BVS
        OpCode::new(BVS, "BVS", 2, 2, AddressingMode::NoneAddressing),
        // CLC
        OpCode::new(CLC, "CLC", 1, 2, AddressingMode::NoneAddressing),
        // CLD
        OpCode::new(CLD, "CLD", 1, 2, AddressingMode::NoneAddressing),
        // CLI
        OpCode::new(CLI, "CLI", 1, 2, AddressingMode::NoneAddressing),
        // CLV
        OpCode::new(CLV, "CLV", 1, 2, AddressingMode::NoneAddressing),
        // CMP
        OpCode::new(CMP_IMMEDIATE, "CMP", 2, 2, AddressingMode::Immediate),
        OpCode::new(CMP_ZERO_PAGE, "CMP", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(CMP_ZERO_PAGE_X, "CMP", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(CMP_ABSOLUTE, "CMP", 3, 4, AddressingMode::Absolute),
        OpCode::new(CMP_ABSOLUTE_X, "CMP", 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(CMP_ABSOLUTE_Y, "CMP", 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(CMP_INDIRECT_X, "CMP", 2, 6, AddressingMode::IndirectX),
        OpCode::new(CMP_INDIRECT_Y, "CMP", 2, 5, AddressingMode::IndirectY),
        // CPX
        OpCode::new(CPX_IMMEDIATE, "CPX", 2, 2, AddressingMode::Immediate),
        OpCode::new(CPX_ZERO_PAGE, "CPX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(CPX_ABSOLUTE, "CPX", 3, 4, AddressingMode::Absolute),
        // CPY
        OpCode::new(CPY_IMMEDIATE, "CPY", 2, 2, AddressingMode::Immediate),
        OpCode::new(CPY_ZERO_PAGE, "CPY", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(CPY_ABSOLUTE, "CPY", 3, 4, AddressingMode::Absolute),
        // DEC
        OpCode::new(DEC_ZERO_PAGE, "DEC", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(DEC_ZERO_PAGE_X, "DEC", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(DEC_ABSOLUTE, "DEC", 3, 6, AddressingMode::Absolute),
        OpCode::new(DEC_ABSOLUTE_X, "DEC", 3, 7, AddressingMode::AbsoluteX),
        // DEX
        OpCode::new(DEX, "DEX", 1, 2, AddressingMode::NoneAddressing),
        // DEY
        OpCode::new(DEY, "DEY", 1, 2, AddressingMode::NoneAddressing),
        // EOR
        OpCode::new(EOR_IMMEDIATE, "EOR", 2, 2, AddressingMode::Immediate),
        OpCode::new(EOR_ZERO_PAGE, "EOR", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(EOR_ZERO_PAGE_X, "EOR", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(EOR_ABSOLUTE, "EOR", 3, 4, AddressingMode::Absolute),
        OpCode::new(EOR_ABSOLUTE_X, "EOR", 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(EOR_ABSOLUTE_Y, "EOR", 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(EOR_INDIRECT_X, "EOR", 2, 6, AddressingMode::IndirectX),
        OpCode::new(EOR_INDIRECT_Y, "EOR", 2, 5, AddressingMode::IndirectY),
        // INC
        OpCode::new(INC_ZERO_PAGE, "INC", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(INC_ZERO_PAGE_X, "INC", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(INC_ABSOLUTE, "INC", 3, 6, AddressingMode::Absolute),
        OpCode::new(INC_ABSOLUTE_X, "INC", 3, 7, AddressingMode::AbsoluteX),
        // INX
        OpCode::new(INX, "INX", 1, 2, AddressingMode::NoneAddressing),
        // INY
        OpCode::new(INY, "INY", 1, 2, AddressingMode::NoneAddressing),
        // JMP
        OpCode::new(JMP_ABSOLUTE, "JMP", 3, 3, AddressingMode::Absolute),
        OpCode::new(JMP_INDIRECT, "JMP", 3, 5, AddressingMode::Indirect),
        // JSR
        OpCode::new(JSR, "JSR", 3, 6, AddressingMode::Absolute),
        // LDA
        OpCode::new(LDA_IMMEDIATE, "LDA", 2, 2, AddressingMode::Immediate),
        OpCode::new(LDA_ZERO_PAGE, "LDA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(LDA_ZERO_PAGE_X, "LDA", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(LDA_ABSOLUTE, "LDA", 3, 4, AddressingMode::Absolute),
        OpCode::new(LDA_ABSOLUTE_X, "LDA", 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(LDA_ABSOLUTE_Y, "LDA", 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(LDA_INDIRECT_X, "LDA", 2, 6, AddressingMode::IndirectX),
        OpCode::new(LDA_INDIRECT_Y, "LDA", 2, 5, AddressingMode::IndirectY),
        // LDX
        OpCode::new(LDX_IMMEDIATE, "LDX", 2, 2, AddressingMode::Immediate),
        OpCode::new(LDX_ZERO_PAGE, "LDX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(LDX_ZERO_PAGE_Y, "LDX", 2, 4, AddressingMode::ZeroPageY),
        OpCode::new(LDX_ABSOLUTE, "LDX", 3, 4, AddressingMode::Absolute),
        OpCode::new(LDX_ABSOLUTE_Y, "LDX", 3, 4, AddressingMode::AbsoluteY),
        // LDY
        OpCode::new(LDY_IMMEDIATE, "LDY", 2, 2, AddressingMode::Immediate),
        OpCode::new(LDY_ZERO_PAGE, "LDY", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(LDY_ZERO_PAGE_X, "LDY", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(LDY_ABSOLUTE, "LDY", 3, 4, AddressingMode::Absolute),
        OpCode::new(LDY_ABSOLUTE_X, "LDY", 3, 4, AddressingMode::AbsoluteX),
        // LSR
        OpCode::new(LSR_ACCUMULATOR, "LSR", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(LSR_ZERO_PAGE, "LSR", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(LSR_ZERO_PAGE_X, "LSR", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(LSR_ABSOLUTE, "LSR", 3, 6, AddressingMode::Absolute),
        OpCode::new(LSR_ABSOLUTE_X, "LSR", 3, 7, AddressingMode::AbsoluteX),
        // NOP
        OpCode::new(NOP, "NOP", 1, 2, AddressingMode::NoneAddressing),
        // ORA
        OpCode::new(ORA_IMMEDIATE, "ORA", 2, 2, AddressingMode::Immediate),
        OpCode::new(ORA_ZERO_PAGE, "ORA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(ORA_ZERO_PAGE_X, "ORA", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(ORA_ABSOLUTE, "ORA", 3, 4, AddressingMode::Absolute),
        OpCode::new(ORA_ABSOLUTE_X, "ORA", 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(ORA_ABSOLUTE_Y, "ORA", 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(ORA_INDIRECT_X, "ORA", 2, 6, AddressingMode::IndirectX),
        OpCode::new(ORA_INDIRECT_Y, "ORA", 2, 5, AddressingMode::IndirectY),
        // PHA
        OpCode::new(PHA, "PHA", 1, 3, AddressingMode::NoneAddressing),
        // PHP
        OpCode::new(PHP, "PHP", 1, 3, AddressingMode::NoneAddressing),
        // PLA
        OpCode::new(PLA, "PLA", 1, 4, AddressingMode::NoneAddressing),
        // PLP
        OpCode::new(PLP, "PLP", 1, 4, AddressingMode::NoneAddressing),
        // ROL
        OpCode::new(ROL_ACCUMULATOR, "ROL", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(ROL_ZERO_PAGE, "ROL", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(ROL_ZERO_PAGE_X, "ROL", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(ROL_ABSOLUTE, "ROL", 3, 6, AddressingMode::Absolute),
        OpCode::new(ROL_ABSOLUTE_X, "ROL", 3, 7, AddressingMode::AbsoluteX),
        // ROR
        OpCode::new(ROR_ACCUMULATOR, "ROR", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(ROR_ZERO_PAGE, "ROR", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(ROR_ZERO_PAGE_X, "ROR", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(ROR_ABSOLUTE, "ROR", 3, 6, AddressingMode::Absolute),
        OpCode::new(ROR_ABSOLUTE_X, "ROR", 3, 7, AddressingMode::AbsoluteX),
        // RTI
        OpCode::new(RTI, "RTI", 1, 6, AddressingMode::NoneAddressing),
        // RTS
        OpCode::new(RTS, "RTS", 1, 6, AddressingMode::NoneAddressing),
        // SBC
        OpCode::new(SBC_IMMEDIATE, "SBC", 2, 2, AddressingMode::Immediate),
        OpCode::new(SBC_ZERO_PAGE, "SBC", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(SBC_ZERO_PAGE_X, "SBC", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(SBC_ABSOLUTE, "SBC", 3, 4, AddressingMode::Absolute),
        OpCode::new(SBC_ABSOLUTE_X, "SBC", 3, 4, AddressingMode::AbsoluteX),
        OpCode::new(SBC_ABSOLUTE_Y, "SBC", 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(SBC_INDIRECT_X, "SBC", 2, 6, AddressingMode::IndirectX),
        OpCode::new(SBC_INDIRECT_Y, "SBC", 2, 5, AddressingMode::IndirectY),
        // SEC
        OpCode::new(SEC, "SEC", 1, 2, AddressingMode::NoneAddressing),
        // SED
        OpCode::new(SED, "SED", 1, 2, AddressingMode::NoneAddressing),
        // SEI
        OpCode::new(SEI, "SEI", 1, 2, AddressingMode::NoneAddressing),
        // STA
        OpCode::new(STA_ZERO_PAGE, "STA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(STA_ZERO_PAGE_X, "STA", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(STA_ABSOLUTE, "STA", 3, 4, AddressingMode::Absolute),
        OpCode::new(STA_ABSOLUTE_X, "STA", 3, 5, AddressingMode::AbsoluteX),
        OpCode::new(STA_ABSOLUTE_Y, "STA", 3, 5, AddressingMode::AbsoluteY),
        OpCode::new(STA_INDIRECT_X, "STA", 2, 6, AddressingMode::IndirectX),
        OpCode::new(STA_INDIRECT_Y, "STA", 2, 6, AddressingMode::IndirectY),
        // STX
        OpCode::new(STX_ZERO_PAGE, "STX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(STX_ZERO_PAGE_Y, "STX", 2, 4, AddressingMode::ZeroPageY),
        OpCode::new(STX_ABSOLUTE, "STX", 3, 4, AddressingMode::Absolute),
        // STY
        OpCode::new(STY_ZERO_PAGE, "STY", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(STY_ZERO_PAGE_X, "STY", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(STY_ABSOLUTE, "STY", 3, 4, AddressingMode::Absolute),
        // TAX
        OpCode::new(TAX, "TAX", 1, 2, AddressingMode::NoneAddressing),
        // TAY
        OpCode::new(TAY, "TAY", 1, 2, AddressingMode::NoneAddressing),
        // TSX
        OpCode::new(TSX, "TSX", 1, 2, AddressingMode::NoneAddressing),
        // TXA
        OpCode::new(TXA, "TXA", 1, 2, AddressingMode::NoneAddressing),
        // TXS
        OpCode::new(TXS, "TXS", 1, 2, AddressingMode::NoneAddressing),
        // TYA
        OpCode::new(TYA, "TYA", 1, 2, AddressingMode::NoneAddressing),
    ];

    let mut op_codes_map = [None; 256];

    let mut i = 0;
    while i < op_codes.len() {
        let op_code = op_codes[i];
        op_codes_map[op_code.code as usize] = Some(op_code);
        i += 1;
    }

    op_codes_map
}
