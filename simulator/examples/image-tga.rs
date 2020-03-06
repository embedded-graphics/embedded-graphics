//! Draw a 16BPP BMP image onto a monochrome display
//!
//! This example uses `impl From<u16> for SimPixelColor` from `src/lib` to convert the image into
//! a black and white pixel iterator. The simulator doesn't currently support drawing with color.

use embedded_graphics::{image::Image, pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use tinytga::Tga;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(128, 128));

    let tga = Tga::from_slice(include_bytes!("./assets/rust-pride.tga")).unwrap();

    let image: Image<Tga, Rgb888> = Image::new(&tga, Point::zero());

    image.translate(Point::new(32, 32)).draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("TGA image", &output_settings).show_static(&display);

    Ok(())
}
