extern crate embedded_graphics;
extern crate simulator;

use std::thread;
use std::time::Duration;

use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Rect, Triangle};

use simulator::DisplayBuilder;

static CIRCLE_SIZE: i32 = 32;

fn main() {
    let mut display = DisplayBuilder::new().size(304, 128).scale(2).build();

    display.draw(
        Circle::new(Coord::new(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32)
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );

    display.draw(
        Circle::new(Coord::new(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32)
            .translate(Coord::new(16, 16))
            .with_stroke(Some(0u8.into()))
            .with_fill(Some(1u8.into()))
            .into_iter(),
    );

    display.draw(
        Circle::new(Coord::new(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32)
            .translate(Coord::new(CIRCLE_SIZE, CIRCLE_SIZE))
            .with_stroke(Some(0u8.into()))
            .with_fill(Some(0u8.into()))
            .into_iter(),
    );

    display.draw(
        Rect::new(Coord::new(0, 0), Coord::new(64, 64))
            .translate(Coord::new(96, 0))
            .with_stroke(Some(1u8.into())),
    );

    display.draw(
        &Rect::new(Coord::new(0, 0), Coord::new(64, 64))
            .translate(Coord::new(96 + 16, 16))
            .with_stroke(Some(0u8.into()))
            .with_fill(Some(1u8.into())),
    );

    display.draw(
        Rect::new(Coord::new(0, 0), Coord::new(64, 64))
            .translate(Coord::new(96 + 32, 32))
            .with_stroke(Some(0u8.into()))
            .with_fill(Some(0u8.into()))
            .into_iter(),
    );

    display.draw(
        Triangle::new(Coord::new(32, 0), Coord::new(0, 64), Coord::new(64, 64))
            .translate(Coord::new(96 * 2, 0))
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );

    display.draw(
        Triangle::new(Coord::new(32, 0), Coord::new(0, 64), Coord::new(64, 64))
            .translate(Coord::new(96 * 2 + 16, 16))
            .with_stroke(Some(0u8.into()))
            .with_fill(Some(1u8.into()))
            .into_iter(),
    );

    display.draw(
        Triangle::new(Coord::new(32, 0), Coord::new(0, 64), Coord::new(64, 64))
            .translate(Coord::new(96 * 2 + 32, 32))
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
