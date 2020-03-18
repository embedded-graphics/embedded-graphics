//! # Example: Multiline text
//!
//! Exercise the font renderer to demonstrate rendering of multiline text

use embedded_graphics::{
    egtext,
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyleBuilder,
    text_style,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(256, 128));

    // Show multiline text example
    let style = TextStyleBuilder::new(Font6x8)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build();

    Text::new("This is a\nmultiline\nHello World!", Point::new(15, 15))
        .into_styled(style)
        .draw(&mut display)?;

    // Show multiline text example using a macro
    egtext!(
        text = "This is a\nmultiline\nHello World!\nwith macro",
        top_left = (15, 64),
        style = text_style!(
            font = Font6x8,
            text_color = BinaryColor::On,
            background_color = BinaryColor::Off
        )
    )
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Fonts", &output_settings).show_static(&display);

    Ok(())
}
