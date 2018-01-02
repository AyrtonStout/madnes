
mod rom_header;
mod cpu_memory;

mod rom;
use rom::Rom as Rom;

mod instruction_set;

mod cpu;
use cpu::CPU as CPU;


fn main() {
    let rom: Rom = rom::read_file().expect("Wow just terrible");

    for i in 0..rom.prg_rom.len() {
        print!("{:X} ", rom.prg_rom[i]);
    }
//    println!("{}", rom.rom_header.chr_rom_size);
    println!("{}", rom.prg_rom.len());
    println!("{}", rom.chr_rom.len());

//    println!("{}", rom.rom_header.get_mapper_number());

    let mut cpu: CPU = CPU::new();

    cpu.read_program_instructions(rom.prg_rom);
}

/*
fn main() {
    rom::print_rom(40);
}
*/

