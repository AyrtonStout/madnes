pub struct CPUMemory {
    memory: [u8; 0x10000]
}

//TODO need to somehow implement memory mirroring on either the getters or the setters
//Not sure which is better yet... probably setters tho

impl CPUMemory {
    pub fn new() -> CPUMemory {
        return CPUMemory {
            memory: [0; 0x10000]
        }
    }

    pub fn get_ppu_io_registers(&mut self) -> *mut u8 {
        unsafe {
            let first_address: *mut u8 = self.memory.first_mut().unwrap();
            return first_address.offset(0x2000);
        }
    }

    pub fn read_ppu_for_nmi(&mut self) -> bool {
        unsafe {
            let first_address: *mut u8 = self.memory.first_mut().unwrap();
            let mut ppu_status_register: *mut u8 = first_address.offset(0x2002);
            let nmi_enabled = (*ppu_status_register & 0x80) == 0x80;
            *ppu_status_register &= !0x80; // Clear the register of the NMI now that it has been read

            return nmi_enabled;
        }
    }

    pub fn are_nmis_enabled(&mut self) -> bool {
        unsafe {
            let first_address: *mut u8 = self.memory.first_mut().unwrap();
            let ppu_control_register_1: *mut u8 = first_address.offset(0x2000);
            return (*ppu_control_register_1 & 0x80) == 0x80;
        }
    }

    pub fn init_prg_rom(&mut self, prg_rom: Vec<u8>) {
        // TODO this will need to be more sophisticated with a ROM that requires bank switching. Should handle this with pointers to different banks
        // TODO can try to use https://stackoverflow.com/a/28224758 here instead of looping
        for i in 0..prg_rom.len() {
            let rom_byte = prg_rom[i];
            self.memory[0x8000 + i] = rom_byte;
        }
    }

    pub fn get_reset_vector(&self) -> u16 {
        return ((self.memory[0xFFFD] as u16) << 8) | (self.memory[0xFFFC] as u16);
    }

    pub fn get_8_bit_value(&self, address: u16) -> u8 {
        return self.memory[address as usize];
    }

    pub fn get_16_bit_value(&self, address: u16) -> u16 {
        // Little Endian. Low byte is stored first
        let high_byte: u16 = (self.memory[address as usize + 1] as u16) << 8;
        let low_byte: u16 = self.memory[address as usize] as u16;
        return high_byte | low_byte;
    }

    pub fn get_memory_range(&self, address: u16, num_bytes: u16) -> Vec<u8> {
        let memory: &[u8] = &self.memory[(address as usize)..(address as usize + num_bytes as usize)];
        let mut memory_copy: Vec<u8> = vec![0; num_bytes as usize];
        memory_copy.copy_from_slice(memory);
        return memory_copy;
    }

    pub fn set_8_bit_value(&mut self, address: u16, value: u8) {
        CPUMemory::check_valid_write(address);

        self.memory[address as usize] = value;
    }

    #[allow(dead_code)]
    pub fn set_16_bit_value(&mut self, address: u16, value: u16) {
        CPUMemory::check_valid_write(address);

        self.memory[address as usize] = value as u8;
        self.memory[address as usize + 1] = (value >> 8) as u8;
    }

    fn check_valid_write(address: u16) {
        if address >= 0x8000 {
            panic!("Attempting to write to read-only memory address: {}!", address);
        }
    }
}

#[cfg(test)]
mod tests {
    use cpu_memory::CPUMemory;

    #[test]
    fn can_get_a_stored_8_bit_value() {
        let mut memory: CPUMemory = CPUMemory::new();
        memory.set_8_bit_value(0x1500, 150);
        assert_eq!(memory.get_8_bit_value(0x1500), 150);
    }

    #[test]
    fn can_get_a_stored_16_bit_value() {
        let mut memory: CPUMemory = CPUMemory::new();

        memory.set_16_bit_value(0x1500, 150); // 8 bit value stored as 16
        assert_eq!(memory.get_16_bit_value(0x1500), 150);

        memory.set_16_bit_value(0x1500, 10450); // 16 bit value also stored as 16
        assert_eq!(memory.get_16_bit_value(0x1500), 10450);
    }

    #[test]
    fn can_store_two_8_bit_values_and_read_back_as_a_16_bit_value() {
        let mut memory: CPUMemory = CPUMemory::new();

        memory.set_8_bit_value(0x1500, 0x42); // 8 bit value stored as 16
        memory.set_8_bit_value(0x1501, 0xA5); // 16 bit value also stored as 16
        assert_eq!(memory.get_16_bit_value(0x1500), 0xA542);
    }

    #[test]
    #[should_panic]
    fn cannot_8_bit_write_to_prg_rom() {
        let mut memory: CPUMemory = CPUMemory::new();
        memory.set_8_bit_value(0x8000, 0x42); // 8 bit value stored as 16
    }

    #[test]
    #[should_panic]
    fn cannot_16_bit_write_to_prg_rom() {
        let mut memory: CPUMemory = CPUMemory::new();
        memory.set_16_bit_value(0x8000, 0x4242); // 8 bit value stored as 16
    }

    #[test]
    fn get_memory_range() {
        let mut memory: CPUMemory = CPUMemory::new();
        memory.memory[0] = 1;
        memory.memory[1] = 2;
        memory.memory[2] = 3;
        memory.memory[3] = 4;

        let bytes = memory.get_memory_range(2, 1);
        assert_eq!(bytes[0], 3);

        let bytes2 = memory.get_memory_range(1, 2);
        assert_eq!(bytes2[0], 2);
        assert_eq!(bytes2[1], 3);
    }

    #[test]
    fn init_prg_rom() {
        let mut memory: CPUMemory = CPUMemory::new();
        memory.init_prg_rom(vec![0xFF as u8; 0x8000]);
        assert_eq!(memory.get_8_bit_value(0x7999), 0x00);
        assert_eq!(memory.get_8_bit_value(0x8000), 0xFF);
        assert_eq!(memory.get_8_bit_value(0xFFFF), 0xFF);
    }
}
