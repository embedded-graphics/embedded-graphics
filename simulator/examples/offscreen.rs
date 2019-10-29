use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics_simulator::SimulatorDisplay;
use std::thread;
use std::time::Duration;

fn main() {
    let mut display = SimulatorDisplay::new(Size::new(32, 32));
    let mut window = display.build_window().title("Offscreen").scale(4).build();

    // Outline
    Rectangle::new(Point::new(0, 0), Point::new(16, 16))
        .stroke_color(Some(BinaryColor::On))
        .translate(Point::new(-8, -8))
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
