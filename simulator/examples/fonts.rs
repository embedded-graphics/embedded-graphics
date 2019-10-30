use embedded_graphics::fonts::{Font6x12, Font6x8, Font8x16};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::{text_12x16, text_6x8};
use embedded_graphics_simulator::{SimulatorDisplay, WindowBuilder};
use std::thread;
use std::time::Duration;

fn main() {
    let mut display = SimulatorDisplay::new(Size::new(256, 128));
    let mut window = WindowBuilder::new(&display).title("Fonts").build();

    // Show smallest font with black font on white background (default value for fonts)
    Font6x8::render_str("Hello World! - default style 6x8")
        .translate(Point::new(15, 15))
        .draw(&mut display);

    // Show smallest font with white font on black background
    Font6x8::render_str("Hello World! - inverse 6x8")
        .stroke_color(Some(BinaryColor::Off))
        .fill_color(Some(BinaryColor::On))
        .translate(Point::new(15, 30))
        .draw(&mut display);

    // Show smallest font with white font on black background using a macro
    text_6x8!(
        "Hello world! - inverse 6x8 with macro",
        stroke_color = Some(BinaryColor::Off),
        fill_color = Some(BinaryColor::On)
    )
    .translate(Point::new(15, 40))
    .draw(&mut display);

    // Show 6x12 Font
    Font6x12::render_str("Hello 6x12!")
        .translate(Point::new(15, 55))
        .draw(&mut display);

    // Show 8x16 Font
    Font8x16::render_str("Hello 8x16!")
        .translate(Point::new(15, 80))
        .draw(&mut display);

    // Show 12x16 Font using a macro
    text_12x16!("Hello 12x16!")
        .translate(Point::new(15, 105))
        .draw(&mut display);

    loop {
        window.update(&display);

        let end = window.handle_events();
        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
