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
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::DisplayBuilder;
use std::thread;
use std::time::Duration;

fn main() {
    let image: ImageBmp<Rgb565> = ImageBmp::new(include_bytes!("./rust-pride.bmp")).unwrap();

    let mut display = DisplayBuilder::new()
        .title("BMP image")
        .size(304, 128)
        .scale(2)
        .build_rgb();

    image.draw(&mut display);

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
