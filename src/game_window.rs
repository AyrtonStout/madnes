extern crate sdl2;
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[allow(dead_code)]
pub struct GameWindow {
    things_to_draw: [[u8; 240]; 256],
    canvas: Canvas<Window>,
    scaling: u8,
    pub sdl_context: Sdl
}

#[allow(dead_code)]
impl GameWindow {
    pub fn new() -> GameWindow {

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let scaling: u8 = 3;

        let width = 256 * scaling as u32;
        let height = 240 * scaling as u32;
        let window = video_subsystem.window("MadNes", width, height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.present();

        return GameWindow {
            things_to_draw: [[0u8; 240]; 256], // TODO make it a color
            canvas,
            scaling,
            sdl_context
        }
    }

    pub fn repaint(&mut self) {
//        println!("Start paint!");
//        self.canvas.clear();
        for y in 0..self.things_to_draw[0].len() {
            for x in 0..self.things_to_draw.len() {
                let color_num = self.things_to_draw[x][y];
                let color = self.get_color(color_num);
                let scaling = self.scaling as i32;
                self.canvas.set_draw_color(color);
                self.canvas.fill_rect(Rect::new(x as i32 * scaling, y as i32 * scaling,
                                                1 * scaling as u32, 1 * scaling as u32)).expect("Hello");
            }
        }
        self.canvas.present();

        let mut event_pump = self.sdl_context.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    panic!("Exiting game");
                },
                _ => { }
            }
        }
//        println!("Stop paint!");
    }

    pub fn get_color(&self, color: u8) -> Color {
        match color {
            0 => Color::RGB(0, 0, 0),
            1 => Color::RGB(75, 75, 75),
            2 => Color::RGB(150, 150, 150),
            3 => Color::RGB(255, 255, 255),
            _ => panic!("Encountered unexpected color value: {}", color)
        }
    }

    // Color will need to be better than a u8 later
    pub fn set_pixel_color(&mut self, color: u8, x: u8, y: u8) {
        self.things_to_draw[x as usize][y as usize] = color;
    }
}


