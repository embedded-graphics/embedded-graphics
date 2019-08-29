use embedded_graphics::geometry::point;
use embedded_graphics::pixelcolor::BinaryColor::Off as C0;
use embedded_graphics::pixelcolor::BinaryColor::On as C1;
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
        Triangle::new(point(0, 0), point(64, 10), point(15, 64))
            .translate(point(PAD, 0))
            .stroke(Some(C1)),
    );

    // flat top
    display.draw(
        Triangle::new(point(5, 0), point(30, 64), point(64, 0))
            .stroke(Some(C1))
            .translate(point(64 + PAD, 0)),
    );

    // flat left
    display.draw(
        Triangle::new(point(0, 0), point(0, 64), point(64, 30))
            .stroke(Some(C1))
            .translate(point((64 + PAD) * 2, 0)),
    );

    // flat bottom
    display.draw(
        Triangle::new(point(22, 0), point(0, 64), point(64, 64))
            .translate(point((64 + PAD) * 3, 0))
            .stroke(Some(C1)),
    );

    // flat right
    display.draw(
        Triangle::new(point(0, 22), point(64, 0), point(64, 64))
            .translate(point((64 + PAD) * 4, 0))
            .stroke(Some(C1)),
    );

    // draw filled above stroke, should not be visible
    display.draw(
        Triangle::new(point(0, 22), point(64, 0), point(64, 64))
            .translate(point((64 + PAD) * 5, 0))
            .stroke(Some(C1)),
    );
    display.draw(
        Triangle::new(point(0, 22), point(64, 0), point(64, 64))
            .translate(point((64 + PAD) * 5, 0))
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
