use crate::sim_pixel_color::SimPixelColor;
use sdl2::pixels::Color;

#[derive(Clone)]
pub enum DisplayTheme {
    Default,
    LcdWhite,
    LcdGreen,
    LcdBlue,
    OledWhite,
    OledBlue,
    ColorOled,
}

impl DisplayTheme {
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
