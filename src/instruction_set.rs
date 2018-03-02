
// I might never use these with the way things are going. Commented them out to remove unused warnings
#[derive(Clone, Copy, Debug)]
pub enum AddressingMode {
//    Immediate,
//    Absolute,
//    ZeroPageAbsolute,
    Implied,
//    Accumulator,
//    Indexed,
//    ZeroPageIndexed,
//    Indirect,
//    PreIndexedIndirect,
//    PostIndexedIndirect,
//    Relative,
    Empty
}

static INSTRUCTIONS: &'static [InstructionType] = &[
    InstructionType { name: "BRK", num_bytes: 1, num_cycles: 7, addressing_mode: AddressingMode::Empty }, // 0
    InstructionType { name: "ORA", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // 1 (Indirect, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 2
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 3
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 4
    InstructionType { name: "ORA", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::Empty }, // 5 (Zero page)
    InstructionType { name: "ASL", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::Empty }, // 6 (Zero page)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 7 [Unofficial]
    InstructionType { name: "PHP", num_bytes: 1, num_cycles: 3, addressing_mode: AddressingMode::Empty }, // 8
    InstructionType { name: "ORA", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // 9 (Immediate)
    InstructionType { name: "ASL", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // A (Accumulator)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // B (Immediate)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // C (Immediate)
    InstructionType { name: "ORA", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // D (Absolute)
    InstructionType { name: "ASL", num_bytes: 3, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // E (Absolute)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // F (Immediate)

    InstructionType { name: "BPL", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: AddressingMode::Empty }, // 10
    InstructionType { name: "ORA", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::Empty }, // 11 (Indirect, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 12
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 13
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 14
    InstructionType { name: "ORA", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // 15 (Zero Page, X)
    InstructionType { name: "ASL", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // 16 (Zero Page, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 17
    InstructionType { name: "CLC", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // 18
    InstructionType { name: "ORA", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // 19 (Absolute, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 1A
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 1B
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 1C
    InstructionType { name: "ORA", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::Empty }, // 1D (Absolute, X)
    InstructionType { name: "ASL", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::Empty }, // 1E (Absolute, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 1F

    InstructionType { name: "JSR", num_bytes: 3, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // 20
    InstructionType { name: "AND", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // 21 (Indirect, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 22
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 23
    InstructionType { name: "BIT", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::Empty }, // 24 (Zero page)
    InstructionType { name: "AND", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::Empty }, // 25 (Zero page)
    InstructionType { name: "ROL", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::Empty }, // 26 (Zero page)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 27
    InstructionType { name: "PLP", num_bytes: 1, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // 28
    InstructionType { name: "AND", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // 29 (Intermediate)
    InstructionType { name: "ROL", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // 2A (Accumulator)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 2B
    InstructionType { name: "BIT", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // 2C (Absolute)
    InstructionType { name: "AND", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // 2D (Absolute)
    InstructionType { name: "ROL", num_bytes: 3, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // 2E (Absolute)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 2F

    InstructionType { name: "BMI", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: AddressingMode::Empty }, // 30
    InstructionType { name: "AND", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::Empty }, // 31 (Indirect, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 32
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 33
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 34
    InstructionType { name: "AND", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // 35 (Zero Page, X)
    InstructionType { name: "ROL", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // 36 (Zero Page, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 37
    InstructionType { name: "SEC", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // 38
    InstructionType { name: "AND", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::Empty }, // 39 (Absolute, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 3A
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 3B
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 3C
    InstructionType { name: "AND", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::Empty }, // 3D (Absolute, X)
    InstructionType { name: "ROL", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::Empty }, // 3E (Absolute, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 3F

    InstructionType { name: "RTI", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // 40
    InstructionType { name: "EOR", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // 41 (Indirect, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 42
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 43
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 44
    InstructionType { name: "EOR", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::Empty }, // 45 (Zero Page)
    InstructionType { name: "LSR", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::Empty }, // 46 (Zero Page)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 47
    InstructionType { name: "PHA", num_bytes: 1, num_cycles: 3, addressing_mode: AddressingMode::Empty }, // 48
    InstructionType { name: "EOR", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // 49 Immediate
    InstructionType { name: "LSR", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // 4A Accumulator
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 4B
    InstructionType { name: "JMP", num_bytes: 3, num_cycles: 3, addressing_mode: AddressingMode::Empty }, // 4C (Absolute)
    InstructionType { name: "EOR", num_bytes: 1, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // 4D (Absolute)
    InstructionType { name: "LSR", num_bytes: 3, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // 4E (Absolute)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 4F

    InstructionType { name: "BVC", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: AddressingMode::Empty }, // 50
    InstructionType { name: "EOR", num_bytes: 2, num_cycles: 5/* * */, addressing_mode: AddressingMode::Empty }, // 51 (Indirect, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 52
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 53
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 54
    InstructionType { name: "EOR", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // 55 (Zero Page, X)
    InstructionType { name: "LSR", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // 56 (Zero Page, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 57
    InstructionType { name: "CLI", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // 58
    InstructionType { name: "EOR", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::Empty }, // 59 (Absolute, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 5A
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 5B
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 5C
    InstructionType { name: "EOR", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::Empty }, // 5D (Absolute, X)
    InstructionType { name: "LSR", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::Empty }, // 5E (Absolute, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 5F

    InstructionType { name: "RTS", num_bytes: 1, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // 60
    InstructionType { name: "ADC", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // 61 (Indirect, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 62
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 63
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 64
    InstructionType { name: "ADC", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::Empty }, // 65 (Zero Page)
    InstructionType { name: "ROR", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::Empty }, // 66 (Zero Page)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 67
    InstructionType { name: "PLA", num_bytes: 1, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // 68
    InstructionType { name: "ADC", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // 69 (Immediate)
    InstructionType { name: "ROR", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // 6A (Accumulator)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 6B
    InstructionType { name: "JMP", num_bytes: 3, num_cycles: 5, addressing_mode: AddressingMode::Empty }, // 6C (Indirect)
    InstructionType { name: "ADC", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // 6D (Absolute)
    InstructionType { name: "ROR", num_bytes: 3, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // 6E (Absolute)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 6F

    InstructionType { name: "BVS", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: AddressingMode::Empty }, // 70
    InstructionType { name: "ADC", num_bytes: 2, num_cycles: 5/* * */, addressing_mode: AddressingMode::Empty }, // 71 (Indirect, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 72
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 73
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 74
    InstructionType { name: "ADC", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // 75 (Zero Page, X)
    InstructionType { name: "ROR", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // 76 (Zero Page, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 77
    InstructionType { name: "SEI", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Implied }, // 78
    InstructionType { name: "ADC", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::Empty }, // 79 (Absolute, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 7A
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 7B
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 7C
    InstructionType { name: "ADC", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::Empty }, // 7D (Absolute, X)
    InstructionType { name: "ROR", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::Empty }, // 7E (Absolute, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 7F

    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 80
    InstructionType { name: "STA", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // 81 (Indirect, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 82
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 83
    InstructionType { name: "STY", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::Empty }, // 84 (Zero Page)
    InstructionType { name: "STA", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::Empty }, // 85 (Zero Page)
    InstructionType { name: "STX", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::Empty }, // 86 (Zero Page)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 87
    InstructionType { name: "DEY", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // 88
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 89
    InstructionType { name: "TXA", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // 8A
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 8B
    InstructionType { name: "STY", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // 8C (Absolute)
    InstructionType { name: "STA", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // 8D (Absolute)
    InstructionType { name: "STX", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // 8E (Absolute)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 8F

    InstructionType { name: "BCC", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: AddressingMode::Empty }, // 90
    InstructionType { name: "STA", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // 91 (Indirect, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 92
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 93
    InstructionType { name: "STY", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // 94 (Zero Page, X)
    InstructionType { name: "STA", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // 95 (Zero Page, X)
    InstructionType { name: "STX", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // 96 (Zero Page, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 97
    InstructionType { name: "TYA", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // 98
    InstructionType { name: "STA", num_bytes: 3, num_cycles: 5, addressing_mode: AddressingMode::Empty }, // 99 (Absolute, Y)
    InstructionType { name: "TXS", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // 9A
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 9B
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 9C
    InstructionType { name: "STA", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: AddressingMode::Empty }, // 9D (Absolute, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 9E
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // 9F

    InstructionType { name: "LDY", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // A0 (Immediate)
    InstructionType { name: "LDA", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // A1 (Indirect, X)
    InstructionType { name: "LDX", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // A2 (Immediate)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // A3
    InstructionType { name: "LDY", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::Empty }, // A4 (Zero Page)
    InstructionType { name: "LDA", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::Empty }, // A5 (Zero Page)
    InstructionType { name: "LDX", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::Empty }, // A6 (Zero Page)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // A7
    InstructionType { name: "TAY", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // A8
    InstructionType { name: "LDA", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // A9 (Immediate)
    InstructionType { name: "TAX", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // AA
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // AB
    InstructionType { name: "LDY", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // AC (Absolute)
    InstructionType { name: "LDA", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // AD (Absolute)
    InstructionType { name: "LDX", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // AE (Absolute)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // AF

    InstructionType { name: "BCS", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: AddressingMode::Empty }, // B0
    InstructionType { name: "LDA", num_bytes: 2, num_cycles: 5/* * */, addressing_mode: AddressingMode::Empty }, // B1 (Indirect, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // B2
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // B3
    InstructionType { name: "LDY", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // B4 (Zero Page, X)
    InstructionType { name: "LDA", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // B5 (Zero Page, X)
    InstructionType { name: "LDX", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // B6 (Zero Page, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // B7
    InstructionType { name: "CLV", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // B8
    InstructionType { name: "LDA", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::Empty }, // B9 (Absolute, Y)
    InstructionType { name: "TSX", num_bytes: 2, num_cycles: 5/* * */, addressing_mode: AddressingMode::Empty }, // BA
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // BB
    InstructionType { name: "LDY", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::Empty }, // BC (Absolute, X)
    InstructionType { name: "LDA", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::Empty }, // BD (Absolute, X)
    InstructionType { name: "LDX", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::Empty }, // BE (Absolute, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // BF

    InstructionType { name: "CPY", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // C0 (Immediate)
    InstructionType { name: "CMP", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // C1 (Indirect, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // C2
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // C3
    InstructionType { name: "CPY", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::Empty }, // C4 (Zero Page)
    InstructionType { name: "CMP", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::Empty }, // C5 (Zero Page)
    InstructionType { name: "DEC", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::Empty }, // C6 (Zero Page)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // C7
    InstructionType { name: "INY", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // C8
    InstructionType { name: "CMP", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // C9 (Immediate)
    InstructionType { name: "DEC", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // CA
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // CB
    InstructionType { name: "CPY", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // CC (Absolute)
    InstructionType { name: "CMP", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // CD (Absolute)
    InstructionType { name: "DEC", num_bytes: 3, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // CE (Absolute)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // CF

    InstructionType { name: "BNE", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: AddressingMode::Empty }, // D0
    InstructionType { name: "CMP", num_bytes: 2, num_cycles: 5/* * */, addressing_mode: AddressingMode::Empty }, // D1 (Indirect, @,Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // D2
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // D3
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // D4
    InstructionType { name: "CMP", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // D5 (Zero page, X)
    InstructionType { name: "DEC", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // D6 (Zero page, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // D7
    InstructionType { name: "CLD", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // D8
    InstructionType { name: "CMP", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::Empty }, // D9 (Absolute, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // DA
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // DB
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // DC
    InstructionType { name: "CMP", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::Empty }, // DD (Absolute, X)
    InstructionType { name: "DEC", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::Empty }, // DE (Absolute, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // DF

    InstructionType { name: "CPX", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // E0 (Immediate)
    InstructionType { name: "SBC", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // E1 (Indirect, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // E2
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // E3
    InstructionType { name: "CPX", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::Empty }, // E4 (Zero Page)
    InstructionType { name: "SBC", num_bytes: 2, num_cycles: 3, addressing_mode: AddressingMode::Empty }, // E5 (Zero Page)
    InstructionType { name: "INC", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::Empty }, // E6 (Zero Page)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // E7
    InstructionType { name: "INX", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // E8
    InstructionType { name: "SBC", num_bytes: 2, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // E9 (Immediate)
    InstructionType { name: "NOP", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // EA
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // EB
    InstructionType { name: "CPX", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // EC (Absolute)
    InstructionType { name: "SBC", num_bytes: 3, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // ED (Absolute)
    InstructionType { name: "INC", num_bytes: 3, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // EE (Absolute)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // EF

    InstructionType { name: "BEQ", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: AddressingMode::Empty }, // F0
    InstructionType { name: "SBC", num_bytes: 2, num_cycles: 5, addressing_mode: AddressingMode::Empty }, // F1 (Indirect, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // F2
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // F3
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // F4
    InstructionType { name: "SBC", num_bytes: 2, num_cycles: 4, addressing_mode: AddressingMode::Empty }, // F5 (Zero Page, X)
    InstructionType { name: "INC", num_bytes: 2, num_cycles: 6, addressing_mode: AddressingMode::Empty }, // F6 (Zero Page, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // F7
    InstructionType { name: "SED", num_bytes: 1, num_cycles: 2, addressing_mode: AddressingMode::Empty }, // F8
    InstructionType { name: "SBC", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::Empty }, // F9 (Absolute, Y)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // FA
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // FB
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // FC
    InstructionType { name: "SBC", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: AddressingMode::Empty }, // FD (Absolute, X)
    InstructionType { name: "INC", num_bytes: 3, num_cycles: 7, addressing_mode: AddressingMode::Empty }, // FE (Absolute, X)
    InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: AddressingMode::Empty }, // FF
];

#[derive(Clone, Copy)]
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
