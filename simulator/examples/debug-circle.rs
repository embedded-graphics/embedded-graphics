//! Delete me! Only used to test #143!

use embedded_graphics::egcircle;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::DisplayBuilder;
use std::thread;
use std::time::Duration;

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Strokes")
        .size(128, 64)
        .scale(10)
        .build_rgb();

    display.draw(egcircle!((10, 16), 3, fill = Some(Rgb888::RED)));

    display.draw(egcircle!(
        (22, 16),
        3,
        fill = Some(Rgb888::GREEN),
        stroke = Some(Rgb888::GREEN)
    ));

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
