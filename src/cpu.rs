use instruction_set::InstructionType;
use instruction_set::get_instruction;

pub struct CPU {
    program_counter: u16,
    status_register: u8,
    accumulator: u8,
    x_register: u8,
    y_register: u8
}

impl CPU {
    pub fn new() -> CPU {
        return CPU {
            program_counter: 0,
            status_register: 0,
            accumulator: 0,
            x_register: 0,
            y_register: 0
        }
    }

    pub fn read_program_instructions(&mut self, prg_rom: Vec<u8>) {
        while self.program_counter < prg_rom.len() as u16 {
            let opcode = prg_rom[self.program_counter as usize];
            println!("Found opcode {:X} at byte {:X}", opcode, self.program_counter as usize);
            let instruction: InstructionType = get_instruction(opcode);
            println!("Moving {} bytes forward", instruction.num_bytes);
            self.program_counter += instruction.num_bytes as u16;

            match opcode {
                0x78 => { self.asm_sei(); }
                0xA9 => { self.asm_lda_immediate(&prg_rom); }
                0xD8 => { self.asm_cld(); }
                _ => {}
            }
        }
    }

    pub fn are_interrupts_disabled(&self) -> bool {
        return (self.status_register & 0x04) == 0x04;
    }

    // Read more about decimal mode here http://6502.org/tutorials/decimal_mode.html
    pub fn is_in_decimal_mode(&self) -> bool {
        return (self.status_register & 0x08) == 0x08;
    }

    // 78 - Sets interrupts as being disabled
    fn asm_sei(&mut self) {
        self.status_register |= 0x04;
    }

    // A9 - Loads a specific value into the accumulator
    fn asm_lda_immediate(&mut self, prg_rom: &[u8]) {
        self.accumulator = prg_rom[(self.program_counter + 1) as usize];
    }

    // D8 - Sets the operational mode to binary instead of decimal
    fn asm_cld(&mut self) {
        self.status_register &= !0x08;
    }

}

#[cfg(test)]
mod tests {
    use cpu::CPU;

    #[test]
    fn test_sei() {
        let mut cpu: CPU = CPU::new();
        cpu.asm_sei();
        assert_eq!(cpu.are_interrupts_disabled(), true);
    }

    #[test]
    fn test_cld() {
        let mut cpu: CPU = CPU::new();
        cpu.asm_cld();
        assert_eq!(cpu.is_in_decimal_mode(), false);
    }

    #[test]
    fn setting_cpu_status_flags_does_not_affect_others() {
        let mut cpu: CPU = CPU::new();

        cpu.asm_sei();
        cpu.asm_cld();

        assert_eq!(cpu.are_interrupts_disabled(), true);
        assert_eq!(cpu.is_in_decimal_mode(), false);

        cpu.asm_sei();

        assert_eq!(cpu.are_interrupts_disabled(), true);
        assert_eq!(cpu.is_in_decimal_mode(), false);
    }

    #[test]
    fn test_lda_immediate() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 1;
        cpu.asm_lda_immediate(&[0x00, 0xA9, 0x22, 0x00]);
        assert_eq!(cpu.accumulator, 0x22);
    }
}