//! Draw a 16BPP BMP image onto a monochrome display
//!
//! This example uses `impl From<u16> for SimPixelColor` from `src/lib` to convert the image into
//! a black and white pixel iterator. The simulator doesn't currently support drawing with colour.

extern crate embedded_graphics;
extern crate simulator;

use std::thread;
use std::time::Duration;

use embedded_graphics::image::ImageBmp;
use embedded_graphics::prelude::*;

use simulator::DisplayBuilder;

fn main() {
    let image = ImageBmp::new(include_bytes!("./rust-pride.bmp")).unwrap();

    let mut display = DisplayBuilder::new().size(304, 128).scale(2).build();

    display.draw(image.into_iter());

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
