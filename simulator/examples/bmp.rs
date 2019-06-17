//! Draw a 16BPP BMP image onto a monochrome display
//!
//! This example uses `impl From<Rgb565> for SimPixelColor` from `src/lib` to convert the image into
//! a colour pixel iterator. The simulator uses the `ColorOled` theme to display the image in colour
//!
//! Note that this requires the `bmp` feature to be turned on for `embedded-graphics`. Turn it on
//! with the following in `Cargo.toml`:
//!
//! [dependencies]
//! embedded-graphics = { version = "*", features = [ "bmp" ] }

use embedded_graphics::image::ImageBmp;
use embedded_graphics::pixelcolor;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{DisplayBuilder, DisplayTheme};
use std::thread;
use std::time::Duration;

fn main() {
    let image: ImageBmp<pixelcolor::Rgb565> =
        ImageBmp::new(include_bytes!("./rust-pride.bmp")).unwrap();

    let mut display = DisplayBuilder::new()
        .size(304, 128)
        .theme(DisplayTheme::ColorOled)
        .scale(2)
        .build();

    // Image has a pixel type of `pixelcolor::Rgb565`. This needs to be converted to a
    // `SimPixelColor` using the `.map()` below.
    display.draw(image.into_iter().map(|p| Pixel(p.0, p.1.into())));

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
