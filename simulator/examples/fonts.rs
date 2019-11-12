use embedded_graphics::fonts::{Font6x12, Font6x8, Font8x16};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::{text_12x16, text_6x8};
use embedded_graphics_simulator::{SimulatorDisplay, WindowBuilder};

fn main() {
    let mut display = SimulatorDisplay::new(Size::new(256, 128));

    // Show smallest font with black font on white background (default value for fonts)
    Font6x8::render_str(
        "Hello World! - default style 6x8",
        TextStyle::new(BinaryColor::On),
    )
    .translate(Point::new(15, 15))
    .draw(&mut display);

    // Show smallest font with white font on black background
    let mut style = TextStyle::new(BinaryColor::Off);
    style.background_color = Some(BinaryColor::On);

    Font6x8::render_str("Hello World! - inverse 6x8", style)
        .translate(Point::new(15, 30))
        .draw(&mut display);

    // Show smallest font with white font on black background using a macro
    text_6x8!(
        "Hello world! - inverse 6x8 with macro",
        text_color = Some(BinaryColor::On),
        background_color = Some(BinaryColor::Off),
    )
    .translate(Point::new(15, 40))
    .draw(&mut display);

    let style = TextStyle::new(BinaryColor::On);

    // Show 6x12 Font
    Font6x12::render_str("Hello 6x12!", style)
        .translate(Point::new(15, 55))
        .draw(&mut display);

    // Show 8x16 Font
    Font8x16::render_str("Hello 8x16!", style)
        .translate(Point::new(15, 80))
        .draw(&mut display);

    // Show 12x16 Font using a macro
    text_12x16!("Hello 12x16!")
        .translate(Point::new(15, 105))
        .draw(&mut display);

    let mut window = WindowBuilder::new(&display).title("Fonts").build();
    window.show_static(&display);
}
