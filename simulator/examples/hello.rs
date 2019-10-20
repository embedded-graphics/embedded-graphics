use embedded_graphics::fonts::Font6x8;
use embedded_graphics::pixelcolor::BinaryColor;
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
    Circle::new(Point::new(64, 64), 64)
        .stroke_color(Some(BinaryColor::On))
        .draw(&mut display);

    // Clock hands
    Line::new(Point::new(64, 64), Point::new(0, 64))
        .stroke_color(Some(BinaryColor::On))
        .draw(&mut display);
    Line::new(Point::new(64, 64), Point::new(80, 80))
        .stroke_color(Some(BinaryColor::On))
        .draw(&mut display);

    Font6x8::render_str("Hello World!")
        .stroke_color(Some(BinaryColor::On))
        .translate(Point::new(5, 50))
        .draw(&mut display);

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
