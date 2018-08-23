use instruction_set::get_instruction;
use cpu_memory::CPUMemory;
use instruction_set::AddressingMode;
use instruction_set::InstructionType;
use ppu::PPU as PPU;
use controlletron::Controlletron as Controlletron;
use std::collections::linked_list::LinkedList;

static STACK_POINTER_OFFSET: u16 = 0x100;

pub struct CPU {
    program_counter: u16,
    stack_pointer: u8,
    status_register: u8,
    accumulator: u8,
    x_register: u8,
    y_register: u8,
    remaining_clock_cycles: i8,
    current_instruction: Option<InstructionType>,
    dma_counter: i16,
    dma_address: u8,
    memory: CPUMemory,
    ppu: *mut PPU,
    controlletron: *mut Controlletron,
    history: LinkedList<String> // Just used for debugging what the emulator has run
}

const OAM_DMA_MEMORY: u16 = 0x4014;
const CONTROLLER1_MEMORY: u16 = 0x4016;
const CONTROLLER2_MEMORY: u16 = 0x4017;

impl CPU {
    pub fn new() -> CPU {
        return CPU {
            program_counter: 0,
            stack_pointer: 0xFD, // This will grow downward (decrement) down to 0. Then it wraps around back to 0xFF
            status_register: 0b0010_0100,
            accumulator: 0,
            x_register: 0,
            y_register: 0,
            remaining_clock_cycles: 0,
            current_instruction: None,
            dma_counter: -1,
            dma_address: 0,
            memory: CPUMemory::new(),
            ppu: 0 as *mut PPU,
            controlletron: 0 as *mut Controlletron,
            history: LinkedList::new()
        }
    }

    // FIXME: Due to shitty separation of concerns (e.g. CPU and PPU both rely on references to each other), this is set after the CPU is newed up
    pub fn init_late_pointers(&mut self, ppu: *mut PPU) {
        self.ppu = ppu;
        unsafe {
            self.controlletron = (*self.ppu).get_controlletron();
        }
    }

    pub fn init_prg_rom(&mut self, prg_rom: Vec<u8>) {
        self.memory.init_prg_rom(prg_rom);
        self.program_counter = self.memory.get_reset_vector();
    }

    pub fn tick(&mut self) {
        if self.dma_counter != -1 {
            self.perform_dma();
            return;
        }

        // Used to load in the very first instruction
        if self.current_instruction.is_none() {
            self.load_new_instruction();
        }

        self.remaining_clock_cycles -= 1;

        if self.remaining_clock_cycles > 0 { // The instruction takes multiple cycles to finish. Keep waiting
            return;
        } else if self.remaining_clock_cycles == 0 { // Instruction is finished. Execute it
            // TODO bake the opcode into the InstructionType, and have handle_instruction take care of all of this nonsense
            let memory_start = self.program_counter;
            let opcode: u8 = self.read_from_memory_8(memory_start);
            // The instruction counter is incremented prior to doing the action
            let num_bytes = self.current_instruction.unwrap().num_bytes;
            let instruction_data: Vec<u8> = self.memory.get_memory_range(self.program_counter + 1, num_bytes as u16 - 1);

//            println!("{:X} {:X} {}", self.program_counter, opcode, self.current_instruction.unwrap().name);
            self.program_counter += num_bytes as u16;

            let instruction = self.current_instruction.unwrap();
            self.handle_instruction(opcode, instruction, instruction_data.as_slice());

            // We finished the last instruction. Grab another instruction
            self.load_new_instruction();
        }
//        self.debug_check_for_instruction_sequence();
    }

    fn load_new_instruction(&mut self) {
        self.handle_nmi();

        let memory_start = self.program_counter;
        let opcode: u8 = self.read_from_memory_8(memory_start);
        self.current_instruction = Some(get_instruction(opcode));
        self.remaining_clock_cycles = self.current_instruction.unwrap().num_cycles as i8;

        self.history.push_back(self.current_instruction.unwrap().name.to_string());
        if self.history.len() > 100 {
            self.history.pop_front();
        }
    }

    // NOTE: There is some tomfoolery possible here. A thing called 'Interrupt Hijacking'. Might have to implement
    fn handle_nmi(&mut self) {
        if self.memory.are_nmis_enabled() {
            if self.memory.read_ppu_for_nmi() {
//                println!("NMI detected for CPU!");
                let program_counter = self.program_counter;
                self.push_stack_16(program_counter);
                self.push_status_to_stack(true);
                self.asm_sei(); // Disable interrupts
                self.program_counter = self.memory.get_16_bit_value(0xFFFA);
            }
        }
    }

