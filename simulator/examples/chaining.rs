//! Demonstrate the chaining abilities of embedded graphics iterators

use embedded_graphics::fonts::Font6x8;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line};
use embedded_graphics_simulator::{BinaryColorTheme, DisplayBuilder};
use std::thread;
use std::time::Duration;

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Chained drawing")
        .theme(BinaryColorTheme::OledBlue)
        .build_binary();

    let objects = Circle::new(Point::new(64, 64), 64)
        .stroke(Some(BinaryColor::On))
        .into_iter()
        .chain(Line::new(Point::new(64, 64), Point::new(0, 64)).stroke(Some(BinaryColor::On)))
        .chain(Line::new(Point::new(64, 64), Point::new(80, 80)).stroke(Some(BinaryColor::On)))
        .chain(
            Font6x8::render_str("Hello World!")
                .stroke(Some(BinaryColor::On))
                .translate(Point::new(5, 50)),
        );

    display.draw(objects);

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
