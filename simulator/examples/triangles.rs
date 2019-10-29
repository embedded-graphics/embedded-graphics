use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Triangle;
use embedded_graphics_simulator::SimulatorDisplay;
use std::thread;
use std::time::Duration;

const PAD: i32 = 10;

fn main() {
    let mut display = SimulatorDisplay::new(Size::new(512, 128));
    let mut window = display.build_window().title("Triangles").scale(2).build();

    // no straight lines
    Triangle::new(Point::new(0, 0), Point::new(64, 10), Point::new(15, 64))
        .translate(Point::new(PAD, 0))
        .stroke_color(Some(BinaryColor::On))
        .draw(&mut display);

    // flat top
    Triangle::new(Point::new(5, 0), Point::new(30, 64), Point::new(64, 0))
        .stroke_color(Some(BinaryColor::On))
        .translate(Point::new(64 + PAD, 0))
        .draw(&mut display);

    // flat left
    Triangle::new(Point::new(0, 0), Point::new(0, 64), Point::new(64, 30))
        .stroke_color(Some(BinaryColor::On))
        .translate(Point::new((64 + PAD) * 2, 0))
        .draw(&mut display);

    // flat bottom
    Triangle::new(Point::new(22, 0), Point::new(0, 64), Point::new(64, 64))
        .translate(Point::new((64 + PAD) * 3, 0))
        .stroke_color(Some(BinaryColor::On))
        .draw(&mut display);

    // flat right
    Triangle::new(Point::new(0, 22), Point::new(64, 0), Point::new(64, 64))
        .translate(Point::new((64 + PAD) * 4, 0))
        .stroke_color(Some(BinaryColor::On))
        .draw(&mut display);

    // draw filled above stroke, should not be visible
    Triangle::new(Point::new(0, 22), Point::new(64, 0), Point::new(64, 64))
        .translate(Point::new((64 + PAD) * 5, 0))
        .stroke_color(Some(BinaryColor::On))
        .draw(&mut display);

    Triangle::new(Point::new(0, 22), Point::new(64, 0), Point::new(64, 64))
        .translate(Point::new((64 + PAD) * 5, 0))
        .fill_color(Some(BinaryColor::On))
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
