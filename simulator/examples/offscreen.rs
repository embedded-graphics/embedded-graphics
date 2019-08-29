use embedded_graphics::geometry::point;
use embedded_graphics::pixelcolor::BinaryColor::On as C1;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics_simulator::DisplayBuilder;
use std::thread;
use std::time::Duration;

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Offscreen")
        .size(32, 32)
        .scale(4)
        .build_binary();

    // Outline
    display.draw(
        Rectangle::new(point(0, 0), point(16, 16))
            .stroke(Some(C1))
            .translate(point(-8, -8)),
    );

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
