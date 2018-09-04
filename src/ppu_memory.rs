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
        // Anything reading from 0x3F00 by 4's, 0x3F04, 0x3F08... etc gets the backdrop color from 0x3F00
        if PPUMemory::is_palette_address(address) && address % 4 == 0 {
            return self.memory[0x3F00];
        }
        return self.memory[address as usize];
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

        if PPUMemory::is_nametable_address(address) {
            self.mirror_nametable_write(address, value);
        }
    }

    // The PPU addresses more space than it can actually use. When operating on certain memory values, it is
    // functionally equivalent to be operating on a different value instead (writing to 0x4000 is the same as writing to 0x0)
    fn get_non_mirrored_address(&self, mut address: u16) -> u16 {
        if address >= 0x4000 {
            address %= 0x4000;
        }

        if address >= 0x3000 && address <= 0x3EFF {
            return address - 0x1000;
        } else if address >= 0x3F00 && address <= 0x3FFF {
            // There are mirrors in mirrors in the palette table. Anything mod 4 wants the backdrop color at 0x3F00
            if address >= 0x3F10 && address % 4 == 0 {
                return address - 0x10;
            }

            return 0x3F00 + address % 0x20;
        } else {
            return address;
        }
    }

    fn is_nametable_address(address: u16) -> bool {
        return address >= 0x2000 && address < 0x3000;
    }

    fn is_palette_address(address: u16) -> bool {
        return address >= 0x3F00 && address < 0x4000;
    }

    // This is a hacky thing that is specifically in use for SMB1. This needs to be replaced with a module for
    // mappers, with proper functionality for detecting horizontal, vertical, or other forms of nametable mirroring
    fn mirror_nametable_write(&mut self, address: u16, value: u8) {
        if address < 0x2800 {
            self.memory[(address + 0x800) as usize] = value;
        } else {
            self.memory[(address - 0x800) as usize] = value;
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
    fn palette_writes_are_mirrored() {
        let mut memory = PPUMemory::new();

        memory.set_8_bit_value(0x3F25, 0x42);
        assert_eq!(memory.memory[0x3F05], 0x42);
        assert_eq!(memory.memory[0x3F25], 0x42);
        assert_eq!(memory.memory[0x3F45], 0x42);
        assert_eq!(memory.memory[0x3F65], 0x42);
        assert_eq!(memory.memory[0x3F85], 0x42);
    }

    #[test]
    fn palette_reads_are_mirrored() {
        let mut memory = PPUMemory::new();

        memory.memory[0x3F05] = 0x42;
        assert_eq!(memory.get_8_bit_value(0x3F25), 0x42);
    }
}
