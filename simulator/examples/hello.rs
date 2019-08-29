use embedded_graphics::fonts::Font6x8;
use embedded_graphics::geometry::point;
use embedded_graphics::pixelcolor::BinaryColor::On as C1;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line};
use embedded_graphics_simulator::{BinaryColorTheme, DisplayBuilder};
use std::thread;
use std::time::Duration;

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Hello World")
        .theme(BinaryColorTheme::OledBlue)
        .build_binary();

    // Outline
    display.draw(Circle::new(point(64, 64), 64).stroke(Some(C1)));

    // Clock hands
    display.draw(Line::new(point(64, 64), point(0, 64)).stroke(Some(C1)));
    display.draw(Line::new(point(64, 64), point(80, 80)).stroke(Some(C1)));

    display.draw(
        Font6x8::render_str("Hello World!")
            .stroke(Some(C1))
            .translate(point(5, 50)),
    );

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
