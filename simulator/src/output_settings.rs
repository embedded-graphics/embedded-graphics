use crate::{display::SimulatorDisplay, theme::BinaryColorTheme};
use embedded_graphics::prelude::*;

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
}

#[cfg(feature = "with-sdl")]
impl OutputSettings {
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
        OutputSettingsBuilder::new().build()
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
    ///
    /// # Panics
    ///
    /// Panics if the scale is set to `0`.
    pub fn scale(mut self, scale: u32) -> Self {
        if scale == 0 {
            panic!("scale must be >= 0");
        }

        self.scale = Some(scale);

        self
    }

    /// Sets the binary color theme.
    ///
    /// The binary color theme defines the mapping between the two display colors
    /// and the output. The variants provided by the [`BinaryColorTheme`] enum
    /// simulate the color scheme of commonly used display types.
    ///
    /// Most binary color displays are relatively small individual pixels
    /// are hard to recognize on higher resolution screens. Because of this
    /// some scaling is automatically applied to the output when a theme is
    /// set and no scaling was specified explicitly.
    ///
    /// Note that a theme should only be set when an monochrome display is used.
    /// Setting a theme when using a color display will cause an corrupted output.
    ///
    /// [`BinaryColorTheme`]: enum.BinaryColorTheme.html
    pub fn theme(mut self, theme: BinaryColorTheme) -> Self {
        self.theme = theme;

        self.scale.get_or_insert(3);
        self.pixel_spacing.get_or_insert(1);

        self
    }

    /// Sets the gap between pixels.
    ///
    /// Most lower resolution displays have visible gaps between individual pixels.
    /// This effect can be simulated by setting the pixel spacing to a value greater
    /// than `0`.
    pub fn pixel_spacing(mut self, pixel_spacing: u32) -> Self {
        self.pixel_spacing = Some(pixel_spacing);

        self
    }

    /// Builds the output settings.
    pub fn build(self) -> OutputSettings {
        OutputSettings {
            scale: self.scale.unwrap_or(1),
            pixel_spacing: self.pixel_spacing.unwrap_or(0),
            theme: self.theme,
        }
    }
}
