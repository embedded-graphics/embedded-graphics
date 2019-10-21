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
//! use embedded_graphics::{egcircle, egline, text_6x8};
//! use embedded_graphics::pixelcolor::BinaryColor;
//! use embedded_graphics_simulator::{DisplayBuilder, BinaryColorTheme, SimulatorEvent};
//! use std::thread;
//! use std::time::Duration;
//!
//! fn main() {
//!     let mut display = DisplayBuilder::new()
//!         .theme(BinaryColorTheme::OledBlue)
//!         .size(128, 64)
//!         .build_binary();
//!
//!     text_6x8!("Hello World!").draw(&mut display);
//!
//!     egcircle!((96, 32), 31, stroke_color = Some(BinaryColor::On)).draw(&mut display);
//!
//!     egline!((32, 32), (1, 32), stroke_color = Some(BinaryColor::On)).translate(Point::new(64, 0)).draw(&mut display);
//!     egline!((32, 32), (40, 40), stroke_color = Some(BinaryColor::On)) .translate(Point::new(64, 0)).draw(&mut display);
//!
//!     loop {
//!         let end = display.run_once();
//!
//!         if end {
//!             break;
//!         }
//!
//!         for event in display.get_input_events() {
//!             if let SimulatorEvent::MouseButtonUp { point, ..} = event {
//!                 println!("Click event at ({}, {})", point.x, point.y);
//!             }
//!         }
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
pub use crate::window::SimulatorEvent;
use crate::window::Window;
use embedded_graphics::drawable::Pixel;
use embedded_graphics::geometry::Point;
use embedded_graphics::geometry::Size;
use embedded_graphics::pixelcolor::{BinaryColor, PixelColor, Rgb888, RgbColor};
use embedded_graphics::primitives::{Circle, Line, Rectangle, Triangle};
use embedded_graphics::DrawTarget;

struct PixelData {
    pub width: usize,
    pub height: usize,
}

/// Simulated binary color display
///
/// You should use [`DisplayBuilder`] to create an instance of `BinaryDisplay`
///
/// [`DisplayBuilder`]: ./display_builder/struct.DisplayBuilder.html
pub struct BinaryDisplay {
    pixels: PixelData,
    theme: BinaryColorTheme,
    window: Window,
}

impl BinaryDisplay {
    /// Clear all pixels to black
    pub fn clear(&mut self) {
        self.window.clear(self.theme.convert(BinaryColor::Off));
    }

    /// Get a vector of detected input events
    pub fn get_input_events(&mut self) -> impl Iterator<Item = SimulatorEvent> + '_ {
        self.window.get_input_events()
    }

    /// Update the display
    pub fn flush(&mut self) {
        self.window.present();
    }
}

impl<C> DrawTarget<C> for BinaryDisplay
where
    C: PixelColor + Into<BinaryColor>,
{
    fn draw_pixel(&mut self, pixel: Pixel<C>) {
        let Pixel(coord, color) = pixel;
        let x = coord[0] as usize;
        let y = coord[1] as usize;

        let color: Rgb888 = self.theme.convert(color.into());

        self.window.draw_pixel(x, y, color);
    }

    fn draw_circle(&mut self, item: &Circle<C>) {
        let Circle {
            center,
            radius,
            style,
        } = item;

        let Point { x, y } = center;

        let fill_color: Option<Rgb888> = if let Some(color) = style.fill_color {
            let color: Rgb888 = self.theme.convert(color.into());
            Some(color)
        } else {
            None
        };
        let stroke_color: Option<Rgb888> = if let Some(color) = style.stroke_color {
            let color: Rgb888 = self.theme.convert(color.into());
            Some(color)
        } else {
            None
        };

        self.window
            .draw_circle(*x, *y, *radius, fill_color, stroke_color);
    }

    fn draw_line(&mut self, item: &Line<C>) {
        let Line { start, end, style } = item;

        let Point { x: x1, y: y1 } = start;
        let Point { x: x2, y: y2 } = end;

        let fill_color: Option<Rgb888> = if let Some(color) = style.fill_color {
            let color: Rgb888 = self.theme.convert(color.into());
            Some(color)
        } else {
            None
        };
        let stroke_color: Option<Rgb888> = if let Some(color) = style.stroke_color {
            let color: Rgb888 = self.theme.convert(color.into());
            Some(color)
        } else {
            None
        };

        self.window
            .draw_line(*x1, *y1, *x2, *y2, fill_color, stroke_color);
    }

    fn draw_triangle(&mut self, _item: &Triangle<C>) {
        unimplemented!();
    }

    fn draw_rectangle(&mut self, item: &Rectangle<C>) {
        let Rectangle {
            top_left,
            bottom_right,
            style,
        } = item;

        let Point { x: x1, y: y1 } = top_left;
        let Point { x: x2, y: y2 } = bottom_right;

        let fill_color: Option<Rgb888> = if let Some(color) = style.fill_color {
            let color: Rgb888 = self.theme.convert(color.into());
            Some(color)
        } else {
            None
        };
        let stroke_color: Option<Rgb888> = if let Some(color) = style.stroke_color {
            let color: Rgb888 = self.theme.convert(color.into());
            Some(color)
        } else {
            None
        };

        self.window
            .draw_rectangle(*x1, *y1, *x2, *y2, fill_color, stroke_color);
    }

    fn size(&self) -> Size {
        Size::new(self.pixels.width as u32, self.pixels.height as u32)
    }
}

