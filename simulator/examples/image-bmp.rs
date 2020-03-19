//! # Example: BMP images
//!
//! Draw a 16BPP BMP image of the Rust logo to a display
//!
//! This example uses the [tinybmp](https://crates.io/crates/tinybmp) crate to load the BMP from a
//! byte slice read in at compile time.

use embedded_graphics::{image::Image, pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use tinybmp::Bmp;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(128, 128));

    // Load the BMP image
    let bmp = Bmp::from_slice(include_bytes!("./assets/rust-pride.bmp")).unwrap();

    // Create a new embedded-graphics Image, wrapping the BMP which provides pixel data. The top
    // left corner of the image is positioned at (32, 32)
    let image: Image<Bmp, Rgb565> = Image::new(&bmp, Point::new(32, 32));

    image.draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("BMP image", &output_settings).show_static(&display);

    Ok(())
}
