//! Draw a 16BPP BMP image onto a monochrome display
//!
//! This example uses `impl From<u16> for SimPixelColor` from `src/lib` to convert the image into
//! a black and white pixel iterator. The simulator doesn't currently support drawing with colour.
//!
//! Note that this requires the `tga` feature to be turned on for `embedded-graphics`. Turn it on
//! with the following in `Cargo.toml`:
//!
//! [dependencies]
//! embedded-graphics = { version = "*", features = [ "tga" ] }

use embedded_graphics::image::ImageTga;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{Display, DisplayBuilder};
use std::thread;
use std::time::Duration;

fn main() {
    let image = ImageTga::new(include_bytes!("./rust-pride.tga")).unwrap();

    let mut display: Display<Rgb888> = DisplayBuilder::new().size(304, 128).scale(2).build();

    display.draw(&image);

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
