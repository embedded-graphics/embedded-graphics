use embedded_graphics::pixelcolor::BinaryColor;
use sdl2::pixels::Color;

/// Color theme for binary displays
#[derive(Clone)]
pub enum BinaryColorTheme {
    /// A simple on/off, non-styled display with white background and black pixels
    Default,

    /// An on/off classic LCD-like display with white background
    LcdWhite,

    /// An on/off classic LCD-like display with green background and dark grey pixels
    LcdGreen,

    /// An on/off LCD-like display with light blue background and blue-white pixels
    LcdBlue,

    /// An on/off OLED-like display with a black background and white pixels
    OledWhite,

    /// An on/off OLED-like display with a dark blue background and light blue pixels
    OledBlue,
}

impl BinaryColorTheme {
    /// Get the theme's pixel color for a given pixel state
    pub fn convert(&self, color: BinaryColor) -> Color {
        match color {
            BinaryColor::Off => match self {
                BinaryColorTheme::Default => Color::RGB(255, 255, 255),
                BinaryColorTheme::LcdWhite => Color::RGB(245, 245, 245),
                BinaryColorTheme::LcdGreen => Color::RGB(120, 185, 50),
                BinaryColorTheme::LcdBlue => Color::RGB(70, 80, 230),
                BinaryColorTheme::OledBlue => Color::RGB(0, 20, 40),
                BinaryColorTheme::OledWhite => Color::RGB(20, 20, 20),
            },
            BinaryColor::On => match self {
                BinaryColorTheme::Default => Color::RGB(0, 0, 0),
                BinaryColorTheme::LcdWhite => Color::RGB(32, 32, 32),
                BinaryColorTheme::LcdGreen => Color::RGB(32, 32, 32),
                BinaryColorTheme::LcdBlue => Color::RGB(230, 230, 255),
                BinaryColorTheme::OledBlue => Color::RGB(0, 210, 255),
                BinaryColorTheme::OledWhite => Color::RGB(255, 255, 255),
            },
        }
    }
}
