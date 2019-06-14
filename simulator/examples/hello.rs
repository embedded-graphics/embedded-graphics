extern crate embedded_graphics;
extern crate simulator;

use std::thread;
use std::time::Duration;

use embedded_graphics::fonts::Font6x8;
use embedded_graphics::icoord;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line};

use simulator::{DisplayBuilder, DisplayTheme};

fn main() {
    let mut display = DisplayBuilder::new().theme(DisplayTheme::OledBlue).build();

    // Outline
    display.draw(Circle::new(icoord!(64, 64), 64).stroke(Some(1u8.into())));

    // Clock hands
    display.draw(Line::new(icoord!(64, 64), icoord!(0, 64)).stroke(Some(1u8.into())));
    display.draw(Line::new(icoord!(64, 64), icoord!(80, 80)).stroke(Some(1u8.into())));

    display.draw(
        Font6x8::render_str("Hello World!")
            .stroke(Some(1u8.into()))
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
