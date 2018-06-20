pub struct PPUMemory {
    memory: [u8; 0x10000]
}

//TODO need to somehow implement memory mirroring on either the getters or the setters
//Not sure which is better yet... probably setters tho

#[allow(dead_code)]
impl PPUMemory {
    pub fn new() -> PPUMemory {
        return PPUMemory {
            memory: [0; 0x10000]
        }
    }

    pub fn init_chr_rom(&mut self, chr_rom: Vec<u8>) {
        if chr_rom.len() > 0x2000 {
            panic!("This ROM isn't supported! It has too much CHR_ROM!");
        }

        // TODO this will need to be more sophisticated with a ROM that requires bank switching. Should handle this with pointers to different banks
        // TODO can try to use https://stackoverflow.com/a/28224758 here instead of looping
        for i in 0..chr_rom.len() {
            let rom_byte = chr_rom[i];
            self.memory[0x0000 + i] = rom_byte;
        }
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
        PPUMemory::check_valid_write(address);

        self.memory[address as usize] = value;
    }

    pub fn set_16_bit_value(&mut self, address: u16, value: u16) {
        PPUMemory::check_valid_write(address);

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
}
