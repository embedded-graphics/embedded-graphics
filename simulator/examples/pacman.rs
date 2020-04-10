//! # Example: Pacman
//!
//! An example displaying an animated Pacman.

use embedded_graphics::{
    pixelcolor::Rgb565, prelude::*, primitives::Circle, primitives::Sector,
    style::PrimitiveStyleBuilder,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use std::{thread, time::Duration};

// the number of steps of the animation
const STEPS: i32 = 10;

fn main() -> Result<(), std::convert::Infallible> {
    // Create a new simulator display with 65x65 pixels.
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(65, 65));

    // Create styles used by the drawing operations.
    let sector_style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::BLACK)
        .stroke_width(2)
        .fill_color(Rgb565::YELLOW)
        .build();
    let eye_style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::BLACK)
        .stroke_width(1)
        .fill_color(Rgb565::BLACK)
        .build();

    let output_settings = OutputSettingsBuilder::new().scale(4).build();
    let mut window = Window::new("Pacman", &output_settings);

    // The current progress of the animation
    let mut progress: i32 = 0;

    'running: loop {
        display.clear(Rgb565::WHITE)?;

        let p = (progress - STEPS).abs();

        // Draw a Sector as the main Pacman feature.
        Sector::new(
            Point::new(2, 2),
            61,
            Angle::from_degrees((p * 30 / STEPS) as f32),
            Angle::from_degrees((360 - 2 * p * 30 / STEPS) as f32),
        )
        .into_styled(sector_style)
        .draw(&mut display)?;

        // Draw a Circle as the eye.
        Circle::new(Point::new(36, 16), 5)
            .into_styled(eye_style)
            .draw(&mut display)?;

        window.update(&display);

        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }
        thread::sleep(Duration::from_millis(50));

        progress = (progress + 1) % (2 * STEPS + 1);
    }
}
