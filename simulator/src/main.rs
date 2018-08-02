extern crate embedded_graphics;
extern crate simulator;

use std::thread;
use std::time::Duration;

use embedded_graphics::coord::Coord;
// use embedded_graphics::fonts::Font6x8;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Line;

use simulator::{Display, SimPixelColor};

fn main() {
    let mut display = Display::new();

    // Outline
    display.draw(Circle::new(Coord::new(64, 64), 63, 1u8).into_iter());

    // Clock hands
    display.draw(Line::new(Coord::new(64, 64), Coord::new(0, 64), SimPixelColor(true)).into_iter());
    display
        .draw(Line::new(Coord::new(64, 64), Coord::new(80, 80), SimPixelColor(true)).into_iter());

    display.draw(
        Font6x8::render_str("Hello World!", 1)
            .translate(Coord::new(5, 50))
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
