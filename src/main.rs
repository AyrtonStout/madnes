use std::io::prelude::*;
use std::fs::File;

mod rom_header;
use rom_header::RomHeader as RomHeader;

fn main() {
	println!("Hello World!");
	read_file();
}

fn read_file() {
    let mut buffer = vec![0; 10];
//    let mut file = File::open("Super Mario Bros 3 (E).nes").expect("Bad things");
    let mut file = File::open("Contra (USA).nes").expect("Bad things");

    file.read_to_end(&mut buffer).expect("More bad things");

    let _header = RomHeader {
        prg_rom_size: 0,
        chr_rom_size: 0,
        flags6: 0,
        flags7: 0,
        prg_ram_size: 0,
        flags9: 0,
        flags10: 0
    };

    let rom_start = find_start_of_rom_data(buffer).expect("Bad thing dude");
    println!("{}", rom_start);
}

fn find_start_of_rom_data(buffer: Vec<u8>) -> Result<u32, String> {
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

            return Ok(index as u32 + 4); // Index of the byte following 'N' 'E' 'S' 'EOF'
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fails_on_no_nes_data() {
        let nes_data: Vec<u8> = vec![];
        let res = super::find_start_of_rom_data(nes_data);
        assert_eq!(true, res.is_err());
    }

    #[test]
    fn fails_on_not_finding_nes_header() {
        let nes_data: Vec<u8> = vec![0, 3, 5];
        let res = super::find_start_of_rom_data(nes_data);
        assert_eq!(true, res.is_err());
    }

    #[test]
    fn fails_on_not_zero_filled_data() {
        let nes_data: Vec<u8> = vec![0, 0, 0];
        let res = super::find_start_of_rom_data(nes_data);
        assert_eq!(true, res.is_err());
    }

    #[test]
    fn fails_on_partial_nes_header() {
        let nes_data: Vec<u8> = vec![0, 0, 'N' as u8, 'E' as u8, 'S' as u8];
        let res = super::find_start_of_rom_data(nes_data);
        assert_eq!(true, res.is_err());
    }

    #[test]
    fn can_find_start_of_rom_data() {
        let nes_data: Vec<u8> = vec![0, 0, 'N' as u8, 'E' as u8, 'S' as u8, 0x1A, 20, 15];
        let res = super::find_start_of_rom_data(nes_data);
        assert_eq!(6, res.unwrap());
    }
}
