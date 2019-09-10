use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Triangle;
use embedded_graphics_simulator::DisplayBuilder;
use std::thread;
use std::time::Duration;

const PAD: i32 = 10;

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Triangles")
        .size(256 * 2, 128)
        .scale(2)
        .build_binary();

    // no straight lines
    display.draw(
        Triangle::new(Point::new(0, 0), Point::new(64, 10), Point::new(15, 64))
            .translate(Point::new(PAD, 0))
            .stroke_color(Some(BinaryColor::On)),
    );

    // flat top
    display.draw(
        Triangle::new(Point::new(5, 0), Point::new(30, 64), Point::new(64, 0))
            .stroke_color(Some(BinaryColor::On))
            .translate(Point::new(64 + PAD, 0)),
    );

    // flat left
    display.draw(
        Triangle::new(Point::new(0, 0), Point::new(0, 64), Point::new(64, 30))
            .stroke_color(Some(BinaryColor::On))
            .translate(Point::new((64 + PAD) * 2, 0)),
    );

    // flat bottom
    display.draw(
        Triangle::new(Point::new(22, 0), Point::new(0, 64), Point::new(64, 64))
            .translate(Point::new((64 + PAD) * 3, 0))
            .stroke_color(Some(BinaryColor::On)),
    );

    // flat right
    display.draw(
        Triangle::new(Point::new(0, 22), Point::new(64, 0), Point::new(64, 64))
            .translate(Point::new((64 + PAD) * 4, 0))
            .stroke_color(Some(BinaryColor::On)),
    );

    // draw filled above stroke, should not be visible
    display.draw(
        Triangle::new(Point::new(0, 22), Point::new(64, 0), Point::new(64, 64))
            .translate(Point::new((64 + PAD) * 5, 0))
            .stroke_color(Some(BinaryColor::On)),
    );
    display.draw(
        Triangle::new(Point::new(0, 22), Point::new(64, 0), Point::new(64, 64))
            .translate(Point::new((64 + PAD) * 5, 0))
            .fill_color(Some(BinaryColor::On)),
    );

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
