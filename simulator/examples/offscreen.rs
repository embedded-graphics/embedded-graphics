extern crate embedded_graphics;
extern crate simulator;

use std::thread;
use std::time::Duration;

use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rect;

use simulator::Display;

fn main() {
    let mut display = Display::new();

    // Outline
    display.draw(
        Rect::new(Coord::new(0, 0), Coord::new(16, 16))
            .with_stroke(Some(1u8.into()))
            .translate(Coord::new(-8, -8))
            .into_iter(),
    );

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
