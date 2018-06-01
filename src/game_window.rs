extern crate sdl2;
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct GameWindow {
    things_to_draw: [u8; 0x100],
    canvas: Canvas<Window>,
    pub sdl_context: Sdl
}

impl GameWindow {
    pub fn new() -> GameWindow {

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.fill_rect(Rect::new(10, 10, 20, 20));
        canvas.present();


        return GameWindow {
            things_to_draw: [0; 0x100],
            canvas: canvas,
            sdl_context: sdl_context
        }
    }

    pub fn repaint(&mut self) {
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas.set_draw_color(Color::RGB(0, 0, 255));
        self.canvas.fill_rect(Rect::new(10, 10, 20, 20));
        self.canvas.present();
    }

    pub fn set_things_to_draw(&self) {
        println!("Setting things to draw!");
    }

    fn check_valid_write(address: u16) {
        println!("This thing is private!");
    }
}


