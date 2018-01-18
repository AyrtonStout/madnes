use instruction_set::get_instruction;
use cpu_memory::CPUMemory;

static STACK_POINTER_OFFSET: u16 = 0x100;

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
        self.program_counter = self.memory.get_reset_vector();
    }

    pub fn tick(&mut self) {
        let memory_start = self.program_counter;
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
            0x20 => { self.asm_jsr(instruction_data) }
            0x78 => { self.asm_sei(); }
            0x85 => { self.asm_sta_zero_page(instruction_data); }
            0x86 => { self.asm_stx_zero_page(instruction_data); }
            0x8D => { self.asm_sta_absolute(instruction_data); }
            0x8E => { self.asm_stx_absolute(instruction_data); }
            0x91 => { self.asm_sta_post_indexed(instruction_data); }
            0x9A => { self.asm_txs(); }
            0xA0 => { self.asm_ldy_immediate(instruction_data); }
            0xA2 => { self.asm_ldx_immediate(instruction_data); }
            0xA9 => { self.asm_lda_immediate(instruction_data); }
            0xB0 => { self.asm_bcs(instruction_data); }
            0xBD => { self.asm_lda_absolute_x(instruction_data); }
            0xAD => { self.asm_lda_absolute(instruction_data); }
            0xC9 => { self.asm_cmp_immediate(instruction_data); }
            0xCA => { self.asm_dex(); }
            0xD0 => { self.asm_bne(instruction_data); }
            0xD8 => { self.asm_cld(); }
            0xE0 => { self.asm_cpx_immediate(instruction_data); }
            _ => {
                println!("Found unimplemented opcode {:X}", opcode);
            }
        }
    }

    pub fn get_ppu_io_registers_address(&mut self) -> *mut u8 {
        return self.memory.get_ppu_io_registers();
    }

    fn set_carry_bit(&mut self, is_set: bool) {
        if is_set {
            self.status_register |= 0x01;
        } else {
            self.status_register &= !0x01;
        }
    }

    fn set_zero_bit(&mut self, is_set: bool) {
        if is_set {
            self.status_register |= 0x02;
        } else {
            self.status_register &= !0x02;
        }
    }

    fn set_sign_bit(&mut self, result: u8) {
        if (result & 0x80) == 0x80 {
            self.status_register |= 0x80;
        } else {
            self.status_register &= !0x80;
        }
    }

    fn convert_to_address(address_data: &[u8]) -> u16 {
        if address_data.len() == 2 {
            return ((address_data[1] as u16) << 8) | (address_data[0] as u16);
        } else {
            return address_data[0] as u16;
        }
    }

    fn get_post_indexed_indirect_address(&self, zero_page_address: u8) -> u16 {
        let address: u16 = self.memory.get_16_bit_value(zero_page_address as u16);
        return address + self.y_register as u16;
    }

    fn push_stack(&mut self, value_to_write: u8) {
        let stack_address: u16 = self.stack_pointer as u16 + STACK_POINTER_OFFSET;
        self.memory.set_8_bit_value(stack_address, value_to_write);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1); // This tells rust we expect to underflow (if that's a word) and wrap around to 0xFF
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

    pub fn is_carry_set(&self) -> bool {
        return (self.status_register & 0x01) == 0x01;
    }

    pub fn is_zero_set(&self) -> bool {
        return (self.status_register & 0x02) == 0x02;
    }

    fn branch(&mut self, offset: u8) {
        self.program_counter = (self.program_counter as i32 + (offset as i8) as i32) as u16;
    }

    // C9 - Compare literal value with value stored in accumulator
    fn compare(&mut self, cpu_data: u8, src: u8) {
        self.set_carry_bit(cpu_data >= src);
        self.set_zero_bit(cpu_data == src);

        let difference = cpu_data.wrapping_sub(src);
        self.set_sign_bit(difference);
    }

    // 10 - Branches on 'result plus' - the result being a positive number
    fn asm_bpl(&mut self, instruction_data: &[u8]) {
        if self.is_result_negative() { return; }

        self.branch(instruction_data[0]);
    }

    // 20 - Have program start executing from a new address. Store current address on the stack
    fn asm_jsr(&mut self, instruction_data: &[u8]) {
        let return_address = self.program_counter - 1;
        self.push_stack((return_address >> 8) as u8);
        self.push_stack((return_address & 0x00FF) as u8);
        self.program_counter = CPU::convert_to_address(instruction_data);
    }

    // 78 - Sets interrupts as being disabled
    fn asm_sei(&mut self) {
        self.status_register |= 0x04;
    }

    // 85 - Puts the value stored in the accumulator into a specific address in the first page of memory
    fn asm_sta_zero_page(&mut self, instruction_data: &[u8]) {
        self.asm_sta_absolute(&[instruction_data[0], 0x00]);
    }

    // 86 - Puts the accumulator into a specific 2-byte memory address
    fn asm_stx_zero_page(&mut self, instruction_data: &[u8]) {
        self.asm_stx_absolute(&[instruction_data[0], 0x00]);
    }

    // 8D - Puts the accumulator into a specific 2-byte memory address
    fn asm_sta_absolute(&mut self, instruction_data: &[u8]) {
        let address: u16 = CPU::convert_to_address(instruction_data);
        self.memory.set_8_bit_value(address, self.accumulator);
    }

    // 8E - Puts the accumulator into a specific 2-byte memory address
    fn asm_stx_absolute(&mut self, instruction_data: &[u8]) {
        let address: u16 = CPU::convert_to_address(instruction_data);
        self.memory.set_8_bit_value(address, self.x_register);
    }

    // 91 - Puts the accumulator into a post-indexed 2-byte memory address
    fn asm_sta_post_indexed(&mut self, instruction_data: &[u8]) {
        let address: u16 = self.get_post_indexed_indirect_address(instruction_data[0]);
        self.memory.set_8_bit_value(address, self.accumulator);
    }

    // 9A - Copies the X register to the stack and moves the stack pointer
    fn asm_txs(&mut self) {
        let x_register = self.x_register;
        self.push_stack(x_register);
    }

    // A0 - Loads a specific value into the Y register
    fn asm_ldy_immediate(&mut self, instruction_data: &[u8]) {
        self.set_sign_bit(instruction_data[0]);
        self.y_register = instruction_data[0];
    }

    // A2 - Loads a specific value into the X register
    fn asm_ldx_immediate(&mut self, instruction_data: &[u8]) {
        self.set_sign_bit(instruction_data[0]);
        self.x_register = instruction_data[0];
    }

    // A9 - Loads a specific value into the accumulator
    fn asm_lda_immediate(&mut self, instruction_data: &[u8]) {
        self.set_sign_bit(instruction_data[0]);
        self.accumulator = instruction_data[0];
    }

    // AD - Loads a specific value into the accumulator
    fn asm_lda_absolute(&mut self, instruction_data: &[u8]) {
        let address = CPU::convert_to_address(instruction_data);
        let memory_value = self.memory.get_8_bit_value(address);

        self.set_sign_bit(memory_value);
        self.accumulator = memory_value;
    }

    // B0 - Branch when carry is set
    fn asm_bcs(&mut self, instruction_data: &[u8]) {
        if !self.is_carry_set() { return; }

        self.branch(instruction_data[0]);
    }

    // BD - Takes two bytes of data representing an address, then adds (in a signed manner) the value in the x_register
    //      Loads the value stored at this memory location into the accumulator
    fn asm_lda_absolute_x(&mut self, instruction_data: &[u8]) {
        let address = CPU::convert_to_address(instruction_data);
        // Temporarily convert to signed numbers because x_register might be negative
        let x_register = (self.x_register as i8) as i16; // Sign extend the number as a (potential) negative number
        let computed_address = address as i16 + x_register;

        let memory_value = self.memory.get_8_bit_value(computed_address as u16);
        self.set_sign_bit(memory_value);
        self.accumulator = memory_value;
    }

    // C9 - Compare literal value with value stored in accumulator
    fn asm_cmp_immediate(&mut self, instruction_data: &[u8]) {
        let accumulator = self.accumulator;
        self.compare(accumulator, instruction_data[0]);
    }

    // CA - Decrements X register by 1
    fn asm_dex(&mut self) {
        let x_register: u8 = self.x_register.wrapping_sub(1);
        self.set_sign_bit(x_register);
        self.set_zero_bit(x_register == 0);
        self.x_register = x_register;
    }

    // D0 - Branch on result not zero
    fn asm_bne(&mut self, instruction_data: &[u8]) {
        if self.is_zero_set() { return; }

        self.branch(instruction_data[0]);
    }

    // D8 - Sets the operational mode to binary instead of decimal
    fn asm_cld(&mut self) {
        self.status_register &= !0x08;
    }

    // E0 - Compare literal value with value stored in the x register
    fn asm_cpx_immediate(&mut self, instruction_data: &[u8]) {
        let x_register = self.x_register;
        self.compare(x_register, instruction_data[0]);
    }

}

