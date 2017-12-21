pub mod instruction_set {
    static INSTRUCTIONS: &'static [InstructionType] = &[
        InstructionType { name: "BRK", num_bytes: 1, num_cycles: 7 }, // 0
        InstructionType { name: "ORA", num_bytes: 2, num_cycles: 6 }, // 1 (Indirect, X)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 2
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 3
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 4
        InstructionType { name: "ORA", num_bytes: 2, num_cycles: 3 }, // 5 (Zero page)
        InstructionType { name: "ASL", num_bytes: 2, num_cycles: 5 }, // 6 (Zero page)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 7
        InstructionType { name: "PHP", num_bytes: 1, num_cycles: 3 }, // 8
        InstructionType { name: "ORA", num_bytes: 2, num_cycles: 2 }, // 9 (Immediate)
        InstructionType { name: "ASL", num_bytes: 1, num_cycles: 2 }, // A (Accumulator)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // B (Immediate)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // C (Immediate)
        InstructionType { name: "ORA", num_bytes: 3, num_cycles: 4 }, // D (Absolute)
        InstructionType { name: "ASL", num_bytes: 3, num_cycles: 6 }, // E (Absolute)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // F (Immediate)

        InstructionType { name: "BPL", num_bytes: 2, num_cycles: 2/* * */ }, // 10
        InstructionType { name: "ORA", num_bytes: 2, num_cycles: 5 }, // 11 (Indirect, Y)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 12
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 13
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 14
        InstructionType { name: "ORA", num_bytes: 2, num_cycles: 4 }, // 15 (Zero Page, X)
        InstructionType { name: "ASL", num_bytes: 2, num_cycles: 6 }, // 16 (Zero Page, X)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 17
        InstructionType { name: "CLC", num_bytes: 1, num_cycles: 2 }, // 18
        InstructionType { name: "ORA", num_bytes: 3, num_cycles: 4 }, // 19 (Absolute, Y)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 1A
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 1B
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 1C
        InstructionType { name: "ORA", num_bytes: 3, num_cycles: 4/* * */ }, // 1D (Absolute, X)
        InstructionType { name: "ASL", num_bytes: 3, num_cycles: 7 }, // 1E (Absolute, X)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 1F

        InstructionType { name: "JSR", num_bytes: 3, num_cycles: 6 }, // 20
        InstructionType { name: "AND", num_bytes: 2, num_cycles: 6 }, // 21 (Indirect, X)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 22
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 23
        InstructionType { name: "BIT", num_bytes: 2, num_cycles: 3 }, // 24 (Zero page)
        InstructionType { name: "AND", num_bytes: 2, num_cycles: 3 }, // 25 (Zero page)
        InstructionType { name: "ROL", num_bytes: 2, num_cycles: 5 }, // 26 (Zero page)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 27
        InstructionType { name: "PLP", num_bytes: 1, num_cycles: 4 }, // 28
        InstructionType { name: "AND", num_bytes: 2, num_cycles: 2 }, // 29 (Intermediate)
        InstructionType { name: "ROL", num_bytes: 1, num_cycles: 2 }, // 2A (Accumulator)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 2B
        InstructionType { name: "BIT", num_bytes: 3, num_cycles: 4 }, // 2C (Absolute)
        InstructionType { name: "AND", num_bytes: 3, num_cycles: 4 }, // 2D (Absolute)
        InstructionType { name: "ROL", num_bytes: 3, num_cycles: 6 }, // 2E (Absolute)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 2F

        InstructionType { name: "BMI", num_bytes: 2, num_cycles: 2/* * */ }, // 30
        InstructionType { name: "AND", num_bytes: 2, num_cycles: 5 }, // 31 (Indirect, Y)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 32
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 33
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 34
        InstructionType { name: "AND", num_bytes: 2, num_cycles: 4 }, // 35 (Zero Page, X)
        InstructionType { name: "ROL", num_bytes: 2, num_cycles: 6 }, // 36 (Zero Page, X)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 37
        InstructionType { name: "SEC", num_bytes: 1, num_cycles: 2 }, // 38
        InstructionType { name: "AND", num_bytes: 3, num_cycles: 4/* * */ }, // 39 (Absolute, Y)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 3A
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 3B
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 3C
        InstructionType { name: "AND", num_bytes: 3, num_cycles: 4/* * */ }, // 3D (Absolute, X)
        InstructionType { name: "ROL", num_bytes: 3, num_cycles: 7 }, // 3E (Absolute, X)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 3F

        InstructionType { name: "RTI", num_bytes: 3, num_cycles: 4 }, // 40
        InstructionType { name: "EOR", num_bytes: 2, num_cycles: 6 }, // 41 (Indirect, X)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 42
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 43
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 44
        InstructionType { name: "EOR", num_bytes: 2, num_cycles: 3 }, // 45 (Zero Page)
        InstructionType { name: "LSR", num_bytes: 2, num_cycles: 5 }, // 46 (Zero Page)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 47
        InstructionType { name: "PHA", num_bytes: 1, num_cycles: 3 }, // 48
        InstructionType { name: "EOR", num_bytes: 2, num_cycles: 2 }, // 49 Immediate
        InstructionType { name: "LSR", num_bytes: 1, num_cycles: 2 }, // 4A Accumulator
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 4B
        InstructionType { name: "JMP", num_bytes: 3, num_cycles: 3 }, // 4C (Absolute)
        InstructionType { name: "EOR", num_bytes: 1, num_cycles: 6 }, // 4D (Absolute)
        InstructionType { name: "LSR", num_bytes: 3, num_cycles: 6 }, // 4E (Absolute)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 4F

        InstructionType { name: "BVC", num_bytes: 2, num_cycles: 2/* * */ }, // 50
        InstructionType { name: "EOR", num_bytes: 2, num_cycles: 5/* * */ }, // 51 (Indirect, Y)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 52
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 53
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 54
        InstructionType { name: "EOR", num_bytes: 2, num_cycles: 4 }, // 55 (Zero Page, X)
        InstructionType { name: "LSR", num_bytes: 2, num_cycles: 6 }, // 56 (Zero Page, X)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 57
        InstructionType { name: "CLI", num_bytes: 1, num_cycles: 2 }, // 58
        InstructionType { name: "EOR", num_bytes: 3, num_cycles: 4/* * */ }, // 59 (Absolute, X)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 5A
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 5B
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 5C
        InstructionType { name: "EOR", num_bytes: 3, num_cycles: 4/* * */ }, // 5D (Absolute, X)
        InstructionType { name: "LSR", num_bytes: 3, num_cycles: 7 }, // 5E (Absolute, X)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 5F

        InstructionType { name: "RTS", num_bytes: 1, num_cycles: 6 }, // 60
        InstructionType { name: "ADC", num_bytes: 2, num_cycles: 6 }, // 61 (Indirect, X)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 62
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 63
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 64
        InstructionType { name: "ADC", num_bytes: 2, num_cycles: 3 }, // 65 (Zero Page)
        InstructionType { name: "ROR", num_bytes: 2, num_cycles: 5 }, // 66 (Zero Page)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 67
        InstructionType { name: "PLA", num_bytes: 1, num_cycles: 4 }, // 68
        InstructionType { name: "ADC", num_bytes: 2, num_cycles: 2 }, // 69 (Immediate)
        InstructionType { name: "ROR", num_bytes: 1, num_cycles: 2 }, // 6A (Accumulator)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 6B
        InstructionType { name: "JMP", num_bytes: 3, num_cycles: 5 }, // 6C (Indirect)
        InstructionType { name: "ADC", num_bytes: 3, num_cycles: 4 }, // 6D (Absolute)
        InstructionType { name: "ROR", num_bytes: 3, num_cycles: 6 }, // 6E (Absolute)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 6F

        InstructionType { name: "BVS", num_bytes: 2, num_cycles: 2/* * */ }, // 70
        InstructionType { name: "ADC", num_bytes: 2, num_cycles: 5/* * */ }, // 71 (Indirect, Y)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 72
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 73
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 74
        InstructionType { name: "ADC", num_bytes: 2, num_cycles: 4 }, // 75 (Zero Page, X)
        InstructionType { name: "ROR", num_bytes: 2, num_cycles: 6 }, // 76 (Zero Page, X)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 77
        InstructionType { name: "SEI", num_bytes: 1, num_cycles: 2 }, // 78
        InstructionType { name: "ADC", num_bytes: 3, num_cycles: 4/* * */ }, // 79 (Absolute, Y)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 7A
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 7B
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 7C
        InstructionType { name: "ADC", num_bytes: 3, num_cycles: 4/* * */ }, // 7D (Absolute, X)
        InstructionType { name: "ROR", num_bytes: 3, num_cycles: 7 }, // 7E (Absolute, X)
        InstructionType { name: "UNDEF", num_bytes: 0, num_cycles: 0 }, // 7F
    ];

    #[derive(Copy, Clone)]
    pub struct InstructionType {
        pub name: &'static str,
        pub num_bytes: u8,
        pub num_cycles: u8
    }

    pub fn get_instruction(opcode: u8) -> InstructionType {
        if opcode as usize > INSTRUCTIONS.len() {
            // NOTE: this can be removed once all 255 opcodes have entries, as the opcode is a u8 and can't ever be larger than 255
            panic!(format!("Attempted to access an invalid op code {:X}!", opcode))
        }

        let found_instruction = INSTRUCTIONS[opcode as usize];

        if found_instruction.num_bytes == 0 {
            panic!(format!("Attempted to access an unimplemented op code {}!", opcode))
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
