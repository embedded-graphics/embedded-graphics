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
use embedded_graphics_simulator::{DisplayBuilder, RgbDisplay, SimulatorEvent};
use sdl2::keyboard::Keycode;

const BACKGROUND_COLOR: Option<Rgb888> = Some(Rgb888::BLACK);
const FOREGROUND_COLOR: Option<Rgb888> = Some(Rgb888::RED);
const KEYBOARD_DELTA: i32 = 20;

fn move_circle(display: &mut RgbDisplay, old_center: Point, new_center: Point) {
    // Clear old circle
    display.draw(Circle::new(old_center, 100).fill_color(BACKGROUND_COLOR));
    // Draw circle at new location
    display.draw(Circle::new(new_center, 100).fill_color(FOREGROUND_COLOR));
}

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Click to move circle")
        .size(800, 480)
        .build_rgb();

    let mut position = Point::new(200, 200);
    display.draw(Circle::new(position, 100).fill_color(FOREGROUND_COLOR));

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        match display.get_input_event() {
            Some(SimulatorEvent::KeyDown { keycode, .. }) => {
                let new_position = match keycode {
                    Some(Keycode::Left) => Point::new(position.x - KEYBOARD_DELTA, position.y),
                    Some(Keycode::Right) => Point::new(position.x + KEYBOARD_DELTA, position.y),
                    Some(Keycode::Up) => Point::new(position.x, position.y - KEYBOARD_DELTA),
                    Some(Keycode::Down) => Point::new(position.x, position.y + KEYBOARD_DELTA),
                    _ => position,
                };
                move_circle(&mut display, position, new_position);
                position = new_position;
            }
            Some(SimulatorEvent::MouseButtonUp { point, .. }) => {
                move_circle(&mut display, position, point);
                position = point;
            }
            _ => {}
        }
    }
}