    fn get_source_address(&mut self, instruction: InstructionType, instruction_data: &[u8]) -> u16 {
        // Make copies so the borrow checker doesn't get mad
        let x_register = self.x_register;
        let y_register = self.y_register;

        match instruction.addressing_mode {
            AddressingMode::Immediate => panic!("Makes no sense! {}", instruction.name),
            AddressingMode::Absolute => CPU::convert_to_address(instruction_data),
            AddressingMode::Implied => panic!("There is no data for implied instructions!"),
            AddressingMode::Accumulator => panic!("There is no data for accumulator instructions!"),
            AddressingMode::AbsoluteX => self.compute_absolute_address(instruction_data, x_register),
            AddressingMode::AbsoluteY => self.compute_absolute_address(instruction_data, y_register),
            AddressingMode::ZeroPageAbsolute => self.compute_zero_page_absolute_address(instruction_data[0], 0),
            AddressingMode::ZeroPageAbsoluteX => self.compute_zero_page_absolute_address(instruction_data[0], x_register),
            AddressingMode::ZeroPageAbsoluteY => self.compute_zero_page_absolute_address(instruction_data[0], y_register),
            AddressingMode::PreIndexedIndirect => self.get_pre_indexed_indirect_address(instruction_data[0]),
            AddressingMode::PostIndexedIndirect => self.get_post_indexed_indirect_address(instruction_data[0]),
            AddressingMode::Relative => panic!("Also makes no sense!"),
            AddressingMode::Indirect => self.get_indirect_address(instruction_data),
            AddressingMode::Empty => panic!("AddressingMode not set for {}!", instruction.name)
        }
    }

    fn handle_instruction(&mut self, opcode: u8, instruction: InstructionType, instruction_data: &[u8]) {
//        println!("Name: {} Opcode: {:X} Data: {:?}", instruction.name, opcode, instruction_data);
        if instruction.addressing_mode == AddressingMode::Implied {
            match instruction.name.as_ref() {
                "CLC" => self.asm_clc(),
                "CLV" => self.asm_clv(),
                "SEC" => self.asm_sec(),
                "PHA" => self.asm_pha(),
                "RTI" => self.asm_rti(),
                "RTS" => self.asm_rts(),
                "PLA" => self.asm_pla(),
                "SEI" => self.asm_sei(),
                "DEY" => self.asm_dey(),
                "TXA" => self.asm_txa(),
                "TYA" => self.asm_tya(),
                "TXS" => self.asm_txs(),
                "TAX" => self.asm_tax(),
                "TAY" => self.asm_tay(),
                "TSX" => self.asm_tsx(),
                "INY" => self.asm_iny(),
                "DEX" => self.asm_dex(),
                "CLD" => self.asm_cld(),
                "INX" => self.asm_inx(),
                "BRK" => self.asm_brk(),
                "NOP" => self.asm_nop(2),
                "SED" => self.asm_sed(),
                "PHP" => self.asm_php(),
                "PLP" => self.asm_plp(),
                _ => panic!("Implied instruction {} not implemented!", instruction.name)
            }
            return;
        }

        if instruction.addressing_mode == AddressingMode::Accumulator {
            match instruction.name.as_ref() {
                "ASL" => self.asm_asl_accumulator(),
                "LSR" => self.asm_lsr_accumulator(),
                "ROL" => self.asm_rol_accumulator(),
                "ROR" => self.asm_ror_accumulator(),
                _ => panic!("Accumulator instruction {} not implemented!", instruction.name)
            }
            return;
        }

        // TODO handle Immediate and Relative (which is basically Immediate) more gracefully
        let mut source_address = 0;
        if instruction.addressing_mode != AddressingMode::Immediate
            && instruction.addressing_mode != AddressingMode::Relative {
            source_address = self.get_source_address(instruction, instruction_data);
            match instruction.name.as_ref() {
                "STA" => { self.asm_sta(source_address); return; },
                "STY" => { self.asm_sty(source_address); return; },
                "STX" => { self.asm_stx(source_address); return; },
                "SAX" => { self.asm_sax(source_address); return; }, // Unofficial opcode
                "JSR" => { self.asm_jsr(source_address); return; },
                "JMP" => { self.asm_jmp(source_address); return; },
                "INC" => { self.asm_inc(source_address); return; },
                "DEC" => { self.asm_dec(source_address); return; },
                "DCP" => { self.asm_dcp(source_address); return; }, // Unofficial opcode
                "ISB" => { self.asm_isb(source_address); return; }, // Unofficial opcode
                "SLO" => { self.asm_slo(source_address); return; }, // Unofficial opcode
                "ROR" => { self.asm_ror_memory(source_address); return; },
                "ROL" => { self.asm_rol_memory(source_address); return; },
                "RLA" => { self.asm_rla(source_address); return; }, // Unofficial opcode
                "RRA" => { self.asm_rra(source_address); return; }, // Unofficial opcode
                "SRE" => { self.asm_sre(source_address); return; }, // Unofficial opcode
                "LSR" => { self.asm_lsr_memory(source_address); return; },
                "ASL" => { self.asm_asl_memory(source_address); return; },
                "DOP" => { self.asm_nop(3); return; }, // Unofficial opcode
                "TOP" => { self.asm_nop(4); return; }, // Unofficial opcode
                _ => ()
            }
        }

        let source_value;
        // Instruction we read in wasn't in the previous groups. So we want the address's value
        if instruction.addressing_mode == AddressingMode::Immediate
            || instruction.addressing_mode == AddressingMode::Relative {
            source_value = instruction_data[0];
        } else {
            source_value = self.read_from_memory_8(source_address);
        }
        match instruction.name.as_ref() {
            "ADC" => { self.asm_adc(source_value); },
            "SBC" => { self.asm_sbc(source_value); },
            "LDA" => { self.asm_lda(source_value); },
            "LDX" => { self.asm_ldx(source_value); },
            "LDY" => { self.asm_ldy(source_value); },
            "LAX" => { self.asm_lax(source_value); },
            "CMP" => { self.asm_cmp(source_value); },
            "CPX" => { self.asm_cpx(source_value); },
            "CPY" => { self.asm_cpy(source_value); },
            "BPL" => { self.asm_bpl(source_value); },
            "BCS" => { self.asm_bcs(source_value); },
            "BNE" => { self.asm_bne(source_value); },
            "BEQ" => { self.asm_beq(source_value); },
            "BVS" => { self.asm_bvs(source_value); },
            "BVC" => { self.asm_bvc(source_value); },
            "BCC" => { self.asm_bcc(source_value); },
            "BMI" => { self.asm_bmi(source_value); },
            "ORA" => { self.asm_ora(source_value); },
            "AND" => { self.asm_and(source_value); },
            "BIT" => { self.asm_bit(source_value); },
            "EOR" => { self.asm_eor(source_value); },
            "DOP" => { self.asm_nop(2); }, // Unofficial
            _ => panic!("Found unimplemented instruction! Name: {} Opcode: {:X}", instruction.name, opcode)
        }

    }

