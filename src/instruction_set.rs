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
        InstructionType { name: "ORA", num_bytes: 3, num_cycles: 4/* * */ }, // 19 (Absolute, Y)

    ];

    #[derive(Copy, Clone)]
    pub struct InstructionType {
        pub name: &'static str,
        pub num_bytes: u8,
        pub num_cycles: u8
    }

    pub fn get_instruction(opcode: u8) -> InstructionType {
        let found_instruction = INSTRUCTIONS[opcode as usize];

        if opcode as usize > INSTRUCTIONS.len() {
            // NOTE: this can be removed once all 255 opcodes have entries, as the opcode is a u8 and can't ever be larger than 255
            panic!(format!("Attempted to access an invalid op code {}!", opcode))
        }
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
