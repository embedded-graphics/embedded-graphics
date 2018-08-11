extern crate embedded_graphics;
extern crate simulator;

use std::thread;
use std::time::Duration;

use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Rect};

use simulator::Display;

fn main() {
    let mut display = Display::new();

    display.draw(
        Circle::new(Coord::new(32, 32), 32)
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );

    display.draw(
        Circle::new(Coord::new(32, 32), 32)
            .translate(Coord::new(16, 16))
            .with_stroke(Some(0u8.into()))
            .with_fill(Some(1u8.into()))
            .into_iter(),
    );

    display.draw(
        Circle::new(Coord::new(32, 32), 32)
            .translate(Coord::new(32, 32))
            .with_stroke(Some(0u8.into()))
            .with_fill(Some(0u8.into()))
            .into_iter(),
    );

    display.draw(
        Rect::new(Coord::new(0, 0), Coord::new(64, 64))
            .translate(Coord::new(96, 0))
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );

    display.draw(
        Rect::new(Coord::new(0, 0), Coord::new(64, 64))
            .translate(Coord::new(96 + 16, 16))
            .with_stroke(Some(0u8.into()))
            .with_fill(Some(1u8.into()))
            .into_iter(),
    );

    display.draw(
        Rect::new(Coord::new(0, 0), Coord::new(64, 64))
            .translate(Coord::new(96 + 32, 32))
            .with_stroke(Some(0u8.into()))
            .with_fill(Some(0u8.into()))
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
