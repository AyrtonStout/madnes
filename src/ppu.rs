use ppu_memory::PPUMemory;
use game_window::GameWindow;
use controlletron::Controlletron;
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
    scanline_counter: u16, // Tracks which scanline is currently being rendered
    clock_cycle_counter: u16, // Tracks when to perform the next scanline. Each scanline lasts for 341 PPU clock cycles
    object_attribute_memory: [u8; 0x100], // Stores current sprite data to render. Copied here by the CPU writing to 0x4014
    high_byte_write: bool, // Used by $2005 and $2006 to control which part of the buffer is written to
    scroll_register_t: u16,
    scroll_register_v: u16,
    scroll_register_x: u8,
    internal_read_buffer: u8, // Reads by the CPU from $2007 are delayed one read
    memory: PPUMemory,
    game_window: GameWindow,
    frame_skip: u8,
    odd_frame: bool
}

const SCREEN_WIDTH: u8 = 255;
const SCREEN_HEIGHT: u8 = 240;

impl PPU {
    pub fn new(io_registers: *mut u8) -> PPU {
        unsafe {
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
                clock_cycle_counter: 341,
                object_attribute_memory: [0; 0x100],
                high_byte_write: true,
                scroll_register_t: 0,
                scroll_register_v: 0,
                scroll_register_x: 0,
                internal_read_buffer: 0,
                memory: PPUMemory::new(),
                game_window: GameWindow::new(),
                frame_skip: 0,
                odd_frame: true
            }
        }
    }

    pub fn init_chr_rom(&mut self, chr_rom: Vec<u8>) {
        self.memory.init_chr_rom(chr_rom);
    }

    pub fn tick(&mut self) {
        self.clock_cycle_counter -= 1;

        if self.clock_cycle_counter == 256 && self.is_rendering_enabled() {
            self.increment_vertical_scroll_register();
        }

        if self.clock_cycle_counter == 257 && self.is_rendering_enabled() {
            self.scroll_register_v &= 0b1111_1011_1110_0000;
            self.scroll_register_v |= self.scroll_register_t & 0b0000_0100_0000_0000;
            self.scroll_register_v |= self.scroll_register_t & 0b0000_0000_0001_1111;
        }

        // "If rendering is enabled, at the end of vblank, shortly after the horizontal bits are copied from t to v at dot 257,
        // the PPU will repeatedly copy the vertical bits from t to v from dots 280 to 304, completing the full initialization of v from t"
        if self.scanline_counter == 20 && self.clock_cycle_counter >= 280 && self.clock_cycle_counter <= 304 && self.is_rendering_enabled() {
            self.scroll_register_v &= 0b0000_0100_0001_1111;
            self.scroll_register_v |= self.scroll_register_t & 0b0111_1000_0000_0000;
            self.scroll_register_v |= self.scroll_register_t & 0b0000_0011_1110_0000;
        }

        if (self.clock_cycle_counter >= 328 || self.clock_cycle_counter <= 256) && self.clock_cycle_counter % 8 == 0 {
            if self.is_rendering_enabled() {
//                if (self.scroll_register_v & 0b0000_0000_0001_1111) == 0x1F {
//                    self.scroll_register_v &= 0b1111_1111_1110_0000; // Clear coarse X
//                    self.scroll_register_v ^= 0b0000_0100_0000_0000; // Switch the horizontal nametable
//                } else {
//                    self.scroll_register_v += 1;
//                }
            }
        }

        if self.clock_cycle_counter > 0 {
            return
        } else {
            // Each scanline lasts 341 (or 340) cycles. This is an imperfect representation, as we render a
            // scanline all at once, instead of doing it one pixel at a time
            self.clock_cycle_counter = 341;

            // This scanline varies in length, depending on whether an even or an odd frame is being rendered.
            // For odd frames, the cycle at the end of the scanline is skipped
            if self.scanline_counter == 20 && self.odd_frame {
                self.clock_cycle_counter -= 1;
            }
        }

        self.scanline_counter += 1;

        if self.scanline_counter == 45 {
//            self.set_sprite0_hit(true);
        }

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
            if self.frame_skip != 0 {
                return;
            }
            // The screen height is 240, one scanline per pixel. Start at 0 and work up to 240 (so we subtract the previous 21 scanlines from this value)
            let scanline_height = (self.scanline_counter - 21) as u8;
            self.draw_scanline(scanline_height);
        } else {
            if self.is_background_rendered() || self.are_sprites_rendered() {
//                let start = Instant::now();
                if self.frame_skip <= 0 {
                    self.game_window.repaint();
                    self.frame_skip = 1;
                } else {
                    self.frame_skip -= 1;
                }
            }

            self.set_vblank_status(true);
            self.scanline_counter = 0;
            self.set_sprite0_hit(false);
            self.odd_frame = !self.odd_frame;
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

    fn draw_scanline(&mut self, line_num: u8) {
        if self.is_background_rendered() {
            self.draw_background(line_num);
        }

        // TODO only the first 8 sprites should be drawn and a sprite overflow flag should be set
        // https://wiki.nesdev.com/w/index.php/Sprite_overflow_games
        if self.are_sprites_rendered() {
            // The sprite rendering lags behind background rendering by 1 scanline... I think
            if line_num > 0 {
                self.draw_sprites(line_num - 1);
            }
        }
    }

    fn draw_sprites(&mut self, line_num: u8) {
        if self.using_16px_height_sprites() {
            panic!("16px sprites are not yet supported!");
        }

        let num_sprites = 64; // Maximum number of sprites an NES game can hold in sprite memory
        let oam_entry_size = 4; // Each sprite takes up 4 bytes

        // Now iterate through all sprites backwards.
        // Backwards because earlier sprites have priority, and need to overwrite later sprites
        for offset in (0..num_sprites).rev() {
            let start_address = offset * oam_entry_size;

            // For whatever reason the NES renders sprites one pixel lower than they say they are. So add 1 to the y_offset here
//            let y_offset = self.object_attribute_memory[start_address].wrapping_add(1);
            let y_offset = self.object_attribute_memory[start_address];

            // This sprite is above the max height of the screen and isn't supposed to be rendered. Just skip any additional logic
            // Otherwise, check if the sprite will be rendered by the current scan line's line height
            if y_offset < line_num || y_offset >= line_num + 8 {
                continue;
            }

            let x_offset = self.object_attribute_memory[start_address + 3];
            let sprite_flags = self.object_attribute_memory[start_address + 2];

            let flip_x = (sprite_flags & 0b0100_0000) != 0;
            let flip_y = (sprite_flags & 0b1000_0000) != 0;
            let force_draw = (sprite_flags & 0b0010_0000) == 0;

            let pattern_num = self.object_attribute_memory[start_address + 1];
            let pattern = self.get_pattern(pattern_num, true, flip_x, flip_y);

            // Check for sprite 0 hit
            if offset == 0 && !self.is_sprite0_hit() {
                self.check_sprite0_hit(pattern, x_offset, y_offset, line_num);
            }

            self.send_pattern_to_window(pattern, x_offset as i16, y_offset, line_num,false, force_draw);
        }
    }

    fn draw_background(&mut self, line_num: u8) {
        let tiles_per_row = 32;
        let nametable_size: u16 = 960;
        let start_address = self.get_base_nametable_address();

        for offset in 0..nametable_size {
            // FIXME I should be able to instead modify the loop to only iterate over things being drawn, but that hurts my head right now
            let start_y: u8 = ((offset / tiles_per_row) * 8) as u8;
            if start_y < line_num || start_y >= line_num + 8 {
                continue;
            }

            let pattern_num = self.memory.get_8_bit_value(start_address + offset);
            let pattern = self.get_pattern(pattern_num, false, false, false);
            let start_x: i16 = ((offset % tiles_per_row) * 8) as i16 - (self.get_coarse_x() * 8 + self.get_fine_x()) as i16;
            self.send_pattern_to_window(pattern, start_x, start_y, line_num, true, false);
        }
    }

    fn send_pattern_to_window(&mut self, pattern: [[u8; 8]; 8], start_x: i16, start_y: u8, line_num: u8, draw_transparent: bool, force_draw: bool) {
        let y: usize = (start_y - line_num) as usize; // We only draw one scanline of the pattern. That scanline number is the y value of the sprite
        for x in 0..pattern.len() {
            let drawn_x = start_x + x as i16;
            let drawn_y = start_y as u16 + y as u16; // Do math greater than a u8 so we can abort drawing if it's out of screen
            if drawn_x > SCREEN_WIDTH as i16 || drawn_x < 0 || drawn_y >= SCREEN_HEIGHT as u16 || drawn_y < 0 {
                continue;
            }

            // Don't draw a transparent pixel if we aren't told to draw transparent pixels
            if pattern[x][y] == 0 && !draw_transparent {
                continue;
            }

            // Now draw our pixel, if the background doesn't have higher priority than us
            if force_draw || self.game_window.is_pixel_transparent(drawn_x as u8, drawn_y as u8) {
                self.game_window.set_pixel_color(pattern[x][y], drawn_x as u8, drawn_y as u8);
            }
        }
    }

    fn get_base_nametable_address(&self) -> u16 {
        let nametable_select = (self.scroll_register_v & 0b0000_1100_0000_0000) >> 10;
        return 0x2000 + (0x400 * nametable_select as u16);
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

    fn is_rendering_enabled(&self) -> bool {
        return self.is_background_rendered() || self.are_sprites_rendered();
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

    fn get_address_increment_amount(&self) -> u8 {
        unsafe {
            let bit_set = (*(self.ppu_control_register) & 0b0000_0100) != 0;
            return if bit_set { 32 } else { 1 }
        }
    }

    fn increment_horizontal_scroll_register(&mut self) {
        self.scroll_register_v += self.get_address_increment_amount() as u16;
    }

    fn increment_vertical_scroll_register(&mut self) {
        if (self.scroll_register_v & 0x7000) != 0x7000 {
            self.scroll_register_v += 0x1000;
        } else {
            self.scroll_register_v &= !0x7000;
            let mut y = (self.scroll_register_v & 0x03E0) >> 5;
            if y == 29 {
                y = 0;
                self.scroll_register_v ^= 0x0800;
            } else if y == 31 {
                y = 0;
            } else {
                y += 1;
            }
            self.scroll_register_v = (self.scroll_register_v & !0x03E0) | (y << 5);
        }
    }

    // This happens on reading 0x2007
    pub fn read_from_ppu_data(&mut self) -> u8 {
        if !self.is_rendering_enabled() {
            self.increment_horizontal_scroll_register();
        }

//        return self.memory.get_8_bit_value(self.scroll_register_v);
        let address = self.scroll_register_v;

        if address < 0x3F00 {
            // Reads from most of VRAM are buffered and delayed by one read
            let rval = self.internal_read_buffer;
            self.internal_read_buffer = self.memory.get_8_bit_value(self.scroll_register_v);

            return rval;
        } else {
            // Reads on palette data happen right away, but still set the internal read buffer to the value
            // it would read from one "page" of memory down (or by subtracting 0x1000 from the address)
            self.internal_read_buffer = self.memory.get_8_bit_value(self.scroll_register_v - 0x1000);
            return self.memory.get_8_bit_value(self.scroll_register_v);
        }
    }

    // CPU needs to call this whenever it reads from 0x2002
    pub fn reset_high_byte_read(&mut self) {
        self.high_byte_write = true;
    }

    pub fn write_to_register(&mut self, address: u16, value: u8) {
        if address == 0x2000 {
            self.scroll_register_t &= 0b0111_0011_1111_1111;
            self.scroll_register_t |= ((value & 0b0000_0011) as u16) << 10;
        } else if address == 0x2004 {
            unsafe {
                let oam_address =  *self.spr_ram_address_register;
                self.object_attribute_memory[oam_address as usize] = value;
                *self.spr_ram_address_register = oam_address.wrapping_add(1);
            }
        } else if address == 0x2005 {
//            println!("High Byte Write 2005: {} {:X}", self.high_byte_write, value);
            if self.high_byte_write {
                self.scroll_register_t &= 0b0111_1111_1110_0000;
                self.scroll_register_t |= (value >> 3) as u16;

                self.scroll_register_x = value & 0b0000_0111;
            } else {
                self.scroll_register_t &= 0b0000_1100_0001_1111;
                self.scroll_register_t |= ((value & 0b1100_0000) as u16) << 2;
                self.scroll_register_t |= ((value & 0b0011_1000) as u16) << 2;
                self.scroll_register_t |= ((value & 0b0000_0111) as u16) << 12;
            }
            self.high_byte_write = !self.high_byte_write;
        } else if address == 0x2006 {
//            println!("High Byte Write 2006: {} {:X}", self.high_byte_write, value);
            if self.high_byte_write {
                self.scroll_register_t &= 0b0000_0000_1111_1111;
                self.scroll_register_t |= ((value & 0b0011_1111) as u16) << 8;
            } else {
//                println!("2006: {:X}", value);
                self.scroll_register_t &= 0b1111_1111_0000_0000;
                self.scroll_register_t |= value as u16;

                self.scroll_register_v = self.scroll_register_t;
//                println!("V: {:X}", self.scroll_register_v);
            }
            self.high_byte_write = !self.high_byte_write;
        } else if address == 0x2007 {
            self.memory.set_8_bit_value(self.scroll_register_v, value);

            if !self.is_rendering_enabled() {
                self.increment_horizontal_scroll_register();
            }
        }
    }

    // Used to fill up the OAM table with new sprite data. No idea what DMA actually stands for
    pub fn receive_dma(&mut self, index: u8, sprite_data: u8) {
        self.object_attribute_memory[index as usize] = sprite_data;
    }

    pub fn get_controlletron(&mut self) -> *mut Controlletron {
        return &mut self.game_window.controlletron;
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

    // Iterate over the pixels of sprite0 on the current scanline (line_num) and check if both the sprite
    // and the background both have non-transparent pixel values. If they do, set sprite0 as being hit
    fn check_sprite0_hit(&mut self, sprite0: [[u8; 8]; 8], x_offset: u8, y_offset: u8, line_num: u8) {
        let sprite_line = y_offset - line_num;
//        println!("{:X} {:X} {:X}", sprite_line, y_offset, line_num);
        for x in 0..sprite0.len() {
            // TODO this background_pixel_exists bool should be negated for the logic to be sound... but sprite0 is never triggered when it is
            let background_pixel_exists = self.game_window.is_pixel_transparent(x_offset + x as u8, line_num);
            let sprite_pixel_exists = sprite0[x][sprite_line as usize] != 0;
//            println!("{} {}", background_pixel_exists, sprite_pixel_exists);
            if sprite_pixel_exists && background_pixel_exists {
//                println!("Hit!");
//                println!("-----------");
//                self.debug_draw_sprite(sprite0);
//                println!("--");
//                self.debug_background(x_offset, y_offset);
                self.set_sprite0_hit(true);
                return;
            }
        }
    }

    #[allow(dead_code)]
    fn debug_draw_sprite(&self, sprite: [[u8; 8]; 8]) {
        for y in 0..sprite.len() {
            for x in 0..sprite.len() {
                print!("{}", sprite[x][y])
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn debug_background(&self, x_offset: u8, y_offset: u8) {
        for y in 0..8 {
            for x in 0..8 {
                print!("{}", self.game_window.get_pixel_value(x_offset + x, y_offset + y));
            }
            println!();
        }
    }

    fn is_sprite0_hit(&self) -> bool {
        unsafe {
            return (*self.ppu_status_register & 0b0100_0000) != 0;
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

    fn get_coarse_x(&self) -> u8 {
        return (self.scroll_register_v & 0b0000_0000_0001_1111) as u8;
    }

    #[allow(dead_code)]
    fn get_coarse_y(&self) -> u8 {
        return ((self.scroll_register_v & 0b0000_0011_1110_0000) >> 5) as u8;
    }

    fn get_fine_x(&self) -> u8 {
        return self.scroll_register_x;
    }

    #[allow(dead_code)]
    fn get_fine_y(&self) -> u8 {
        return ((self.scroll_register_v & 0b0111_0000_0000_0000) >> 12) as u8;
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