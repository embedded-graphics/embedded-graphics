//! # Embedded graphics simulator
//!
//! ![It can display all sorts of embedded-graphics test code.](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/assets/simulator-demo.png)
//!
//! The simulator can be used to test and debug
//! [embedded-graphics](https://crates.io/crates/embedded-graphics) code, or produce snazzy examples
//! for people to try drivers out without needing physical hardware to run on.
//!
//! # Setup
//!
//! The simulator uses SDL and its development libraries which must be installed to build and run
//! it.
//!
//! ## Linux (`apt`)
//!
//! ```bash
//! sudo apt install libsdl2-dev
//! ```
//!
//! ## macOS (`brew`)
//!
//! ```bash
//! brew install sdl2
//! ```
//!
//! ## Windows
//!
//! The Windows install process is a bit more involved, but it _does_ work. See [the SDL2
//! wiki](https://wiki.libsdl.org/Installation#WinRT.2FWindows_8.2FWinPhone) for instructions.
//!
//! # Examples
//!
//! ## Simulate a 128x64 SSD1306 OLED
//!
//! ```rust,no_run
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::{icoord, circle, line, text_6x8};
//! use embedded_graphics_simulator::{DisplayBuilder, DisplayTheme};
//! use std::thread;
//! use std::time::Duration;
//!
//! fn main() {
//!     let mut display = DisplayBuilder::new()
//!         .theme(DisplayTheme::OledBlue)
//!         .size(128, 64)
//!         .build();
//!
//!     display.draw(text_6x8!("Hello World!"));
//!
//!     display.draw(circle!((96, 32), 31, stroke = Some(1u8.into())));
//!
//!     display.draw(line!((32, 32), (1, 32), stroke = Some(1u8.into())).translate(icoord!(64, 0)));
//!     display.draw(line!((32, 32), (40, 40), stroke = Some(1u8.into())).translate(icoord!(64, 0)));
//!
//!     loop {
//!         let end = display.run_once();
//!
//!         if end {
//!             break;
//!         }
//!
//!         thread::sleep(Duration::from_millis(200));
//!     }
//! }
//! ```

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
