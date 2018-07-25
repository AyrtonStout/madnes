use ppu_memory::PPUMemory;
use game_window::GameWindow;
//use std::time::Instant;

#[allow(dead_code)]
pub struct PPU {
    ppu_control_register: *const u8, // 0x2000 Read-only
    ppu_mask_register: *const u8, // 0x2001 Read-only
    ppu_status_register: *mut u8, // 0x2002 Used by CPU to read status from PPU
    spr_ram_address_register: *mut u8, // 0x2003 Somehow used to load sprites?
    spr_ram_io_register: *const u8, // 0x2004 Also somehow used to load sprites?
    vram_scroll_register: *const u8, // 0x2005 Probably the low byte for a vram read / write (Or maybe this is purely for scrolling?)
    vram_address_register: *const u8, // 0x2006 Probably the high byte for a vram read / write
    vram_data_register: *mut u8, // 0x2007 Reads or writes a byte from VRAM at the current location
    scanline_counter: u16, // Tracks when to VBlank / Render,
    object_attribute_memory: [u8; 0x100], // Stores current sprite data to render. Copied here by the CPU writing to 0x4014
    high_byte_write: bool, // Used by $2005 and $2006 to control which part of the buffer is written to
    vram_scroll_address: u16,
    vram_write_address: u16,
    memory: PPUMemory,
    game_window: GameWindow
}

const SCREEN_WIDTH: u8 = 255;
const SCREEN_HEIGHT: u8 = 240;

impl PPU {
    pub fn new(io_registers: *mut u8) -> PPU {
        unsafe {
//            let game_window = GameWindow::new();
            return PPU {
                ppu_control_register: io_registers.offset(0),
                ppu_mask_register: io_registers.offset(1),
                ppu_status_register: io_registers.offset(2),
                spr_ram_address_register: io_registers.offset(3),
                spr_ram_io_register: io_registers.offset(4),
                vram_scroll_register: io_registers.offset(5),
                vram_address_register: io_registers.offset(6),
                vram_data_register: io_registers.offset(7),
                scanline_counter: 0,
                object_attribute_memory: [0; 0x100],
                high_byte_write: true,
                vram_scroll_address: 0,
                vram_write_address: 0,
                memory: PPUMemory::new(),
                game_window: GameWindow::new()
            }
        }
    }

    pub fn init_chr_rom(&mut self, chr_rom: Vec<u8>) {
        self.memory.init_chr_rom(chr_rom);
    }

    //TODO I don't think the frame counter is actually incremented every clock tick. Sounds like it's more like every 4th tick or something
    pub fn tick(&mut self) {
        self.scanline_counter += 1;

        if self.scanline_counter < 20 {
            // We are in VBlank time and likely will do nothing
        } else if self.scanline_counter == 20 {
            self.set_vblank_status(false);
            self.set_sprite0_hit(true);

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
            // TODO draw things at the appropriate times, not all at once
            if self.is_background_rendered() {
                self.draw_background();
            }
            if self.are_sprites_rendered() {
                self.draw_sprites();
//                self.debug_pattern_table();
            }

            if self.is_background_rendered() || self.are_sprites_rendered() {
//                let start = Instant::now();
                self.game_window.repaint();
//                let end = Instant::now();
//                let elapsed: u64 = end.duration_since(start).subsec_nanos() as u64 / 1000 / 1000;
//                println!("{:?}", elapsed);
            }

            println!("Scroll: {:X} Other: {:X} Coarse: {:X}", self.get_scroll(), self.vram_scroll_address, self.get_coarse_x());

            self.set_vblank_status(true);
            self.scanline_counter = 0;
            self.set_sprite0_hit(false);
        }
    }

    #[allow(dead_code)]
    fn debug_pattern_table(&mut self) {
        for i in 0x0..0x2000 {
            if self.memory.get_8_bit_value(i) != 0 {
                println!("Found something {:X}", i);
                panic!(i);
            }
        }
        println!("Sup");
        let pattern_table_size = 255;
        let pattern_size = 16;
        for offset in 0..pattern_table_size {
            let pattern = self.get_pattern(offset, true, false, false);
            println!("{:X}", offset as u16 * pattern_size as u16);
            for y in 0..8 {
                for x in 0..8 {
                    print!("{}", pattern[x][y]);
                }
                println!();
            }
            println!();
        }
//        panic!();
    }

    fn draw_sprites(&mut self) {
        if self.using_16px_height_sprites() {
            panic!("16px sprites are not yet supported!");
        }

//        println!("{:?}", self.object_attribute_memory.to_vec());
        let num_sprites = 64; // Maximum number of sprites an NES game can display
        let oam_entry_size = 4;
        // Now iterate through all sprites backwards.
        // Backwards because earlier sprites have priority, and need to overwrite later sprites
        for offset in (0..num_sprites).rev() {
            let start_address = offset * oam_entry_size;

            let y_offset = self.object_attribute_memory[start_address].wrapping_add(1);
            // This sprite is above the max height of the screen and isn't supposed to be rendered. Just skip any additional logic
            if y_offset > 0xEF {
                continue;
            }

            let x_offset = self.object_attribute_memory[start_address + 3];
            let sprite_flags = self.object_attribute_memory[start_address + 2];

            let flip_x = (sprite_flags & 0b0100_0000) != 0;
            let flip_y = (sprite_flags & 0b1000_0000) != 0;
            let draw_on_transparent = (sprite_flags & 0b0010_0000) != 0;

            let pattern_num = self.object_attribute_memory[start_address + 1];
            let pattern = self.get_pattern(pattern_num, true, flip_x, flip_y);
            self.send_pattern_to_window(pattern, x_offset, y_offset, true, draw_on_transparent);
        }
    }

