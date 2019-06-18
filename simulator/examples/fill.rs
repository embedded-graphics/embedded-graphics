use embedded_graphics::icoord;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Rect, Triangle};
use embedded_graphics_simulator::DisplayBuilder;
use std::thread;
use std::time::Duration;

static CIRCLE_SIZE: i32 = 32;

fn main() {
    let mut display = DisplayBuilder::new().size(304, 128).scale(2).build();

    display.draw(
        Circle::new(icoord!(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32).stroke(Some(1u8.into())),
    );

    display.draw(
        Circle::new(icoord!(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32)
            .translate(icoord!(16, 16))
            .stroke(Some(0u8.into()))
            .fill(Some(1u8.into())),
    );

    display.draw(
        Circle::new(icoord!(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32)
            .translate(icoord!(CIRCLE_SIZE, CIRCLE_SIZE))
            .stroke(Some(0u8.into()))
            .fill(Some(0u8.into())),
    );

    display.draw(
        Rect::new(icoord!(0, 0), icoord!(64, 64))
            .translate(icoord!(96, 0))
            .stroke(Some(1u8.into())),
    );

    display.draw(
        &Rect::new(icoord!(0, 0), icoord!(64, 64))
            .translate(icoord!(96 + 16, 16))
            .stroke(Some(0u8.into()))
            .fill(Some(1u8.into())),
    );

    display.draw(
        Rect::new(icoord!(0, 0), icoord!(64, 64))
            .translate(icoord!(96 + 32, 32))
            .stroke(Some(0u8.into()))
            .fill(Some(0u8.into())),
    );

    display.draw(
        Triangle::new(icoord!(32, 0), icoord!(0, 64), icoord!(64, 64))
            .translate(icoord!(96 * 2, 0))
            .stroke(Some(1u8.into())),
    );

    display.draw(
        Triangle::new(icoord!(32, 0), icoord!(0, 64), icoord!(64, 64))
            .translate(icoord!(96 * 2 + 16, 16))
            .stroke(Some(0u8.into()))
            .fill(Some(1u8.into())),
    );

    display.draw(
        Triangle::new(icoord!(32, 0), icoord!(0, 64), icoord!(64, 64))
            .translate(icoord!(96 * 2 + 32, 32))
            .stroke(Some(0u8.into()))
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
