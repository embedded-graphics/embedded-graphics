use embedded_graphics::geometry::point;
use embedded_graphics::pixelcolor::BinaryColor::Off as C0;
use embedded_graphics::pixelcolor::BinaryColor::On as C1;
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

    display.draw(Circle::new(point(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32).stroke(Some(C1)));

    display.draw(
        Circle::new(point(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32)
            .translate(point(16, 16))
            .stroke(Some(C0))
            .fill(Some(C1)),
    );

    display.draw(
        Circle::new(point(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32)
            .translate(point(CIRCLE_SIZE, CIRCLE_SIZE))
            .stroke(Some(C0))
            .fill(Some(C0)),
    );

    display.draw(
        Rectangle::new(point(0, 0), point(64, 64))
            .translate(point(96, 0))
            .stroke(Some(C1)),
    );

    display.draw(
        &Rectangle::new(point(0, 0), point(64, 64))
            .translate(point(96 + 16, 16))
            .stroke(Some(C0))
            .fill(Some(C1)),
    );

    display.draw(
        Rectangle::new(point(0, 0), point(64, 64))
            .translate(point(96 + 32, 32))
            .stroke(Some(C0))
            .fill(Some(C0)),
    );

    display.draw(
        Triangle::new(point(32, 0), point(0, 64), point(64, 64))
            .translate(point(96 * 2, 0))
            .stroke(Some(C1)),
    );

    display.draw(
        Triangle::new(point(32, 0), point(0, 64), point(64, 64))
            .translate(point(96 * 2 + 16, 16))
            .stroke(Some(C0))
            .fill(Some(C1)),
    );

    display.draw(
        Triangle::new(point(32, 0), point(0, 64), point(64, 64))
            .translate(point(96 * 2 + 32, 32))
            .stroke(Some(C0))
            .fill(Some(C0)),
    );

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
