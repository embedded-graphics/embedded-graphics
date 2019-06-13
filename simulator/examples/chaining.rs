//! Demonstrate the chaining abilities of embedded graphics iterators

extern crate embedded_graphics;
extern crate simulator;

use std::thread;
use std::time::Duration;

use embedded_graphics::coord::Coord;
use embedded_graphics::fonts::Font6x8;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line};

use simulator::{DisplayBuilder, DisplayTheme};

fn main() {
    let mut display = DisplayBuilder::new().theme(DisplayTheme::OledBlue).build();

    let objects = Circle::new(Coord::new(64, 64), 64)
        .stroke(Some(1u8.into()))
        .into_iter()
        .chain(Line::new(Coord::new(64, 64), Coord::new(0, 64)).stroke(Some(1u8.into())))
        .chain(Line::new(Coord::new(64, 64), Coord::new(80, 80)).stroke(Some(1u8.into())))
        .chain(
            Font6x8::render_str("Hello World!")
                .stroke(Some(1u8.into()))
                .translate(Coord::new(5, 50)),
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
