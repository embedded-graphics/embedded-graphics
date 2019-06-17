use embedded_graphics::fonts::Font6x8;
use embedded_graphics::icoord;
use embedded_graphics::pixelcolor::BinaryColor::On as C1;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line};
use embedded_graphics_simulator::{BinaryColorTheme, DisplayBuilder};
use std::thread;
use std::time::Duration;

fn main() {
    let mut display = DisplayBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    // Outline
    display.draw(Circle::new(icoord!(64, 64), 64).stroke(Some(C1)));

    // Clock hands
    display.draw(Line::new(icoord!(64, 64), icoord!(0, 64)).stroke(Some(C1)));
    display.draw(Line::new(icoord!(64, 64), icoord!(80, 80)).stroke(Some(C1)));

    display.draw(
        Font6x8::render_str("Hello World!")
            .stroke(Some(C1))
            .translate(icoord!(5, 50)),
    );

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
