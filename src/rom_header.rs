pub struct RomHeader {
     pub prg_rom_size: u8, // Size of PRG ROM in 16 KB units
     pub chr_rom_size: u8, // Size of CHR ROM in 8 KB units (Value 0 means the board uses CHR RAM)
     pub flags6: u8,
     pub flags7: u8,
     pub prg_ram_size: u8, // Size of PRG RAM in 8 KB units (Value 0 infers 8 KB for compatibility; see PRG RAM circuit)
     pub flags9: u8,
     pub flags10: u8 // unofficial flags
}