    pub fn get_ppu_io_registers_address(&mut self) -> *mut u8 {
        return self.memory.get_ppu_io_registers();
    }

    // This function might not stick around in the code for long but it wraps the CPUMemory calls because
    // sometimes we might need to do extra things if we write to certain memory-mapped locations
    fn write_to_memory_8(&mut self, address: u16, new_value: u8) {
//        println!("Address: {:X}, Value: {:X}", address, new_value);
        self.memory.set_8_bit_value(address, new_value);

        if address >= 0x2000 && address <= 0x2007 {
            unsafe {
                (*self.ppu).write_to_register(address, new_value);
            }
        } else if address == OAM_DMA_MEMORY {
            // We're going to start performing DMA. Set the counter to 0 and each two cycles we will copy
            // some data to PPU OAM
            self.dma_counter = 0;
            self.dma_address = new_value;
        } else if address == CONTROLLER1_MEMORY {
            unsafe {
                (*self.controlletron).receive_memory_write(new_value);
            }
        }
    }

    fn read_from_memory_8(&mut self, address: u16) -> u8 {
        let memory = self.memory.get_8_bit_value(address);

        if address >= 0x2000 && address <= 0x2007 {
            unsafe {
                (*self.ppu).read_from_register(address);
            }
        }
        if address == CONTROLLER1_MEMORY || address == CONTROLLER2_MEMORY {
            unsafe {
                return (*self.controlletron).read_controller_value(address);
            }
        }

        return memory;
    }

    // DMA sends 256 bytes of sprite data to the PPU. The offset determines which address to start at, in 256 byte increments
    // So if memory_offset was 0x12, we'd send 0x1200 to 0x12FF to the PPU to get stored in OAM
    fn perform_dma(&mut self) {
        self.dma_counter += 1;
        // If we've hit cycle 513 then we are done performing the DMA
        if self.dma_counter == 513 {
            self.dma_counter = -1;
            return;
        }

        // We perform a transfer every 2 cycles. So only transfer data on an even cycle
        if self.dma_counter % 2 != 0 {
            return;
        }

        let starting_address = self.dma_address as u16 * 0x100;
        let address_offset = (self.dma_counter / 2) as u8;
        let sprite_data = self.memory.get_8_bit_value(starting_address + address_offset as u16);
        unsafe {
            (*self.ppu).receive_dma(address_offset, sprite_data);
        }
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

    // Some resource seem to disagree about this bit. I actually think this might be wrong...
    fn set_brk_bit(&mut self, is_set: bool) {
        if is_set {
            self.status_register |= 0b0001_0000;
        } else {
            self.status_register &= !0b0001_0000;
        }
    }

    fn convert_to_address(address_data: &[u8]) -> u16 {
        if address_data.len() == 2 {
            return ((address_data[1] as u16) << 8) | (address_data[0] as u16);
        } else {
            return address_data[0] as u16;
        }
    }

    fn get_pre_indexed_indirect_address(&mut self, zero_page_address: u8) -> u16 {
        let address = zero_page_address.wrapping_add(self.x_register);
        return if address == 0xFF {
            let low_byte = self.read_from_memory_8(0xFF);
            let high_byte = self.read_from_memory_8(0x00);
            CPU::convert_to_address(&[low_byte, high_byte])
        } else {
            self.memory.get_16_bit_value(address as u16)
        }
    }

    fn get_post_indexed_indirect_address(&mut self, zero_page_address: u8) -> u16 {
        // TODO instead of having this (and pre-indexed) both do this check, add a function to CPUMemory to get a zero-page address
        let address = if zero_page_address == 0xFF {
            let low_byte = self.read_from_memory_8(0xFF);
            let high_byte = self.read_from_memory_8(0x00);
            CPU::convert_to_address(&[low_byte, high_byte])
        } else {
            self.memory.get_16_bit_value(zero_page_address as u16)
        };

        return address.wrapping_add(self.y_register as u16);
    }

    // Used only by JMP. Returns the address stored at the address
    fn get_indirect_address(&mut self, instruction_data: &[u8]) -> u16 {
        // This is a bug in the 6502 itself that has to be reproduced for accuracy
        if instruction_data[0] == 0xFF {
            let low_byte = self.memory.get_8_bit_value(CPU::convert_to_address(instruction_data));
            let high_byte = self.memory.get_8_bit_value(CPU::convert_to_address(&[0x00, instruction_data[1]]));
            return (high_byte as u16) << 8 | low_byte as u16
        } else {
            let address = CPU::convert_to_address(instruction_data);
            return self.memory.get_16_bit_value(address);
        };
    }

    fn compute_absolute_address(&mut self, instruction_data: &[u8], offset: u8) -> u16 {
        let address = CPU::convert_to_address(instruction_data);
        return address.wrapping_add(offset as u16);
    }

    fn compute_zero_page_absolute_address(&mut self, starting_address: u8, offset: u8) -> u16 {
        return starting_address.wrapping_add(offset) as u16;
    }

    fn push_stack(&mut self, value_to_write: u8) {
        let stack_address: u16 = self.stack_pointer as u16 + STACK_POINTER_OFFSET;
        self.write_to_memory_8(stack_address, value_to_write);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1); // This tells rust we expect to underflow (if that's a word) and wrap around to 0xFF
    }

