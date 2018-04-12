use instruction_set::get_instruction;
use cpu_memory::CPUMemory;
use instruction_set::AddressingMode;
use instruction_set::InstructionType;

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
        self.handle_nmi();

        let memory_start = self.program_counter;
        let opcode: u8 = self.memory.get_8_bit_value(memory_start);
        let instruction = get_instruction(opcode);
        let num_bytes: u8 = instruction.num_bytes;

        let instruction_data: Vec<u8> = self.memory.get_memory_range(memory_start + 1, num_bytes as u16 - 1);

        // Some instructions (like BPL) seem to indicate that the program counter is incremented prior to the instruction's action
        self.program_counter += num_bytes as u16;

        self.handle_instruction(opcode, instruction, instruction_data.as_slice());
    }

    // NOTE: There is some tomfoolery possible here. A thing called 'Interrupt Hijacking'. Might have to implement
    fn handle_nmi(&mut self) {
        if self.memory.are_nmis_enabled() {
            if self.memory.read_ppu_for_nmi() {
                println!("NMI detected for CPU!");
                let program_counter = self.program_counter;
                let status_register = self.status_register;
                self.push_stack_16(program_counter);
                self.push_stack(status_register);
                self.asm_sei(); // Disable interrupts
                self.program_counter = self.memory.get_16_bit_value(0xFFFA);
            }
        }
    }

    fn get_source_address(&mut self, instruction: InstructionType, instruction_data: &[u8]) -> u16 {
        match instruction.addressing_mode {
            AddressingMode::Immediate => panic!("Makes no sense! {}", instruction.name),
            AddressingMode::Absolute => CPU::convert_to_address(instruction_data),
            AddressingMode::ZeroPageAbsolute => CPU::convert_to_address(&[instruction_data[0], 0x00]),
            AddressingMode::Implied => panic!("There is no data for implied instructions!"),
            AddressingMode::Accumulator => panic!("There is no data for accumulator instructions!"),
            AddressingMode::AbsoluteX => self.compute_absolute_x_address(instruction_data),
            AddressingMode::AbsoluteY => self.compute_absolute_y_address(instruction_data),
            AddressingMode::ZeroPageAbsoluteX => self.compute_absolute_x_address(&[instruction_data[0], 0x00]),
            AddressingMode::ZeroPageAbsoluteY => self.compute_absolute_y_address(&[instruction_data[0], 0x00]),
            AddressingMode::PreIndexedIndirect => self.get_pre_indexed_indirect_address(instruction_data[0]),
            AddressingMode::PostIndexedIndirect => self.get_post_indexed_indirect_address(instruction_data[0]),
            AddressingMode::Relative => panic!("Also makes no sense!"),
            AddressingMode::Indirect => panic!("Not done!"),
            AddressingMode::Empty => panic!("AddressingMode not set for {}!", instruction.name)
        }
    }

    fn handle_instruction(&mut self, opcode: u8, instruction: InstructionType, instruction_data: &[u8]) {
        if instruction.addressing_mode == AddressingMode::Implied {
            match instruction.name.as_ref() {
                "CLC" => self.asm_clc(),
                "SEC" => self.asm_sec(),
                "PHA" => self.asm_pha(),
                "RTI" => self.asm_rti(),
                "RTS" => self.asm_rts(),
                "PLA" => self.asm_pla(),
                "SEI" => self.asm_sei(),
                "DEY" => self.asm_dey(),
                "TXA" => self.asm_txa(),
                "TXS" => self.asm_txs(),
                "TAX" => self.asm_tax(),
                "INY" => self.asm_iny(),
                "DEX" => self.asm_dex(),
                "CLD" => self.asm_cld(),
                "INX" => self.asm_inx(),
                _ => println!("Implied instruction {} not implemented!", instruction.name)
            }
            return;
        }

        if instruction.addressing_mode == AddressingMode::Accumulator {
            match instruction.name.as_ref() {
                "LSR" => self.asm_lsr_accumulator(),
                "ROL" => self.asm_rol_accumulator(),
                _ => println!("Accumulator instruction {} not implemented!", instruction.name)
            }
            return;
        }

        // TODO handle Immediate and Relative (which is basically Immediate) more gracefully
        let mut source_address = 0;
        if instruction.addressing_mode != AddressingMode::Immediate
            && instruction.addressing_mode != AddressingMode::Relative {
            source_address = self.get_source_address(instruction, instruction_data);
            match instruction.name.as_ref() {
                "STA" => { self.sta(source_address); return; },
                "STY" => { self.asm_sty(source_address); return; },
                "STX" => { self.asm_stx(source_address); return; },
                "JSR" => { self.asm_jsr(source_address); return; },
                "JMP" => { self.asm_jmp(source_address); return; },
                "INC" => { self.inc(source_address); return; },
                "DEC" => { self.dec(source_address); return; },
                _ => ()
            }
        }

        let source_value;
        // Instruction we read in wasn't in the previous groups. So we want the address's value
        if instruction.addressing_mode == AddressingMode::Immediate
            || instruction.addressing_mode == AddressingMode::Relative {
            source_value = instruction_data[0];
        } else {
            source_value = self.memory.get_8_bit_value(source_address);
        }
        match instruction.name.as_ref() {
            "SBC" => { self.asm_sbc(source_value); },
            "LDA" => { self.asm_lda(source_value); },
            "LDX" => { self.asm_ldx(source_value); },
            "LDY" => { self.asm_ldy(source_value); },
            "CMP" => { self.asm_cmp(source_value); },
            "CPX" => { self.asm_cpx(source_value); },
            "CPY" => { self.asm_cpy(source_value); },
            "BPL" => { self.asm_bpl(source_value); },
            "BCS" => { self.asm_bcs(source_value); },
            "BNE" => { self.asm_bne(source_value); },
            "BEQ" => { self.asm_beq(source_value); },
            "BCC" => { self.asm_bcc(source_value); },
            "ORA" => { self.asm_ora(source_value); },
            "AND" => { self.asm_and(source_value); },
            "BIT" => { self.asm_bit(source_value); },
            "EOR" => { self.asm_eor(source_value); },
            _ => println!("Found unimplemented instruction! Name: {} Opcode: {}", instruction.name, opcode)
        }

        /*
        let instruction_name = get_instruction(opcode).name;
        if instruction_data.is_empty() {
            print!("DEBUG - Opcode: {} ({:X})", instruction_name, opcode);
        } else if instruction_data.len() == 1 {
            print!("DEBUG - Opcode: {} ({:X})  Data: ({:X})", instruction_name, opcode, instruction_data[0]);
        } else {
            print!("DEBUG - Opcode: {} ({:X})  Data: ({:X} {:X})", instruction_name, opcode, instruction_data[0], instruction_data[1]);
        }
        println!("  Program Counter: {} ({:X})", self.program_counter, self.program_counter);
        */
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

    fn set_sign(&mut self, result: u8) {
        if (result & 0x80) == 0x80 {
            self.status_register |= 0x80;
        } else {
            self.status_register &= !0x80;
        }
    }

    fn set_overflow_bit(&mut self, is_set: bool) {
        if is_set {
            self.status_register |= 0x40;
        } else {
            self.status_register &= !0x40;
        }
    }

    fn set_zero(&mut self, result: u8) {
        if result == 0 {
            self.status_register |= 0x02;
        } else {
            self.status_register &= !0x02;
        }
    }

    fn convert_to_address(address_data: &[u8]) -> u16 {
        if address_data.len() == 2 {
            return ((address_data[1] as u16) << 8) | (address_data[0] as u16);
        } else {
            return address_data[0] as u16;
        }
    }

    fn get_pre_indexed_indirect_address(&self, zero_page_address: u8) -> u16 {
        let address = zero_page_address + self.x_register;
        return self.memory.get_16_bit_value(address as u16);
    }

    fn get_post_indexed_indirect_address(&self, zero_page_address: u8) -> u16 {
        let address: u16 = self.memory.get_16_bit_value(zero_page_address as u16);
        return address + self.y_register as u16;
    }

    fn compute_absolute_y_address(&mut self, instruction_data: &[u8]) -> u16 {
        let address = CPU::convert_to_address(instruction_data);
        // Temporarily convert to signed numbers because y_register might be negative
        let y_register = (self.y_register as i8) as i16; // Sign extend the number as a (potential) negative number
        return (address as i16 + y_register) as u16;
    }

    fn compute_absolute_x_address(&mut self, instruction_data: &[u8]) -> u16 {
        let address = CPU::convert_to_address(instruction_data);
        // Temporarily convert to signed numbers because x_register might be negative
        let x_register = (self.x_register as i8) as i16; // Sign extend the number as a (potential) negative number
        return (address as i16 + x_register) as u16;
    }

    fn push_stack(&mut self, value_to_write: u8) {
        let stack_address: u16 = self.stack_pointer as u16 + STACK_POINTER_OFFSET;
        self.memory.set_8_bit_value(stack_address, value_to_write);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1); // This tells rust we expect to underflow (if that's a word) and wrap around to 0xFF
    }

    fn push_stack_16(&mut self, value_to_write: u16) {
        self.push_stack(value_to_write as u8);
        self.push_stack((value_to_write >> 8) as u8);
    }

    fn pull_stack(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let stack_address: u16 = self.stack_pointer as u16 + STACK_POINTER_OFFSET;
        return self.memory.get_8_bit_value(stack_address);
    }

    fn pull_stack_16(&mut self) -> u16 {
        let high_byte: u16 = (self.pull_stack() as u16) << 8;
        let low_bye: u16 = self.pull_stack() as u16;
        return high_byte | low_bye;
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

    pub fn is_negative_set(&self) -> bool {
        return (self.status_register & 0x80) == 0x80;
    }

    pub fn is_carry_set(&self) -> bool {
        return (self.status_register & 0x01) == 0x01;
    }

    pub fn is_zero_set(&self) -> bool {
        return (self.status_register & 0x02) == 0x02;
    }

    pub fn is_overflow_set(&self) -> bool {
        return (self.status_register & 0x40) == 0x40;
    }

    // Change the program counter by an offset
    fn branch(&mut self, offset: u8) {
        self.program_counter = (self.program_counter as i32 + (offset as i8) as i32) as u16;
    }

    // Compare a value with value stored in the CPU
    fn compare(&mut self, cpu_data: u8, src: u8) {
        self.set_carry_bit(cpu_data >= src);
        self.set_zero_bit(cpu_data == src);

        let difference = cpu_data.wrapping_sub(src);
        self.set_sign(difference);
    }

    fn asm_ora(&mut self, source: u8) {
        let result = source | self.accumulator;
        self.set_sign(result);
        self.set_zero(result);
        self.accumulator = result;
    }

    // Store the accumulator at a memory location
    fn sta(&mut self, source: u16) {
        self.memory.set_8_bit_value(source, self.accumulator);
    }

    fn asm_and(&mut self, source: u8) {
        let result = source & self.accumulator;
        self.set_sign(result);
        self.set_zero(result);
        self.accumulator = result;
    }

    // Load a value into the accumulator
    fn asm_lda(&mut self, source: u8) {
        self.set_sign(source);
        self.set_zero(source);
        self.accumulator = source;
    }

    // Load a value into the Y register
    fn asm_ldy(&mut self, source: u8) {
        self.set_sign(source);
        self.set_zero(source);
        self.y_register = source;
    }

    // Load a value into the X register
    fn asm_ldx(&mut self, source: u8) {
        self.set_sign(source);
        self.set_zero(source);
        self.x_register = source;
    }

    // Increment the value stored at a memory location
    fn inc(&mut self, address: u16) {
        let memory_value = self.memory.get_8_bit_value(address);
        let new_memory_value = memory_value.wrapping_add(1);
        self.memory.set_8_bit_value(address, new_memory_value);
        self.set_sign(new_memory_value);
        self.set_zero(new_memory_value);
    }

    // Subtraction (with carry)
    // http://www.6502.org/tutorials/vflag.html#2.4
    fn asm_sbc(&mut self, source: u8) {
        let carry: u16 = if self.is_carry_set() { 0 } else { 1 };
        let accumulator: u16 = self.accumulator as u16;
        let temp: u16 = accumulator.wrapping_sub(source as u16 + carry);

        self.set_sign(temp as u8);
        self.set_zero(temp as u8);
        self.set_overflow_bit(((accumulator ^ temp) & 0x80) > 0
            && ((accumulator ^ source as u16) & 0x80) > 0);
        self.set_carry_bit(temp < 0x100);

        self.accumulator = temp as u8;
    }

    // Branches on 'result plus' - the result being a positive number
    fn asm_bpl(&mut self, source: u8) {
        if self.is_negative_set() { return; }

        self.branch(source);
    }

    // Clears the carry flag so that it is not set
    fn asm_clc(&mut self) {
        self.set_carry_bit(false);
    }

    // Have program start executing from a new address. Store current address on the stack
    fn asm_jsr(&mut self, source: u16) {
        let return_address = self.program_counter - 1;
        self.push_stack((return_address >> 8) as u8);
        self.push_stack((return_address & 0x00FF) as u8);
        self.program_counter = source;
    }

    // Bitshift accumulator to the left by 1
    fn asm_rol_accumulator(&mut self) {
        let accumulator = self.accumulator;
        let shifted_accumulator = self.accumulator << 1;
        let final_accumulator = if self.is_carry_set() { shifted_accumulator | 0x01 } else { shifted_accumulator }; // Why Rust no have ternary
        self.set_zero(final_accumulator);
        self.set_sign(final_accumulator);
        self.set_carry_bit((accumulator & 0x80) == 0x80);
        self.accumulator = final_accumulator;
    }

    // Sets various flags based off the current accumulator and memory address
    fn asm_bit(&mut self, source: u8) {
        let accumulator = self.accumulator;
        self.set_sign(source);
        self.set_overflow_bit((source & 0x40) == 0x40);
        self.set_zero(source & accumulator);
    }

    // Exclusive OR with a memory location and the accumulator
    fn asm_eor(&mut self, source: u8) {
        let result = self.accumulator ^ source;
        self.set_sign(result);
        self.set_zero(result);
        self.accumulator = result;
    }

    // Sets carry flag as being set
    fn asm_sec(&mut self) {
        self.set_carry_bit(true);
    }

    // Push accumulator onto the stack
    fn asm_pha(&mut self) {
        let accumulator = self.accumulator;
        self.push_stack(accumulator);
    }

    // Bitshift accumulator to the right by 1
    fn asm_lsr_accumulator(&mut self) {
        let accumulator = self.accumulator;
        let shifted_accumulator = self.accumulator >> 1;
        let final_accumulator = if self.is_carry_set() { shifted_accumulator | 0x80 } else { shifted_accumulator }; // Why Rust no have ternary
        self.set_zero(final_accumulator);
        self.set_sign(final_accumulator);
        self.set_carry_bit((accumulator & 0x01) == 0x01);
        self.accumulator = final_accumulator;
    }

    // Start program execution at a value stored at a location in memory
    fn asm_jmp(&mut self, address: u16) {
        self.program_counter = address;
    }

    // Return the program from an interrupt routine
    fn asm_rti(&mut self) {
        self.status_register = self.pull_stack();
        self.program_counter = self.pull_stack_16();
    }

    // Have program return to the instruction it last jumped from
    fn asm_rts(&mut self) {
        let lower_byte: u8 = self.pull_stack();
        let upper_byte: u8 = self.pull_stack();
        self.program_counter = CPU::convert_to_address(&[lower_byte, upper_byte]) + 1;
    }

    // Pull accumulator from the stack
    fn asm_pla(&mut self) {
        let accumulator = self.pull_stack();
        self.accumulator = accumulator;
    }

    // Sets interrupts as being disabled
    fn asm_sei(&mut self) {
        self.status_register |= 0x04;
    }

    // Decrements Y register by 1
    fn asm_dey(&mut self) {
        let y_register: u8 = self.y_register.wrapping_sub(1);
        self.set_sign(y_register);
        self.set_zero_bit(y_register == 0);
        self.y_register = y_register;
    }

    // Puts the X register into the accumulator
    fn asm_txa(&mut self) {
        let x_register = self.x_register;
        self.set_sign(x_register);
        self.set_zero(x_register);
        self.accumulator = x_register;
    }

    fn asm_stx(&mut self, address: u16) {
        self.memory.set_8_bit_value(address, self.x_register);
    }

    fn asm_sty(&mut self, address: u16) {
        self.memory.set_8_bit_value(address, self.y_register);
    }

    // Branches on 'carry clear' - the carry bit being 0 / not set
    fn asm_bcc(&mut self, source: u8) {
        if self.is_carry_set() { return; }

        self.branch(source);
    }

    // Copies the X register to the stack and moves the stack pointer
    fn asm_txs(&mut self) {
        let x_register = self.x_register;
        self.push_stack(x_register);
    }

    // Transfers the accumulator into index X
    fn asm_tax(&mut self) {
        let accumulator = self.accumulator;
        self.set_sign(accumulator);
        self.set_zero(accumulator);
        self.x_register = accumulator;
    }

    // Branch when carry is set
    fn asm_bcs(&mut self, source: u8) {
        if !self.is_carry_set() { return; }

        self.branch(source);
    }

    // Compare a value with value stored in the y register and set various flags
    fn asm_cpy(&mut self, source: u8) {
        let y_register = self.y_register;
        self.compare(y_register, source);
    }

    // Increments Y register by 1
    fn asm_iny(&mut self) {
        let y_register: u8 = self.y_register.wrapping_add(1);
        self.set_sign(y_register);
        self.set_zero_bit(y_register == 0);
        self.y_register = y_register;
    }

    // Compare a value with value stored in the accumulator and set various flags
    fn asm_cmp(&mut self, source: u8) {
        let accumulator = self.accumulator;
        self.compare(accumulator, source);
    }

    // Decrements X register by 1
    fn asm_dex(&mut self) {
        let x_register: u8 = self.x_register.wrapping_sub(1);
        self.set_sign(x_register);
        self.set_zero_bit(x_register == 0);
        self.x_register = x_register;
    }

    // Decrements the value at a memory location by 1
    fn dec(&mut self, address: u16) {
        let memory_value: u8 = self.memory.get_8_bit_value(address);
        let new_memory_value = memory_value.wrapping_sub(1);
        self.memory.set_8_bit_value(address, new_memory_value);
        self.set_sign(new_memory_value);
        self.set_zero(new_memory_value);
    }

    // Branch on result not zero
    fn asm_bne(&mut self, source: u8) {
        if self.is_zero_set() { return; }

        self.branch(source);
    }

    // Sets the operational mode to binary instead of decimal
    fn asm_cld(&mut self) {
        self.status_register &= !0x08;
    }

    // Compare literal value with value stored in the x register
    fn asm_cpx(&mut self, source: u8) {
        let x_register = self.x_register;
        self.compare(x_register, source);
    }

    // Increments X register by 1
    fn asm_inx(&mut self) {
        let x_register: u8 = self.x_register.wrapping_add(1);
        self.set_sign(x_register);
        self.set_zero_bit(x_register == 0);
        self.x_register = x_register;
    }

    // Branches on 'result zero' - the last result having been zero
    fn asm_beq(&mut self, source: u8) {
        if !self.is_zero_set() { return; }

        self.branch(source);
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
        cpu.asm_bpl(0x17);
        assert_eq!(cpu.program_counter, 0x37);
    }

    #[test]
    fn test_bpl_false_condition() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 0x30;
        cpu.status_register = 0x80;
        cpu.asm_bpl(0x87);
        assert_eq!(cpu.program_counter, 0x30);
    }

    #[test]
    fn test_beq() {
        let mut cpu: CPU = CPU::new();
        cpu.set_zero_bit(true);
        cpu.program_counter = 0x20;
        cpu.asm_beq(0x17);
        assert_eq!(cpu.program_counter, 0x37);
    }

    #[test]
    fn test_beq_false_condition() {
        let mut cpu: CPU = CPU::new();
        cpu.set_zero_bit(false);
        cpu.program_counter = 0x20;
        cpu.asm_beq(0x17);
        assert_eq!(cpu.program_counter, 0x20);
    }

    #[test]
    fn test_bcs() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 0x30;
        cpu.set_carry_bit(true);
        cpu.asm_bcs(0x08);

        assert_eq!(cpu.program_counter, 0x38);
    }

    #[test]
    fn test_bcs_negative_condition() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 0x38;
        cpu.set_carry_bit(false);
        cpu.asm_bcs(0xFA);

        assert_eq!(cpu.program_counter, 0x38);
    }

    #[test]
    fn test_bcc() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 0x30;
        cpu.set_carry_bit(false);
        cpu.asm_bcc(0x08);

        assert_eq!(cpu.program_counter, 0x38);
    }

    #[test]
    fn test_bcc_negative_condition() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 0x38;
        cpu.set_carry_bit(true);
        cpu.asm_bcc(0xFA);

        assert_eq!(cpu.program_counter, 0x38);
    }

    #[test]
    fn test_bne() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 0x30;
        cpu.asm_bne(0x08);

        assert_eq!(cpu.program_counter, 0x38);
    }

    #[test]
    fn test_bne_negative_condition() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 0x30;
        cpu.status_register = 0x02;
        cpu.asm_bne(0x08);

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
    fn test_lda() {
        let mut cpu: CPU = CPU::new();

        cpu.asm_lda(0x22);
        assert_eq!(cpu.accumulator, 0x22);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.asm_lda(0x83);
        assert_eq!(cpu.accumulator, 0x83);
        assert_eq!(cpu.is_negative_set(), true);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.asm_lda(0x00);
        assert_eq!(cpu.accumulator, 0x00);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), true);
    }

    #[test]
    fn test_ldy() {
        let mut cpu: CPU = CPU::new();

        cpu.asm_ldy(0x22);
        assert_eq!(cpu.y_register, 0x22);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.asm_ldy(0x83);
        assert_eq!(cpu.y_register, 0x83);
        assert_eq!(cpu.is_negative_set(), true);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.asm_ldy(0x00);
        assert_eq!(cpu.y_register, 0x00);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), true);
    }

    #[test]
    fn test_ldx() {
        let mut cpu: CPU = CPU::new();

        cpu.asm_ldx(0x22);
        assert_eq!(cpu.x_register, 0x22);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.asm_ldx(0x83);
        assert_eq!(cpu.x_register, 0x83);
        assert_eq!(cpu.is_negative_set(), true);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.asm_ldx(0x00);
        assert_eq!(cpu.x_register, 0x00);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), true);
    }

    #[test]
    fn test_sta() {
        let mut cpu: CPU = CPU::new();

        cpu.accumulator = 0x42;
        cpu.sta(0x1022);

        let actual: u8 = cpu.memory.get_8_bit_value(0x1022);
        assert_eq!(0x42, actual);
    }

    #[test]
    fn test_stx() {
        let mut cpu: CPU = CPU::new();

        cpu.x_register = 0x42;
        cpu.asm_stx(0x1022);

        let actual: u8 = cpu.memory.get_8_bit_value(0x1022);
        assert_eq!(0x42, actual);
    }

    #[test]
    fn test_compare() {
        let mut cpu: CPU = CPU::new();
        let cpu_data = 0x30;
        cpu.compare(cpu_data,0x20);

        assert_eq!(cpu.is_carry_set(), true);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.compare(cpu_data, 0x30);
        assert_eq!(cpu.is_carry_set(), true);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), true);

        cpu.compare(cpu_data, 0x94);
        assert_eq!(cpu.is_carry_set(), false);
        assert_eq!(cpu.is_negative_set(), true);
        assert_eq!(cpu.is_zero_set(), false);
    }

    #[test]
    fn test_cmp() { // Most compare stuff is tested in the generic compare function
        let mut cpu: CPU = CPU::new();
        cpu.accumulator = 0x30;
        cpu.asm_cmp(0x20);

        assert_eq!(cpu.is_carry_set(), true);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), false);
    }

    #[test]
    fn test_cpx() { // Most compare stuff is tested in the generic compare function
        let mut cpu: CPU = CPU::new();
        cpu.x_register = 0x30;
        cpu.asm_cpx(0x20);

        assert_eq!(cpu.is_carry_set(), true);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), false);
    }

    #[test]
    fn test_cpy() { // Most compare stuff is tested in the generic compare function
        let mut cpu: CPU = CPU::new();
        cpu.y_register = 0x30;
        cpu.asm_cpy(0x20);

        assert_eq!(cpu.is_carry_set(), true);
        assert_eq!(cpu.is_negative_set(), false);
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
        assert_eq!(cpu.is_negative_set(), false);

        cpu.asm_dex();
        assert_eq!(cpu.x_register, 0x00);
        assert_eq!(cpu.is_zero_set(), true);
        assert_eq!(cpu.is_negative_set(), false);

        cpu.asm_dex();
        assert_eq!(cpu.x_register, 0xFF);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), true);
    }

    #[test]
    fn test_dey() {
        let mut cpu: CPU = CPU::new();
        cpu.y_register = 0x02;
        cpu.asm_dey();
        assert_eq!(cpu.y_register, 0x01);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), false);

        cpu.asm_dey();
        assert_eq!(cpu.y_register, 0x00);
        assert_eq!(cpu.is_zero_set(), true);
        assert_eq!(cpu.is_negative_set(), false);

        cpu.asm_dey();
        assert_eq!(cpu.y_register, 0xFF);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), true);
    }

    #[test]
    fn test_iny() {
        let mut cpu: CPU = CPU::new();
        cpu.y_register = 0x02;
        cpu.asm_iny();
        assert_eq!(cpu.y_register, 0x03);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), false);

        cpu.y_register = 0xFF;
        cpu.asm_iny();
        assert_eq!(cpu.y_register, 0x00);
        assert_eq!(cpu.is_zero_set(), true);
        assert_eq!(cpu.is_negative_set(), false);

        cpu.y_register = 0x7F;
        cpu.asm_iny();
        assert_eq!(cpu.y_register, 0x80);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), true);
    }

    #[test]
    fn test_inx() {
        let mut cpu: CPU = CPU::new();
        cpu.x_register = 0x02;
        cpu.asm_inx();
        assert_eq!(cpu.x_register, 0x03);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), false);

        cpu.x_register = 0xFF;
        cpu.asm_inx();
        assert_eq!(cpu.x_register, 0x00);
        assert_eq!(cpu.is_zero_set(), true);
        assert_eq!(cpu.is_negative_set(), false);

        cpu.x_register = 0x7F;
        cpu.asm_inx();
        assert_eq!(cpu.x_register, 0x80);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), true);
    }

    #[test]
    fn test_inc() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.set_8_bit_value(0x1020, 0x50);
        cpu.inc(0x1020);
        assert_eq!(cpu.memory.get_8_bit_value(0x1020), 0x51);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), false);

        cpu.memory.set_8_bit_value(0x1020, 0x7F);
        cpu.inc(0x1020);
        assert_eq!(cpu.memory.get_8_bit_value(0x1020), 0x80);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), true);

        cpu.memory.set_8_bit_value(0x1020, 0xFF);
        cpu.inc(0x1020);
        assert_eq!(cpu.memory.get_8_bit_value(0x1020), 0x00);
        assert_eq!(cpu.is_zero_set(), true);
        assert_eq!(cpu.is_negative_set(), false);
    }

    #[test]
    fn test_dec() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.set_8_bit_value(0x1020, 0x50);
        cpu.dec(0x1020);
        assert_eq!(cpu.memory.get_8_bit_value(0x1020), 0x4F);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), false);

        cpu.memory.set_8_bit_value(0x1020, 0x81);
        cpu.dec(0x1020);
        assert_eq!(cpu.memory.get_8_bit_value(0x1020), 0x80);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), true);

        cpu.memory.set_8_bit_value(0x1020, 0x01);
        cpu.dec(0x1020);
        assert_eq!(cpu.memory.get_8_bit_value(0x1020), 0x00);
        assert_eq!(cpu.is_zero_set(), true);
        assert_eq!(cpu.is_negative_set(), false);
    }

    #[test]
    fn test_jsr() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 0x8054;
        cpu.asm_jsr(0x9035);

        assert_eq!(cpu.program_counter, 0x9035);
        assert_eq!(cpu.stack_pointer, 0xFD);
        assert_eq!(cpu.memory.get_8_bit_value(0x1FF), 0x80);
        assert_eq!(cpu.memory.get_8_bit_value(0x1FE), 0x53);
    }

    #[test]
    fn test_jmp() {
        let mut cpu: CPU = CPU::new();
        cpu.asm_jmp(0x2050);

        assert_eq!(cpu.program_counter, 0x2050);
    }

    #[test]
    fn test_txa() {
        let mut cpu: CPU = CPU::new();
        cpu.x_register = 0x21;
        cpu.asm_txa();

        assert_eq!(cpu.accumulator, 0x21);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), false);

        cpu.x_register = 0x91;
        cpu.asm_txa();
        assert_eq!(cpu.is_negative_set(), true);

        cpu.x_register = 0x00;
        cpu.asm_txa();
        assert_eq!(cpu.is_zero_set(), true);
    }

    #[test]
    fn test_tax() {
        let mut cpu: CPU = CPU::new();
        cpu.accumulator = 0x21;
        cpu.asm_tax();

        assert_eq!(cpu.x_register, 0x21);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), false);

        cpu.accumulator = 0x91;
        cpu.asm_tax();
        assert_eq!(cpu.is_negative_set(), true);

        cpu.accumulator = 0x00;
        cpu.asm_tax();
        assert_eq!(cpu.is_zero_set(), true);
    }

    #[test]
    fn test_rts() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 0x8054;
        cpu.asm_jsr(0x9035);
        cpu.asm_rts();

        assert_eq!(cpu.program_counter, 0x8054);
        assert_eq!(cpu.stack_pointer, 0xFF);
    }

    #[test]
    fn test_pha() {
        let mut cpu: CPU = CPU::new();
        cpu.accumulator = 0x42;
        cpu.asm_pha();

        let accumulator_in_stack = cpu.memory.get_8_bit_value(0x01FF);

        assert_eq!(accumulator_in_stack, cpu.accumulator);
    }

    #[test]
    fn test_pla() {
        let mut cpu: CPU = CPU::new();
        cpu.push_stack(0x56);
        cpu.asm_pla();

        assert_eq!(cpu.accumulator, 0x56);
    }

    #[test]
    fn test_lsr_accumulator() {
        let mut cpu: CPU = CPU::new();
        cpu.accumulator = 0b10010001;
        cpu.asm_lsr_accumulator();

        assert_eq!(cpu.accumulator, 0b01001000);
        assert_eq!(cpu.is_carry_set(), true);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.accumulator = 0b01000000;
        cpu.set_carry_bit(true);
        cpu.asm_lsr_accumulator();

        assert_eq!(cpu.accumulator, 0b10100000);
        assert_eq!(cpu.is_carry_set(), false);
        assert_eq!(cpu.is_negative_set(), true);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.accumulator = 0b00000001;
        cpu.set_carry_bit(false);
        cpu.asm_lsr_accumulator();

        assert_eq!(cpu.accumulator, 0b00000000);
        assert_eq!(cpu.is_carry_set(), true);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), true);
    }

    #[test]
    fn test_rol_accumulator() {
        let mut cpu: CPU = CPU::new();
        cpu.accumulator = 0b10010001;
        cpu.asm_rol_accumulator();

        assert_eq!(cpu.accumulator, 0b00100010);
        assert_eq!(cpu.is_carry_set(), true);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.accumulator = 0b01000001;
        cpu.set_carry_bit(true);
        cpu.asm_rol_accumulator();

        assert_eq!(cpu.accumulator, 0b10000011);
        assert_eq!(cpu.is_carry_set(), false);
        assert_eq!(cpu.is_negative_set(), true);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.accumulator = 0b00000000;
        cpu.set_carry_bit(false);
        cpu.asm_rol_accumulator();

        assert_eq!(cpu.accumulator, 0b00000000);
        assert_eq!(cpu.is_carry_set(), false);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), true);
    }

    #[test]
    fn test_bit() {
        let mut cpu: CPU = CPU::new();

        cpu.accumulator = 0x20;
        cpu.asm_bit(0x6A);

        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_overflow_set(), true);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.accumulator = 0x60;
        cpu.asm_bit(0x9F);

        assert_eq!(cpu.is_negative_set(), true);
        assert_eq!(cpu.is_overflow_set(), false);
        assert_eq!(cpu.is_zero_set(), true);
    }

    #[test]
    fn test_eor() {
        let mut cpu: CPU = CPU::new();

        cpu.accumulator = 0x42;
        cpu.asm_eor(0x22);
        assert_eq!(cpu.accumulator, 0x60);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.accumulator = 0x80;
        cpu.asm_eor(0x02);
        assert_eq!(cpu.accumulator, 0x82);
        assert_eq!(cpu.is_negative_set(), true);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.accumulator = 0x80;
        cpu.asm_eor(0x80);
        assert_eq!(cpu.accumulator, 0x00);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), true);
    }

    #[test]
    fn test_ora() {
        let mut cpu: CPU = CPU::new();

        cpu.accumulator = 0x22;
        cpu.asm_ora(0x11);
        assert_eq!(cpu.accumulator, 0x33);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.accumulator = 0x00;
        cpu.asm_ora(0x00);
        assert_eq!(cpu.accumulator, 0x00);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), true);

        cpu.accumulator = 0x12;
        cpu.asm_ora(0x80);
        assert_eq!(cpu.accumulator, 0x92);
        assert_eq!(cpu.is_negative_set(), true);
        assert_eq!(cpu.is_zero_set(), false);
    }

    #[test]
    fn test_and() {
        let mut cpu: CPU = CPU::new();

        cpu.accumulator = 0x23;
        cpu.asm_and(0x25);
        assert_eq!(cpu.accumulator, 0x21);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.accumulator = 0x22;
        cpu.asm_and(0x11);
        assert_eq!(cpu.accumulator, 0x00);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), true);

        cpu.accumulator = 0x89;
        cpu.asm_and(0x81);
        assert_eq!(cpu.accumulator, 0x81);
        assert_eq!(cpu.is_negative_set(), true);
        assert_eq!(cpu.is_zero_set(), false);
    }

    #[test]
    fn test_sbc() {
        let mut cpu: CPU = CPU::new();

        cpu.accumulator = 0x02;
        cpu.set_carry_bit(false);
        cpu.asm_sbc(0x01);

        assert_eq!(cpu.accumulator, 0x00);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), true);
        assert_eq!(cpu.is_carry_set(), true);
        assert_eq!(cpu.is_overflow_set(), false);

        cpu.accumulator = 0x01;
        cpu.set_carry_bit(true);
        cpu.asm_sbc(0x02);

        assert_eq!(cpu.accumulator, 0xFF);
        assert_eq!(cpu.is_negative_set(), true);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_carry_set(), false);
        assert_eq!(cpu.is_overflow_set(), false);

        cpu.accumulator = 0x80;
        cpu.set_carry_bit(true);
        cpu.asm_sbc(0x01);

        assert_eq!(cpu.accumulator, 0x7F);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_carry_set(), true);
        assert_eq!(cpu.is_overflow_set(), true);
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

    #[test]
    fn nmi_routine() {
        let mut prg_rom: Vec<u8> = vec![0 as u8; 0x8000];

        // Normal instruction data
        prg_rom[0x0000] = 0xE8; // INX
        prg_rom[0x0001] = 0x38; // SEC (Set carry flag. This should persist after NMI)
        prg_rom[0x0002] = 0xE8; // INX

         // NMI Routine
        prg_rom[0x1000] = 0xC8; // INY
        prg_rom[0x1001] = 0x18; // CLC (Clear carry flag. This should get undone after RTI)
        prg_rom[0x1002] = 0x40; // RTI

        prg_rom[0x7FFC] = 0x00; // Reset vector. Initializes program counter to 8000
        prg_rom[0x7FFD] = 0x80;

        prg_rom[0x7FFA] = 0x00; // Interrupt vector. Initializes program counter to 9000
        prg_rom[0x7FFB] = 0x90;

        let mut cpu: CPU = CPU::new();
        cpu.init_prg_rom(prg_rom);
        cpu.tick(); // Executes INX
        cpu.tick(); // Executes SEC

        // Enable NMIs and set one as having happened
        let mut ppu_ctrl_register = cpu.memory.get_8_bit_value(0x2000);
        ppu_ctrl_register |= 0x80;
        cpu.memory.set_8_bit_value(0x2000, ppu_ctrl_register);

        let mut ppu_status_register = cpu.memory.get_8_bit_value(0x2002);
        ppu_status_register |= 0x80;
        cpu.memory.set_8_bit_value(0x2002, ppu_status_register);

        // This should now be in the NMI routine
        cpu.tick(); // Executes INY
        cpu.tick(); // Executes CLC
        cpu.tick(); // Executes RTI

        // We should now be back in the normal flow
        cpu.tick(); // Executes INX

        assert_eq!(cpu.x_register, 0x02);
        assert_eq!(cpu.y_register, 0x01);
        assert_eq!(cpu.is_carry_set(), true);
    }
}