/// Simulated RGB display
///
/// You should use [`DisplayBuilder`] to create an instance of `RgbDisplay`
///
/// [`DisplayBuilder`]: ./display_builder/struct.DisplayBuilder.html
pub struct RgbDisplay {
    pixels: PixelData,
    window: Window,
}

impl RgbDisplay {
    /// Clear all pixels to black
    pub fn clear(&mut self) {
        self.window.clear(Rgb888::BLACK);
    }

    /// Get a vector of detected input events
    pub fn get_input_events(&mut self) -> impl Iterator<Item = SimulatorEvent> + '_ {
        self.window.get_input_events()
    }

    /// Update the display
    pub fn flush(&mut self) {
        self.window.present();
    }
}

impl<C> DrawTarget<C> for RgbDisplay
where
    C: RgbColor + Into<Rgb888>,
{
    fn draw_pixel(&mut self, pixel: Pixel<C>) {
        let Pixel(coord, color) = pixel;
        let x = coord[0] as usize;
        let y = coord[1] as usize;
        self.window.draw_pixel(x, y, color.into());
    }

    fn draw_circle(&mut self, item: &Circle<C>) {
        let Circle {
            center,
            radius,
            style,
        } = item;

        let Point { x, y } = center;

        //really?
        let fill_color: Option<Rgb888> = if let Some(color) = style.fill_color {
            Some(color.into())
        } else {
            None
        };
        let stroke_color: Option<Rgb888> = if let Some(color) = style.stroke_color {
            Some(color.into())
        } else {
            None
        };

        self.window
            .draw_circle(*x, *y, *radius, fill_color, stroke_color);
    }

    fn draw_line(&mut self, item: &Line<C>) {
        let Line { start, end, style } = item;

        let Point { x: x1, y: y1 } = start;
        let Point { x: x2, y: y2 } = end;

        //really?
        let fill_color: Option<Rgb888> = if let Some(color) = style.fill_color {
            Some(color.into())
        } else {
            None
        };
        let stroke_color: Option<Rgb888> = if let Some(color) = style.stroke_color {
            Some(color.into())
        } else {
            None
        };

        self.window
            .draw_line(*x1, *y1, *x2, *y2, fill_color, stroke_color);
    }

    fn draw_triangle(&mut self, _item: &Triangle<C>) {
        unimplemented!();
    }

    fn draw_rectangle(&mut self, item: &Rectangle<C>) {
        let Rectangle {
            top_left,
            bottom_right,
            style,
        } = item;

        let Point { x: x1, y: y1 } = top_left;
        let Point { x: x2, y: y2 } = bottom_right;

        //really?
        let fill_color: Option<Rgb888> = if let Some(color) = style.fill_color {
            Some(color.into())
        } else {
            None
        };
        let stroke_color: Option<Rgb888> = if let Some(color) = style.stroke_color {
            Some(color.into())
        } else {
            None
        };

        self.window
            .draw_rectangle(*x1, *y1, *x2, *y2, fill_color, stroke_color);
    }

    fn size(&self) -> Size {
        Size::new(self.pixels.width as u32, self.pixels.height as u32)
    }
}
