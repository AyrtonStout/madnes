
mod rom_header;

mod rom;
use rom::Rom as Rom;

mod instruction_set;
use instruction_set::instruction_set::get_instruction;
use instruction_set::instruction_set::InstructionType;

fn main() {
    let rom: Rom = rom::read_file().expect("Wow just terrible");
    println!("{}", rom.rom_header.prg_rom_size)
}

fn read_program_instructions(start_index: usize, prg_rom_size: u8, rom_data: &[u8]) {
    let bytes_to_read: u32 = prg_rom_size as u32 * 16384;
    let mut current_byte: u32 = 0;

    while current_byte < bytes_to_read {
        let opcode = rom_data[current_byte as usize + start_index];
        println!("Found opcode {:X} at byte {:X}", opcode, current_byte as usize + start_index);
        let instruction: InstructionType = get_instruction(opcode);
        println!("Moving {} bytes forward", instruction.num_bytes);
        current_byte += instruction.num_bytes as u32;
    }

}

