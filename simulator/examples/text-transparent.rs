//! # Example: Transparent fonts
//!
//! Demonstrate the background styles and transparency behaviors of different font styles.

use embedded_graphics::{
    egcircle, egrectangle, egtext, fonts::Font6x8, pixelcolor::Rgb565, prelude::*, primitive_style,
    text_style,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(256, 128));

    egcircle!(
        center = (20, 20),
        radius = 20 as u32,
        style = primitive_style!(fill_color = Rgb565::RED)
    )
    .into_iter()
    .chain(&egrectangle!(
        top_left = (20, 20),
        bottom_right = (100, 80),
        style = primitive_style!(fill_color = Rgb565::RED)
    ))
    .draw(&mut display)
    .unwrap();

    egtext!(
        text = "Hello world! - no background",
        top_left = (15, 15),
        style = text_style!(font = Font6x8, text_color = Rgb565::WHITE,)
    )
    .draw(&mut display)
    .unwrap();

    egtext!(
        text = "Hello world! - filled background",
        top_left = (15, 30),
        style = text_style!(
            font = Font6x8,
            text_color = Rgb565::YELLOW,
            background_color = Rgb565::BLUE
        )
    )
    .draw(&mut display)
    .unwrap();

    egtext!(
        text = "Hello world! - inverse background",
        top_left = (15, 45),
        style = text_style!(
            font = Font6x8,
            text_color = Rgb565::BLUE,
            background_color = Rgb565::YELLOW
        )
    )
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(3).build();
    Window::new("Fonts with transparent background", &output_settings).show_static(&display);

    Ok(())
}
