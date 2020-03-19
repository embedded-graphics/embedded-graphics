//! # Example: Fonts
//!
//! Demonstrate the available builtin fonts.

use embedded_graphics::{
    egtext,
    fonts::{Font12x16, Font24x32, Font6x12, Font6x6, Font6x8, Font8x16, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::{TextStyle, TextStyleBuilder},
    text_style,
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

    // Show smallest font with white font on black background using a macro
    egtext!(
        text = "Hello world! - inverse 6x8 with macro",
        top_left = (15, 40),
        style = text_style!(
            font = Font6x8,
            text_color = BinaryColor::On,
            background_color = BinaryColor::Off
        )
    )
    .draw(&mut display)?;

    // Show 6x12 Font
    Text::new("Hello 6x12!", Point::new(15, 55))
        .into_styled(TextStyle::new(Font6x12, BinaryColor::On))
        .draw(&mut display)?;

    // Show 8x16 Font
    Text::new("Hello 8x16!", Point::new(15, 80))
        .into_styled(TextStyle::new(Font8x16, BinaryColor::On))
        .draw(&mut display)?;

    // Show 12x16 Font using a macro
    egtext!(
        text = "Hello 12x16!",
        top_left = (15, 105),
        style = text_style!(font = Font12x16, text_color = BinaryColor::On)
    )
    .draw(&mut display)?;

    // Show 24x32 Font using a macro
    egtext!(
        text = "Hello 24x32!",
        top_left = (15, 128),
        style = text_style!(font = Font24x32, text_color = BinaryColor::On)
    )
    .draw(&mut display)?;

    // Show 6x6 Font using a macro
    egtext!(
        text = "Hello 6x6!",
        top_left = (15, 170),
        style = text_style!(font = Font6x6, text_color = BinaryColor::On)
    )
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Fonts", &output_settings).show_static(&display);

    Ok(())
}
