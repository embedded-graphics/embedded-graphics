//! # Example: Hello
//!
//! Draw some shapes and text to the display

use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line},
    style::{PrimitiveStyle, TextStyle},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(129, 129));

    let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

    // Draw a circle 64px in radius with a center at (64, 64)
    Circle::new(Point::new(64, 64), 64)
        .into_styled(line_style)
        .draw(&mut display)?;

    // Draw a horizontal line
    Line::new(Point::new(64, 64), Point::new(0, 64))
        .into_styled(line_style)
        .draw(&mut display)?;

    // Draw a diagonal line
    Line::new(Point::new(64, 64), Point::new(80, 80))
        .into_styled(line_style)
        .draw(&mut display)?;

    // Write the text "Hello world!" to the screen
    Text::new("Hello World!", Point::new(5, 50))
        .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    Window::new("Hello World", &output_settings).show_static(&display);

    Ok(())
}
