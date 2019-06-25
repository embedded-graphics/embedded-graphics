//! Demonstrate usage of primitives like `fill.rs` but use macros instead for shorter code

use embedded_graphics::icoord;
use embedded_graphics::prelude::*;
use embedded_graphics::{egcircle, egline, egrectangle, egtriangle};
use embedded_graphics_simulator::DisplayBuilder;
use std::thread;
use std::time::Duration;

static CIRCLE_SIZE: i32 = 32;

fn main() {
    let mut display = DisplayBuilder::new().size(384, 128).scale(2).build();

    display.draw(egcircle!(
        (CIRCLE_SIZE, CIRCLE_SIZE),
        CIRCLE_SIZE as u32,
        stroke = Some(1u8.into())
    ));

    display.draw(
        egcircle!(
            (CIRCLE_SIZE, CIRCLE_SIZE),
            CIRCLE_SIZE as u32,
            stroke = Some(0u8.into()),
            fill = Some(1u8.into())
        )
        .translate(icoord!(16, 16)),
    );

    display.draw(
        egcircle!(
            (CIRCLE_SIZE, CIRCLE_SIZE),
            CIRCLE_SIZE as u32,
            stroke = Some(0u8.into()),
            fill = Some(0u8.into())
        )
        .translate(icoord!(CIRCLE_SIZE, CIRCLE_SIZE)),
    );

    display
        .draw(egrectangle!((0, 0), (64, 64), stroke = Some(1u8.into())).translate(icoord!(96, 0)));

    display.draw(
        &egrectangle!(
            (0, 0),
            (64, 64),
            stroke = Some(0u8.into()),
            fill = Some(1u8.into())
        )
        .translate(icoord!(96 + 16, 16)),
    );

    display.draw(
        egrectangle!(
            (0, 0),
            (64, 64),
            stroke = Some(0u8.into()),
            fill = Some(0u8.into())
        )
        .translate(icoord!(96 + 32, 32)),
    );

    display.draw(
        egtriangle!((32, 0), (0, 64), (64, 64), stroke = Some(1u8.into()))
            .translate(icoord!(96 * 2, 0)),
    );

    display.draw(
        egtriangle!(
            (32, 0),
            (0, 64),
            (64, 64),
            stroke = Some(0u8.into()),
            fill = Some(1u8.into())
        )
        .translate(icoord!(96 * 2 + 16, 16)),
    );

    display.draw(
        egtriangle!(
            (32, 0),
            (0, 64),
            (64, 64),
            stroke = Some(0u8.into()),
            fill = Some(0u8.into())
        )
        .translate(icoord!(96 * 2 + 32, 32)),
    );

    display.draw(
        egline!((0, 0), (64, 64), stroke = Some(1u8.into()),).translate(icoord!(256 + 32, 0)),
    );

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
