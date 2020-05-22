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
//! use embedded_graphics::fonts::{Font6x8, Text};
//! use embedded_graphics::pixelcolor::BinaryColor;
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::primitives::{Circle, Line};
//! use embedded_graphics::style::{PrimitiveStyle, TextStyle};
//! use embedded_graphics_simulator::{
//!     BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
//! };
//! use std::thread;
//! use std::time::Duration;
//!
//! let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));
//! let output_settings = OutputSettingsBuilder::new()
//!     .theme(BinaryColorTheme::OledBlue)
//!     .build();
//! let mut window = Window::new("Example", &output_settings);
//!
//! Text::new("Hello World!", Point::zero())
//!     .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
//!     .draw(&mut display);
//!
//! Circle::new(Point::new(65, 1), 63)
//!     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
//!     .draw(&mut display);
//!
//! Line::new(Point::new(32, 32), Point::new(1, 32))
//!     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
//!     .translate(Point::new(64, 0))
//!     .draw(&mut display);
//! Line::new(Point::new(32, 32), Point::new(40, 40))
//!     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
//!     .translate(Point::new(64, 0))
//!     .draw(&mut display);
//!
//! 'running: loop {
//!     window.update(&display);
//!
//!     for event in window.events() {
//!         match event {
//!             SimulatorEvent::MouseButtonUp { point, .. } => {
//!                 println!("Click event at ({}, {})", point.x, point.y);
//!             }
//!             SimulatorEvent::Quit => break 'running,
//!             _ => {}
//!         }
//!
//!         thread::sleep(Duration::from_millis(200));
//!     }
//! }
//! ```

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
