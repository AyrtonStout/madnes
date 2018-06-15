#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AddressingMode {
    Immediate,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    ZeroPageAbsolute,
    ZeroPageAbsoluteX,
    ZeroPageAbsoluteY,
    Implied,
    Accumulator,
    Indirect,
    PreIndexedIndirect,
    PostIndexedIndirect,
    Relative,
    Empty
}

//TODO get rid of num_bytes. The number of bytes can be determined by the addressing mode
static INSTRUCTIONS: &'static [InstructionType] = &[
    InstructionType { name: "BRK", num_bytes: 1, num_cycles: 7, addressing_mode: AddressingMode::Implied }, // 0
    InstructionType { name: "ORA", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::PreIndexedIndirect }, // 1 (Indirect, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 2
    InstructionType { name: "SLO", num_bytes: 2, num_cycles: 8, addressing_mode: AddressingMode::PreIndexedIndirect }, // 3 ** Unofficial **
    InstructionType { name: "DOP", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::ZeroPageAbsolute }, // 4 ** Unofficial **
    InstructionType { name: "ORA", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::ZeroPageAbsolute }, // 5 (Zero page)
    InstructionType { name: "ASL", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::ZeroPageAbsolute }, // 6 (Zero page)
    InstructionType { name: "SLO", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::ZeroPageAbsolute }, // 7 ** Unofficial **
    InstructionType { name: "PHP", num_bytes: 1, num_cycles: 3, addressing_mode: AddressingMode::Implied }, // 8
    InstructionType { name: "ORA", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Immediate }, // 9 (Immediate)
    InstructionType { name: "ASL", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Accumulator }, // A (Accumulator)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // B (Immediate)
    InstructionType { name: "TOP", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Absolute }, // C (Immediate) ** Unofficial **
    InstructionType { name: "ORA", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Absolute }, // D (Absolute)
    InstructionType { name: "ASL", num_bytes: 3, num_cycles: 6, addressing_mode: AddressingMode::Absolute }, // E (Absolute)
    InstructionType { name: "SLO", num_bytes: 3, num_cycles: 6, addressing_mode: AddressingMode::Absolute }, // F ** Unofficial **

    InstructionType { name: "BPL", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: AddressingMode::Relative }, // 10
    InstructionType { name: "ORA", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::PostIndexedIndirect }, // 11 (Indirect, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 12
    InstructionType { name: "SLO", num_bytes: 2, num_cycles: 8, addressing_mode: AddressingMode::PostIndexedIndirect }, // 13 ** Unofficial **
    InstructionType { name: "DOP", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // 14 ** Unofficial **
    InstructionType { name: "ORA", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // 15 (Zero Page, X)
    InstructionType { name: "ASL", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // 16 (Zero Page, X)
    InstructionType { name: "SLO", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // 17 ** Unofficial **
    InstructionType { name: "CLC", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // 18
    InstructionType { name: "ORA", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::AbsoluteY }, // 19 (Absolute, Y)
    InstructionType { name: "NOP", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // 1A ** Unofficial **
    InstructionType { name: "SLO", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::AbsoluteY }, // 1B ** Unofficial **
    InstructionType { name: "TOP", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::AbsoluteX }, // 1C (Immediate) ** Unofficial **
    InstructionType { name: "ORA", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::AbsoluteX }, // 1D (Absolute, X)
    InstructionType { name: "ASL", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::AbsoluteX }, // 1E (Absolute, X)
    InstructionType { name: "SLO", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::AbsoluteX }, // 1F ** Unofficial **

    InstructionType { name: "JSR", num_bytes: 3, num_cycles: 6, addressing_mode: AddressingMode::Absolute }, // 20
    InstructionType { name: "AND", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::PreIndexedIndirect }, // 21 (Indirect, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 22
    InstructionType { name: "RLA", num_bytes: 2, num_cycles: 8, addressing_mode: AddressingMode::PreIndexedIndirect }, // 23 ** Unofficial **
    InstructionType { name: "BIT", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::ZeroPageAbsolute }, // 24 (Zero page)
    InstructionType { name: "AND", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::ZeroPageAbsolute }, // 25 (Zero page)
    InstructionType { name: "ROL", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::ZeroPageAbsolute }, // 26 (Zero page)
    InstructionType { name: "RLA", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::ZeroPageAbsolute }, // 27 ** Unofficial **
    InstructionType { name: "PLP", num_bytes: 1, num_cycles: 4, addressing_mode: AddressingMode::Implied }, // 28
    InstructionType { name: "AND", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Immediate }, // 29 (Immediate)
    InstructionType { name: "ROL", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Accumulator }, // 2A (Accumulator)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 2B
    InstructionType { name: "BIT", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Absolute }, // 2C (Absolute)
    InstructionType { name: "AND", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Absolute }, // 2D (Absolute)
    InstructionType { name: "ROL", num_bytes: 3, num_cycles: 6, addressing_mode: AddressingMode::Absolute }, // 2E (Absolute)
    InstructionType { name: "RLA", num_bytes: 3, num_cycles: 6, addressing_mode: AddressingMode::Absolute }, // 2F ** Unofficial **

    InstructionType { name: "BMI", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: AddressingMode::Relative }, // 30
    InstructionType { name: "AND", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::PostIndexedIndirect }, // 31 (Indirect, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 32
    InstructionType { name: "RLA", num_bytes: 2, num_cycles: 8, addressing_mode: AddressingMode::PostIndexedIndirect }, // 33 ** Unofficial **
    InstructionType { name: "DOP", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // 34 ** Unofficial **
    InstructionType { name: "AND", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // 35 (Zero Page, X)
    InstructionType { name: "ROL", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // 36 (Zero Page, X)
    InstructionType { name: "RLA", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // 37 ** Unofficial **
    InstructionType { name: "SEC", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // 38
    InstructionType { name: "AND", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::AbsoluteY }, // 39 (Absolute, Y)
    InstructionType { name: "NOP", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // 3A ** Unofficial **
    InstructionType { name: "RLA", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::AbsoluteY }, // 3B ** Unofficial **
    InstructionType { name: "TOP", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::AbsoluteX }, // 3C (Immediate) ** Unofficial **
    InstructionType { name: "AND", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::AbsoluteX }, // 3D (Absolute, X)
    InstructionType { name: "ROL", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::AbsoluteX }, // 3E (Absolute, X)
    InstructionType { name: "RLA", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::AbsoluteX }, // 3F ** Unofficial **

    InstructionType { name: "RTI", num_bytes: 1, num_cycles: 4, addressing_mode: AddressingMode::Implied }, // 40
    InstructionType { name: "EOR", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::PreIndexedIndirect }, // 41 (Indirect, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 42
    InstructionType { name: "SRE", num_bytes: 2, num_cycles: 8, addressing_mode: AddressingMode::PreIndexedIndirect }, // 43 ** Unofficial **
    InstructionType { name: "DOP", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::ZeroPageAbsolute }, // 44 ** Unofficial **
    InstructionType { name: "EOR", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::ZeroPageAbsolute }, // 45 (Zero Page)
    InstructionType { name: "LSR", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::ZeroPageAbsolute }, // 46 (Zero Page)
    InstructionType { name: "SRE", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::ZeroPageAbsolute }, // 47 ** Unofficial **
    InstructionType { name: "PHA", num_bytes: 1, num_cycles: 3, addressing_mode: AddressingMode::Implied }, // 48
    InstructionType { name: "EOR", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Immediate }, // 49 Immediate
    InstructionType { name: "LSR", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Accumulator }, // 4A Accumulator
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 4B
    InstructionType { name: "JMP", num_bytes: 3, num_cycles: 3, addressing_mode: AddressingMode::Absolute }, // 4C (Absolute)
    InstructionType { name: "EOR", num_bytes: 3, num_cycles: 6, addressing_mode: AddressingMode::Absolute }, // 4D (Absolute)
    InstructionType { name: "LSR", num_bytes: 3, num_cycles: 6, addressing_mode: AddressingMode::Absolute }, // 4E (Absolute)
    InstructionType { name: "SRE", num_bytes: 3, num_cycles: 6, addressing_mode: AddressingMode::Absolute }, // 4F ** Unofficial **

    InstructionType { name: "BVC", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: AddressingMode::Relative }, // 50
    InstructionType { name: "EOR", num_bytes: 2, num_cycles: 5/* * */, addressing_mode: AddressingMode::PostIndexedIndirect }, // 51 (Indirect, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 52
    InstructionType { name: "SRE", num_bytes: 2, num_cycles: 8, addressing_mode: AddressingMode::PostIndexedIndirect }, // 53 ** Unofficial **
    InstructionType { name: "DOP", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // 54 ** Unofficial **
    InstructionType { name: "EOR", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // 55 (Zero Page, X)
    InstructionType { name: "LSR", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // 56 (Zero Page, X)
    InstructionType { name: "SRE", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // 57 ** Unofficial **
    InstructionType { name: "CLI", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // 58
    InstructionType { name: "EOR", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::AbsoluteY }, // 59 (Absolute, Y)
    InstructionType { name: "NOP", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // 5A ** Unofficial **
    InstructionType { name: "SRE", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::AbsoluteY }, // 5B ** Unofficial **
    InstructionType { name: "TOP", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::AbsoluteX }, // 5C (Immediate) ** Unofficial **
    InstructionType { name: "EOR", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::AbsoluteX }, // 5D (Absolute, X)
    InstructionType { name: "LSR", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::AbsoluteX }, // 5E (Absolute, X)
    InstructionType { name: "SRE", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::AbsoluteX }, // 5F ** Unofficial **

    InstructionType { name: "RTS", num_bytes: 1, num_cycles: 6, addressing_mode: AddressingMode::Implied }, // 60
    InstructionType { name: "ADC", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::PreIndexedIndirect }, // 61 (Indirect, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 62
    InstructionType { name: "RRA", num_bytes: 2, num_cycles: 8, addressing_mode: AddressingMode::PreIndexedIndirect }, // 63 ** Unofficial **
    InstructionType { name: "DOP", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::ZeroPageAbsolute }, // 64 ** Unofficial **
    InstructionType { name: "ADC", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::ZeroPageAbsolute }, // 65 (Zero Page)
    InstructionType { name: "ROR", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::ZeroPageAbsolute }, // 66 (Zero Page)
    InstructionType { name: "RRA", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::ZeroPageAbsolute }, // 67 ** Unofficial **
    InstructionType { name: "PLA", num_bytes: 1, num_cycles: 4, addressing_mode: AddressingMode::Implied }, // 68
    InstructionType { name: "ADC", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Immediate }, // 69 (Immediate)
    InstructionType { name: "ROR", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Accumulator }, // 6A (Accumulator)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 6B
    InstructionType { name: "JMP", num_bytes: 3, num_cycles: 5, addressing_mode: AddressingMode::Indirect }, // 6C (Indirect)
    InstructionType { name: "ADC", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Absolute }, // 6D (Absolute)
    InstructionType { name: "ROR", num_bytes: 3, num_cycles: 6, addressing_mode: AddressingMode::Absolute }, // 6E (Absolute)
    InstructionType { name: "RRA", num_bytes: 3, num_cycles: 6, addressing_mode: AddressingMode::Absolute }, // 6F ** Unofficial **

    InstructionType { name: "BVS", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: AddressingMode::Relative }, // 70
    InstructionType { name: "ADC", num_bytes: 2, num_cycles: 5/* * */, addressing_mode: AddressingMode::PostIndexedIndirect }, // 71 (Indirect, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 72
    InstructionType { name: "RRA", num_bytes: 2, num_cycles: 8, addressing_mode: AddressingMode::PostIndexedIndirect }, // 73 ** Unofficial **
    InstructionType { name: "DOP", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // 74 ** Unofficial **
    InstructionType { name: "ADC", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // 75 (Zero Page, X)
    InstructionType { name: "ROR", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // 76 (Zero Page, X)
    InstructionType { name: "RRA", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // 77 ** Unofficial **
    InstructionType { name: "SEI", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // 78
    InstructionType { name: "ADC", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::AbsoluteY }, // 79 (Absolute, Y)
    InstructionType { name: "NOP", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // 7A ** Unofficial **
    InstructionType { name: "RRA", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::AbsoluteY }, // 7B ** Unofficial **
    InstructionType { name: "TOP", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::AbsoluteX }, // 7C (Immediate) ** Unofficial **
    InstructionType { name: "ADC", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::AbsoluteX }, // 7D (Absolute, X)
    InstructionType { name: "ROR", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::AbsoluteX }, // 7E (Absolute, X)
    InstructionType { name: "RRA", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::AbsoluteX }, // 7F ** Unofficial **

    InstructionType { name: "DOP", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Immediate }, // 80 ** Unofficial **
    InstructionType { name: "STA", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::PreIndexedIndirect }, // 81 (Indirect, X)
    InstructionType { name: "DOP", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Immediate }, // 82 ** Unofficial **
    InstructionType { name: "SAX", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::PreIndexedIndirect }, // 83 ** Unofficial **
    InstructionType { name: "STY", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::ZeroPageAbsolute }, // 84 (Zero Page)
    InstructionType { name: "STA", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::ZeroPageAbsolute }, // 85 (Zero Page)
    InstructionType { name: "STX", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::ZeroPageAbsolute }, // 86 (Zero Page)
    InstructionType { name: "SAX", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::ZeroPageAbsolute }, // 87 ** Unofficial **
    InstructionType { name: "DEY", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // 88
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 89
    InstructionType { name: "TXA", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // 8A
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 8B
    InstructionType { name: "STY", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Absolute }, // 8C (Absolute)
    InstructionType { name: "STA", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Absolute }, // 8D (Absolute)
    InstructionType { name: "STX", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Absolute }, // 8E (Absolute)
    InstructionType { name: "SAX", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Absolute }, // 8F ** Unofficial **

    InstructionType { name: "BCC", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: AddressingMode::Relative }, // 90
    InstructionType { name: "STA", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::PostIndexedIndirect }, // 91 (Indirect, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 92
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 93
    InstructionType { name: "STY", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // 94 (Zero Page, X)
    InstructionType { name: "STA", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // 95 (Zero Page, X)
    InstructionType { name: "STX", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::ZeroPageAbsoluteY }, // 96 (Zero Page, Y)
    InstructionType { name: "SAX", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::ZeroPageAbsoluteY }, // 97 ** Unofficial **
    InstructionType { name: "TYA", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // 98
    InstructionType { name: "STA", num_bytes: 3, num_cycles: 5, addressing_mode: AddressingMode::AbsoluteY }, // 99 (Absolute, Y)
    InstructionType { name: "TXS", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // 9A
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 9B
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 9C
    InstructionType { name: "STA", num_bytes: 3, num_cycles: 2/* * */, addressing_mode: AddressingMode::AbsoluteX }, // 9D (Absolute, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 9E
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 9F

    InstructionType { name: "LDY", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Immediate }, // A0 (Immediate)
    InstructionType { name: "LDA", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::PreIndexedIndirect }, // A1 (Indirect, X)
    InstructionType { name: "LDX", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Immediate }, // A2 (Immediate)
    InstructionType { name: "LAX", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::PreIndexedIndirect }, // A3 ** Unofficial **
    InstructionType { name: "LDY", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::ZeroPageAbsolute }, // A4 (Zero Page)
    InstructionType { name: "LDA", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::ZeroPageAbsolute }, // A5 (Zero Page)
    InstructionType { name: "LDX", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::ZeroPageAbsolute }, // A6 (Zero Page)
    InstructionType { name: "LAX", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::ZeroPageAbsolute }, // A7 ** Unofficial **
    InstructionType { name: "TAY", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // A8
    InstructionType { name: "LDA", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Immediate }, // A9 (Immediate)
    InstructionType { name: "TAX", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // AA
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // AB
    InstructionType { name: "LDY", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Absolute }, // AC (Absolute)
    InstructionType { name: "LDA", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Absolute }, // AD (Absolute)
    InstructionType { name: "LDX", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Absolute }, // AE (Absolute)
    InstructionType { name: "LAX", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Absolute }, // AF ** Unofficial **

    InstructionType { name: "BCS", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: AddressingMode::Relative }, // B0
    InstructionType { name: "LDA", num_bytes: 2, num_cycles: 5/* * */, addressing_mode: AddressingMode::PostIndexedIndirect }, // B1 (Indirect, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // B2
    InstructionType { name: "LAX", num_bytes: 2, num_cycles: 5/* * */, addressing_mode: AddressingMode::PostIndexedIndirect }, // B3 ** Unofficial **
    InstructionType { name: "LDY", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // B4 (Zero Page, X)
    InstructionType { name: "LDA", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // B5 (Zero Page, X)
    InstructionType { name: "LDX", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::ZeroPageAbsoluteY }, // B6 (Zero Page, Y)
    InstructionType { name: "LAX", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::ZeroPageAbsoluteY }, // B7 ** Unofficial **
    InstructionType { name: "CLV", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // B8
    InstructionType { name: "LDA", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::AbsoluteY }, // B9 (Absolute, Y)
    InstructionType { name: "TSX", num_bytes: 1, num_cycles: 5/* * */, addressing_mode: AddressingMode::Implied }, // BA
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // BB
    InstructionType { name: "LDY", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::AbsoluteX }, // BC (Absolute, X)
    InstructionType { name: "LDA", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::AbsoluteX }, // BD (Absolute, X)
    InstructionType { name: "LDX", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::AbsoluteY }, // BE (Absolute, Y)
    InstructionType { name: "LAX", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::AbsoluteY }, // BF ** Unofficial **

    InstructionType { name: "CPY", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Immediate }, // C0 (Immediate)
    InstructionType { name: "CMP", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::PreIndexedIndirect }, // C1 (Indirect, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // C2
    InstructionType { name: "DCP", num_bytes: 2, num_cycles: 8, addressing_mode: AddressingMode::PreIndexedIndirect }, // C3 ** Unofficial **
    InstructionType { name: "CPY", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::ZeroPageAbsolute }, // C4 (Zero Page)
    InstructionType { name: "CMP", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::ZeroPageAbsolute }, // C5 (Zero Page)
    InstructionType { name: "DEC", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::ZeroPageAbsolute }, // C6 (Zero Page)
    InstructionType { name: "DCP", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::ZeroPageAbsolute }, // C7 ** Unofficial **
    InstructionType { name: "INY", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // C8
    InstructionType { name: "CMP", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Immediate }, // C9 (Immediate)
    InstructionType { name: "DEX", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // CA
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // CB
    InstructionType { name: "CPY", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Absolute }, // CC (Absolute)
    InstructionType { name: "CMP", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Absolute }, // CD (Absolute)
    InstructionType { name: "DEC", num_bytes: 3, num_cycles: 6, addressing_mode: AddressingMode::Absolute }, // CE (Absolute)
    InstructionType { name: "DCP", num_bytes: 3, num_cycles: 6, addressing_mode: AddressingMode::Absolute }, // CF ** Unofficial **

    InstructionType { name: "BNE", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: AddressingMode::Relative }, // D0
    InstructionType { name: "CMP", num_bytes: 2, num_cycles: 5/* * */, addressing_mode: AddressingMode::PostIndexedIndirect }, // D1 (Indirect, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // D2
    InstructionType { name: "DCP", num_bytes: 2, num_cycles: 8, addressing_mode: AddressingMode::PostIndexedIndirect }, // D3 ** Unofficial **
    InstructionType { name: "DOP", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // D4 ** Unofficial **
    InstructionType { name: "CMP", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // D5 (Zero page, X)
    InstructionType { name: "DEC", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // D6 (Zero page, X)
    InstructionType { name: "DCP", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // D7 ** Unofficial **
    InstructionType { name: "CLD", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // D8
    InstructionType { name: "CMP", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::AbsoluteY }, // D9 (Absolute, Y)
    InstructionType { name: "NOP", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // DA ** Unofficial **
    InstructionType { name: "DCP", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::AbsoluteY }, // DF ** Unofficial **
    InstructionType { name: "TOP", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::AbsoluteX }, // DC (Immediate) ** Unofficial **
    InstructionType { name: "CMP", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::AbsoluteX }, // DD (Absolute, X)
    InstructionType { name: "DEC", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::AbsoluteX }, // DE (Absolute, X)
    InstructionType { name: "DCP", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::AbsoluteX }, // DF ** Unofficial **

    InstructionType { name: "CPX", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Immediate }, // E0 (Immediate)
    InstructionType { name: "SBC", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::PreIndexedIndirect }, // E1 (Indirect, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // E2
    InstructionType { name: "ISB", num_bytes: 2, num_cycles: 8, addressing_mode: AddressingMode::PreIndexedIndirect }, // E3 ** Unofficial **
    InstructionType { name: "CPX", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::ZeroPageAbsolute }, // E4 (Zero Page)
    InstructionType { name: "SBC", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::ZeroPageAbsolute }, // E5 (Zero Page)
    InstructionType { name: "INC", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::ZeroPageAbsolute }, // E6 (Zero Page)
    InstructionType { name: "ISB", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::ZeroPageAbsolute }, // E7 ** Unofficial **
    InstructionType { name: "INX", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // E8
    InstructionType { name: "SBC", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Immediate }, // E9 (Immediate)
    InstructionType { name: "NOP", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // EA
    InstructionType { name: "SBC", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Immediate }, // EB // ** Unofficial ** (but same as official)
    InstructionType { name: "CPX", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Absolute }, // EC (Absolute)
    InstructionType { name: "SBC", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Absolute }, // ED (Absolute)
    InstructionType { name: "INC", num_bytes: 3, num_cycles: 6, addressing_mode: AddressingMode::Absolute }, // EE (Absolute)
    InstructionType { name: "ISB", num_bytes: 3, num_cycles: 6, addressing_mode: AddressingMode::Absolute }, // EF ** Unofficial **

    InstructionType { name: "BEQ", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: AddressingMode::Relative }, // F0
    InstructionType { name: "SBC", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::PostIndexedIndirect }, // F1 (Indirect, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // F2
    InstructionType { name: "ISB", num_bytes: 2, num_cycles: 8, addressing_mode: AddressingMode::PostIndexedIndirect }, // F3 ** Unofficial **
    InstructionType { name: "DOP", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // F4 ** Unofficial **
    InstructionType { name: "SBC", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // F5 (Zero Page, X)
    InstructionType { name: "INC", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // F6 (Zero Page, X)
    InstructionType { name: "ISB", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::ZeroPageAbsoluteX }, // F7 ** Unofficial **
    InstructionType { name: "SED", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // F8
    InstructionType { name: "SBC", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::AbsoluteY }, // F9 (Absolute, Y)
    InstructionType { name: "NOP", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // FA ** Unofficial **
    InstructionType { name: "ISB", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::AbsoluteY }, // FB ** Unofficial **
    InstructionType { name: "TOP", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::AbsoluteX }, // FC (Immediate) ** Unofficial **
    InstructionType { name: "SBC", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::AbsoluteX }, // FD (Absolute, X)
    InstructionType { name: "INC", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::AbsoluteX }, // FE (Absolute, X)
    InstructionType { name: "ISB", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::AbsoluteX }, // FF ** Unofficial **
];

#[derive(Clone, Copy, Debug)]
pub struct InstructionType {
    pub name: &'static str,
    pub num_bytes: u8,
    pub num_cycles: u8,
    pub addressing_mode: AddressingMode
}

pub fn get_instruction(opcode: u8) -> InstructionType {
    let found_instruction = INSTRUCTIONS[opcode as usize];

    if found_instruction.num_bytes == 0 {
        panic!(format!("Attempted to access an unimplemented op code {:X}!", opcode))
    }

    return found_instruction;
}


#[cfg(test)]
mod tests {

    use instruction_set;

    #[test]
    fn can_find_valid_instruction() {
        let instruction = instruction_set::get_instruction(8);
        assert_eq!(instruction.name, "PHP");
        assert_eq!(instruction.num_cycles, 3);
        assert_eq!(instruction.num_bytes, 1);
    }

    #[test]
    #[should_panic]
    fn errors_on_unimplemented_instruction() {
        instruction_set::get_instruction(3);
    }
}
