
pub struct PPU {
    ppu_control_register_1: *const u8, // 0x2000 Read-only
    ppu_control_register_2: *const u8, // 0x2001 Read-only
    ppu_status_register: *mut u8, // 0x2002 Used by CPU to read status from PPU
    spr_ram_address_register: *const u8, // 0x2003 Somehow used to load sprites?
    spr_ram_io_register: *const u8, // 0x2004 Also somehow used to load sprites?
    vram_address_register_1: *const u8, // 0x2005 Probably the low byte for a vram read / write
    vram_address_register_2: *const u8, // 0x2006 Probably the high byte for a vram read / write
    vram_io_register: *mut u8 // 0x2007 Reads or writes a byte from VRAM at the current location
}

impl PPU {
    pub fn new(io_registers: *mut u8) -> PPU {
        unsafe {
            return PPU {
                ppu_control_register_1: io_registers.offset(0),
                ppu_control_register_2: io_registers.offset(1),
                ppu_status_register: io_registers.offset(2),
                spr_ram_address_register: io_registers.offset(3),
                spr_ram_io_register: io_registers.offset(4),
                vram_address_register_1: io_registers.offset(5),
                vram_address_register_2: io_registers.offset(6),
                vram_io_register: io_registers.offset(7),
            }
        }
    }

    pub fn tick(&self) {

    }

//    pub fn read_program_instructions(&mut self, prg_rom: Vec<u8>) {
//
//    }

//    fn convert_to_address(address_data: &[u8]) -> u16 {
//        return 0;
//    }

}

#[cfg(test)]
mod tests {
    use ppu::PPU;

    #[test]
    fn init_pointers_set_up_correctly() {
        let mut memory: Vec<u8> = vec!(0x00, 0x12, 0x23, 0x31, 0x48, 0x51, 0x70, 0xAB, 0xE2);

        unsafe {
            let first_address: *mut u8 = memory.first_mut().unwrap();
            let ppu = PPU::new(first_address.offset(1));
            assert_eq!(*ppu.ppu_control_register_1, 0x12);
            assert_eq!(*ppu.ppu_control_register_2, 0x23);
            assert_eq!(*ppu.ppu_status_register, 0x31);
            assert_eq!(*ppu.spr_ram_address_register, 0x48);
            assert_eq!(*ppu.spr_ram_io_register, 0x51);
            assert_eq!(*ppu.vram_address_register_1, 0x70);
            assert_eq!(*ppu.vram_address_register_2, 0xAB);
            assert_eq!(*ppu.vram_io_register, 0xE2);
        }
    }

    #[test]
    fn ppu_memory_is_shared() {
        let mut memory: Vec<u8> = vec!(0x12, 0x23, 0x31, 0x48, 0x51, 0x70, 0xAB, 0xE2);
        let first_address: *mut u8 = memory.first_mut().unwrap();
        let ppu = PPU::new(first_address);

        unsafe {
            assert_eq!(*ppu.ppu_control_register_1, 0x12);
            memory[0] = 0x23;
            assert_eq!(*ppu.ppu_control_register_1, 0x23);

            *ppu.ppu_status_register = 0x40;
            assert_eq!(memory[2], 0x40);
        }
    }
}