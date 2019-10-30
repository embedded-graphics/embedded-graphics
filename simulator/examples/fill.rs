use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Rectangle, Triangle};
use embedded_graphics_simulator::{SimulatorDisplay, WindowBuilder};
use std::thread;
use std::time::Duration;

static CIRCLE_SIZE: i32 = 32;

fn main() {
    let mut display = SimulatorDisplay::new(Size::new(304, 128));
    let mut window = WindowBuilder::new(&display)
        .title("Filled primitives")
        .scale(2)
        .build();

    Circle::new(Point::new(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32)
        .stroke_color(Some(BinaryColor::On))
        .draw(&mut display);

    Circle::new(Point::new(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32)
        .translate(Point::new(16, 16))
        .stroke_color(Some(BinaryColor::Off))
        .fill_color(Some(BinaryColor::On))
        .draw(&mut display);

    Circle::new(Point::new(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32)
        .translate(Point::new(CIRCLE_SIZE, CIRCLE_SIZE))
        .stroke_color(Some(BinaryColor::Off))
        .fill_color(Some(BinaryColor::Off))
        .draw(&mut display);

    Rectangle::new(Point::new(0, 0), Point::new(64, 64))
        .translate(Point::new(96, 0))
        .stroke_color(Some(BinaryColor::On))
        .draw(&mut display);

    Rectangle::new(Point::new(0, 0), Point::new(64, 64))
        .translate(Point::new(96 + 16, 16))
        .stroke_color(Some(BinaryColor::Off))
        .fill_color(Some(BinaryColor::On))
        .draw(&mut display);

    Rectangle::new(Point::new(0, 0), Point::new(64, 64))
        .translate(Point::new(96 + 32, 32))
        .stroke_color(Some(BinaryColor::Off))
        .fill_color(Some(BinaryColor::Off))
        .draw(&mut display);

    Triangle::new(Point::new(32, 0), Point::new(0, 64), Point::new(64, 64))
        .translate(Point::new(96 * 2, 0))
        .stroke_color(Some(BinaryColor::On))
        .draw(&mut display);

    Triangle::new(Point::new(32, 0), Point::new(0, 64), Point::new(64, 64))
        .translate(Point::new(96 * 2 + 16, 16))
        .stroke_color(Some(BinaryColor::Off))
        .fill_color(Some(BinaryColor::On))
        .draw(&mut display);

    Triangle::new(Point::new(32, 0), Point::new(0, 64), Point::new(64, 64))
        .translate(Point::new(96 * 2 + 32, 32))
        .stroke_color(Some(BinaryColor::Off))
        .fill_color(Some(BinaryColor::Off))
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
