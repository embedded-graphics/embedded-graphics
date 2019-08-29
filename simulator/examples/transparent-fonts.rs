use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::{egcircle, egrectangle, text_6x8};
use embedded_graphics_simulator::DisplayBuilder;
use std::thread;
use std::time::Duration;

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Fonts with transparent background")
        .scale(3)
        .build_rgb();

    display.draw(
        egcircle!(
            (20, 20),
            20 as u32,
            stroke = Some(Rgb565::RED),
            fill = Some(Rgb565::RED)
        )
        .into_iter()
        .chain(egrectangle!((20, 20), (100, 80), fill = Some(Rgb565::RED))),
    );

    display.draw(
        text_6x8!("Hello world! - no background", stroke = Some(Rgb565::WHITE))
            .translate(Point::new(15, 15)),
    );

    display.draw(
        text_6x8!(
            "Hello world! - filled background",
            stroke = Some(Rgb565::YELLOW),
            fill = Some(Rgb565::BLUE)
        )
        .translate(Point::new(15, 30)),
    );

    display.draw(
        text_6x8!(
            "Hello world! - inverse background",
            stroke = Some(Rgb565::BLUE),
            fill = Some(Rgb565::YELLOW)
        )
        .translate(Point::new(15, 45)),
    );

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
