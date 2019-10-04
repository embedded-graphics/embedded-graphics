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

    egcircle!(
        (CIRCLE_SIZE, CIRCLE_SIZE),
        CIRCLE_SIZE as u32,
        stroke_color = Some(BinaryColor::On)
    ).draw(&mut display);

    egcircle!(
        (CIRCLE_SIZE, CIRCLE_SIZE),
        CIRCLE_SIZE as u32,
        stroke_color = Some(BinaryColor::Off),
        fill_color = Some(BinaryColor::On)
    )
    .translate(Point::new(16, 16))
    .draw(&mut display);

    egcircle!(
        (CIRCLE_SIZE, CIRCLE_SIZE),
        CIRCLE_SIZE as u32,
        stroke_color = Some(BinaryColor::Off),
        fill_color = Some(BinaryColor::Off)
    )
    .translate(Point::new(CIRCLE_SIZE, CIRCLE_SIZE))
    .draw(&mut display);

    egrectangle!((0, 0), (64, 64), stroke_color = Some(BinaryColor::On))
        .translate(Point::new(96, 0))
        .draw(&mut display);

    egrectangle!(
        (0, 0),
        (64, 64),
        stroke_color = Some(BinaryColor::Off),
        fill_color = Some(BinaryColor::On)
    ).translate(Point::new(96 + 16, 16))
     .draw(&mut display);

    egrectangle!(
        (0, 0),
        (64, 64),
        stroke_color = Some(BinaryColor::Off),
        fill_color = Some(BinaryColor::Off)
    )
    .translate(Point::new(96 + 32, 32))
    .draw(&mut display);

    egtriangle!(
        (32, 0),
        (0, 64),
        (64, 64),
        stroke_color = Some(BinaryColor::On)
    )
    .translate(Point::new(96 * 2, 0))
    .draw(&mut display);

    egtriangle!(
        (32, 0),
        (0, 64),
        (64, 64),
        stroke_color = Some(BinaryColor::Off),
        fill_color = Some(BinaryColor::On)
    )
    .translate(Point::new(96 * 2 + 16, 16))
    .draw(&mut display);

    egtriangle!(
        (32, 0),
        (0, 64),
        (64, 64),
        stroke_color = Some(BinaryColor::Off),
        fill_color = Some(BinaryColor::Off)
    )
    .translate(Point::new(96 * 2 + 32, 32))
    .draw(&mut display);

    egline!((0, 0), (64, 64), stroke_color = Some(BinaryColor::Off),)
        .translate(Point::new(256 + 32, 0))
        .draw(&mut display);

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
