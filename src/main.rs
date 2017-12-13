use std::io::prelude::*;
use std::fs::File;

mod rom_header;
use rom_header::RomHeader as RomHeader;

fn main() {
	read_file();
}

fn read_file() {
    let mut buffer = vec![0; 10];
//    let mut file = File::open("Super Mario Bros 3 (E).nes").expect("Bad things");
    let mut file = File::open("Contra (USA).nes").expect("Bad things");

    file.read_to_end(&mut buffer).expect("More bad things");

    let rom_data = buffer.as_slice(); // We no longer need the mutable vector
    let rom_start = find_start_of_rom_data(rom_data).expect("Bad thing dude");

    let header = parse_header_struct(rom_start, rom_data).expect("Disaster");
    println!("{}", header.prg_rom_size);
}

fn find_start_of_rom_data(buffer: &[u8]) -> Result<usize, String> {
    if buffer.len() == 0 {
        return Err("ROM data buffer was empty".to_owned());
    }

    let first_index  = buffer.iter().position(|&x| x != 0);

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

            return Ok(index + 4); // Index of the byte following 'N' 'E' 'S' 'EOF'
        }
    }
}

fn parse_header_struct(start_index: usize, rom_data: &[u8]) -> Result<RomHeader, String> {
    if start_index + 6 >= rom_data.len() {
        return Err("Rom data is not large enough to parse headers from".to_owned());
    }

    return Ok(RomHeader {
        prg_rom_size: rom_data[start_index],
        chr_rom_size: rom_data[start_index + 1],
        flags6: rom_data[start_index + 2],
        flags7: rom_data[start_index + 3],
        prg_ram_size: rom_data[start_index + 4],
        flags9: rom_data[start_index + 5],
        flags10: rom_data[start_index + 6]
    });
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
        assert_eq!(6, res.unwrap());
    }

    #[test]
    fn fails_to_parse_too_small_header() {
        let nes_data: [u8; 8] = [0, 0, 16, 8, 10, 5, 5, 5];
        let res = super::parse_header_struct(2, &nes_data);
        assert_eq!(true, res.is_err());
    }

    #[test]
    fn can_parse_header_data() {
        let nes_data: [u8; 9] = [0, 0, 16, 8, 2, 0, 1, 0, 0]; // Omitted previous header data for brevity
        let res = super::parse_header_struct(2, &nes_data);
        let header = res.unwrap();

        assert_eq!(header.prg_rom_size, 16);
        assert_eq!(header.chr_rom_size, 8);
        assert_eq!(header.flags6, 2);
        assert_eq!(header.flags7, 0);
        assert_eq!(header.prg_ram_size, 1);
        assert_eq!(header.flags9, 0);
        assert_eq!(header.flags10, 0);
    }
}
