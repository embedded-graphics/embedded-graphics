extern crate embedded_graphics;
extern crate embedded_graphics_simulator;

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Circle;
use embedded_graphics_simulator::SimulatorDisplay;

const BACKGROUND_COLOR: Option<Rgb888> = Some(Rgb888::BLACK);
const FOREGROUND_COLOR: Option<Rgb888> = Some(Rgb888::RED);

fn move_circle(display: &mut SimulatorDisplay<Rgb888>, old_center: Point, new_center: Point) {
    // Clear old circle
    Circle::new(old_center, 100)
        .fill_color(BACKGROUND_COLOR)
        .draw(display);
    // Draw circle at new location
    Circle::new(new_center, 100)
        .fill_color(FOREGROUND_COLOR)
        .draw(display);
}

fn main() {
    let mut display = SimulatorDisplay::new(Size::new(800, 480));
    let mut window = display.build_window().title("Benchmark").build();

    let mut position = Point::new(200, 200);
    Circle::new(position, 100)
        .fill_color(FOREGROUND_COLOR)
        .draw(&mut display);

    for _ in 0..500 {
        window.update(&display);

        let end = window.handle_events();
        if end {
            break;
        }

        let new_position = position + Point::new(1, 0);
        move_circle(&mut display, position, new_position);
        position = new_position;
    }
}
