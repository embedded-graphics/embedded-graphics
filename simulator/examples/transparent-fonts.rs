use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::{egcircle, egrectangle, text_6x8};
use embedded_graphics_simulator::{SimulatorDisplay, WindowBuilder};
use std::thread;
use std::time::Duration;

fn main() {
    let mut display = SimulatorDisplay::new(Size::new(256, 128));
    let mut window = WindowBuilder::new(&display)
        .title("Fonts with transparent background")
        .scale(3)
        .build();

    egcircle!(
        (20, 20),
        20 as u32,
        stroke_color = Some(Rgb565::RED),
        fill_color = Some(Rgb565::RED)
    )
    .into_iter()
    .chain(egrectangle!(
        (20, 20),
        (100, 80),
        fill_color = Some(Rgb565::RED)
    ))
    .draw(&mut display);

    text_6x8!(
        "Hello world! - no background",
        stroke_color = Some(Rgb565::WHITE)
    )
    .translate(Point::new(15, 15))
    .draw(&mut display);

    text_6x8!(
        "Hello world! - filled background",
        stroke_color = Some(Rgb565::YELLOW),
        fill_color = Some(Rgb565::BLUE)
    )
    .translate(Point::new(15, 30))
    .draw(&mut display);

    text_6x8!(
        "Hello world! - inverse background",
        stroke_color = Some(Rgb565::BLUE),
        fill_color = Some(Rgb565::YELLOW)
    )
    .translate(Point::new(15, 45))
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
