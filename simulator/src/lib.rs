extern crate embedded_graphics;
extern crate sdl2;

use embedded_graphics::coord::Coord;
use embedded_graphics::drawable::Pixel;
use embedded_graphics::Drawing;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render;

const DISPLAY_SIZE: usize = 128;

pub struct Display {
    pixels: [[bool; DISPLAY_SIZE]; DISPLAY_SIZE],
    canvas: render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
}

impl Display {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(
                "graphics-emulator",
                DISPLAY_SIZE as u32,
                DISPLAY_SIZE as u32,
            )
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            pixels: [[false; DISPLAY_SIZE]; DISPLAY_SIZE],
            canvas,
            event_pump,
        }
    }

    pub fn run_once(&mut self) -> bool {
        // Handle events
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return true;
                }
                _ => {}
            }
        }

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        for (y, line) in self.pixels.iter().enumerate() {
            for (x, value) in line.iter().enumerate() {
                if *value {
                    let x = x as i32;
                    let y = y as i32;
                    self.canvas.fill_rect(Rect::new(x, y, 1, 1)).unwrap();
                }
            }
        }

        self.canvas.present();
        false
    }
}

impl Drawing for Display {
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: Iterator<Item = Pixel>,
    {
        for (Coord(x, y), color) in item_pixels {
            if x >= DISPLAY_SIZE as u32 || y >= DISPLAY_SIZE as u32 {
                continue;
            }
            self.pixels[y as usize][x as usize] = color == 1;
        }
    }
}
