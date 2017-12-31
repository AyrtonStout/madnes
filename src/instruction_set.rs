pub mod instruction_set {

    #[derive(Clone, Copy)]
    pub enum AddressingMode {
        Immediate,
        Absolute,
        ZeroPageAbsolute,
        Implied,
        Accumulator,
        Indexed,
        ZeroPageIndexed,
        Indirect,
        PreIndexedIndirect,
        PostIndexedIndirect,
        Relative
    }

    static INSTRUCTIONS: &'static [InstructionType] = &[
        InstructionType { name: "BRK", num_bytes: 1, num_cycles: 7, addressing_mode: None }, // 0
        InstructionType { name: "ORA", num_bytes: 2, num_cycles: 6, addressing_mode: None }, // 1 (Indirect, X)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 2
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 3
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 4
        InstructionType { name: "ORA", num_bytes: 2, num_cycles: 3, addressing_mode: None }, // 5 (Zero page)
        InstructionType { name: "ASL", num_bytes: 2, num_cycles: 5, addressing_mode: None }, // 6 (Zero page)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 7 [Unofficial]
        InstructionType { name: "PHP", num_bytes: 1, num_cycles: 3, addressing_mode: None }, // 8
        InstructionType { name: "ORA", num_bytes: 2, num_cycles: 2, addressing_mode: None }, // 9 (Immediate)
        InstructionType { name: "ASL", num_bytes: 1, num_cycles: 2, addressing_mode: None }, // A (Accumulator)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // B (Immediate)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // C (Immediate)
        InstructionType { name: "ORA", num_bytes: 3, num_cycles: 4, addressing_mode: None }, // D (Absolute)
        InstructionType { name: "ASL", num_bytes: 3, num_cycles: 6, addressing_mode: None }, // E (Absolute)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // F (Immediate)

        InstructionType { name: "BPL", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: None }, // 10
        InstructionType { name: "ORA", num_bytes: 2, num_cycles: 5, addressing_mode: None }, // 11 (Indirect, Y)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 12
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 13
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 14
        InstructionType { name: "ORA", num_bytes: 2, num_cycles: 4, addressing_mode: None }, // 15 (Zero Page, X)
        InstructionType { name: "ASL", num_bytes: 2, num_cycles: 6, addressing_mode: None }, // 16 (Zero Page, X)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 17
        InstructionType { name: "CLC", num_bytes: 1, num_cycles: 2, addressing_mode: None }, // 18
        InstructionType { name: "ORA", num_bytes: 3, num_cycles: 4, addressing_mode: None }, // 19 (Absolute, Y)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 1A
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 1B
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 1C
        InstructionType { name: "ORA", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: None }, // 1D (Absolute, X)
        InstructionType { name: "ASL", num_bytes: 3, num_cycles: 7, addressing_mode: None }, // 1E (Absolute, X)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 1F

        InstructionType { name: "JSR", num_bytes: 3, num_cycles: 6, addressing_mode: None }, // 20
        InstructionType { name: "AND", num_bytes: 2, num_cycles: 6, addressing_mode: None }, // 21 (Indirect, X)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 22
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 23
        InstructionType { name: "BIT", num_bytes: 2, num_cycles: 3, addressing_mode: None }, // 24 (Zero page)
        InstructionType { name: "AND", num_bytes: 2, num_cycles: 3, addressing_mode: None }, // 25 (Zero page)
        InstructionType { name: "ROL", num_bytes: 2, num_cycles: 5, addressing_mode: None }, // 26 (Zero page)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 27
        InstructionType { name: "PLP", num_bytes: 1, num_cycles: 4, addressing_mode: None }, // 28
        InstructionType { name: "AND", num_bytes: 2, num_cycles: 2, addressing_mode: None }, // 29 (Intermediate)
        InstructionType { name: "ROL", num_bytes: 1, num_cycles: 2, addressing_mode: None }, // 2A (Accumulator)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 2B
        InstructionType { name: "BIT", num_bytes: 3, num_cycles: 4, addressing_mode: None }, // 2C (Absolute)
        InstructionType { name: "AND", num_bytes: 3, num_cycles: 4, addressing_mode: None }, // 2D (Absolute)
        InstructionType { name: "ROL", num_bytes: 3, num_cycles: 6, addressing_mode: None }, // 2E (Absolute)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 2F

        InstructionType { name: "BMI", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: None }, // 30
        InstructionType { name: "AND", num_bytes: 2, num_cycles: 5, addressing_mode: None }, // 31 (Indirect, Y)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 32
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 33
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 34
        InstructionType { name: "AND", num_bytes: 2, num_cycles: 4, addressing_mode: None }, // 35 (Zero Page, X)
        InstructionType { name: "ROL", num_bytes: 2, num_cycles: 6, addressing_mode: None }, // 36 (Zero Page, X)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 37
        InstructionType { name: "SEC", num_bytes: 1, num_cycles: 2, addressing_mode: None }, // 38
        InstructionType { name: "AND", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: None }, // 39 (Absolute, Y)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 3A
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 3B
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 3C
        InstructionType { name: "AND", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: None }, // 3D (Absolute, X)
        InstructionType { name: "ROL", num_bytes: 3, num_cycles: 7, addressing_mode: None }, // 3E (Absolute, X)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 3F

        InstructionType { name: "RTI", num_bytes: 3, num_cycles: 4, addressing_mode: None }, // 40
        InstructionType { name: "EOR", num_bytes: 2, num_cycles: 6, addressing_mode: None }, // 41 (Indirect, X)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 42
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 43
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 44
        InstructionType { name: "EOR", num_bytes: 2, num_cycles: 3, addressing_mode: None }, // 45 (Zero Page)
        InstructionType { name: "LSR", num_bytes: 2, num_cycles: 5, addressing_mode: None }, // 46 (Zero Page)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 47
        InstructionType { name: "PHA", num_bytes: 1, num_cycles: 3, addressing_mode: None }, // 48
        InstructionType { name: "EOR", num_bytes: 2, num_cycles: 2, addressing_mode: None }, // 49 Immediate
        InstructionType { name: "LSR", num_bytes: 1, num_cycles: 2, addressing_mode: None }, // 4A Accumulator
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 4B
        InstructionType { name: "JMP", num_bytes: 3, num_cycles: 3, addressing_mode: None }, // 4C (Absolute)
        InstructionType { name: "EOR", num_bytes: 1, num_cycles: 6, addressing_mode: None }, // 4D (Absolute)
        InstructionType { name: "LSR", num_bytes: 3, num_cycles: 6, addressing_mode: None }, // 4E (Absolute)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 4F

        InstructionType { name: "BVC", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: None }, // 50
        InstructionType { name: "EOR", num_bytes: 2, num_cycles: 5/* * */, addressing_mode: None }, // 51 (Indirect, Y)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 52
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 53
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 54
        InstructionType { name: "EOR", num_bytes: 2, num_cycles: 4, addressing_mode: None }, // 55 (Zero Page, X)
        InstructionType { name: "LSR", num_bytes: 2, num_cycles: 6, addressing_mode: None }, // 56 (Zero Page, X)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 57
        InstructionType { name: "CLI", num_bytes: 1, num_cycles: 2, addressing_mode: None }, // 58
        InstructionType { name: "EOR", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: None }, // 59 (Absolute, X)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 5A
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 5B
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 5C
        InstructionType { name: "EOR", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: None }, // 5D (Absolute, X)
        InstructionType { name: "LSR", num_bytes: 3, num_cycles: 7, addressing_mode: None }, // 5E (Absolute, X)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 5F

        InstructionType { name: "RTS", num_bytes: 1, num_cycles: 6, addressing_mode: None }, // 60
        InstructionType { name: "ADC", num_bytes: 2, num_cycles: 6, addressing_mode: None }, // 61 (Indirect, X)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 62
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 63
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 64
        InstructionType { name: "ADC", num_bytes: 2, num_cycles: 3, addressing_mode: None }, // 65 (Zero Page)
        InstructionType { name: "ROR", num_bytes: 2, num_cycles: 5, addressing_mode: None }, // 66 (Zero Page)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 67
        InstructionType { name: "PLA", num_bytes: 1, num_cycles: 4, addressing_mode: None }, // 68
        InstructionType { name: "ADC", num_bytes: 2, num_cycles: 2, addressing_mode: None }, // 69 (Immediate)
        InstructionType { name: "ROR", num_bytes: 1, num_cycles: 2, addressing_mode: None }, // 6A (Accumulator)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 6B
        InstructionType { name: "JMP", num_bytes: 3, num_cycles: 5, addressing_mode: None }, // 6C (Indirect)
        InstructionType { name: "ADC", num_bytes: 3, num_cycles: 4, addressing_mode: None }, // 6D (Absolute)
        InstructionType { name: "ROR", num_bytes: 3, num_cycles: 6, addressing_mode: None }, // 6E (Absolute)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 6F

        InstructionType { name: "BVS", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: None }, // 70
        InstructionType { name: "ADC", num_bytes: 2, num_cycles: 5/* * */, addressing_mode: None }, // 71 (Indirect, Y)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 72
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 73
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 74
        InstructionType { name: "ADC", num_bytes: 2, num_cycles: 4, addressing_mode: None }, // 75 (Zero Page, X)
        InstructionType { name: "ROR", num_bytes: 2, num_cycles: 6, addressing_mode: None }, // 76 (Zero Page, X)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 77
        InstructionType { name: "SEI", num_bytes: 1, num_cycles: 2, addressing_mode: None }, // 78
        InstructionType { name: "ADC", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: None }, // 79 (Absolute, Y)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 7A
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 7B
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 7C
        InstructionType { name: "ADC", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: None }, // 7D (Absolute, X)
        InstructionType { name: "ROR", num_bytes: 3, num_cycles: 7, addressing_mode: None }, // 7E (Absolute, X)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 7F

        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 80
        InstructionType { name: "STA", num_bytes: 2, num_cycles: 6, addressing_mode: None }, // 81 (Indirect, X)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 82
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 83
        InstructionType { name: "STY", num_bytes: 2, num_cycles: 3, addressing_mode: None }, // 84 (Zero Page)
        InstructionType { name: "STA", num_bytes: 2, num_cycles: 3, addressing_mode: None }, // 85 (Zero Page)
        InstructionType { name: "STX", num_bytes: 2, num_cycles: 3, addressing_mode: None }, // 86 (Zero Page)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 87
        InstructionType { name: "DEY", num_bytes: 1, num_cycles: 2, addressing_mode: None }, // 88
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 89
        InstructionType { name: "TXA", num_bytes: 1, num_cycles: 2, addressing_mode: None }, // 8A
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 8B
        InstructionType { name: "STY", num_bytes: 3, num_cycles: 4, addressing_mode: None }, // 8C (Absolute)
        InstructionType { name: "STA", num_bytes: 3, num_cycles: 4, addressing_mode: None }, // 8D (Absolute)
        InstructionType { name: "STX", num_bytes: 3, num_cycles: 4, addressing_mode: None }, // 8E (Absolute)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 8F

        InstructionType { name: "BCC", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: None }, // 90
        InstructionType { name: "STA", num_bytes: 2, num_cycles: 6, addressing_mode: None }, // 91 (Indirect, Y)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 92
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 93
        InstructionType { name: "STY", num_bytes: 2, num_cycles: 4, addressing_mode: None }, // 94 (Zero Page, X)
        InstructionType { name: "STA", num_bytes: 2, num_cycles: 4, addressing_mode: None }, // 95 (Zero Page, X)
        InstructionType { name: "STX", num_bytes: 2, num_cycles: 4, addressing_mode: None }, // 96 (Zero Page, X)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 97
        InstructionType { name: "TYA", num_bytes: 1, num_cycles: 2, addressing_mode: None }, // 98
        InstructionType { name: "STA", num_bytes: 3, num_cycles: 5, addressing_mode: None }, // 99 (Absolute, Y)
        InstructionType { name: "TXS", num_bytes: 1, num_cycles: 2, addressing_mode: None }, // 9A
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 9B
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 9C
        InstructionType { name: "STA", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: None }, // 9D (Absolute, X)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 9E
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // 9F

        InstructionType { name: "LDY", num_bytes: 2, num_cycles: 2, addressing_mode: None }, // A0 (Immediate)
        InstructionType { name: "LDA", num_bytes: 2, num_cycles: 6, addressing_mode: None }, // A1 (Indirect, X)
        InstructionType { name: "LDX", num_bytes: 2, num_cycles: 2, addressing_mode: None }, // A2 (Immediate)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // A3
        InstructionType { name: "LDY", num_bytes: 2, num_cycles: 3, addressing_mode: None }, // A4 (Zero Page)
        InstructionType { name: "LDA", num_bytes: 2, num_cycles: 3, addressing_mode: None }, // A5 (Zero Page)
        InstructionType { name: "LDX", num_bytes: 2, num_cycles: 3, addressing_mode: None }, // A6 (Zero Page)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // A7
        InstructionType { name: "TAY", num_bytes: 1, num_cycles: 2, addressing_mode: None }, // A8
        InstructionType { name: "LDA", num_bytes: 2, num_cycles: 2, addressing_mode: None }, // A9 (Immediate)
        InstructionType { name: "TAX", num_bytes: 1, num_cycles: 2, addressing_mode: None }, // AA
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // AB
        InstructionType { name: "LDY", num_bytes: 3, num_cycles: 4, addressing_mode: None }, // AC (Absolute)
        InstructionType { name: "LDA", num_bytes: 3, num_cycles: 4, addressing_mode: None }, // AD (Absolute)
        InstructionType { name: "LDX", num_bytes: 3, num_cycles: 4, addressing_mode: None }, // AE (Absolute)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // AF

        InstructionType { name: "BCS", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: None }, // B0
        InstructionType { name: "LDA", num_bytes: 2, num_cycles: 5/* * */, addressing_mode: None }, // B1 (Indirect, Y)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // B2
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // B3
        InstructionType { name: "LDY", num_bytes: 2, num_cycles: 4, addressing_mode: None }, // B4 (Zero Page, X)
        InstructionType { name: "LDA", num_bytes: 2, num_cycles: 4, addressing_mode: None }, // B5 (Zero Page, X)
        InstructionType { name: "LDX", num_bytes: 2, num_cycles: 4, addressing_mode: None }, // B6 (Zero Page, Y)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // B7
        InstructionType { name: "CLV", num_bytes: 1, num_cycles: 2, addressing_mode: None }, // B8
        InstructionType { name: "LDA", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: None }, // B9 (Absolute, Y)
        InstructionType { name: "TSX", num_bytes: 2, num_cycles: 5/* * */, addressing_mode: None }, // BA
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // BB
        InstructionType { name: "LDY", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: None }, // BC (Absolute, X)
        InstructionType { name: "LDA", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: None }, // BD (Absolute, X)
        InstructionType { name: "LDX", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: None }, // BE (Absolute, Y)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // BF

        InstructionType { name: "CPY", num_bytes: 2, num_cycles: 2, addressing_mode: None }, // C0 (Immediate)
        InstructionType { name: "CMP", num_bytes: 2, num_cycles: 6, addressing_mode: None }, // C1 (Indirect, X)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // C2
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // C3
        InstructionType { name: "CPY", num_bytes: 2, num_cycles: 3, addressing_mode: None }, // C4 (Zero Page)
        InstructionType { name: "CMP", num_bytes: 2, num_cycles: 3, addressing_mode: None }, // C5 (Zero Page)
        InstructionType { name: "DEC", num_bytes: 2, num_cycles: 5, addressing_mode: None }, // C6 (Zero Page)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // C7
        InstructionType { name: "INY", num_bytes: 1, num_cycles: 2, addressing_mode: None }, // C8
        InstructionType { name: "CMP", num_bytes: 2, num_cycles: 2, addressing_mode: None }, // C9 (Immediate)
        InstructionType { name: "DEC", num_bytes: 1, num_cycles: 2, addressing_mode: None }, // CA
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // CB
        InstructionType { name: "CPY", num_bytes: 3, num_cycles: 4, addressing_mode: None }, // CC (Absolute)
        InstructionType { name: "CMP", num_bytes: 3, num_cycles: 4, addressing_mode: None }, // CD (Absolute)
        InstructionType { name: "DEC", num_bytes: 3, num_cycles: 6, addressing_mode: None }, // CE (Absolute)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // CF

        InstructionType { name: "BNE", num_bytes: 2, num_cycles: 2/* * */, addressing_mode: None }, // D0
        InstructionType { name: "CMP", num_bytes: 2, num_cycles: 5/* * */, addressing_mode: None }, // D1 (Indirect, @,Y)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // D2
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // D3
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // D4
        InstructionType { name: "CMP", num_bytes: 2, num_cycles: 4, addressing_mode: None }, // D5 (Zero page, X)
        InstructionType { name: "DEC", num_bytes: 2, num_cycles: 6, addressing_mode: None }, // D6 (Zero page, X)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // D7
        InstructionType { name: "CLD", num_bytes: 1, num_cycles: 2, addressing_mode: None }, // D8
        InstructionType { name: "CMP", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: None }, // D9 (Absolute, Y)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // DA
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // DB
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // DC
        InstructionType { name: "CMP", num_bytes: 3, num_cycles: 4/* * */, addressing_mode: None }, // DD (Absolute, X)
        InstructionType { name: "DEC", num_bytes: 3, num_cycles: 7, addressing_mode: None }, // DE (Absolute, X)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // DF

        InstructionType { name: "CPX", num_bytes: 2, num_cycles: 2, addressing_mode: None }, // E0 (Immediate)
        InstructionType { name: "SBC", num_bytes: 2, num_cycles: 6, addressing_mode: None }, // E1 (Indirect, X)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // E2
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // E3
        InstructionType { name: "CPX", num_bytes: 2, num_cycles: 3, addressing_mode: None }, // E4 (Zero Page)
        InstructionType { name: "SBC", num_bytes: 2, num_cycles: 3, addressing_mode: None }, // E5 (Zero Page)
        InstructionType { name: "INC", num_bytes: 2, num_cycles: 5, addressing_mode: None }, // E6 (Zero Page)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // E7
        InstructionType { name: "INX", num_bytes: 1, num_cycles: 2, addressing_mode: None }, // E8
        InstructionType { name: "SBC", num_bytes: 2, num_cycles: 2, addressing_mode: None }, // E9 (Immediate)
        InstructionType { name: "NOP", num_bytes: 1, num_cycles: 2, addressing_mode: None }, // EA
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // EB
        InstructionType { name: "CPX", num_bytes: 3, num_cycles: 4, addressing_mode: None }, // EC (Absolute)
        InstructionType { name: "SBC", num_bytes: 3, num_cycles: 4, addressing_mode: None }, // ED (Absolute)
        InstructionType { name: "INC", num_bytes: 3, num_cycles: 6, addressing_mode: None }, // EE (Absolute)
        InstructionType { name: "---", num_bytes: 0, num_cycles: 0, addressing_mode: None }, // EF
    ];

    #[derive(Clone, Copy)]
    pub struct InstructionType {
        pub name: &'static str,
        pub num_bytes: u8,
        pub num_cycles: u8,
        pub addressing_mode: Option<AddressingMode>
    }

    pub fn get_instruction(opcode: u8) -> InstructionType {
        let found_instruction = INSTRUCTIONS[opcode as usize];

        if found_instruction.num_bytes == 0 {
            panic!(format!("Attempted to access an unimplemented op code {:X}!", opcode))
        }

        return found_instruction;
    }

}

#[cfg(test)]
mod tests {
    use super::instruction_set;

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
