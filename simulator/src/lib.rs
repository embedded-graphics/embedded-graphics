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
//! use embedded_graphics::{
//!     fonts::{Font6x8, Text},
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     primitives::{Circle, Line, Rectangle},
//!     style::{PrimitiveStyle, TextStyle},
//! };
//! use embedded_graphics_simulator::{
//!     BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
//! };
//!
//! fn main() -> Result<(), core::convert::Infallible> {
//!     let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));
//!
//!     let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
//!
//!     Circle::new(Point::new(72, 8), 48)
//!         .into_styled(line_style)
//!         .draw(&mut display)?;
//!
//!     Line::new(Point::new(48, 16), Point::new(8, 16))
//!         .into_styled(line_style)
//!         .draw(&mut display)?;
//!
//!     Line::new(Point::new(48, 16), Point::new(64, 32))
//!         .into_styled(line_style)
//!         .draw(&mut display)?;
//!
//!     Rectangle::new(Point::new(79, 15), Size::new(34, 34))
//!         .into_styled(line_style)
//!         .draw(&mut display)?;
//!
//!     Text::new("Hello World!", Point::new(5, 5))
//!         .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
//!         .draw(&mut display)?;
//!
//!     let output_settings = OutputSettingsBuilder::new()
//!         .theme(BinaryColorTheme::OledBlue)
//!         .build();
//!     Window::new("Hello World", &output_settings).show_static(&display);
//!
//!     Ok(())
//! }
//! ```
//!
//! ![Screenshot of the simple hello world example](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/assets/simulator-hello-world.png)

#![deny(missing_docs)]

mod check_readme;
mod display;
mod framebuffer;
mod output_settings;
mod theme;

#[cfg(feature = "with-sdl")]
mod window;

#[cfg(feature = "with-sdl")]
pub use window::{SimulatorEvent, Window};

pub use crate::{
    display::SimulatorDisplay,
    output_settings::{OutputSettings, OutputSettingsBuilder},
    theme::BinaryColorTheme,
};
