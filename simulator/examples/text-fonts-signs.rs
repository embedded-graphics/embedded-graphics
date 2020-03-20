use embedded_graphics::{
    fonts::{Text, *},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyle,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(1200, 160));

    let test_text  = "¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ";
    let test_text_with_line_break  = "¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏ\nÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ";

    // Show smallest font with black font on white background (default value for fonts)
    Text::new(test_text, Point::new(15, 15))
        .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
        .draw(&mut display)?;

    // Doesn't support as much
    // // Show 6x12 Font
    Text::new("-- Font 6x12 not support!", Point::new(15, 30))
        .into_styled(TextStyle::new(Font6x12, BinaryColor::On))
        .draw(&mut display)?;

    // Show 8x16 Font
    Text::new(test_text, Point::new(15, 45))
        .into_styled(TextStyle::new(Font8x16, BinaryColor::On))
        .draw(&mut display)?;

    // Show 12x16 Font
    Text::new(test_text, Point::new(15, 65))
        .into_styled(TextStyle::new(Font12x16, BinaryColor::On))
        .draw(&mut display)?;

    // Show 24x32 Font
    Text::new(test_text_with_line_break, Point::new(15, 85))
        .into_styled(TextStyle::new(Font24x32, BinaryColor::On))
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    Window::new("Fonts", &output_settings).show_static(&display);

    Ok(())
}
