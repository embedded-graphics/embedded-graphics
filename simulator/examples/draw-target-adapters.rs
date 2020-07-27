//! # Example: Draw target adapters
//!
//! TODO: docs

use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::Rectangle,
    style::TextStyle,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(128, 64));

    Text::new("Original text", Point::zero())
        .into_styled(TextStyle::new(Font6x8, Rgb565::GREEN))
        .draw(&mut display)?;

    let mut translated = display.translated(Point::new(20, 20));
    Text::new("Translated text", Point::zero())
        .into_styled(TextStyle::new(Font6x8, Rgb565::BLUE))
        .draw(&mut translated)?;

    let text = Text::new("Clipped text", Point::new(10, 50));
    text.into_styled(TextStyle::new(Font6x8, Rgb565::RED))
        .draw(&mut display)?;

    let mut clipped = display.clipped(Rectangle::new(Point::new(10, 54), Size::new(128, 4)));
    text.into_styled(TextStyle::new(Font6x8, Rgb565::YELLOW))
        .draw(&mut clipped)?;

    let output_settings = OutputSettingsBuilder::new().scale(5).build();

    Window::new("Draw target adapters", &output_settings).show_static(&display);

    Ok(())
}
