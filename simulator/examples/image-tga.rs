//! # Example: TGA images
//!
//! Draw a 24BPP TGA image to the display
//!
//! This example uses the [tinytga](https://crates.io/crates/tinytga) crate to load the TGA from a
//! byte slice read in at compile time.
//!
//! The `graphics` feature of `tinytga` needs to be enabled in `Cargo.toml` to use the `Tga` object
//! with embedded-graphics.

use embedded_graphics::{image::Image, pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use tinytga::Tga;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(128, 128));

    // Load the TGA image.
    // The color type must be specified explicitly to match the color format used by the image,
    // otherwise the compiler may infer an incorrect type.
    let tga: Tga<Rgb888> = Tga::from_slice(include_bytes!("./assets/rust-pride.tga")).unwrap();

    // To draw the `tga` object to the display it needs to be wrapped in an `Image` object to set
    // the position at which it should drawn. Here, the top left corner of the image is set to
    // `(32, 32)`.
    let image = Image::new(&tga, Point::new(32, 32));

    // Display the image
    image.draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("TGA image", &output_settings).show_static(&display);

    Ok(())
}
