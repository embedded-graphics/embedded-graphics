//! # Example: Input Handling
//!
//! This example allows you to move a red circle to the location of a click on the simulator
//! screen, or move the circle using the arrow keys. Although input handling is not a part of the
//! embedded-graphics API, the simulator can be used to emulate input controls in order to
//! represent more complex UI systems such as touch screens.

extern crate embedded_graphics;
extern crate embedded_graphics_simulator;

use embedded_graphics::{
    pixelcolor::Rgb888, prelude::*, primitives::Circle, style::PrimitiveStyle,
};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};
use sdl2::keyboard::Keycode;

const BACKGROUND_COLOR: Rgb888 = Rgb888::BLACK;
const FOREGROUND_COLOR: Rgb888 = Rgb888::RED;
const KEYBOARD_DELTA: i32 = 20;

fn move_circle(
    display: &mut SimulatorDisplay<Rgb888>,
    old_pos: Point,
    new_pos: Point,
) -> Result<(), core::convert::Infallible> {
    // Clear old circle
    Circle::new(old_pos, 200)
        .into_styled(PrimitiveStyle::with_fill(BACKGROUND_COLOR))
        .draw(display)?;

    // Draw circle at new location
    Circle::new(new_pos, 200)
        .into_styled(PrimitiveStyle::with_fill(FOREGROUND_COLOR))
        .draw(display)?;

    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(800, 480));
    let mut window = Window::new("Click to move circle", &OutputSettings::default());

    let mut position = Point::new(100, 100);
    Circle::new(position, 200)
        .into_styled(PrimitiveStyle::with_fill(FOREGROUND_COLOR))
        .draw(&mut display)?;

    'running: loop {
        window.update(&display);

        for event in window.events() {
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
                    move_circle(&mut display, position, new_position)?;
                    position = new_position;
                }
                SimulatorEvent::MouseButtonUp { point, .. } => {
                    let point = point - Size::new(100, 100);
                    move_circle(&mut display, position, point)?;
                    position = point;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
