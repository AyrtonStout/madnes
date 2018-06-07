extern crate sdl2;

//use sdl2::event::Event;
//use sdl2::keyboard::Keycode;

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
//use game_window::GameWindow;
//use std::thread;
//use std::time::Duration;

fn main() {

    /*
    let mut window = GameWindow::new();


    let mut event_pump = window.sdl_context.event_pump().unwrap();

    let mut x = 0;
    let mut y = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            println!("{:?}", event);
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    x -= 1;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    y -= 1;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    x += 1;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    y += 1;
                }
                _ => {}
            }
        }

        window.repaint();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...
    }
    */

//    /*
    let rom: Rom = rom::read_file().expect("Wow just terrible");
    let mut cpu: CPU = CPU::new();
    cpu.init_prg_rom(rom.prg_rom);
    let mut ppu: PPU = PPU::new(cpu.get_ppu_io_registers_address());
    cpu.init_ppu(&mut ppu as *mut PPU);

    // 46.561 microseconds
//    let sleep_nanoseconds = Duration::new(0, 46_561);

    loop {
        // Jank timer 'implementation'
//        thread::sleep(sleep_nanoseconds);
        cpu.tick();
        ppu.tick();
    }
//    */
}

