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
//! use embedded_graphics::pixelcolor::BinaryColor;
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::{egcircle, egline, text_6x8};
//! use embedded_graphics_simulator::{
//!     BinaryColorTheme, SimulatorDisplay, SimulatorEvent, WindowBuilder,
//! };
//! use std::thread;
//! use std::time::Duration;
//!
//! fn main() {
//!     let mut display = SimulatorDisplay::new(Size::new(128, 64));
//!     let mut window = WindowBuilder::new(&display)
//!         .theme(BinaryColorTheme::OledBlue)
//!         .build();
//!
//!     text_6x8!("Hello World!").draw(&mut display);
//!
//!     egcircle!((96, 32), 31, stroke_color = Some(BinaryColor::On)).draw(&mut display);
//!
//!     egline!((32, 32), (1, 32), stroke_color = Some(BinaryColor::On))
//!         .translate(Point::new(64, 0))
//!         .draw(&mut display);
//!     egline!((32, 32), (40, 40), stroke_color = Some(BinaryColor::On))
//!         .translate(Point::new(64, 0))
//!         .draw(&mut display);
//!
//!     'running: loop {
//!         window.update(&display);
//!
//!         for event in window.events() {
//!             match event {
//!                 SimulatorEvent::MouseButtonUp { point, .. } => {
//!                     println!("Click event at ({}, {})", point.x, point.y);
//!                 }
//!                 SimulatorEvent::Quit => break 'running,
//!                 _ => {}
//!             }
//!
//!             thread::sleep(Duration::from_millis(200));
//!         }
//!     }
//! }
//! ```

#![deny(missing_docs)]

mod display;
mod theme;
mod window;
mod window_builder;

pub use crate::display::SimulatorDisplay;
pub use crate::theme::BinaryColorTheme;
pub use crate::window::{SimulatorEvent, Window};
pub use crate::window_builder::WindowBuilder;
