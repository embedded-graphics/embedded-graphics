use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rectangle, Triangle};
use embedded_graphics_simulator::{SimulatorDisplay, WindowBuilder};

const PADDING: i32 = 16;

fn main() {
    let mut display = SimulatorDisplay::new(Size::new(320, 256));

    let triangle = Triangle::new(Point::new(0, 64), Point::new(64, 0), Point::new(64, 64))
        .into_styled()
        .translate(Point::new(0, 0))
        .stroke_color(Some(BinaryColor::On));

    let rect = Rectangle::new(Point::new(0, 0), Point::new(64, 64))
        .into_styled()
        .translate(Point::new(64 + PADDING, 0))
        .stroke_color(Some(BinaryColor::On));

    let line = Line::new(Point::new(0, 0), Point::new(64, 64))
        .into_styled()
        .translate(Point::new(128 + PADDING * 2, 0))
        .stroke_color(Some(BinaryColor::On));

    let circle = Circle::new(Point::new(32, 32), 32)
        .translate(Point::new(192 + PADDING * 3, 0))
        .into_styled()
        .stroke_color(Some(BinaryColor::On));

    circle
        .into_iter()
        .chain(rect.into_iter())
        .chain(line.into_iter())
        .chain(triangle.into_iter())
        .draw(&mut display);

    circle
        .translate(Point::new(0, 64 + PADDING))
        .stroke_width(3)
        .into_iter()
        .chain(&rect.translate(Point::new(0, 64 + PADDING)).stroke_width(3))
        .chain(&line.translate(Point::new(0, 64 + PADDING)).stroke_width(3))
        .chain(
            &triangle
                .translate(Point::new(0, 64 + PADDING))
                .stroke_width(3),
        )
        .draw(&mut display);

    circle
        .translate(Point::new(0, 128 + PADDING * 2))
        .stroke_width(10)
        .into_iter()
        .chain(
            &rect
                .translate(Point::new(0, 128 + PADDING * 2))
                .stroke_width(10),
        )
        .chain(
            &line
                .translate(Point::new(0, 128 + PADDING * 2))
                .stroke_width(10),
        )
        .chain(
            &triangle
                .translate(Point::new(0, 128 + PADDING * 2))
                .stroke_width(10),
        )
        .draw(&mut display);

    let mut window = WindowBuilder::new(&display).title("Strokes").build();
    window.show_static(&display);
}
