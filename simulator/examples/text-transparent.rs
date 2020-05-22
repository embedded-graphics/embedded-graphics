//! # Example: Transparent fonts
//!
//! Demonstrate the background styles and transparency behaviors of different font styles.

use embedded_graphics::{
    egcircle, egrectangle,
    fonts::{Font6x8, Text},
    pixelcolor::Rgb565,
    prelude::*,
    primitive_style,
    style::TextStyleBuilder,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(256, 128));

    egcircle!(
        top_left = (0, 0),
        diameter = 41,
        style = primitive_style!(fill_color = Rgb565::RED)
    )
    .into_iter()
    .chain(&egrectangle!(
        top_left = (20, 20),
        size = (80, 60),
        style = primitive_style!(fill_color = Rgb565::RED)
    ))
    .draw(&mut display)
    .unwrap();

    Text::new("Hello world! - no background", Point::new(15, 15))
        .into_styled(
            // Can also be written in the shorter form: TextStyle:new(Font6x8, Rgb565::WHITE)
            TextStyleBuilder::new(Font6x8)
                .text_color(Rgb565::WHITE)
                .build(),
        )
        .draw(&mut display)
        .unwrap();

    Text::new("Hello world! - filled background", Point::new(15, 30))
        .into_styled(
            TextStyleBuilder::new(Font6x8)
                .text_color(Rgb565::YELLOW)
                .background_color(Rgb565::BLUE)
                .build(),
        )
        .draw(&mut display)
        .unwrap();

    Text::new("Hello world! - inverse background", Point::new(15, 45))
        .into_styled(
            TextStyleBuilder::new(Font6x8)
                .text_color(Rgb565::BLUE)
                .background_color(Rgb565::YELLOW)
                .build(),
        )
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(3).build();
    Window::new("Fonts with transparent background", &output_settings).show_static(&display);

    Ok(())
}
