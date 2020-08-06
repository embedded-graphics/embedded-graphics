//! # Example: BMP images
//!
//! Draw a 16BPP BMP image of the Rust logo to a display
//!
//! This example uses the [tinybmp](https://crates.io/crates/tinybmp) crate to load the BMP from a
//! byte slice read in at compile time.
//!
//! The `graphics` feature of `tinybmp` needs to be enabled in `Cargo.toml` to use the `Bmp` object
//! with embedded-graphics.

use embedded_graphics::{image::Image, pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use tinybmp::Bmp;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(128, 128));

    // Load the BMP image.
    // The color type must be specified explicitly to match the color format used by the image,
    // otherwise the compiler may infer an incorrect type.
    let bmp: Bmp<Rgb565> = Bmp::from_slice(include_bytes!("./assets/rust-pride.bmp")).unwrap();

    // To draw the `bmp` object to the display it needs to be wrapped in an `Image` object to set
    // the position at which it should drawn. Here, the top left corner of the image is set to
    // `(32, 32)`.
    let image = Image::new(&bmp, Point::new(32, 32));

    // Display the image
    image.draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("BMP image", &output_settings).show_static(&display);

    Ok(())
}
