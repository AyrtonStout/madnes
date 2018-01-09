use instruction_set::get_instruction;
use cpu_memory::CPUMemory;

static STACK_POINTER_OFFSET: u16 = 0x100;
static PRG_ROM_OFFSET: u16 = 0x8000;

pub struct CPU {
    program_counter: u16,
    stack_pointer: u8,
    status_register: u8,
    accumulator: u8,
    x_register: u8,
    y_register: u8,
    memory: CPUMemory
}

impl CPU {
    pub fn new() -> CPU {
        return CPU {
            program_counter: 0,
            stack_pointer: 0xFF, // This will grow downward (decrement) down to 0. Then it wraps around back to 0xFF
            status_register: 0,
            accumulator: 0,
            x_register: 0,
            y_register: 0,
            memory: CPUMemory::new()
        }
    }

    pub fn init_prg_rom(&mut self, prg_rom: Vec<u8>) {
        self.memory.init_prg_rom(prg_rom);
    }

    pub fn tick(&mut self) {
        let memory_start = self.program_counter + PRG_ROM_OFFSET;
        let opcode: u8 = self.memory.get_8_bit_value(memory_start);
        let num_bytes: u8 = get_instruction(opcode).num_bytes;

        let instruction_data: Vec<u8> = self.memory.get_memory_range(memory_start + 1, num_bytes as u16 - 1);

        // Some instructions (like BPL) seem to indicate that the program counter is incremented prior to the instruction's action
        self.program_counter += num_bytes as u16;

        self.handle_instruction(opcode, instruction_data.as_slice());
    }

    fn handle_instruction(&mut self, opcode: u8, instruction_data: &[u8]) {
        match opcode {
            0x10 => { self.asm_bpl(instruction_data) }
            0x78 => { self.asm_sei(); }
            0x8D => { self.asm_sta_absolute(instruction_data); }
            0x9A => { self.asm_txs(); }
            0xA2 => { self.asm_ldx_immediate(instruction_data); }
            0xA9 => { self.asm_lda_immediate(instruction_data); }
            0xAD => { self.asm_lda_absolute(instruction_data); }
            0xD8 => { self.asm_cld(); }
            _ => {
                println!("Found unimplemented opcode {:X}", opcode);
            }
        }
    }

    pub fn get_ppu_io_registers_address(&mut self) -> *mut u8 {
        return self.memory.get_ppu_io_registers();
    }

    fn convert_to_address(address_data: &[u8]) -> u16 {
        if address_data.len() == 2 {
            return ((address_data[1] as u16) << 8) | (address_data[0] as u16);
        } else {
            return address_data[0] as u16;
        }
    }

    #[allow(dead_code)]
    pub fn are_interrupts_disabled(&self) -> bool {
        return (self.status_register & 0x04) == 0x04;
    }

    // Read more about decimal mode here http://6502.org/tutorials/decimal_mode.html
    #[allow(dead_code)]
    pub fn is_in_decimal_mode(&self) -> bool {
        return (self.status_register & 0x08) == 0x08;
    }

    pub fn is_result_negative(&self) -> bool {
        return (self.status_register & 0x80) == 0x80;
    }

    // 10 - Branches on 'result plus' - the result being a positive number
    fn asm_bpl(&mut self, instruction_data: &[u8]) {
        if self.is_result_negative() { return; }

        let offset: i8 = instruction_data[0] as i8;
        self.program_counter = (self.program_counter as i32 + offset as i32) as u16;
    }

    // 78 - Sets interrupts as being disabled
    fn asm_sei(&mut self) {
        self.status_register |= 0x04;
    }

    // 8D - Puts the accumulator into a specific 2-byte memory address
    fn asm_sta_absolute(&mut self, instruction_data: &[u8]) {
        let address: u16 = CPU::convert_to_address(instruction_data);

        self.memory.set_8_bit_value(address, self.accumulator);
    }

    // 9A - Copies the X register to the stack and moves the stack pointer
    fn asm_txs(&mut self) {
        let stack_address: u16 = self.stack_pointer as u16 + STACK_POINTER_OFFSET;
        self.memory.set_8_bit_value(stack_address, self.x_register);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1); // This tells rust we expect to underflow (if that's a word) and wrap around to 0xFF
    }

    // A2 - Loads a specific value into the X register
    fn asm_ldx_immediate(&mut self, instruction_data: &[u8]) {
        self.x_register = instruction_data[0];
    }

    // A9 - Loads a specific value into the accumulator
    fn asm_lda_immediate(&mut self, instruction_data: &[u8]) {
        self.accumulator = instruction_data[0];
    }

    // AD - Loads a specific value into the accumulator
    fn asm_lda_absolute(&mut self, instruction_data: &[u8]) {
        let address = CPU::convert_to_address(instruction_data);
        self.accumulator = self.memory.get_8_bit_value(address);
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
        cpu.asm_bpl(&[0xF9]);
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

    #[test]
    fn test_lda_absolute() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.set_8_bit_value(0x0271, 0xB4);
        cpu.asm_lda_absolute(&[0x71, 0x02]);
        assert_eq!(cpu.accumulator, 0xB4);
    }

    #[test]
    fn test_sta_absolute() {
        let mut cpu: CPU = CPU::new();

        cpu.accumulator = 0x42;
        cpu.asm_sta_absolute(&[0x22, 0x10]);

        let actual: u8 = cpu.memory.get_8_bit_value(0x1022);
        assert_eq!(0x42, actual);
    }

    #[test]
    fn test_ldx_immediate() {
        let mut cpu: CPU = CPU::new();
        cpu.asm_ldx_immediate(&[0x52]);
        assert_eq!(0x52, cpu.x_register);
    }

    #[test]
    fn stack_pointer_initialized_correctly() {
        let cpu: CPU = CPU::new();
        assert_eq!(0xFF, cpu.stack_pointer);
    }

    #[test]
    fn test_asm_txs() {
        let mut cpu: CPU = CPU::new();

        cpu.x_register = 0x14;
        cpu.asm_txs();

        cpu.x_register = 0x24;
        cpu.asm_txs();

        assert_eq!(cpu.memory.get_8_bit_value(0x01FF), 0x14);
        assert_eq!(cpu.memory.get_8_bit_value(0x01FE), 0x24);
    }

    #[test]
    fn test_stack_pointer_decrement_rollover() {
        let mut cpu: CPU = CPU::new();

        cpu.stack_pointer = 0x00;
        cpu.x_register = 0x14;
        cpu.asm_txs(); // This will move the stack pointer below 0, and wrap back around to 0xFF

        assert_eq!(cpu.memory.get_8_bit_value(0x0100), 0x14);
        assert_eq!(cpu.stack_pointer, 0xFF);
    }

    #[test]
    fn instruction_chaining() {
        // These are the first few instructions of Super Mario Bros 1.
        // More of an integration test. Tests for stuff like program counter increments

        let mut cpu: CPU = CPU::new();
        cpu.init_prg_rom(vec![0x78, 0xD8, 0xA9, 0x10]);
        cpu.tick(); // Executes 0x78
        cpu.tick(); // Executes 0xD8
        cpu.tick(); // Executes 0xA9 [0x10]

        assert_eq!(cpu.are_interrupts_disabled(), true);
        assert_eq!(cpu.is_in_decimal_mode(), false);
        assert_eq!(cpu.accumulator, 0x10);
        assert_eq!(cpu.program_counter, 4);
    }
}