    fn push_stack_16(&mut self, value_to_write: u16) {
        self.push_stack((value_to_write >> 8) as u8);
        self.push_stack(value_to_write as u8);
    }

    fn pull_stack(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let stack_address: u16 = self.stack_pointer as u16 + STACK_POINTER_OFFSET;
        return self.read_from_memory_8(stack_address);
    }

    fn pull_stack_16(&mut self) -> u16 {
        let low_bye: u16 = self.pull_stack() as u16;
        let high_byte: u16 = (self.pull_stack() as u16) << 8;
        return high_byte | low_bye;
    }

    #[allow(dead_code)]
    pub fn are_interrupts_disabled(&self) -> bool {
        return (self.status_register & 0x04) == 0x04;
    }

    // Read more about decimal mode here http://6502.org/tutorials/decimal_mode.html
    // Actually, it seems the NES might not GAF about Decimal Mode, and this stuff is just here for other 6502 processors
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

    #[allow(dead_code)]
    pub fn is_overflow_set(&self) -> bool {
        return (self.status_register & 0x40) == 0x40;
    }

    // Change the program counter by an offset
    // Branch instructions do signed arithmetic for their offsets
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

    // TODO test
    fn push_status_to_stack(&mut self, from_interrupt: bool) {
        let mut status_register = self.status_register;
        status_register |= 0b0010_0000; // Bit 5 of the status register is always set

        // Bit 4 is conditionally set, depending on what triggered it. However this only is set on the stack. Not the current register
        if from_interrupt {
            status_register &= !0b0001_0000;
        } else {
            status_register |= 0b0001_0000;
        }

        self.push_stack(status_register);
    }

    // TODO test
    fn pull_status_from_stack(&mut self) {
        let mut status_register = self.pull_stack();
        status_register &= 0b1100_1111; // Ignore bits 4 and 5
        status_register |= 0b0011_0000 & self.status_register; // Keep the bits 4 and 5 that we already have
        self.status_register = status_register;
    }

    fn asm_ora(&mut self, source: u8) {
        let result = source | self.accumulator;
        self.set_sign(result);
        self.set_zero(result);
        self.accumulator = result;
    }

    // Store the accumulator at a memory location
    fn asm_sta(&mut self, source: u16) {
        let accumulator = self.accumulator;
        self.write_to_memory_8(source, accumulator);
    }