    fn draw_background(&mut self) {
        let tiles_per_row = 32;
        let nametable_size: u16 = 960;
        let start_address = self.get_base_nametable_address();

        for offset in 0..nametable_size {
            let pattern_num = self.memory.get_8_bit_value(start_address + offset);
            let pattern = self.get_pattern(pattern_num, false, false, false);
            let start_x: u8 = ((offset % tiles_per_row) * 8) as u8;
            let start_y: u8 = ((offset / tiles_per_row) * 8) as u8;
            self.send_pattern_to_window(pattern, start_x, start_y, false, false);
        }
    }

    fn send_pattern_to_window(&mut self, pattern: [[u8; 8]; 8], start_x: u8, start_y: u8, draw_transparent: bool, draw_on_transparent: bool) {
        for y in 0..pattern[0].len() {
            for x in 0..pattern.len() {
                let drawn_x = start_x as u16 + x as u16; // Do math greater than a u8 so we can abort drawing if it's out of screen
                let drawn_y = start_y as u16 + y as u16;
                if drawn_x > SCREEN_WIDTH as u16 || drawn_y > SCREEN_HEIGHT as u16 {
                    continue;
                }

                // Don't draw a transparent pixel if we aren't told to draw transparent pixels (aka we're a sprite, don't draw over a background)
                if pattern[x][y] == 0 && draw_transparent {
                    continue;
                }

                // Now draw our pixel, if the background doesn't have higher priority than us
                if draw_on_transparent || self.game_window.is_pixel_transparent(x as u8, y as u8) {
                    self.game_window.set_pixel_color(pattern[x][y], drawn_x as u8, drawn_y as u8);
                }
            }
        }
    }

    fn get_base_nametable_address(&self) -> u16 {
        unsafe {
            let bit_values: u8 = *(self.ppu_control_register) & 0b0000_0011;
            return 0x2000 + (0x400 * bit_values as u16);
        }
    }

    // An 8x8 sprite is composed of 8x16 bits. The first 8x8 set is added to the second 8x8 set to get 1 of 4 possible
    // values (0 - 3), each corresponding to a particular color. Though somewhat wasteful on memory, this is represented as
    // an 8x8 array of u8 to make the calling code simpler (no need for calling code to mask bits)
    fn get_pattern(&self, pattern_num: u8, is_sprite_pattern: bool, flip_x: bool, flip_y: bool) -> [[u8; 8]; 8] {
        let start_address = if is_sprite_pattern { self.get_sprite_pattern_table_address() } else { self.get_background_pattern_table_address() };
        let sprite_size: u8 = 8; // This might not be the same for every game. There is a flag to determine this I think

        let mut sprite = [[0u8; 8]; 8];
        for byte_offset in 0..sprite_size {
            // Each line of the sprite is composed of 2 different bytes. The second byte is offset 8 further down than the first. Grab these bytes
            let pattern_address = start_address + byte_offset as u16 + (pattern_num as u16 * sprite_size as u16 * 2);
//            println!("Pattern address: {:X}", pattern_address);
            let low_byte = self.memory.get_8_bit_value(pattern_address);
            let high_byte = self.memory.get_8_bit_value(pattern_address + 8);
            for bit_offset in 0..8 {
                // Now that we have our high and low bytes, combine them into a single sprite row so we can determine color
                let mask = 1 << bit_offset;
                let low_bit = if low_byte & mask != 0 { 1 } else { 0 };
                let high_bit = if high_byte & mask != 0 { 2 } else { 0 };

                // Though it doesn't make a lot of sense to me why, perhaps a bug somewhere else in the emulator,
                // we now default the sprites to horizontally flip so that they look normal
                let x = if flip_x { bit_offset } else { 7u8 - bit_offset };
                let y = if flip_y { byte_offset } else { byte_offset };

                sprite[x as usize][y as usize] = low_bit + high_bit;
            }
        }

        return sprite;
    }

    fn get_sprite_pattern_table_address(&self) -> u16 {
        unsafe {
            let bit_set: bool = (*(self.ppu_control_register) & 0b0000_1000) != 0;
            if bit_set {
                return 0x1000;
            } else {
                return 0x0; // Though I realize 0x0 == 0, putting the 0x in front makes it more obvious to me that I'm referring to an address. Don't hate
            }
        }
    }

