//! # Example: Fonts
//!
//! Demonstrate the available builtin fonts.

use embedded_graphics::{
    fonts::{Font12x16, Font24x32, Font6x12, Font6x6, Font6x8, Font8x16, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::{TextStyle, TextStyleBuilder},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(350, 200));

    // Show smallest font with black font on white background (default value for fonts)
    Text::new("Hello World! - default style 6x8", Point::new(15, 15))
        .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
        .draw(&mut display)?;

    // Show smallest font with white font on black background
    let style = TextStyleBuilder::new(Font6x8)
        .text_color(BinaryColor::Off)
        .background_color(BinaryColor::On)
        .build();

    Text::new("Hello World! - inverse 6x8", Point::new(15, 30))
        .into_styled(style)
        .draw(&mut display)?;

    // Show 6x12 Font
    Text::new("Hello 6x12!", Point::new(15, 45))
        .into_styled(TextStyle::new(Font6x12, BinaryColor::On))
        .draw(&mut display)?;

    // Show 8x16 Font
    Text::new("Hello 8x16!", Point::new(15, 70))
        .into_styled(TextStyle::new(Font8x16, BinaryColor::On))
        .draw(&mut display)?;

    // Show 12x16 Font
    Text::new("Hello 12x16!", Point::new(15, 95))
        .into_styled(TextStyle::new(Font12x16, BinaryColor::On))
        .draw(&mut display)?;

    // Show 24x32 Font
    Text::new("Hello 24x32!", Point::new(15, 118))
        .into_styled(TextStyle::new(Font24x32, BinaryColor::On))
        .draw(&mut display)?;

    // Show 6x6 Font
    Text::new("Hello 6x6!", Point::new(15, 160))
        .into_styled(TextStyle::new(Font6x6, BinaryColor::On))
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Fonts", &output_settings).show_static(&display);

    Ok(())
}
