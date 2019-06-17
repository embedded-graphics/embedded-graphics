use crate::sim_pixel_color::SimPixelColor;
use sdl2::pixels::Color;

/// Display theme to use
#[derive(Clone)]
pub enum DisplayTheme {
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

    /// An OLED-like display that supports 24 bit colour output
    ColorOled,
}

impl DisplayTheme {
    /// Get the theme's pixel colour for a given pixel
    ///
    /// For on/off displays, a pixel value of `0, 0, 0` will be off, whilst any other value will be
    /// interpreted as an "on" pixel.
    pub fn pixel_color(&self, pixel: &SimPixelColor) -> Option<Color> {
        match self {
            DisplayTheme::ColorOled => Some(Color::RGB(pixel.0, pixel.1, pixel.2)),
            theme => {
                if *pixel != SimPixelColor(0, 0, 0) {
                    match theme {
                        DisplayTheme::Default => Some(Color::RGB(0, 0, 0)),
                        DisplayTheme::LcdWhite => Some(Color::RGB(32, 32, 32)),
                        DisplayTheme::LcdGreen => Some(Color::RGB(32, 32, 32)),
                        DisplayTheme::LcdBlue => Some(Color::RGB(230, 230, 255)),
                        DisplayTheme::OledBlue => Some(Color::RGB(0, 210, 255)),
                        DisplayTheme::OledWhite => Some(Color::RGB(255, 255, 255)),
                        _ => unreachable!(),
                    }
                } else {
                    None
                }
            }
        }
    }

    /// Get the background colour for the current theme
    pub fn background_color(&self) -> Color {
        match self {
            DisplayTheme::Default => Color::RGB(255, 255, 255),
            DisplayTheme::LcdWhite => Color::RGB(245, 245, 245),
            DisplayTheme::LcdGreen => Color::RGB(120, 185, 50),
            DisplayTheme::LcdBlue => Color::RGB(70, 80, 230),
            DisplayTheme::OledBlue => Color::RGB(0, 20, 40),
            DisplayTheme::OledWhite | DisplayTheme::ColorOled => Color::RGB(20, 20, 20),
        }
    }
}
