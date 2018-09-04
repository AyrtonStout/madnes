extern crate sdl2;
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use controlletron::Controlletron as Controlletron;

#[allow(dead_code)]
pub struct GameWindow {
    things_to_draw: [[u8; 240]; 256],
    canvas: Canvas<Window>,
    pub sdl_context: Sdl,
    pub controlletron: Controlletron
}

const SCREEN_WIDTH: u16 = 256;
const SCREEN_HEIGHT: u16 = 240;
const SCALING: u8 = 3;

#[allow(dead_code)]
impl GameWindow {
    //noinspection RsFieldInitShorthand
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
            things_to_draw: things_to_draw, // TODO make it a color
            canvas: canvas,
            sdl_context: sdl_context,
            controlletron: Controlletron::new()
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
                Event::KeyDown { keycode, ..} => {
                    self.controlletron.receive_key_input(keycode.unwrap(), true)
                },
                Event::KeyUp { keycode, ..} => {
                    self.controlletron.receive_key_input(keycode.unwrap(), false)
                }
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

    // Color will need to be better than a u8 later
    pub fn set_pixel_color(&mut self, color: u8, x: u8, y: u8) {
        self.things_to_draw[x as usize][y as usize] = color;
    }

    // TODO make transparency work with color
    pub fn is_pixel_transparent(&self, x: u8, y: u8) -> bool {
        return self.things_to_draw[x as usize][y as usize] == 0;
    }

    // Used for debugging right now
    pub fn get_pixel_value(&self, x: u8, y: u8) -> u8 {
        return self.things_to_draw[x as usize][y as usize];
    }

    pub fn get_color(&self, color: u8) -> Color {
        match color {
            0x00 => Color::RGB(84, 84, 84),
            0x01 => Color::RGB(0, 20, 116),
            0x02 => Color::RGB(8, 16, 144),
            0x03 => Color::RGB(48, 0, 136),
            0x04 => Color::RGB(68, 0, 100),
            0x05 => Color::RGB(92, 0, 48),
            0x06 => Color::RGB(84, 4, 0),
            0x07 => Color::RGB(60, 24, 0),
            0x08 => Color::RGB(32, 42, 0),
            0x09 => Color::RGB(8, 58, 0),
            0x0A => Color::RGB(0, 64, 0),
            0x0B => Color::RGB(0, 60, 0),
            0x0C => Color::RGB(0, 50, 60),
            0x0D => Color::RGB(0, 0, 0),
            0x0E => Color::RGB(0, 0, 0),
            0x0F => Color::RGB(0, 0, 0),

            0x10 => Color::RGB(152, 150, 152),
            0x11 => Color::RGB(8, 76, 196),
            0x12 => Color::RGB(48, 50, 226),
            0x13 => Color::RGB(92, 30, 228),
            0x14 => Color::RGB(136, 20, 176),
            0x15 => Color::RGB(160, 20, 100),
            0x16 => Color::RGB(152, 34, 32),
            0x17 => Color::RGB(120, 60, 0),
            0x18 => Color::RGB(84, 90, 0),
            0x19 => Color::RGB(40, 114, 0),
            0x1A => Color::RGB(8, 124, 0),
            0x1B => Color::RGB(0, 118, 40),
            0x1C => Color::RGB(0, 102, 120),
            0x1D => Color::RGB(0, 0, 0),
            0x1E => Color::RGB(0, 0, 0),
            0x1F => Color::RGB(0, 0, 0),

            0x20 => Color::RGB(236, 238, 236),
            0x21 => Color::RGB(76, 154, 236),
            0x22 => Color::RGB(120, 124, 236),
            0x23 => Color::RGB(176, 98, 236),
            0x24 => Color::RGB(228, 84, 236),
            0x25 => Color::RGB(236, 88, 180),
            0x26 => Color::RGB(236, 106, 100),
            0x27 => Color::RGB(212, 136, 32),
            0x28 => Color::RGB(160, 170, 0),
            0x29 => Color::RGB(116, 196, 0),
            0x2A => Color::RGB(76, 208, 32),
            0x2B => Color::RGB(56, 204, 108),
            0x2C => Color::RGB(56, 180, 204),
            0x2D => Color::RGB(60, 60, 60),
            0x2E => Color::RGB(0, 0, 0),
            0x2F => Color::RGB(0, 0, 0),

            0x30 => Color::RGB(236, 238, 236),
            0x31 => Color::RGB(168, 204, 236),
            0x32 => Color::RGB(188, 188, 236),
            0x33 => Color::RGB(212, 178, 236),
            0x34 => Color::RGB(236, 174, 236),
            0x35 => Color::RGB(236, 174, 212),
            0x36 => Color::RGB(236, 180, 176),
            0x37 => Color::RGB(228, 196, 114),
            0x38 => Color::RGB(204, 210, 120),
            0x39 => Color::RGB(180, 222, 120),
            0x3A => Color::RGB(168, 226, 114),
            0x3B => Color::RGB(152, 226, 180),
            0x3C => Color::RGB(160, 214, 228),
            0x3D => Color::RGB(160, 162, 160),
            0x3E => Color::RGB(0, 0, 0),
            0x3F => Color::RGB(0, 0, 0),
            _ => panic!("Encountered unexpected color value: {}", color)
        }
    }

}


