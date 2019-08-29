//! Demonstrate usage of primitives like `fill.rs` but use macros instead for shorter code

use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::{egcircle, egline, egrectangle, egtriangle};
use embedded_graphics_simulator::DisplayBuilder;
use std::thread;
use std::time::Duration;

static CIRCLE_SIZE: i32 = 32;

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Filled primitives using macros")
        .size(384, 128)
        .scale(2)
        .build_binary();

    display.draw(egcircle!(
        (CIRCLE_SIZE, CIRCLE_SIZE),
        CIRCLE_SIZE as u32,
        stroke = Some(BinaryColor::On)
    ));

    display.draw(
        egcircle!(
            (CIRCLE_SIZE, CIRCLE_SIZE),
            CIRCLE_SIZE as u32,
            stroke = Some(BinaryColor::Off),
            fill = Some(BinaryColor::On)
        )
        .translate(Point::new(16, 16)),
    );

    display.draw(
        egcircle!(
            (CIRCLE_SIZE, CIRCLE_SIZE),
            CIRCLE_SIZE as u32,
            stroke = Some(BinaryColor::Off),
            fill = Some(BinaryColor::Off)
        )
        .translate(Point::new(CIRCLE_SIZE, CIRCLE_SIZE)),
    );

    display.draw(
        egrectangle!((0, 0), (64, 64), stroke = Some(BinaryColor::On)).translate(Point::new(96, 0)),
    );

    display.draw(
        &egrectangle!(
            (0, 0),
            (64, 64),
            stroke = Some(BinaryColor::Off),
            fill = Some(BinaryColor::On)
        )
        .translate(Point::new(96 + 16, 16)),
    );

    display.draw(
        egrectangle!(
            (0, 0),
            (64, 64),
            stroke = Some(BinaryColor::Off),
            fill = Some(BinaryColor::Off)
        )
        .translate(Point::new(96 + 32, 32)),
    );

    display.draw(
        egtriangle!((32, 0), (0, 64), (64, 64), stroke = Some(BinaryColor::On))
            .translate(Point::new(96 * 2, 0)),
    );

    display.draw(
        egtriangle!(
            (32, 0),
            (0, 64),
            (64, 64),
            stroke = Some(BinaryColor::Off),
            fill = Some(BinaryColor::On)
        )
        .translate(Point::new(96 * 2 + 16, 16)),
    );

    display.draw(
        egtriangle!(
            (32, 0),
            (0, 64),
            (64, 64),
            stroke = Some(BinaryColor::Off),
            fill = Some(BinaryColor::Off)
        )
        .translate(Point::new(96 * 2 + 32, 32)),
    );

    display.draw(
        egline!((0, 0), (64, 64), stroke = Some(BinaryColor::Off),)
            .translate(Point::new(256 + 32, 0)),
    );

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