    // ** Unofficial Instruction **
    // ANDs the X register with the accumulator and stores the result in memory
    // TODO test
    fn asm_sax(&mut self, source: u16) {
        let result = self.x_register & self.accumulator;
        self.memory.set_8_bit_value(source, result);
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

    // ** Unofficial Instruction **
    // Load a value into both the X register and the accumulator
    // TODO test
    fn asm_lax(&mut self, source: u8) {
        self.set_sign(source);
        self.set_zero(source);
        self.accumulator = source;
        self.x_register = source;
    }

    // Increment the value stored at a memory location
    fn asm_inc(&mut self, address: u16) {
        let memory_value = self.read_from_memory_8(address);
        let new_memory_value = memory_value.wrapping_add(1);
        self.write_to_memory_8(address, new_memory_value);
        self.set_sign(new_memory_value);
        self.set_zero(new_memory_value);
    }

    // ** Unofficial **
    // Equivalent to an INC of the value then an SBC
    // TODO test
    fn asm_isb(&mut self, address: u16) {
        self.asm_inc(address);

        let value = self.read_from_memory_8(address);
        self.asm_sbc(value);
    }

    // ** Unofficial **
    // Equivalent to an ASL of the value then an ORA
    // TODO test
    fn asm_slo(&mut self, address: u16) {
        self.asm_asl_memory(address);

        let value = self.read_from_memory_8(address);
        self.asm_ora(value);
    }

    // ** Unofficial **
    // Equivalent to a ROL of the value then an AND
    // TODO test
    fn asm_rla(&mut self, address: u16) {
        self.asm_rol_memory(address);

        let value = self.read_from_memory_8(address);
        self.asm_and(value);
    }

    // ** Unofficial **
    // Equivalent to a ROL of the value then an AND
    // TODO test
    fn asm_rra(&mut self, address: u16) {
        self.asm_ror_memory(address);

        let value = self.read_from_memory_8(address);
        self.asm_adc(value);
    }

    // ** Unofficial **
    // Equivalent to a LSR of the value then an EOR
    // TODO test
    fn asm_sre(&mut self, address: u16) {
        self.asm_lsr_memory(address);

        let value = self.read_from_memory_8(address);
        self.asm_eor(value);
    }

    // Subtraction (with carry)
    // http://www.6502.org/tutorials/vflag.html#2.4
    fn asm_adc(&mut self, source: u8) {
        let carry: u16 = if self.is_carry_set() { 1 } else { 0 };
        let accumulator: u16 = self.accumulator as u16;
        let temp: u16 = accumulator + carry + source as u16;

        self.set_sign(temp as u8);
        self.set_zero(temp as u8);
        self.set_overflow_bit(((accumulator ^ source as u16) & 0x80) == 0
            && ((accumulator ^ temp) & 0x80) > 0);
        self.set_carry_bit(temp > 0xFF);

        self.accumulator = temp as u8;
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

    // Clears the carry flag
    fn asm_clc(&mut self) {
        self.set_carry_bit(false);
    }

    // Clears the overflow flag
    fn asm_clv(&mut self) {
        self.set_overflow_bit(false);
    }

    // Have program start executing from a new address. Store current address on the stack
    fn asm_jsr(&mut self, source: u16) {
        let return_address = self.program_counter - 1;
        self.push_stack((return_address >> 8) as u8);
        self.push_stack((return_address & 0x00FF) as u8);
        self.program_counter = source;
    }

    // Bitshift accumulator to the left by 1, making the LSB the value of the current carry
    fn asm_rol_accumulator(&mut self) {
        let accumulator = self.accumulator;
        self.accumulator = self.asm_rol(accumulator);
    }

    fn asm_ror(&mut self, source: u8) -> u8 {
        let shifted_source = source >> 1;
        let result = if self.is_carry_set() { shifted_source | 0x80 } else { shifted_source };
        self.set_zero(result);
        self.set_sign(result);
        self.set_carry_bit((source & 0x01) == 0x01);
        return result;
    }

    fn asm_rol(&mut self, source: u8) -> u8 {
        let shifted_source = source << 1;
        let final_source = if self.is_carry_set() { shifted_source | 0x01 } else { shifted_source }; // Why Rust no have ternary
        self.set_zero(final_source);
        self.set_sign(final_source);
        self.set_carry_bit((source & 0x80) == 0x80);
        return final_source;
    }

    fn asm_lsr(&mut self, source: u8) -> u8 {
        let shifted_source = source >> 1;
        self.set_zero(shifted_source);
        self.set_sign(shifted_source);
        self.set_carry_bit((source & 0x01) == 0x01);
        return shifted_source;
    }

    fn asm_asl(&mut self, source: u8) -> u8 {
        let shifted_source = source << 1;
        self.set_zero(shifted_source);
        self.set_sign(shifted_source);
        self.set_carry_bit((source & 0x80) == 0x80);
        return shifted_source;
    }

    // Bitshift accumulator to the right by 1, making the MSB the value of the current carry
    fn asm_ror_accumulator(&mut self) {
        let accumulator = self.accumulator;
        self.accumulator = self.asm_ror(accumulator);
    }

    // Bitshift a memory location to the right by 1, making the MSB the value of the current carry. If LSB is set, set the carry flag
    fn asm_ror_memory(&mut self, address: u16) {
        let memory_value = self.read_from_memory_8(address);
        let new_value = self.asm_ror(memory_value);
        self.write_to_memory_8(address, new_value);
    }

    // Bitshift a memory location to the left by 1, making the LSB the value of the current carry. If MSB is set, set the carry flag
    fn asm_rol_memory(&mut self, address: u16) {
        let memory_value = self.read_from_memory_8(address);
        let new_value = self.asm_rol(memory_value);
        self.write_to_memory_8(address, new_value);
    }

    // Bitshift a memory location to the right by 1. If the LSB is set, set the carry flag
    // TODO test
    fn asm_lsr_memory(&mut self, address: u16) {
        let memory_value = self.read_from_memory_8(address);
        let new_value = self.asm_lsr(memory_value);
        self.write_to_memory_8(address, new_value);
    }

    // Bitshift a memory location to left right by 1.
    // TODO test
    fn asm_asl_memory(&mut self, address: u16) {
        let memory_value = self.read_from_memory_8(address);
        let new_value = self.asm_asl(memory_value);
        self.write_to_memory_8(address, new_value);
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
        self.accumulator = self.asm_lsr(accumulator);
    }

    // Bitshift accumulator to the left by 1
    fn asm_asl_accumulator(&mut self) {
        let accumulator = self.accumulator;
        self.accumulator = self.asm_asl(accumulator);
    }

    // Start program execution at a value stored at a location in memory
    fn asm_jmp(&mut self, address: u16) {
        self.program_counter = address;
    }

    // Return the program from an interrupt routine
    fn asm_rti(&mut self) {
        self.pull_status_from_stack();
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
        self.set_sign(accumulator);
        self.set_zero(accumulator);
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

    // Puts the Y register into the accumulator
    fn asm_tya(&mut self) {
        let y_register = self.y_register;
        self.set_sign(y_register);
        self.set_zero(y_register);
        self.accumulator = y_register;
    }

    fn asm_stx(&mut self, address: u16) {
        let x_register = self.x_register;
        self.write_to_memory_8(address, x_register);
    }

    fn asm_sty(&mut self, address: u16) {
        let y_register = self.y_register;
        self.write_to_memory_8(address, y_register);
    }

    // Branches on 'carry clear' - the carry bit being 0 / not set
    fn asm_bcc(&mut self, source: u8) {
        if self.is_carry_set() { return; }

        self.branch(source);
    }

    // Branches on 'result minus'
    // TODO test
    fn asm_bmi(&mut self, source: u8) {
        if !self.is_negative_set() { return; }

        self.branch(source);
    }

    // Copies the X register to the stack pointer. This does not mean pushing the X value onto the stack
    fn asm_txs(&mut self) {
        self.stack_pointer = self.x_register;
    }

    // Transfers the accumulator into index X
    fn asm_tax(&mut self) {
        let accumulator = self.accumulator;
        self.set_sign(accumulator);
        self.set_zero(accumulator);
        self.x_register = accumulator;
    }

    // Transfers the accumulator into index Y
    fn asm_tay(&mut self) {
        let accumulator = self.accumulator;
        self.set_sign(accumulator);
        self.set_zero(accumulator);
        self.y_register = accumulator;
    }

    // Transfers the stack pointer into index X
    fn asm_tsx(&mut self) {
        let stack_pointer = self.stack_pointer;
        self.set_sign(stack_pointer);
        self.set_zero(stack_pointer);
        self.x_register = stack_pointer;
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
    fn asm_dec(&mut self, address: u16) {
        let memory_value: u8 = self.read_from_memory_8(address);
        let new_memory_value = memory_value.wrapping_sub(1);
        self.write_to_memory_8(address, new_memory_value);
        self.set_sign(new_memory_value);
        self.set_zero(new_memory_value);
    }

    // ** Unofficial **
    // Equivalent to a DEC then a CMP of the value
    // TODO test
    fn asm_dcp(&mut self, address: u16) {
        self.asm_dec(address);

        let value = self.read_from_memory_8(address);
        self.asm_cmp(value);
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

    // Sets the operational mode to decimal instead of binary
    fn asm_sed(&mut self) {
        self.status_register |= 0x08;
    }

    // Pushes the status register onto the stack
    // TODO test
    fn asm_php(&mut self) {
        self.push_status_to_stack(false);
    }

    // Sets the status register with what is next to come off the stack
    // TODO test
    fn asm_plp(&mut self) {
        self.pull_status_from_stack();
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

    // Triggers an error interrupt
    // TODO test
    fn asm_brk(&mut self) {
        let return_address = self.program_counter + 1;

        self.push_stack_16(return_address);
        self.set_brk_bit(true); // Is there also a "set interrupt" bit?
        self.push_status_to_stack(false);

        let error_handler_address = self.memory.get_16_bit_value(0xFFFE);
        self.program_counter = error_handler_address;

        panic!("BRK hit");
    }

    // Does nothing at all
    // TODO test? Maybe? Maybe once I have accurate timings?
    fn asm_nop(&self, _num_cycles: u8) {
        // TODO burn specific number of cycles or something depending on how I have this work
    }

    // Branches on 'result zero' - the last result having been zero
    fn asm_beq(&mut self, source: u8) {
        if !self.is_zero_set() { return; }

        self.branch(source);
    }

    // Branches on 'overflow set'
    // TODO test
    fn asm_bvs(&mut self, source: u8) {
        if !self.is_overflow_set() { return; }

        self.branch(source);
    }

    // Branches on 'overflow clear'
    // TODO test
    fn asm_bvc(&mut self, source: u8) {
        if self.is_overflow_set() { return; }

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
        cpu.asm_sta(0x1022);

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
        assert_eq!(0xFD, cpu.stack_pointer);
    }

    #[test]
    fn test_txs() {
        let mut cpu: CPU = CPU::new();

        cpu.x_register = 0x14;
        cpu.asm_txs();

        assert_eq!(cpu.stack_pointer, 0x14);
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
        cpu.asm_inc(0x1020);
        assert_eq!(cpu.memory.get_8_bit_value(0x1020), 0x51);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), false);

        cpu.memory.set_8_bit_value(0x1020, 0x7F);
        cpu.asm_inc(0x1020);
        assert_eq!(cpu.memory.get_8_bit_value(0x1020), 0x80);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), true);

        cpu.memory.set_8_bit_value(0x1020, 0xFF);
        cpu.asm_inc(0x1020);
        assert_eq!(cpu.memory.get_8_bit_value(0x1020), 0x00);
        assert_eq!(cpu.is_zero_set(), true);
        assert_eq!(cpu.is_negative_set(), false);
    }

    #[test]
    fn test_dec() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.set_8_bit_value(0x1020, 0x50);
        cpu.asm_dec(0x1020);
        assert_eq!(cpu.memory.get_8_bit_value(0x1020), 0x4F);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), false);

