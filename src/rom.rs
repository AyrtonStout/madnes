use std::fs::File;
use std::io::prelude::*;

use rom_header::RomHeader as RomHeader;

pub struct Rom {
    pub rom_header: RomHeader,
    pub trainer: Vec<u8>,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub playchoice_inst_rom: Vec<u8>,
    pub playchoice_prom: Vec<u8>,
}

pub fn read_file() -> Result<Rom, String> {
    let mut buffer = vec![0; 10];
//    let mut file = File::open("Super Mario Bros 3 (E).nes").expect("Bad things");
    let mut file = File::open("Super Mario Bros (E).nes").expect("Bad things");
//    let mut file = File::open("Contra (USA).nes").expect("Bad things");

    file.read_to_end(&mut buffer).expect("More bad things");

    let rom_data = buffer.as_slice(); // We no longer need the mutable vector
    let mut current_rom_offset = find_start_of_rom_data(rom_data).expect("Bad thing dude");

    let header = parse_header_struct(&rom_data[current_rom_offset..]).expect("Disaster");

    current_rom_offset += 16; // 16 is the number of bytes the header takes up in the ROM
    let prg_rom_data = parse_prg_rom_data(header.prg_rom_size, &rom_data[current_rom_offset..]);

    current_rom_offset += prg_rom_data.len();
    let chr_rom_data = parse_chr_rom_data(header.chr_rom_size, &rom_data[current_rom_offset..]);


    if header.rom_has_trainer_data() {
        panic!("ROMs with trainers not yet supported!");
    }

    if header.rom_has_playchoice_data() {
        panic!("ROMs with playchoice data not yet supported!");
    }

    return Ok(Rom {
        rom_header: header,
        trainer: vec!(),
        prg_rom: prg_rom_data,
        chr_rom: chr_rom_data,
        playchoice_inst_rom: vec!(),
        playchoice_prom: vec!()
    });
}

pub fn print_rom(bytes_to_print: usize) {
    let mut buffer = vec![0; 10];
//    let mut file = File::open("Super Mario Bros 3 (E).nes").expect("Bad things");
    let mut file = File::open("Super Mario Bros (E)").expect("Bad things");
//    let mut file = File::open("Contra (USA).nes").expect("Bad things");

    file.read_to_end(&mut buffer).expect("More bad things");

    let rom_data = buffer.as_slice(); // We no longer need the mutable vector

    for i in 0..bytes_to_print {
        print!("{:X} ", rom_data[i]);
    }
}

fn find_start_of_rom_data(buffer: &[u8]) -> Result<usize, String> {
    if buffer.len() == 0 {
        return Err("ROM data buffer was empty".to_owned());
    }

    let first_index = buffer.iter().position(|&x| x != 0);

    match first_index {
        None => return Err("No non-zero data found in ROM".to_owned()),
        Some(index) => {
            if buffer[index] != 'N' as u8 {
                return Err(format!("First non-zero ROM data was {} not 'N'", buffer[index]));
            }

            // We found an N. Check that the following characters are 'E' 'S' 'EOF'
            if index  + 3 >= buffer.len() { // We would access invalid indexes if we continued
                return Err("First non-zero ROM data was 'N' but ROM data was too small to continue parsing".to_owned())
            }

            if buffer[index  + 1] != 'E' as u8 {
                return Err(format!("Found 'N', but 'E' was not the next byte. It was {}", buffer[index  + 1]));
            } else if buffer[index  + 2] != 'S' as u8 {
                return Err(format!("Found 'N' and 'E', but 'S' was not the next byte. It was {}", buffer[index  + 2]));
            } else if buffer[index  + 3] != 0x1A {
                return Err(format!("Found 'N', 'E' and 'S', but the next byte was not the MS-DOS EOF character. It was {}", buffer[index  + 3]));
            }

            return Ok(index); // Index of the 'N' byte in 'N' 'E' 'S' 'EOF'
        }
    }
}

fn parse_header_struct(rom_data: &[u8]) -> Result<RomHeader, String> {
    let index = 4 as usize; // skip past the starting 'N' 'E' 'S' 'EOF'. We don't need to parse these
    if index + 11 >= rom_data.len() {
        return Err("Rom data is not large enough to parse headers from".to_owned());
    }

    for i in 7..12 {
        if rom_data[index + i] != 0 {
            return Err(format!("Expected byte {} to be zero but it was {}", index + i, rom_data[index + i]));
        }
    }

    return Ok(RomHeader {
        prg_rom_size: rom_data[index],
        chr_rom_size: rom_data[index + 1],
        flags6: rom_data[index + 2],
        flags7: rom_data[index + 3],
        prg_ram_size: rom_data[index + 4],
        flags9: rom_data[index + 5],
        flags10: rom_data[index + 6]
    });
}

