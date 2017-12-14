mod instruction_set {
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
