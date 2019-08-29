use embedded_graphics::fonts::{Font6x12, Font6x8, Font8x16};
use embedded_graphics::geometry::point;
use embedded_graphics::pixelcolor::BinaryColor::Off as C0;
use embedded_graphics::pixelcolor::BinaryColor::On as C1;
use embedded_graphics::prelude::*;
use embedded_graphics::{text_12x16, text_6x8};
use embedded_graphics_simulator::DisplayBuilder;
use std::thread;
use std::time::Duration;

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Fonts")
        .size(256, 128)
        .build_binary();

    // Show smallest font with black font on white background (default value for fonts)
    display.draw(Font6x8::render_str("Hello World! - default style 6x8").translate(point(15, 15)));

    // Show smallest font with white font on black background
    display.draw(
        Font6x8::render_str("Hello World! - inverse 6x8")
            .stroke(Some(C0))
            .fill(Some(C1))
            .translate(point(15, 30)),
    );

    // Show smallest font with white font on black background using a macro
    display.draw(
        text_6x8!(
            "Hello world! - inverse 6x8 with macro",
            stroke = Some(C0),
            fill = Some(C1)
        )
        .translate(point(15, 40)),
    );

    // Show 6x12 Font
    display.draw(Font6x12::render_str("Hello 6x12!").translate(point(15, 55)));

    // Show 8x16 Font
    display.draw(Font8x16::render_str("Hello 8x16!").translate(point(15, 80)));

    // Show 12x16 Font using a macro
    display.draw(text_12x16!("Hello 12x16!").translate(point(15, 105)));

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
