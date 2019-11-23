use embedded_graphics::egtext;
use embedded_graphics::fonts::{Text, Font12x16, Font6x12, Font6x8, Font8x16};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::style::TextStyle;
use embedded_graphics_simulator::{SimulatorDisplay, WindowBuilder};

fn main() {
    let mut display = SimulatorDisplay::new(Size::new(256, 128));

    // Show smallest font with black font on white background (default value for fonts)
    Text::new("Hello World! - default style 6x8", Point::new(15, 15))
        .into_styled(TextStyle::with_text_color(Font6x8, BinaryColor::On))
        .draw(&mut display);

    // Show smallest font with white font on black background
    let style = TextStyle {
        font: Font6x8,
        text_color: Some(BinaryColor::Off),
        background_color: Some(BinaryColor::On),
    };

    Text::new("Hello World! - inverse 6x8", Point::new(15, 30))
        .into_styled(style)
        .draw(&mut display);

    // Show smallest font with white font on black background using a macro
    egtext!(
        "Hello world! - inverse 6x8 with macro",
        font = Font6x8,
        text_color = Some(BinaryColor::On),
        background_color = Some(BinaryColor::Off),
    )
    .translate(Point::new(15, 40))
    .draw(&mut display);

    // Show 6x12 Font
    Text::new("Hello 6x12!", Point::new(15, 55))
        .into_styled(TextStyle::with_text_color(Font6x12, BinaryColor::On))
        .draw(&mut display);

    // Show 8x16 Font
    Text::new("Hello 8x16!", Point::new(15, 80))
        .into_styled(TextStyle::with_text_color(Font8x16, BinaryColor::On))
        .draw(&mut display);

    // Show 12x16 Font using a macro
    egtext!("Hello 12x16!", font = Font12x16)
        .translate(Point::new(15, 105))
        .draw(&mut display);

    let mut window = WindowBuilder::new(&display).title("Fonts").build();
    window.show_static(&display);
}
