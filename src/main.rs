mod rom_header;
mod cpu_memory;
mod rom;
mod cpu;
mod ppu;
mod instruction_set;

use rom::Rom as Rom;
use cpu::CPU as CPU;
use ppu::PPU as PPU;
use std::thread;
use std::time::Duration;

fn main() {
    let rom: Rom = rom::read_file().expect("Wow just terrible");
    let mut cpu: CPU = CPU::new();
    cpu.init_prg_rom(rom.prg_rom);
    let mut ppu: PPU = PPU::new(cpu.get_ppu_io_registers_address());

    // 46.561 microseconds
    let sleep_nanoseconds = Duration::new(0, 46_561_000);

    loop {
        // Jank timer 'implementation'
        thread::sleep(sleep_nanoseconds);
        cpu.tick();
        ppu.tick();
    }
}

/*
fn main() {
    rom::print_rom(40);
}
*/

