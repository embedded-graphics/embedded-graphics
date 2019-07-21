use embedded_graphics::pixelcolor::{BinaryColor, Rgb888, RgbColor};

/// Color theme for binary displays
#[derive(Clone)]
pub enum BinaryColorTheme {
    /// A simple on/off, non-styled display with black background and white pixels
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

pub fn map_color(color: BinaryColor, color_off: Rgb888, color_on: Rgb888) -> Rgb888 {
    match color {
        BinaryColor::On => color_on,
        BinaryColor::Off => color_off,
    }
}

impl BinaryColorTheme {
    /// Get the theme's pixel color for a given pixel state
    pub fn convert(&self, color: BinaryColor) -> Rgb888 {
        match self {
            BinaryColorTheme::Default => color.into(),
            BinaryColorTheme::LcdWhite => {
                map_color(color, Rgb888::new(245, 245, 245), Rgb888::new(32, 32, 32))
            }
            BinaryColorTheme::LcdGreen => {
                map_color(color, Rgb888::new(120, 185, 50), Rgb888::new(32, 32, 32))
            }
            BinaryColorTheme::LcdBlue => {
                map_color(color, Rgb888::new(70, 80, 230), Rgb888::new(230, 230, 255))
            }
            BinaryColorTheme::OledBlue => {
                map_color(color, Rgb888::new(0, 20, 40), Rgb888::new(0, 210, 255))
            }
            BinaryColorTheme::OledWhite => map_color(color, Rgb888::new(20, 20, 20), Rgb888::WHITE),
        }
    }
}
