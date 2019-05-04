extern crate embedded_graphics;
extern crate simulator;

use std::thread;
use std::time::Duration;

use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Triangle;

use simulator::DisplayBuilder;

const PAD: i32 = 10;

fn main() {
    let mut display = DisplayBuilder::new().size(256*2, 128).scale(2).build();

    // no straight lines
    display.draw(
        Triangle::new(Coord::new(0, 0), Coord::new(64, 10), Coord::new(15, 64))
            .translate(Coord::new(PAD, 0))
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );

    // flat top
    display.draw(
        Triangle::new(Coord::new(5, 0), Coord::new(30, 64), Coord::new(64, 0))
            .with_stroke(Some(1u8.into()))
            .translate(Coord::new(64 + PAD, 0))
            .into_iter(),
    );

    // flat left
    display.draw(
        Triangle::new(Coord::new(0, 0), Coord::new(0, 64), Coord::new(64, 30))
            .with_stroke(Some(1u8.into()))
            .translate(Coord::new((64 + PAD) * 2, 0))
            .into_iter(),
    );

    // flat bottom
    display.draw(
        Triangle::new(Coord::new(22, 0), Coord::new(0, 64), Coord::new(64, 64))
            .translate(Coord::new((64 + PAD) * 3, 0))
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );

    // flat right
    display.draw(
        Triangle::new(Coord::new(0, 22), Coord::new(64, 0), Coord::new(64, 64))
            .translate(Coord::new((64 + PAD) * 4, 0))
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );

    // draw filled above stroke, should not be visible
    display.draw(
        Triangle::new(Coord::new(0, 22), Coord::new(64, 0), Coord::new(64, 64))
            .translate(Coord::new((64 + PAD) * 5, 0))
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );
    display.draw(
        Triangle::new(Coord::new(0, 22), Coord::new(64, 0), Coord::new(64, 64))
            .translate(Coord::new((64 + PAD) * 5, 0))
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