    // It seems like you shouldn't have to use two different bits to determine this (sprite vs background). But maybe sometimes you don't use one or the other?
    fn get_background_pattern_table_address(&self) -> u16 {
        unsafe {
            let bit_set: bool = (*(self.ppu_control_register) & 0b0001_0000) != 0;
            if bit_set {
                return 0x1000;
            } else {
                return 0x0;
            }
        }
    }

    fn get_scroll(&self) -> u8 {
        unsafe {
            return *(self.vram_scroll_register);
        }
    }

    #[allow(dead_code)]
    fn using_16px_height_sprites(&self) -> bool {
        unsafe {
            return (*(self.ppu_control_register) & 0b0010_0000) != 0;
        }
    }

    // If this is false, the background shouldn't be rendered on the leftmost 8 pixels
    #[allow(dead_code)]
    fn is_background_to_left_edge(&self) -> bool {
        unsafe {
            return (*(self.ppu_mask_register) & 0b0000_0010) != 0;
        }
    }

    // If this is false, the sprites shouldn't be rendered on the leftmost 8 pixels
    #[allow(dead_code)]
    fn are_sprites_to_left_edge(&self) -> bool {
        unsafe {
            return (*(self.ppu_mask_register) & 0b0000_0100) != 0;
        }
    }

    fn is_background_rendered(&self) -> bool {
        unsafe {
            return (*(self.ppu_mask_register) & 0b0000_1000) != 0;
        }
    }

    fn are_sprites_rendered(&self) -> bool {
        unsafe {
            return (*(self.ppu_mask_register) & 0b0001_0000) != 0;
        }
    }

    // CPU needs to call this whenever it reads from 0x2002
    pub fn status_register_read(&mut self) {
        self.high_byte_write = true;
    }

    pub fn write_to_register(&mut self, address: u16, value: u8) {
        if address == 0x2004 {
            unsafe {
                let oam_address =  *self.spr_ram_address_register;
                self.object_attribute_memory[oam_address as usize] = value;
                *self.spr_ram_address_register = oam_address.wrapping_add(1);
            }
        } else if address == 0x2005 {
            println!("High Byte Write 2005: {} {:X}", self.high_byte_write, value);
            if self.high_byte_write {
                self.vram_scroll_address = ((value as u16) << 8) | (self.vram_scroll_address & 0x00FF);
            } else {
                self.vram_scroll_address = (self.vram_scroll_address & 0xFF00) | (value as u16);
            }
            self.high_byte_write = !self.high_byte_write;
        } else if address == 0x2006 {
            println!("High Byte Write 2006: {} {:X}", self.high_byte_write, value);
            if self.high_byte_write {
                self.vram_write_address = ((value as u16) << 8) | (self.vram_write_address & 0x00FF);
            } else {
                self.vram_write_address = (self.vram_write_address & 0xFF00) | (value as u16);
            }
            self.high_byte_write = !self.high_byte_write;
        } else if address == 0x2007 {
            self.memory.set_8_bit_value(self.vram_write_address, value);

            // TODO this needs to sometimes be 16 or 32 or something based off a PPU CTRL flag
            self.vram_write_address += 1;
        }
    }

    // Used to fill up the OAM table with new sprite data. No idea what DMA actually stands for
    pub fn receive_dma(&mut self, sprite_data: Vec<u8>) {
        for i in 0..self.object_attribute_memory.len() {
            let sprite_byte = sprite_data[i];
            self.object_attribute_memory[i] = sprite_byte;
        }
    }

    fn set_vblank_status(&mut self, is_set: bool) {
//        println!("VBlank set to {}", is_set);
        unsafe {
            if is_set {
                *self.ppu_status_register |= 0b1000_0000;
            } else {
                *self.ppu_status_register &= !0b1000_0000;
            }
        }
    }

    fn set_sprite0_hit(&mut self, is_set: bool) {
        unsafe {
            if is_set {
                *self.ppu_status_register |= 0b0100_0000;
            } else {
                *self.ppu_status_register &= !0b0100_0000;
            }
        }
    }

    fn get_coarse_x(&self) -> u16 {
        return self.vram_scroll_address & 0b0000_0000_0001_1111;
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
            assert_eq!(*ppu.ppu_control_register, 0x12);
            assert_eq!(*ppu.ppu_mask_register, 0x23);
            assert_eq!(*ppu.ppu_status_register, 0x31);
            assert_eq!(*ppu.spr_ram_address_register, 0x48);
            assert_eq!(*ppu.spr_ram_io_register, 0x51);
            assert_eq!(*ppu.vram_scroll_register, 0x70);
            assert_eq!(*ppu.vram_address_register, 0xAB);
            assert_eq!(*ppu.vram_data_register, 0xE2);
        }
    }

    #[test]
    fn ppu_memory_is_shared() {
        let mut memory: Vec<u8> = vec!(0x12, 0x23, 0x31, 0x48, 0x51, 0x70, 0xAB, 0xE2);
        let first_address: *mut u8 = memory.first_mut().unwrap();
        let ppu = PPU::new(first_address);

        unsafe {
            assert_eq!(*ppu.ppu_control_register, 0x12);
            memory[0] = 0x23;
            assert_eq!(*ppu.ppu_control_register, 0x23);

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