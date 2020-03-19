//! # Example: TGA images
//!
//! Draw a 16BPP TGA image to the display
//!
//! This example uses the [tinytga](https://crates.io/crates/tinytga) crate to load the TGA from a
//! byte slice read in at compile time.

use embedded_graphics::{image::Image, pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use tinytga::Tga;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(128, 128));

    // Load the TGA image
    let tga = Tga::from_slice(include_bytes!("./assets/rust-pride.tga")).unwrap();

    // Create a new embedded-graphics Image, wrapping the TGA which provides pixel data. The top
    // left corner of the image is positioned at (32, 32). It is important to specify the color
    // format used by the image, otherwise the compiler may infer an incorrect type. This image is
    // in 24BPP RGB888 format, so the Rgb888 pixel color type is used.
    let image: Image<Tga, Rgb888> = Image::new(&tga, Point::new(32, 32));

    // Display the image
    image.draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("TGA image", &output_settings).show_static(&display);

    Ok(())
}
