extern crate embedded_graphics;
extern crate simulator;

use std::thread;
use std::time::Duration;

use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rect};

use simulator::{Display, SimPixelColor};

const PADDING: i32 = 16;

fn main() {
    let mut display = Display::new();

    let circ = Circle::new(Coord::new(32, 32), 32).with_stroke(Some(SimPixelColor(true)));

    let rect = Rect::new(Coord::new(0, 0), Coord::new(64, 64))
        .translate(Coord::new(64 + PADDING, 0))
        .with_stroke(Some(SimPixelColor(true)));

    let line = Line::new(Coord::new(0, 0), Coord::new(64, 64))
        .translate(Coord::new(128 + PADDING * 2, 0))
        .with_stroke(Some(SimPixelColor(true)));

    display.draw(
        circ.into_iter()
            .chain(rect.into_iter())
            .chain(line.into_iter()),
    );

    display.draw(
        circ.translate(Coord::new(0, 64 + PADDING))
            .with_stroke_width(3)
            .into_iter()
            .chain(
                rect.translate(Coord::new(0, 64 + PADDING))
                    .with_stroke_width(3)
                    .into_iter(),
            )
            .chain(
                line.translate(Coord::new(0, 64 + PADDING))
                    .with_stroke_width(3)
                    .into_iter(),
            ),
    );

    display.draw(
        circ.translate(Coord::new(0, 128 + PADDING * 2))
            .with_stroke_width(10)
            .into_iter()
            .chain(
                rect.translate(Coord::new(0, 128 + PADDING * 2))
                    .with_stroke_width(10)
                    .into_iter(),
            )
            .chain(
                line.translate(Coord::new(0, 128 + PADDING * 2))
                    .with_stroke_width(10)
                    .into_iter(),
            ),
    );

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
