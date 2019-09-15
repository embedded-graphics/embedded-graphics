//! # Example: Input Handling
//!
//! This example allows you to move a red circle to the location of a click on the simulator
//! screen. Although input handling is not a part of the embedded-graphics API, the simulator can
//! be used to emulate input controls in order to represent more complex UI systems such as touch
//! screens.
extern crate embedded_graphics;
extern crate embedded_graphics_simulator;

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Circle;
use embedded_graphics_simulator::DisplayBuilder;

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Click to move circle")
        .size(800, 480)
        .build_rgb();
    let background_color = Some(Rgb888::BLACK);
    let foreground_color = Some(Rgb888::RED);

    let mut position = Point::new(200, 200);
    display.draw(Circle::new(position, 100).fill_color(foreground_color));

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        if let Some(new_position) = display.get_input_event() {
            // Clear old circle
            display.draw(Circle::new(position, 100).fill_color(background_color));
            position = new_position;
            // Draw circle at new location
            display.draw(Circle::new(position, 100).fill_color(foreground_color));
        }
    }
}
