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
//! use embedded_graphics::{icoord, egcircle, egline, text_6x8};
//! use embedded_graphics::pixelcolor::BinaryColor::Off as C0;
//! use embedded_graphics::pixelcolor::BinaryColor::On as C1;
//! use embedded_graphics_simulator::{DisplayBuilder, BinaryColorTheme};
//! use std::thread;
//! use std::time::Duration;
//!
//! fn main() {
//!     let mut display = DisplayBuilder::new()
//!         .theme(BinaryColorTheme::OledBlue)
//!         .size(128, 64)
//!         .build_binary();
//!
//!     display.draw(text_6x8!("Hello World!"));
//!
//!     display.draw(egcircle!((96, 32), 31, stroke = Some(C1)));
//!
//!     display.draw(egline!((32, 32), (1, 32), stroke = Some(C1))
//!         .translate(icoord!(64, 0)));
//!     display.draw(egline!((32, 32), (40, 40), stroke = Some(C1))
//!         .translate(icoord!(64, 0)));
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
mod window;

pub use crate::display_builder::DisplayBuilder;
pub use crate::display_theme::BinaryColorTheme;
pub use crate::window::Window;
use embedded_graphics::drawable::Pixel;
use embedded_graphics::pixelcolor::{BinaryColor, Rgb888, RgbColor};
use embedded_graphics::prelude::*;
use embedded_graphics::Drawing;

struct PixelData<C> {
    pub width: usize,
    pub height: usize,
    data: Box<[C]>,
}

impl<C> PixelData<C>
where
    C: PixelColor,
{
    fn new(width: usize, height: usize) -> Self {
        let data = vec![C::DEFAULT_BG; width * height];

        Self {
            width,
            height,
            data: data.into_boxed_slice(),
        }
    }

    fn get(&self, x: usize, y: usize) -> C {
        self.data[x + y * self.width]
    }

    fn set(&mut self, x: usize, y: usize, color: C) {
        if x < self.width && y < self.height {
            self.data[x + y * self.width] = color;
        }
    }
}

/// Simulated binary color display
///
/// You should use [`DisplayBuilder`] to create an instance of `BinaryDisplay`
///
/// [`DisplayBuilder`]: ./display_builder/struct.DisplayBuilder.html
pub struct BinaryDisplay {
    pixels: PixelData<BinaryColor>,
    theme: BinaryColorTheme,
    window: Window,
}

impl BinaryDisplay {
    /// Update the display to show drawn pixels
    pub fn run_once(&mut self) -> bool {
        if self.window.handle_events() {
            return true;
        }

        self.window.clear(self.theme.convert(BinaryColor::Off));

        for y in 0..self.pixels.height {
            for x in 0..self.pixels.width {
                let color = self.pixels.get(x, y);
                let color = self.theme.convert(color);
                self.window.draw_pixel(x, y, color);
            }
        }

        self.window.present();
        false
    }
}

impl Drawing<BinaryColor> for BinaryDisplay {
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: IntoIterator<Item = Pixel<BinaryColor>>,
    {
        for Pixel(coord, color) in item_pixels {
            let x = coord[0] as usize;
            let y = coord[1] as usize;

            self.pixels.set(x, y, color);
        }
    }
}

/// Simulated RGB display
///
/// You should use [`DisplayBuilder`] to create an instance of `RgbDisplay`
///
/// [`DisplayBuilder`]: ./display_builder/struct.DisplayBuilder.html
pub struct RgbDisplay {
    pixels: PixelData<Rgb888>,
    window: Window,
}

impl RgbDisplay {
    /// Update the display to show drawn pixels
    pub fn run_once(&mut self) -> bool {
        if self.window.handle_events() {
            return true;
        }

        self.window.clear(Rgb888::BLACK);

        for y in 0..self.pixels.height {
            for x in 0..self.pixels.width {
                let color = self.pixels.get(x, y);
                self.window.draw_pixel(x, y, color);
            }
        }

        self.window.present();
        false
    }
}

impl<C> Drawing<C> for RgbDisplay
where
    C: PixelColor + Into<Rgb888>,
{
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: IntoIterator<Item = Pixel<C>>,
    {
        for Pixel(coord, color) in item_pixels {
            let x = coord[0] as usize;
            let y = coord[1] as usize;

            self.pixels.set(x, y, color.into());
        }
    }
}
