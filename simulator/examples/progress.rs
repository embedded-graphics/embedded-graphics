//! # Example: Progress
//!
//! An example displaying a progress circle.

use embedded_graphics::{
    fonts::{Font12x16, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::Arc,
    style::{PrimitiveStyleBuilder, StrokeAlignment, TextStyle},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use std::{thread, time::Duration};

fn main() -> Result<(), std::convert::Infallible> {
    // Create a new simulator display with 64x64 pixels.
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(64, 64));

    // Create styles used by the drawing operations.
    let arc_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(5)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();
    let text_style = TextStyle::new(Font12x16, BinaryColor::On);

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Progress", &output_settings);

    // The current progress percentage
    let mut progress = 0;

    'running: loop {
        display.clear(BinaryColor::Off)?;

        let sweep = progress as f32 * 360.0 / 100.0;

        // Draw an arc with a 5px wide stroke.
        Arc::new(Point::new(2, 2), 64 - 4, 90.0.deg(), sweep.deg())
            .into_styled(arc_stroke)
            .draw(&mut display)?;

        // Draw centered text.
        let text = format!("{}%", progress);
        let width = text.len() as i32 * 12;
        Text::new(&text, Point::new(32 - width / 2, 32 - 16 / 2))
            .into_styled(text_style)
            .draw(&mut display)?;

        window.update(&display);

        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }
        thread::sleep(Duration::from_millis(50));

        progress = (progress + 1) % 101;
    }
}
