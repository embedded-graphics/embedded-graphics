use embedded_graphics::egtext;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, WindowBuilder};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct SevenSegmentFont;

impl Font for SevenSegmentFont {
    const FONT_IMAGE: &'static [u8] = include_bytes!("seven-segment.raw");
    const FONT_IMAGE_WIDTH: u32 = 224;

    const CHARACTER_SIZE: Size = Size::new(22, 40);
    const CHARACTER_SPACING: u32 = 4;

    fn char_offset(c: char) -> u32 {
        c.to_digit(10).unwrap_or(0)
    }
}

fn main() {
    let mut display = SimulatorDisplay::new(Size::new(128, 128));

    let position = Point::new(27, 22);
    let row_offset = Point::new(0, 44);

    egtext!(
        "123",
        position,
        font = SevenSegmentFont,
        text_color = Some(BinaryColor::On),
    )
    .draw(&mut display);

    egtext!(
        "456",
        position + row_offset,
        font = SevenSegmentFont,
        text_color = Some(BinaryColor::On),
    )
    .draw(&mut display);

    let mut window = WindowBuilder::new(&display)
        .title("Custom font")
        .theme(BinaryColorTheme::OledBlue)
        .build();
    window.show_static(&display);
}
