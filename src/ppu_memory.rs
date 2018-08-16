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

    pub fn get_8_bit_value(&self, mut address: u16) -> u8 {
        address = self.get_non_mirrored_address(address);
        return self.memory[address as usize];
    }

    pub fn get_16_bit_value(&self, mut address: u16) -> u16 {
        address = self.get_non_mirrored_address(address);

        // Little Endian. Low byte is stored first
        let high_byte: u16 = (self.memory[address as usize + 1] as u16) << 8;
        let low_byte: u16 = self.memory[address as usize] as u16;
        return high_byte | low_byte;
    }

    pub fn get_memory_range(&self, mut address: u16, num_bytes: u16) -> Vec<u8> {
        address = self.get_non_mirrored_address(address);

        let memory: &[u8] = &self.memory[(address as usize)..(address as usize + num_bytes as usize)];
        let mut memory_copy: Vec<u8> = vec![0; num_bytes as usize];
        memory_copy.copy_from_slice(memory);
        return memory_copy;
    }

    pub fn set_8_bit_value(&mut self, mut address: u16, value: u8) {
        address = self.get_non_mirrored_address(address);

        self.memory[address as usize] = value;
    }

    pub fn set_16_bit_value(&mut self, mut address: u16, value: u16) {
        address = self.get_non_mirrored_address(address);

        self.memory[address as usize] = value as u8;
        self.memory[address as usize + 1] = (value >> 8) as u8;
    }

    // The PPU addresses more space than it can actually use. When operating on certain memory values, it is
    // functionally equivalent to be operating on a different value instead (writing to 0x4000 is the same as writing to 0x0)
    fn get_non_mirrored_address(&self, mut address: u16) -> u16 {
        if address >= 0x4000 {
            address %= 0x4000;
        }

        if address >= 0x3000 && address <= 0x3EFF {
            return address - 0x1000;
        } else if address >= 0x3F20 && address <= 0x3FFF {
            let mini_address = address % 0x20; // This is a shitty name but I can't think of a better one right now
            return 0x3F00 + mini_address;
        } else {
            return address;
        }
    }
}

#[cfg(test)]
mod tests {
    use ppu_memory::PPUMemory;

    #[test]
    fn can_get_a_stored_8_bit_value() {
        let mut memory = PPUMemory::new();
        memory.set_8_bit_value(0x1500, 150);
        assert_eq!(memory.get_8_bit_value(0x1500), 150);

        memory.set_8_bit_value(0x4500, 150);
        assert_eq!(memory.get_8_bit_value(0x500), 150);
    }

    #[test]
    fn can_get_a_stored_16_bit_value() {
        let mut memory = PPUMemory::new();

        memory.set_16_bit_value(0x1500, 150); // 8 bit value stored as 16
        assert_eq!(memory.get_16_bit_value(0x1500), 150);

        memory.set_16_bit_value(0x1500, 10450); // 16 bit value also stored as 16
        assert_eq!(memory.get_16_bit_value(0x1500), 10450);

        memory.set_16_bit_value(0x4500, 150); // 8 bit value stored as 16
        assert_eq!(memory.get_16_bit_value(0x500), 150);
    }

    #[test]
    fn can_store_two_8_bit_values_and_read_back_as_a_16_bit_value() {
        let mut memory = PPUMemory::new();

        memory.set_8_bit_value(0x1500, 0x42); // 8 bit value stored as 16
        memory.set_8_bit_value(0x1501, 0xA5); // 16 bit value also stored as 16
        assert_eq!(memory.get_16_bit_value(0x1500), 0xA542);
    }
}
