//! Draw a 16BPP BMP image onto a monochrome display
//!
//! This example uses `impl From<Rgb565> for SimPixelColor` from `src/lib` to convert the image into
//! a color pixel iterator. The simulator uses the `ColorOled` theme to display the image in color
//!
//! Note that this requires the `bmp` feature to be turned on for `embedded-graphics`. Turn it on
//! with the following in `Cargo.toml`:
//!
//! [dependencies]
//! embedded-graphics = { version = "*", features = [ "bmp" ] }

use embedded_graphics::image::Image;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{SimulatorDisplay, WindowBuilder};
use tinybmp::Bmp;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(128, 128));

    let bmp = Bmp::from_slice(include_bytes!("./rust-pride.bmp")).unwrap();

    let image = Image::new(&bmp);

    image.translate(Point::new(32, 32)).draw(&mut display)?;

    let mut window = WindowBuilder::new(&display)
        .title("BMP image")
        .scale(2)
        .build();

    window.show_static(&display);

    Ok(())
}
