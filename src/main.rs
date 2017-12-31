
mod rom_header;

mod rom;
use rom::Rom as Rom;

mod instruction_set;
use instruction_set::instruction_set::get_instruction;
use instruction_set::instruction_set::InstructionType;

fn main() {
    let rom: Rom = rom::read_file().expect("Wow just terrible");

    for i in 0..rom.prg_rom.len() {
        print!("{:X} ", rom.prg_rom[i]);
    }
//    println!("{}", rom.rom_header.chr_rom_size);
    println!("{}", rom.prg_rom.len());
    println!("{}", rom.chr_rom.len());

//    println!("{}", rom.rom_header.get_mapper_number());
    read_program_instructions(rom.prg_rom);

}

/*
fn main() {
    rom::print_rom(40);
}
*/

fn read_program_instructions(prg_rom: Vec<u8>) {
    let mut current_byte: u32 = 0;

    while current_byte < prg_rom.len() as u32 {
        let opcode = prg_rom[current_byte as usize];
        println!("Found opcode {:X} at byte {:X}", opcode, current_byte as usize);
        let instruction: InstructionType = get_instruction(opcode);
        println!("Moving {} bytes forward", instruction.num_bytes);
        current_byte += instruction.num_bytes as u32;
    }

}

