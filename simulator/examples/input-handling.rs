//! # Example: Input Handling
//!
//! This example allows you to move a red circle to the location of a click on the simulator
//! screen, or move the circle using the arrow keys. Although input handling is not a part of the
//! embedded-graphics API, the simulator can be used to emulate input controls in order to
//! represent more complex UI systems such as touch screens.
extern crate embedded_graphics;
extern crate embedded_graphics_simulator;

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Rectangle};
use embedded_graphics_simulator::{DisplayBuilder, RgbDisplay, SimulatorEvent};
use sdl2::keyboard::Keycode;

const BACKGROUND_COLOR: Option<Rgb888> = Some(Rgb888::BLACK);
const FOREGROUND_COLOR: Option<Rgb888> = Some(Rgb888::RED);
const KEYBOARD_DELTA: i32 = 20;

fn move_circle(display: &mut RgbDisplay, old_center: Point, new_center: Point) {
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
    let mut display = DisplayBuilder::new()
        .title("Click to move circle")
        .size(800, 480)
        .build_rgb();

    let position2 = Point::new(1, 1);
    let position3 = Point::new(100, 100);
    Rectangle::new(position2, position3)
        .fill_color(FOREGROUND_COLOR)
        .draw(&mut display);

    let mut position = Point::new(200, 200);
    Circle::new(position, 100)
        .fill_color(FOREGROUND_COLOR)
        .draw(&mut display);

    'running: loop {
        for event in display.get_input_events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown { keycode, .. } => {
                    let delta = match keycode {
                        Keycode::Left => Point::new(-KEYBOARD_DELTA, 0),
                        Keycode::Right => Point::new(KEYBOARD_DELTA, 0),
                        Keycode::Up => Point::new(0, -KEYBOARD_DELTA),
                        Keycode::Down => Point::new(0, KEYBOARD_DELTA),
                        _ => Point::zero(),
                    };
                    let new_position = position + delta;
                    move_circle(&mut display, position, new_position);
                    position = new_position;
                }
                SimulatorEvent::MouseButtonUp { point, .. } => {
                    move_circle(&mut display, position, point);
                    position = point;
                }
                _ => {}
            }
        }

        display.flush();
    }
}
