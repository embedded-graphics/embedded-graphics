extern crate embedded_graphics;
extern crate simulator;

use std::thread;
use std::time::Duration;

use embedded_graphics::coord::Coord;
use embedded_graphics::fonts::{Font12x16, Font6x12, Font6x8, Font8x16};
use embedded_graphics::prelude::*;

use simulator::Display;

fn main() {
    let mut display = Display::new();

    // Show smallest font with black font on white background (default value for fonts)
    display.draw(
        Font6x8::render_str("Hello World! - default style 6x8")
            .translate(Coord::new(15, 15))
            .into_iter(),
    );

    // Show smallest font with white font on black background
    display.draw(
        Font6x8::render_str("Hello World! - inverse 6x8 ")
            .with_stroke(Some(0u8.into()))
            .with_fill(Some(1u8.into()))
            .translate(Coord::new(15, 30))
            .into_iter(),
    );

    // Show 6x12 Font
    display.draw(
        Font6x12::render_str("Hello 6x12!")
            .translate(Coord::new(15, 45))
            .into_iter(),
    );

    // Show 8x16 Font
    display.draw(
        Font8x16::render_str("Hello 8x16!")
            .translate(Coord::new(15, 70))
            .into_iter(),
    );

    // Show 12x16 Font
    display.draw(
        Font12x16::render_str("Hello 12x16!")
            .translate(Coord::new(15, 95))
            .into_iter(),
    );

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
