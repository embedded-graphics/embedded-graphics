//! # Example: Circle Debugging
//!
//! Use <kbd>Up</kb>/<kbd>Down</kbd> to change circle size. Circle's edge should lie on horizontal
//! line for correct size. The red/green circles should look identicle (aside from colour).
//!
//! Use <kbd>Left</kb>/<kbd>Right</kbd> to change circle stroke width.

extern crate embedded_graphics;
extern crate embedded_graphics_simulator;

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line};
use embedded_graphics::style::PrimitiveStyle;
use embedded_graphics::text_6x8;
use embedded_graphics_simulator::{SimulatorDisplay, SimulatorEvent, WindowBuilder};
use sdl2::keyboard::Keycode;

fn main() {
    let mut display = SimulatorDisplay::new(Size::new(300, 100));
    let mut window = WindowBuilder::new(&display)
        .scale(3)
        .title("Debug circle")
        .build();

    let mut size: u32 = 4;
    let mut stroke_width: u32 = 1;

    'running: loop {
        display.clear(Rgb888::BLACK);

        text_6x8!(
            &format!("Size: {}", size),
            text_color = Some(Rgb888::WHITE),
            background_color = Some(Rgb888::BLACK),
        )
        .draw(&mut display);

        text_6x8!(
            &format!("Stroke: {}", stroke_width),
            text_color = Some(Rgb888::WHITE),
            background_color = Some(Rgb888::BLACK),
        )
        .translate(Point::new(0, 8))
        .draw(&mut display);

        // Bounding lines to check size
        Line::new(
            Point::new(0, 50 - size as i32),
            Point::new(300, 50 - size as i32),
        )
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::BLUE, 1))
        .draw(&mut display);

        Circle::new(Point::new(50, 50), size)
            .into_styled(PrimitiveStyle::with_fill(Rgb888::RED))
            .draw(&mut display);

        Circle::new(Point::new(150, 50), size)
            .into_styled(PrimitiveStyle {
                // fill_color: Some(Rgb888::GREEN),
                fill_color: None,
                stroke_color: Some(Rgb888::GREEN),
                stroke_width,
            })
            .draw(&mut display);

        Circle::new(Point::new(250, 50), size)
            .into_styled(PrimitiveStyle {
                fill_color: Some(Rgb888::MAGENTA),
                stroke_color: Some(Rgb888::CYAN),
                stroke_width,
            })
            .draw(&mut display);

        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown { keycode, .. } => {
                    match keycode {
                        Keycode::Up => size += 1,
                        Keycode::Down => size = if size > 0 { size - 1 } else { 0 },
                        Keycode::Right => stroke_width += 1,
                        Keycode::Left => {
                            stroke_width = if stroke_width > 0 {
                                stroke_width - 1
                            } else {
                                0
                            }
                        }
                        _ => (),
                    };
                }

                _ => {}
            }
        }
    }
}