fn parse_prg_rom_data(prg_rom_size: u8, rom_data: &[u8]) -> Vec<u8> {
    let bytes_to_read: u32 = prg_rom_size as u32 * 16384; // prg_rom_size comes in as 16 kb units
    return rom_data[..bytes_to_read as usize].to_vec();
}

fn parse_chr_rom_data(chr_rom_size: u8, rom_data: &[u8]) -> Vec<u8> {
    let bytes_to_read: u32 = chr_rom_size as u32 * 8192; // chr_rom_size comes in as 8 kb units
    return rom_data[..bytes_to_read as usize].to_vec();
}

#[cfg(test)]
mod tests {
    #[test]
    fn fails_on_no_nes_data() {
        let nes_data: [u8; 0] = [];
        let res = super::find_start_of_rom_data(&nes_data);
        assert_eq!(true, res.is_err());
    }

    #[test]
    fn fails_on_not_finding_nes_header() {
        let nes_data: [u8; 3] = [0, 3, 5];
        let res = super::find_start_of_rom_data(&nes_data);
        assert_eq!(true, res.is_err());
    }

    #[test]
    fn fails_on_not_zero_filled_data() {
        let nes_data: [u8; 3] = [0, 0, 0];
        let res = super::find_start_of_rom_data(&nes_data);
        assert_eq!(true, res.is_err());
    }

    #[test]
    fn fails_on_partial_nes_header() {
        let nes_data: [u8; 5] = [0, 0, 'N' as u8, 'E' as u8, 'S' as u8];
        let res = super::find_start_of_rom_data(&nes_data);
        assert_eq!(true, res.is_err());
    }

    #[test]
    fn can_find_start_of_rom_data() {
        let nes_data: [u8; 8] = [0, 0, 'N' as u8, 'E' as u8, 'S' as u8, 0x1A, 20, 15];
        let res = super::find_start_of_rom_data(&nes_data);
        assert_eq!(2, res.unwrap());
    }

    #[test]
    fn fails_to_parse_too_small_header() {
        let nes_data: [u8; 10] = ['N' as u8, 'E' as u8, 'S' as u8, 0x1A, 16, 8, 10, 5, 5, 5];
        let res = super::parse_header_struct(&nes_data);
        assert_eq!(true, res.is_err());
    }

    #[test]
    fn zero_filled_header_data_required() {
        let nes_data: [u8; 16] = ['N' as u8, 'E' as u8, 'S' as u8, 0x1A, 16, 8, 2, 0, 1, 0, 0, 0, 0, 0, 0, 4]; // Last 4 should be zero filled
        let res = super::parse_header_struct(&nes_data);
        assert_eq!(true, res.is_err())
    }

    #[test]
    fn can_parse_header_data() {
        let nes_data: [u8; 16] = ['N' as u8, 'E' as u8, 'S' as u8, 0x1A, 16, 8, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0]; // Omitted previous header data for brevity
        let res = super::parse_header_struct(&nes_data);
        let header = res.unwrap();

        assert_eq!(header.prg_rom_size, 16);
        assert_eq!(header.chr_rom_size, 8);
        assert_eq!(header.flags6, 2);
        assert_eq!(header.flags7, 0);
        assert_eq!(header.prg_ram_size, 1);
        assert_eq!(header.flags9, 0);
        assert_eq!(header.flags10, 0);
    }

    #[test]
    fn parse_rom_trainer_bit() {
        let nes_data_without_trainer: [u8; 16] = ['N' as u8, 'E' as u8, 'S' as u8, 0x1A, 16, 8, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]; // Omitted previous header data for brevity
        let res_without_trainer = super::parse_header_struct(&nes_data_without_trainer);
        let header_without_trainer = res_without_trainer.unwrap();

        let nes_data_with_trainer: [u8; 16] = ['N' as u8, 'E' as u8, 'S' as u8, 0x1A, 16, 8, 4, 0, 1, 0, 0, 0, 0, 0, 0, 0];
        let res_with_trainer = super::parse_header_struct(&nes_data_with_trainer);
        let header_with_trainer = res_with_trainer.unwrap();

        assert_eq!(header_without_trainer.rom_has_trainer_data(), false);
        assert_eq!(header_with_trainer.rom_has_trainer_data(), true);
    }

    #[test]
    fn parse_mapper_number() {
        let nes_data: [u8; 16] = ['N' as u8, 'E' as u8, 'S' as u8, 0x1A, 16, 8, 32, 64, 1, 0, 0, 0, 0, 0, 0, 0]; // Omitted previous header data for brevity
        let res = super::parse_header_struct(&nes_data);
        let header_without_trainer = res.unwrap();

        assert_eq!(header_without_trainer.get_mapper_number(), 66);
    }
}
