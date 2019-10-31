use embedded_graphics::fonts::Font6x8;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line};
use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, WindowBuilder};

fn main() {
    let mut display = SimulatorDisplay::new(Size::new(129, 129));

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

    let mut window = WindowBuilder::new(&display)
        .title("Hello World")
        .theme(BinaryColorTheme::OledBlue)
        .build();
    window.show_static(&display);
}
