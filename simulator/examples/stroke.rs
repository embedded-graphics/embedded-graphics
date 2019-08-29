use embedded_graphics::geometry::point;
use embedded_graphics::pixelcolor::BinaryColor::On as C1;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rectangle, Triangle};
use embedded_graphics_simulator::DisplayBuilder;
use std::thread;
use std::time::Duration;

const PADDING: i32 = 16;

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Strokes")
        .size(320, 256)
        .build_binary();

    let triangle = Triangle::new(point(0, 64), point(64, 0), point(64, 64))
        .translate(point(0, 0))
        .stroke(Some(C1));

    let rect = Rectangle::new(point(0, 0), point(64, 64))
        .translate(point(64 + PADDING, 0))
        .stroke(Some(C1));

    let line = Line::new(point(0, 0), point(64, 64))
        .translate(point(128 + PADDING * 2, 0))
        .stroke(Some(C1));

    let circ = Circle::new(point(32, 32), 32)
        .translate(point(192 + PADDING * 3, 0))
        .stroke(Some(C1));

    display.draw(
        circ.into_iter()
            .chain(rect.into_iter())
            .chain(line.into_iter())
            .chain(triangle.into_iter()),
    );

    display.draw(
        circ.translate(point(0, 64 + PADDING))
            .stroke_width(3)
            .into_iter()
            .chain(rect.translate(point(0, 64 + PADDING)).stroke_width(3))
            .chain(line.translate(point(0, 64 + PADDING)).stroke_width(3))
            .chain(triangle.translate(point(0, 64 + PADDING)).stroke_width(3)),
    );

    display.draw(
        circ.translate(point(0, 128 + PADDING * 2))
            .stroke_width(10)
            .into_iter()
            .chain(rect.translate(point(0, 128 + PADDING * 2)).stroke_width(10))
            .chain(line.translate(point(0, 128 + PADDING * 2)).stroke_width(10))
            .chain(
                triangle
                    .translate(point(0, 128 + PADDING * 2))
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
