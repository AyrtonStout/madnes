pub struct CPUMemory {
    memory: [u8; 0xFFFF]
}

impl CPUMemory {
    pub fn new() -> CPUMemory {
        return CPUMemory {
            memory: [0; 0xFFFF]
        }
    }

    #[allow(dead_code)]
    pub fn get_8_bit_value(&self, address: u16) -> u8 {
        return self.memory[address as usize];
    }

    #[allow(dead_code)]
    pub fn get_16_bit_value(&self, address: u16) -> u16 {
        // Little Endian. Low byte is stored first
        let high_byte: u16 = (self.memory[address as usize + 1] as u16) << 8;
        let low_byte: u16 = self.memory[address as usize] as u16;
        return high_byte | low_byte;
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

}
