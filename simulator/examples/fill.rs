use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Rectangle, Triangle};
use embedded_graphics_simulator::DisplayBuilder;
use std::thread;
use std::time::Duration;

static CIRCLE_SIZE: i32 = 32;

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Filled primitives")
        .size(304, 128)
        .scale(2)
        .build_binary();

    display.draw(
        Circle::new(Point::new(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32)
            .stroke(Some(BinaryColor::On)),
    );

    display.draw(
        Circle::new(Point::new(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32)
            .translate(Point::new(16, 16))
            .stroke(Some(BinaryColor::Off))
            .fill(Some(BinaryColor::On)),
    );

    display.draw(
        Circle::new(Point::new(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32)
            .translate(Point::new(CIRCLE_SIZE, CIRCLE_SIZE))
            .stroke(Some(BinaryColor::Off))
            .fill(Some(BinaryColor::Off)),
    );

    display.draw(
        Rectangle::new(Point::new(0, 0), Point::new(64, 64))
            .translate(Point::new(96, 0))
            .stroke(Some(BinaryColor::On)),
    );

    display.draw(
        &Rectangle::new(Point::new(0, 0), Point::new(64, 64))
            .translate(Point::new(96 + 16, 16))
            .stroke(Some(BinaryColor::Off))
            .fill(Some(BinaryColor::On)),
    );

    display.draw(
        Rectangle::new(Point::new(0, 0), Point::new(64, 64))
            .translate(Point::new(96 + 32, 32))
            .stroke(Some(BinaryColor::Off))
            .fill(Some(BinaryColor::Off)),
    );

    display.draw(
        Triangle::new(Point::new(32, 0), Point::new(0, 64), Point::new(64, 64))
            .translate(Point::new(96 * 2, 0))
            .stroke(Some(BinaryColor::On)),
    );

    display.draw(
        Triangle::new(Point::new(32, 0), Point::new(0, 64), Point::new(64, 64))
            .translate(Point::new(96 * 2 + 16, 16))
            .stroke(Some(BinaryColor::Off))
            .fill(Some(BinaryColor::On)),
    );

    display.draw(
        Triangle::new(Point::new(32, 0), Point::new(0, 64), Point::new(64, 64))
            .translate(Point::new(96 * 2 + 32, 32))
            .stroke(Some(BinaryColor::Off))
            .fill(Some(BinaryColor::Off)),
    );

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
