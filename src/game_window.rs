extern crate sdl2;
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[allow(dead_code)]
pub struct GameWindow {
    things_to_draw: [[u8; 240]; 256],
    canvas: Canvas<Window>,
    pub sdl_context: Sdl
}

const SCREEN_WIDTH: u16 = 256;
const SCREEN_HEIGHT: u16 = 240;
const SCALING: u8 = 3;

#[allow(dead_code)]
impl GameWindow {
    pub fn new() -> GameWindow {

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("MadNes", SCREEN_WIDTH as u32 * SCALING as u32, SCREEN_HEIGHT as u32 * SCALING as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_scale(SCALING as f32, SCALING as f32).unwrap();

        let things_to_draw = [[0u8; 240]; 256];

        canvas.present();

        return GameWindow {
            things_to_draw, // TODO make it a color
            canvas,
            sdl_context
        }
    }

    pub fn repaint(&mut self) {
        self.canvas.clear();

        self.create_texture();
        self.canvas.present();
        self.things_to_draw = [[0u8; 240]; 256];

        let mut event_pump = self.sdl_context.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    panic!("Exiting game");
                },
                _ => { }
            }
        }
    }

    fn create_texture(&mut self) {
        let texture_creator = self.canvas.texture_creator();
        let width = 256;
        let height = 240;

        let mut texture = texture_creator.create_texture_streaming(
            PixelFormatEnum::RGB24, width, height).unwrap();

        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..240 {
                for x in 0..255 {
                    let offset = x*3 + y*pitch;
                    let color = self.get_color(self.things_to_draw[x][y]);
                    buffer[offset] = color.r;
                    buffer[offset + 1] = color.g;
                    buffer[offset + 2] = color.b;
                }
            }
        }).unwrap();

        self.canvas.copy(&texture, None, Some(Rect::new(0, 0, width, height))).unwrap();
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

    pub fn is_pixel_transparent(&self, x: u8, y: u8) -> bool {
        return self.things_to_draw[x as usize][y as usize] == 0;
    }
}


