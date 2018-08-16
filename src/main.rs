extern crate sdl2;

mod rom_header;
mod cpu_memory;
mod rom;
mod cpu;
mod ppu;
mod ppu_memory;
mod instruction_set;
mod game_window;

use rom::Rom as Rom;
use cpu::CPU as CPU;
use ppu::PPU as PPU;
//use std::thread;
//use std::time::Duration;

fn main() {

    let rom: Rom = rom::read_file().expect("Wow just terrible");
    let mut cpu: CPU = CPU::new();
    cpu.init_prg_rom(rom.prg_rom);
    let mut ppu: PPU = PPU::new(cpu.get_ppu_io_registers_address());
    cpu.init_ppu(&mut ppu as *mut PPU);
    ppu.init_chr_rom(rom.chr_rom);

    // 46.561 microseconds
//    let sleep_nanoseconds = Duration::new(0, 46_561);

    let mut clock_cycle = 0;
    loop {
        // The CPU receives a clock signal only once for every 3 times the PPU does
        clock_cycle -= 1;
        if clock_cycle <= 0 {
            cpu.tick();
            clock_cycle = 3;
        }

        ppu.tick();

        // Jank timer 'implementation'
//        thread::sleep(sleep_nanoseconds);
    }
//    */
}

