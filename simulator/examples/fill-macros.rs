//! Demonstrate usage of primitives like `fill.rs` but use macros instead for shorter code

extern crate embedded_graphics;
extern crate simulator;

use std::thread;
use std::time::Duration;

use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;

use embedded_graphics::{circle, line, rect, triangle};

use simulator::DisplayBuilder;

static CIRCLE_SIZE: i32 = 32;

fn main() {
    let mut display = DisplayBuilder::new().size(384, 128).scale(2).build();

    display.draw(circle!(
        (CIRCLE_SIZE, CIRCLE_SIZE),
        CIRCLE_SIZE as u32,
        stroke = Some(1u8.into())
    ));

    display.draw(
        circle!(
            (CIRCLE_SIZE, CIRCLE_SIZE),
            CIRCLE_SIZE as u32,
            stroke = Some(0u8.into()),
            fill = Some(1u8.into())
        )
        .translate(Coord::new(16, 16)),
    );

    display.draw(
        circle!(
            (CIRCLE_SIZE, CIRCLE_SIZE),
            CIRCLE_SIZE as u32,
            stroke = Some(0u8.into()),
            fill = Some(0u8.into())
        )
        .translate(Coord::new(CIRCLE_SIZE, CIRCLE_SIZE)),
    );

    display.draw(rect!((0, 0), (64, 64), stroke = Some(1u8.into())).translate(Coord::new(96, 0)));

    display.draw(
        &rect!(
            (0, 0),
            (64, 64),
            stroke = Some(0u8.into()),
            fill = Some(1u8.into())
        )
        .translate(Coord::new(96 + 16, 16)),
    );

    display.draw(
        rect!(
            (0, 0),
            (64, 64),
            stroke = Some(0u8.into()),
            fill = Some(0u8.into())
        )
        .translate(Coord::new(96 + 32, 32)),
    );

    display.draw(
        triangle!((32, 0), (0, 64), (64, 64), stroke = Some(1u8.into()))
            .translate(Coord::new(96 * 2, 0)),
    );

    display.draw(
        triangle!(
            (32, 0),
            (0, 64),
            (64, 64),
            stroke = Some(0u8.into()),
            fill = Some(1u8.into())
        )
        .translate(Coord::new(96 * 2 + 16, 16)),
    );

    display.draw(
        triangle!(
            (32, 0),
            (0, 64),
            (64, 64),
            stroke = Some(0u8.into()),
            fill = Some(0u8.into())
        )
        .translate(Coord::new(96 * 2 + 32, 32)),
    );

    display.draw(
        line!((0, 0), (64, 64), stroke = Some(1u8.into()),).translate(Coord::new(256 + 32, 0)),
    );

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
