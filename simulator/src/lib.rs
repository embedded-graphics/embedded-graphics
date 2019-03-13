extern crate embedded_graphics;
extern crate sdl2;

use embedded_graphics::drawable::Pixel;
use embedded_graphics::prelude::*;
use embedded_graphics::Drawing;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render;

#[derive(Clone, Copy, PartialEq)]
pub struct SimPixelColor(pub bool);

impl PixelColor for SimPixelColor {}

impl From<u8> for SimPixelColor {
    fn from(other: u8) -> Self {
        SimPixelColor(other != 0)
    }
}

pub struct Display {
    width: usize,
    height: usize,
    pixels: Box<[SimPixelColor]>,
    canvas: render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
}

impl Display {
    pub fn new(width: usize, height: usize) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(
                "graphics-emulator",
                width as u32,
                height as u32,
            )
            .position_centered()
            .build()
            .unwrap();

        let pixels = vec![SimPixelColor(false); width * height].into_boxed_slice();
        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            width,
            height,
            pixels,
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
        for (index, value) in self.pixels.iter().enumerate() {
            if *value == SimPixelColor(true) {
                let x = (index % self.width) as i32;
                let y = (index / self.width) as i32;
                self.canvas.fill_rect(Rect::new(x, y, 1, 1)).unwrap();
            }
        }

        self.canvas.present();
        false
    }
}

impl Drawing<SimPixelColor> for Display {
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: Iterator<Item = Pixel<SimPixelColor>>,
    {
        for Pixel(coord, color) in item_pixels {
            let x = coord[0] as usize;
            let y = coord[1] as usize;

            if x >= self.width || y >= self.height {
                continue;
            }

            self.pixels[y * self.width + x] = color;
        }
    }
}
