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
            let instruction_type: InstructionType = get_instruction(opcode);
            println!("Moving {} bytes forward", instruction_type.num_bytes);
            let instruction_start_byte = (self.program_counter + 1) as usize;
            let instruction_end_byte = (self.program_counter + instruction_type.num_bytes as u16) as usize;

            // instruction_data is the data after the opcode. Depending on the instruction it might be of length 0, 1, or 2
            let instruction_data: &[u8] = &prg_rom[instruction_start_byte..instruction_end_byte];

            // Some instructions (like BPL) seem to indicate that the program counter is incremented prior to the instruction's action
            self.program_counter += instruction_type.num_bytes as u16;

            match opcode {
                0x10 => { self.asm_bpl(instruction_data) }
                0x78 => { self.asm_sei(); }
                0xA9 => { self.asm_lda_immediate(instruction_data); }
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

    pub fn is_result_negative(&self) -> bool {
        return (self.status_register & 0x80) == 0x80;
    }

    // 10 - Branches on 'result plus' - the result being a positive number
    fn asm_bpl(&mut self, instruction_data: &[u8]) {
        if self.is_result_negative() { return; }

        // The offset is treated as a sign-magnitude number. Not 2s complement
        let is_negative = (instruction_data[0] & 0x80) == 0x80;
        let offset: u8 = instruction_data[0] & !0x80;

        if is_negative {
            self.program_counter -= offset as u16;
        } else {
            self.program_counter += offset as u16;
        }
    }

    // 78 - Sets interrupts as being disabled
    fn asm_sei(&mut self) {
        self.status_register |= 0x04;
    }

    // A9 - Loads a specific value into the accumulator
    fn asm_lda_immediate(&mut self, prg_rom: &[u8]) {
        self.accumulator = prg_rom[0];
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
    fn test_bpl_positive_offset() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 0x20;
        cpu.asm_bpl(&[0x17]);
        assert_eq!(cpu.program_counter, 0x37);
    }

    #[test]
    fn test_bpl_negative_offset() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 0x20;
        cpu.asm_bpl(&[0x87]);
        assert_eq!(cpu.program_counter, 0x19);
    }

    #[test]
    fn test_bpl_false_condition() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 0x30;
        cpu.status_register = 0x80;
        cpu.asm_bpl(&[0x87]);
        assert_eq!(cpu.program_counter, 0x30);
    }

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
        cpu.asm_lda_immediate(&[0x22]);
        assert_eq!(cpu.accumulator, 0x22);
    }
}