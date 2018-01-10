
pub struct PPU {
    ppu_control_register_1: *const u8, // 0x2000 Read-only
    ppu_control_register_2: *const u8, // 0x2001 Read-only
    ppu_status_register: *mut u8, // 0x2002 Used by CPU to read status from PPU
    spr_ram_address_register: *const u8, // 0x2003 Somehow used to load sprites?
    spr_ram_io_register: *const u8, // 0x2004 Also somehow used to load sprites?
    vram_address_register_1: *const u8, // 0x2005 Probably the low byte for a vram read / write
    vram_address_register_2: *const u8, // 0x2006 Probably the high byte for a vram read / write
    vram_io_register: *mut u8, // 0x2007 Reads or writes a byte from VRAM at the current location
    scanline_counter: u16 // Tracks when to VBlank / Render
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
                scanline_counter: 0
            }
        }
    }

    //TODO I don't think the frame counter is actually incremented every clock tick. Sounds like it's more like every 4th tick or something
    pub fn tick(&mut self) {
        self.scanline_counter += 1;

        if self.scanline_counter < 20 {
            // We are in VBlank time and likely will do nothing
        } else if self.scanline_counter == 20 {
            self.set_vblank_status(false);

            /* TODO from http://nesdev.com/2C02%20technical%20reference.TXT
            After 20 scanlines worth of time go by (since the VINT flag was set), the PPU starts to render scanlines. This first scanline is a dummy one;
            although it will access it's external memory in the same sequence it would for drawing a valid scanline, no on-screen pixels are rendered during this
            time, making the fetched background data immaterial. Both horizontal *and* vertical scroll counters are updated (presumably) at cc offset 256 in this
            scanline. Other than that, the operation of this scanline is identical to any other. The primary reason this scanline exists is to start the object
            render pipeline, since it takes 256 cc's worth of time to determine which objects are in range or not for any particular scanline.
            */
        } else if self.scanline_counter < 261 {
            // TODO Scanline logic
        } else {
            self.set_vblank_status(true);
            // TODO Send NMI
            self.scanline_counter = 0;
        }
    }

    fn set_vblank_status(&mut self, is_set: bool) {
        println!("VBlank set to {}", is_set);
        unsafe {
            if is_set {
                *self.ppu_status_register |= 0x80;
            } else {
                *self.ppu_status_register &= !0x80;
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use ppu::PPU;

    fn create_test_ppu() -> PPU {
        let mut memory: Vec<u8> = vec!(0x12, 0x23, 0x31, 0x48, 0x51, 0x70, 0xAB, 0xE2);
        let first_address: *mut u8 = memory.first_mut().unwrap();
        return PPU::new(first_address);
    }

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

    #[test]
    fn vblank_status_bit() {
        let mut ppu: PPU = create_test_ppu();

        unsafe {
            *ppu.ppu_status_register = 0x04;
            ppu.set_vblank_status(true);
            assert_eq!(*ppu.ppu_status_register, 0x84);
            ppu.set_vblank_status(false);
            assert_eq!(*ppu.ppu_status_register, 0x04);
        }
    }
}