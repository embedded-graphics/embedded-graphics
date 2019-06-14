extern crate embedded_graphics;
extern crate simulator;

use std::thread;
use std::time::Duration;

use embedded_graphics::icoord;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rect, Triangle};

use simulator::{DisplayBuilder, SimPixelColor};

const PADDING: i32 = 16;

fn main() {
    let mut display = DisplayBuilder::new().size(320, 256).build();

    let triangle = Triangle::new(icoord!(0, 64), icoord!(64, 0), icoord!(64, 64))
        .translate(icoord!(0, 0))
        .stroke(Some(SimPixelColor(true)));

    let rect = Rect::new(icoord!(0, 0), icoord!(64, 64))
        .translate(icoord!(64 + PADDING, 0))
        .stroke(Some(SimPixelColor(true)));

    let line = Line::new(icoord!(0, 0), icoord!(64, 64))
        .translate(icoord!(128 + PADDING * 2, 0))
        .stroke(Some(SimPixelColor(true)));

    let circ = Circle::new(icoord!(32, 32), 32)
        .translate(icoord!(192 + PADDING * 3, 0))
        .stroke(Some(SimPixelColor(true)));

    display.draw(
        circ.into_iter()
            .chain(rect.into_iter())
            .chain(line.into_iter())
            .chain(triangle.into_iter()),
    );

    display.draw(
        circ.translate(icoord!(0, 64 + PADDING))
            .stroke_width(3)
            .into_iter()
            .chain(rect.translate(icoord!(0, 64 + PADDING)).stroke_width(3))
            .chain(line.translate(icoord!(0, 64 + PADDING)).stroke_width(3))
            .chain(triangle.translate(icoord!(0, 64 + PADDING)).stroke_width(3)),
    );

    display.draw(
        circ.translate(icoord!(0, 128 + PADDING * 2))
            .stroke_width(10)
            .into_iter()
            .chain(
                rect.translate(icoord!(0, 128 + PADDING * 2))
                    .stroke_width(10),
            )
            .chain(
                line.translate(icoord!(0, 128 + PADDING * 2))
                    .stroke_width(10),
            )
            .chain(
                triangle
                    .translate(icoord!(0, 128 + PADDING * 2))
                    .stroke_width(10),
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
