//! Draw a 16BPP BMP image onto a monochrome display
//!
//! This example uses `impl From<u16> for SimPixelColor` from `src/lib` to convert the image into
//! a black and white pixel iterator. The simulator doesn't currently support drawing with color.
//!
//! Note that this requires the `tga` feature to be turned on for `embedded-graphics`. Turn it on
//! with the following in `Cargo.toml`:
//!
//! [dependencies]
//! embedded-graphics = { version = "*", features = [ "tga" ] }

use embedded_graphics::image::ImageTga;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{SimulatorDisplay, WindowBuilder};

fn main() {
    let mut display = SimulatorDisplay::new(Size::new(128, 128));

    let image: ImageTga<Rgb888> = ImageTga::new(include_bytes!("./rust-pride.tga")).unwrap();
    image.translate(Point::new(32, 32)).draw(&mut display);

    let mut window = WindowBuilder::new(&display)
        .title("TGA image")
        .scale(2)
        .build();
    window.show_static(&display);
}
