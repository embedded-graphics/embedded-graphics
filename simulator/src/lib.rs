//! ![It can display all sorts of embedded-graphics test code.](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/assets/simulator-demo.png)
//!
//! The simulator can be used to test and debug
//! [embedded-graphics](https://crates.io/crates/embedded-graphics) code, or produce examples and
//! interactive demos to show of embedded graphics features.
//!
//! # Setup
//!
//! The simulator uses SDL2 and its development libraries which must be installed to build and run
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
//! The Windows install process is a bit more involved, but it _does_ work. See [the Rust-SDL2
//! crate's README](https://github.com/Rust-SDL2/rust-sdl2) for instructions. There are multiple
//! ways to get it working, but probably the simplest method is copying the binaries as shown
//! [here](https://github.com/Rust-SDL2/rust-sdl2#windows-msvc).
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
//! use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, Window, OutputSettingsBuilder};
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
//! # Creating screenshots
//!
//! Screenshots of programs, that use `Window` to display a simulated display, can be created by
//! setting the `EG_SIMULATOR_DUMP` environment variable:
//!
//! ```bash
//! EG_SIMULATOR_DUMP=screenshot.png cargo run
//! ```
//!
//! By setting the variable the display passed to the first `Window::update` call gets exported as a
//! PNG file to the specified path. After the file is exported the process is terminated.
//!
//! # Exporting images
//!
//! If a program doesn't require to display a window and only needs to export one or more images, a
//! `SimulatorDisplay` can also be converted to an `image` crate `ImageBuffer` by using the
//! `to_image_buffer` method. The resulting buffer can then be used to save the display content to
//! any format supported by `image`.
//!
//! # Usage without SDL2
//!
//! When the simulator is used in headless/CI environments that don't require showing a window, SDL2
//! support can be disabled. This removes the requirement of SDL2 being installed on the target machine,
//! but still allows the simulator to be used to generate images.
//!
//! The `with-sdl` feature is enabled by default and can be disabled by adding `default-features = false` to the dependency:
//!
//! ```toml
//! [dependencies.embedded-graphics-simulator]
//! version = "0.2.0"
//! default-features = false
//! ```
//!
//! See the [Choosing
//! Features](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#choosing-features)
//! Cargo manifest documentation for more details.

#![deny(missing_docs)]

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
