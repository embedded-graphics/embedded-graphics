extern crate embedded_graphics;
extern crate simulator;

use std::thread;
use std::time::Duration;

use embedded_graphics::icoord;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Triangle;

use simulator::DisplayBuilder;

const PAD: i32 = 10;

fn main() {
    let mut display = DisplayBuilder::new().size(256 * 2, 128).scale(2).build();

    // no straight lines
    display.draw(
        Triangle::new(icoord!(0, 0), icoord!(64, 10), icoord!(15, 64))
            .translate(icoord!(PAD, 0))
            .stroke(Some(1u8.into())),
    );

    // flat top
    display.draw(
        Triangle::new(icoord!(5, 0), icoord!(30, 64), icoord!(64, 0))
            .stroke(Some(1u8.into()))
            .translate(icoord!(64 + PAD, 0)),
    );

    // flat left
    display.draw(
        Triangle::new(icoord!(0, 0), icoord!(0, 64), icoord!(64, 30))
            .stroke(Some(1u8.into()))
            .translate(icoord!((64 + PAD) * 2, 0)),
    );

    // flat bottom
    display.draw(
        Triangle::new(icoord!(22, 0), icoord!(0, 64), icoord!(64, 64))
            .translate(icoord!((64 + PAD) * 3, 0))
            .stroke(Some(1u8.into())),
    );

    // flat right
    display.draw(
        Triangle::new(icoord!(0, 22), icoord!(64, 0), icoord!(64, 64))
            .translate(icoord!((64 + PAD) * 4, 0))
            .stroke(Some(1u8.into())),
    );

    // draw filled above stroke, should not be visible
    display.draw(
        Triangle::new(icoord!(0, 22), icoord!(64, 0), icoord!(64, 64))
            .translate(icoord!((64 + PAD) * 5, 0))
            .stroke(Some(1u8.into())),
    );
    display.draw(
        Triangle::new(icoord!(0, 22), icoord!(64, 0), icoord!(64, 64))
            .translate(icoord!((64 + PAD) * 5, 0))
            .fill(Some(0u8.into())),
    );

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