#[cfg(test)]
mod tests {
    use cpu::CPU;

    #[test]
    fn test_branch_positive_offset() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 0x30;
        cpu.branch(0x08);
        assert_eq!(cpu.program_counter, 0x38);
    }

    #[test]
    fn test_branch_negative_offset() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 0x38;
        cpu.branch(0xFA);
        assert_eq!(cpu.program_counter, 0x32);
    }

    #[test]
    fn test_bpl() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 0x20;
        cpu.asm_bpl(&[0x17]);
        assert_eq!(cpu.program_counter, 0x37);
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
    fn test_bcs() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 0x30;
        cpu.status_register = 0x01;
        cpu.asm_bcs(&[0x08]);

        assert_eq!(cpu.program_counter, 0x38);
    }

    #[test]
    fn test_bcs_negative_condition() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 0x38;
        cpu.asm_bcs(&[0xFA]);

        assert_eq!(cpu.program_counter, 0x38);
    }

    #[test]
    fn test_bne() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 0x30;
        cpu.asm_bne(&[0x08]);

        assert_eq!(cpu.program_counter, 0x38);
    }

    #[test]
    fn test_bne_negative_condition() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 0x30;
        cpu.status_register = 0x02;
        cpu.asm_bne(&[0x08]);

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
        assert_eq!(cpu.is_result_negative(), false);

        cpu.asm_lda_immediate(&[0xA2]);
        assert_eq!(cpu.accumulator, 0xA2);
        assert_eq!(cpu.is_result_negative(), true);
    }

    #[test]
    fn test_lda_absolute() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.set_8_bit_value(0x0271, 0xB4);
        cpu.asm_lda_absolute(&[0x71, 0x02]);
        assert_eq!(cpu.accumulator, 0xB4);
        assert_eq!(cpu.is_result_negative(), true);

        cpu.memory.set_8_bit_value(0x0272, 0x04);
        cpu.asm_lda_absolute(&[0x72, 0x02]);
        assert_eq!(cpu.accumulator, 0x04);
        assert_eq!(cpu.is_result_negative(), false);
    }

    #[test]
    fn test_lda_absolute_x() {
        let mut cpu: CPU = CPU::new();
        cpu.x_register = 0x38;
        cpu.memory.set_8_bit_value(0x616B, 0x50);

        cpu.asm_lda_absolute_x(&[0x33, 0x61]);
        assert_eq!(cpu.accumulator, 0x50);
        assert_eq!(cpu.is_result_negative(), false);

        cpu.x_register = 0xFE;
        cpu.memory.set_8_bit_value(0x6131, 0xB2);

        cpu.asm_lda_absolute_x(&[0x33, 0x61]);
        assert_eq!(cpu.accumulator, 0xB2);
        assert_eq!(cpu.is_result_negative(), true);
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
    fn test_sta_zero_page() {
        let mut cpu: CPU = CPU::new();

        cpu.accumulator = 0x42;
        cpu.asm_sta_zero_page(&[0x22]);

        let actual: u8 = cpu.memory.get_8_bit_value(0x0022);
        assert_eq!(0x42, actual);
    }

    #[test]
    fn test_sta_post_indexed() {
        let mut cpu: CPU = CPU::new();

        cpu.memory.set_16_bit_value(0x004C, 0x2100);
        cpu.y_register = 0x05;
        cpu.accumulator = 0x15;

        cpu.asm_sta_post_indexed(&[0x4C]);

        let actual: u8 = cpu.memory.get_8_bit_value(0x2105);
        assert_eq!(0x15, actual);
    }

    #[test]
    fn test_stx_absolute() {
        let mut cpu: CPU = CPU::new();

        cpu.x_register = 0x42;
        cpu.asm_stx_absolute(&[0x22, 0x10]);

        let actual: u8 = cpu.memory.get_8_bit_value(0x1022);
        assert_eq!(0x42, actual);
    }

    #[test]
    fn test_stx_zero_page() {
        let mut cpu: CPU = CPU::new();

        cpu.x_register = 0x42;
        cpu.asm_stx_zero_page(&[0x22]);

        let actual: u8 = cpu.memory.get_8_bit_value(0x0022);
        assert_eq!(0x42, actual);
    }

    #[test]
    fn test_ldy_immediate() {
        let mut cpu: CPU = CPU::new();
        cpu.asm_ldy_immediate(&[0x52]);

        assert_eq!(0x52, cpu.y_register);
        assert_eq!(cpu.is_result_negative(), false);

        cpu.asm_ldy_immediate(&[0x98]);
        assert_eq!(0x98, cpu.y_register);
        assert_eq!(cpu.is_result_negative(), true);
    }

    #[test]
    fn test_ldx_immediate() {
        let mut cpu: CPU = CPU::new();
        cpu.asm_ldx_immediate(&[0x52]);

        assert_eq!(0x52, cpu.x_register);
        assert_eq!(cpu.is_result_negative(), false);

        cpu.asm_ldx_immediate(&[0x98]);
        assert_eq!(0x98, cpu.x_register);
        assert_eq!(cpu.is_result_negative(), true);
    }

    #[test]
    fn test_compare() {
        let mut cpu: CPU = CPU::new();
        let cpu_data = 0x30;
        cpu.compare(cpu_data,0x20);

        assert_eq!(cpu.is_carry_set(), true);
        assert_eq!(cpu.is_result_negative(), false);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.compare(cpu_data, 0x30);
        assert_eq!(cpu.is_carry_set(), true);
        assert_eq!(cpu.is_result_negative(), false);
        assert_eq!(cpu.is_zero_set(), true);

        cpu.compare(cpu_data, 0x94);
        assert_eq!(cpu.is_carry_set(), false);
        assert_eq!(cpu.is_result_negative(), true);
        assert_eq!(cpu.is_zero_set(), false);
    }

    #[test]
    fn test_cmp_immediate() { // Most compare stuff is tested in the generic compare function
        let mut cpu: CPU = CPU::new();
        cpu.accumulator = 0x30;
        cpu.asm_cmp_immediate(&[0x20]);

        assert_eq!(cpu.is_carry_set(), true);
        assert_eq!(cpu.is_result_negative(), false);
        assert_eq!(cpu.is_zero_set(), false);
    }

    #[test]
    fn test_cpx_immediate() { // Most compare stuff is tested in the generic compare function
        let mut cpu: CPU = CPU::new();
        cpu.x_register = 0x30;
        cpu.asm_cpx_immediate(&[0x20]);

        assert_eq!(cpu.is_carry_set(), true);
        assert_eq!(cpu.is_result_negative(), false);
        assert_eq!(cpu.is_zero_set(), false);
    }

    #[test]
    fn stack_pointer_initialized_correctly() {
        let cpu: CPU = CPU::new();
        assert_eq!(0xFF, cpu.stack_pointer);
    }

    #[test]
    fn test_txs() {
        let mut cpu: CPU = CPU::new();

        cpu.x_register = 0x14;
        cpu.asm_txs();

        cpu.x_register = 0x24;
        cpu.asm_txs();

        assert_eq!(cpu.memory.get_8_bit_value(0x01FF), 0x14);
        assert_eq!(cpu.memory.get_8_bit_value(0x01FE), 0x24);
    }

    #[test]
    fn test_dex() {
        let mut cpu: CPU = CPU::new();
        cpu.x_register = 0x02;
        cpu.asm_dex();
        assert_eq!(cpu.x_register, 0x01);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_result_negative(), false);

        cpu.asm_dex();
        assert_eq!(cpu.x_register, 0x00);
        assert_eq!(cpu.is_zero_set(), true);
        assert_eq!(cpu.is_result_negative(), false);

        cpu.asm_dex();
        assert_eq!(cpu.x_register, 0xFF);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_result_negative(), true);
    }

    #[test]
    fn test_jsr() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 0x8054;
        cpu.asm_jsr(&[0x35, 0x90]);

        assert_eq!(cpu.program_counter, 0x9035);
        assert_eq!(cpu.stack_pointer, 0xFD);
        assert_eq!(cpu.memory.get_8_bit_value(0x1FF), 0x80);
        assert_eq!(cpu.memory.get_8_bit_value(0x1FE), 0x53);
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
        let mut prg_rom: Vec<u8> = vec![0 as u8; 0x8000];
        prg_rom[0x0000] = 0x78; // Instruction data
        prg_rom[0x0001] = 0xD8;
        prg_rom[0x0002] = 0xA9;
        prg_rom[0x0003] = 0x10;

        prg_rom[0x7FFC] = 0x00; // Reset vector. Initializes program counter to 8000
        prg_rom[0x7FFD] = 0x80;

        let mut cpu: CPU = CPU::new();
        cpu.init_prg_rom(prg_rom);
        cpu.tick(); // Executes 0x78
        cpu.tick(); // Executes 0xD8
        cpu.tick(); // Executes 0xA9 [0x10]

        assert_eq!(cpu.are_interrupts_disabled(), true);
        assert_eq!(cpu.is_in_decimal_mode(), false);
        assert_eq!(cpu.accumulator, 0x10);
        assert_eq!(cpu.program_counter, 0x8004);
    }
}