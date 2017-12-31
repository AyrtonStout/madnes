use instruction_set::InstructionType;
use instruction_set::get_instruction;

pub struct CPU {
    program_counter: u32,
    status_register: u8
}

impl CPU {
    pub fn new() -> CPU {
        return CPU {
            program_counter: 0,
            status_register: 0
        }
    }

    pub fn read_program_instructions(&mut self, prg_rom: Vec<u8>) {
        while self.program_counter < prg_rom.len() as u32 {
            let opcode = prg_rom[self.program_counter as usize];
            println!("Found opcode {:X} at byte {:X}", opcode, self.program_counter as usize);
            let instruction: InstructionType = get_instruction(opcode);
            println!("Moving {} bytes forward", instruction.num_bytes);
            self.program_counter += instruction.num_bytes as u32;

            match opcode {
                0x78 => {
                    self.handle_opcode_78();
                }
                _ => {
                }
            }
        }
    }

    pub fn interrupts_disabled(&self) -> bool {
        return (self.status_register & 0x04) == 0x04;
    }

    fn handle_opcode_78(&mut self) {
        self.status_register |= 0x04;
    }

}

#[cfg(test)]
mod tests {
    use cpu::CPU;

    #[test]
    fn test_opcode_78() {
        let mut cpu: CPU = CPU::new();
        cpu.handle_opcode_78();
        assert_eq!(cpu.interrupts_disabled(), true);
    }
}