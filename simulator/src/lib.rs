//! # Embedded graphics simulator
//!
//! ![It can display all sorts of embedded-graphics test code.](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/assets/simulator-demo.png)

#![deny(missing_docs)]

mod display_builder;
mod display_theme;
mod sim_pixel_color;

pub use crate::display_builder::DisplayBuilder;
pub use crate::display_theme::DisplayTheme;
pub use crate::sim_pixel_color::SimPixelColor;
use embedded_graphics::drawable::Pixel;
use embedded_graphics::Drawing;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render;

/// Create a new window with a simulated display to draw into
///
/// You should use [`DisplayBuilder`] to create an instance of `Display`
///
/// [`DisplayBuilder`]: ./display_builder/struct.DisplayBuilder.html
pub struct Display {
    width: usize,
    height: usize,
    scale: usize,
    pixel_spacing: usize,
    theme: DisplayTheme,
    pixels: Box<[SimPixelColor]>,
    canvas: render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
}

impl Display {
    /// Update the display to show drawn pixels
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

        self.canvas.set_draw_color(self.theme.background_color());
        self.canvas.clear();

        let pitch = self.scale + self.pixel_spacing;
        for (index, value) in self.pixels.iter().enumerate() {
            if let Some(c) = self.theme.pixel_color(value) {
                self.canvas.set_draw_color(c);

                let x = (index % self.width * pitch) as i32;
                let y = (index / self.width * pitch) as i32;
                let r = Rect::new(x, y, self.scale as u32, self.scale as u32);
                self.canvas.fill_rect(r).unwrap();
            }
        }

        self.canvas.present();
        false
    }
}

impl Drawing<SimPixelColor> for Display {
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: IntoIterator<Item = Pixel<SimPixelColor>>,
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