        cpu.memory.set_8_bit_value(0x1020, 0x81);
        cpu.asm_dec(0x1020);
        assert_eq!(cpu.memory.get_8_bit_value(0x1020), 0x80);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), true);

        cpu.memory.set_8_bit_value(0x1020, 0x01);
        cpu.asm_dec(0x1020);
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
        assert_eq!(cpu.stack_pointer, 0xFB);
    }

    #[test]
    fn test_jmp() {
        let mut cpu: CPU = CPU::new();
        cpu.asm_jmp(0x2050);

        assert_eq!(cpu.program_counter, 0x2050);
    }

    #[test]
    fn test_indirect_page_bug() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.set_8_bit_value(0x3000, 0x40);
        cpu.memory.set_8_bit_value(0x30FF, 0x80);
        cpu.memory.set_8_bit_value(0x3100, 0x50);

        let result = cpu.get_indirect_address(&[0xFF, 0x30]);

        assert_eq!(result, 0x4080);
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
    fn test_tya() {
        let mut cpu: CPU = CPU::new();
        cpu.y_register = 0x21;
        cpu.asm_tya();

        assert_eq!(cpu.accumulator, 0x21);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), false);

        cpu.y_register = 0x91;
        cpu.asm_tya();
        assert_eq!(cpu.is_negative_set(), true);

        cpu.y_register = 0x00;
        cpu.asm_tya();
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
    fn test_tay() {
        let mut cpu: CPU = CPU::new();
        cpu.accumulator = 0x21;
        cpu.asm_tay();

        assert_eq!(cpu.y_register, 0x21);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), false);

        cpu.accumulator = 0x91;
        cpu.asm_tay();
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), true);

        cpu.accumulator = 0x00;
        cpu.asm_tay();
        assert_eq!(cpu.is_zero_set(), true);
        assert_eq!(cpu.is_negative_set(), false);
    }

    #[test]
    fn test_tsx() {
        let mut cpu: CPU = CPU::new();
        cpu.stack_pointer = 0x21;
        cpu.asm_tsx();

        assert_eq!(cpu.x_register, 0x21);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), false);

        cpu.stack_pointer = 0x91;
        cpu.asm_tsx();
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_negative_set(), true);

        cpu.stack_pointer = 0x00;
        cpu.asm_tsx();
        assert_eq!(cpu.is_zero_set(), true);
        assert_eq!(cpu.is_negative_set(), false);
    }

    #[test]
    fn test_rts() {
        let mut cpu: CPU = CPU::new();
        cpu.program_counter = 0x8054;
        cpu.asm_jsr(0x9035);
        cpu.asm_rts();

        assert_eq!(cpu.program_counter, 0x8054);
        assert_eq!(cpu.stack_pointer, 0xFD);
    }

    #[test]
    fn test_pha() {
        let mut cpu: CPU = CPU::new();
        cpu.accumulator = 0x42;
        cpu.asm_pha();

        assert_eq!(0x42, cpu.pull_stack());
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
        cpu.accumulator = 0b10010000;
        cpu.asm_lsr_accumulator();

        assert_eq!(cpu.accumulator, 0b01001000);
        assert_eq!(cpu.is_carry_set(), false);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.accumulator = 0b00000001;
        cpu.asm_lsr_accumulator();

        assert_eq!(cpu.accumulator, 0b00000000);
        assert_eq!(cpu.is_carry_set(), true);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), true);
    }

    #[test]
    fn test_asl_accumulator() {
        let mut cpu: CPU = CPU::new();
        cpu.accumulator = 0b00010001;
        cpu.asm_asl_accumulator();

        assert_eq!(cpu.accumulator, 0b00100010);
        assert_eq!(cpu.is_carry_set(), false);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.accumulator = 0b11000000;
        cpu.asm_asl_accumulator();

        assert_eq!(cpu.accumulator, 0b10000000);
        assert_eq!(cpu.is_carry_set(), true);
        assert_eq!(cpu.is_negative_set(), true);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.accumulator = 0b10000000;
        cpu.asm_asl_accumulator();

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
    fn test_ror_accumulator() {
        let mut cpu: CPU = CPU::new();
        cpu.accumulator = 0b10010001;
        cpu.asm_ror_accumulator();

        assert_eq!(cpu.accumulator, 0b01001000);
        assert_eq!(cpu.is_carry_set(), true);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.accumulator = 0b01000010;
        cpu.set_carry_bit(true);
        cpu.asm_ror_accumulator();

        assert_eq!(cpu.accumulator, 0b10100001);
        assert_eq!(cpu.is_carry_set(), false);
        assert_eq!(cpu.is_negative_set(), true);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.accumulator = 0b00000000;
        cpu.set_carry_bit(false);
        cpu.asm_ror_accumulator();

        assert_eq!(cpu.accumulator, 0b00000000);
        assert_eq!(cpu.is_carry_set(), false);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), true);
    }

    #[test]
    fn test_ror_memory() {
        let mut cpu: CPU = CPU::new();
        cpu.memory.set_8_bit_value(0x2500, 0b10010001);
        cpu.asm_ror_memory(0x2500);

        assert_eq!(cpu.memory.get_8_bit_value(0x2500), 0b01001000);
        assert_eq!(cpu.is_carry_set(), true);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.memory.set_8_bit_value(0x2500, 0b01000010);
        cpu.set_carry_bit(true);
        cpu.asm_ror_memory(0x2500);

        assert_eq!(cpu.memory.get_8_bit_value(0x2500), 0b10100001);
        assert_eq!(cpu.is_carry_set(), false);
        assert_eq!(cpu.is_negative_set(), true);
        assert_eq!(cpu.is_zero_set(), false);

        cpu.memory.set_8_bit_value(0x2500, 0b00000000);
        cpu.set_carry_bit(false);
        cpu.asm_ror_memory(0x2500);

        assert_eq!(cpu.memory.get_8_bit_value(0x2500), 0b00000000);
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
    fn test_adc() {
        let mut cpu: CPU = CPU::new();

        cpu.accumulator = 0x02;
        cpu.set_carry_bit(false);
        cpu.asm_adc(0x01);

        assert_eq!(cpu.accumulator, 0x03);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_carry_set(), false);
        assert_eq!(cpu.is_overflow_set(), false);

        cpu.accumulator = 0x01;
        cpu.set_carry_bit(false);
        cpu.asm_adc(0xFF);

        assert_eq!(cpu.accumulator, 0x00);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), true);
        assert_eq!(cpu.is_carry_set(), true);
        assert_eq!(cpu.is_overflow_set(), false);

        cpu.accumulator = 0x7F;
        cpu.set_carry_bit(false);
        cpu.asm_adc(0x01);

        assert_eq!(cpu.accumulator, 0x80);
        assert_eq!(cpu.is_negative_set(), true);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_carry_set(), false);
        assert_eq!(cpu.is_overflow_set(), true);

        cpu.accumulator = 0x02;
        cpu.set_carry_bit(true);
        cpu.asm_adc(0x01);

        assert_eq!(cpu.accumulator, 0x04);
        assert_eq!(cpu.is_negative_set(), false);
        assert_eq!(cpu.is_zero_set(), false);
        assert_eq!(cpu.is_carry_set(), false);
        assert_eq!(cpu.is_overflow_set(), false);
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
        cpu.push_stack(0x5); // This will move the stack pointer below 0, and wrap back around to 0xFF

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
        cpu.tick(); cpu.tick(); // Executes 0x78 (2 cycles)
        cpu.tick(); cpu.tick(); // Executes 0xD8 (2 cycles)
        cpu.tick(); cpu.tick(); // Executes 0xA9 [0x10] (2 cycles)

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
        cpu.tick(); cpu.tick(); // Executes INX
        cpu.tick(); cpu.tick(); // Executes SEC

        // Enable NMIs and set one as having happened
        let mut ppu_ctrl_register = cpu.memory.get_8_bit_value(0x2000);
        ppu_ctrl_register |= 0x80;
        cpu.memory.set_8_bit_value(0x2000, ppu_ctrl_register);

        let mut ppu_status_register = cpu.memory.get_8_bit_value(0x2002);
        ppu_status_register |= 0x80;
        cpu.memory.set_8_bit_value(0x2002, ppu_status_register);

        // This should now be in the NMI routine
        cpu.tick(); cpu.tick(); // Executes INY
        cpu.tick(); cpu.tick(); // Executes CLC
        cpu.tick(); // Executes RTI

        // We should now be back in the normal flow
        cpu.tick(); cpu.tick(); // Executes INX

        assert_eq!(cpu.x_register, 0x02);
        assert_eq!(cpu.y_register, 0x01);
        assert_eq!(cpu.is_carry_set(), false);
    }
}
