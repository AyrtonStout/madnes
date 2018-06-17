extern crate sdl2;
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[allow(dead_code)]
pub struct GameWindow {
    things_to_draw: [[u8; 30]; 32],
    canvas: Canvas<Window>,
    pub sdl_context: Sdl
}

#[allow(dead_code)]
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

        canvas.present();

        return GameWindow {
            things_to_draw: [[0u8; 30]; 32], // TODO make it a color
            canvas,
            sdl_context
        }
    }

    pub fn repaint(&mut self) {
        self.canvas.clear();
        for y in 0..self.things_to_draw.len() {
            for x in 0..self.things_to_draw[0].len() {
                let color_num = self.things_to_draw[x][y];
                let color = self.get_color(color_num);
                self.canvas.set_draw_color(color);
                self.canvas.fill_rect(Rect::new(10, 10, 20, 20)).expect("Hello");
            }
        }
        self.canvas.present();

//        let mut event_pump = self.sdl_context.event_pump().unwrap();

//        for _event in event_pump.poll_iter() { }
    }

    pub fn get_color(&self, color: u8) -> Color {
        match color {
            0 => Color::RGB(0, 0, 0),
            1 => Color::RGB(75, 75, 75),
            2 => Color::RGB(150, 150, 150),
            _ => Color::RGB(255, 255, 255)
        }
    }

    // Color will need to be better than a u8 later
    pub fn set_pixel_color(&mut self, color: u8, x: u8, y: u8) {
        self.things_to_draw[y as usize][x as usize] = color;
    }
}


