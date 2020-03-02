use crate::{display::SimulatorDisplay, theme::BinaryColorTheme};
use embedded_graphics::{
    geometry::{Point, Size},
    pixelcolor::PixelColor,
    DrawTarget,
};

/// Output settings.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OutputSettings {
    /// Pixel scale.
    pub scale: u32,
    /// Spacing between pixels.
    pub pixel_spacing: u32,
    /// Binary color theme.
    pub theme: BinaryColorTheme,
}

impl OutputSettings {
    /// Calculates the size of the framebuffer required to display the scaled display.
    pub(crate) fn framebuffer_size<C>(&self, display: &SimulatorDisplay<C>) -> Size
    where
        C: PixelColor,
    {
        let width = display.size().width;
        let height = display.size().height;
        let output_width = width * self.scale + (width - 1) * self.pixel_spacing;
        let output_height = height * self.scale + (height - 1) * self.pixel_spacing;

        Size::new(output_width, output_height)
    }

    /// Translates a output coordinate to the corresponding display coordinate.
    pub(crate) const fn output_to_display(&self, output_point: Point) -> Point {
        let pitch = self.pixel_pitch() as i32;
        Point::new(output_point.x / pitch, output_point.y / pitch)
    }

    pub(crate) const fn pixel_pitch(&self) -> u32 {
        self.scale + self.pixel_spacing
    }
}

impl Default for OutputSettings {
    fn default() -> Self {
        Self {
            scale: 1,
            pixel_spacing: 0,
            theme: BinaryColorTheme::Default,
        }
    }
}

/// Output settings builder.
pub struct OutputSettingsBuilder {
    scale: Option<u32>,
    pixel_spacing: Option<u32>,
    theme: BinaryColorTheme,
}

impl OutputSettingsBuilder {
    /// Creates new output settings builder.
    pub fn new() -> Self {
        Self {
            scale: None,
            pixel_spacing: None,
            theme: BinaryColorTheme::Default,
        }
    }

    /// Sets the pixel scale.
    ///
    /// A scale of `2` or higher is useful for viewing the simulator on high DPI displays.
    pub fn scale(mut self, scale: u32) -> Self {
        if scale == 0 {
            panic!("scale must be >= 0");
        }

        self.scale = Some(scale);

        self
    }

    /// Sets the binary color theme for the display to use.
    pub fn theme(mut self, theme: BinaryColorTheme) -> Self {
        self.theme = theme;

        // Most binary color displays are small and individual pixels are hard to recognize
        // on higher resolution screens. So apply some default scaling when no scaling was explicitly
        // set.
        self.scale.get_or_insert(3);
        self.pixel_spacing.get_or_insert(1);

        self
    }

    /// Adds a gap between pixels, simulating the same effect of a physical display
    pub fn pixel_spacing(mut self, pixel_spacing: u32) -> Self {
        self.pixel_spacing = Some(pixel_spacing);

        self
    }

    /// Builds the window.
    pub fn build(self) -> OutputSettings {
        OutputSettings {
            scale: self.scale.unwrap_or(1),
            pixel_spacing: self.pixel_spacing.unwrap_or(0),
            theme: self.theme,
        }
    }
}
