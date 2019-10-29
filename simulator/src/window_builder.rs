use crate::theme::BinaryColorTheme;
use crate::window::Window;
use embedded_graphics::geometry::Size;

/// Create a simulator display using the builder pattern
#[derive(Debug)]
pub struct WindowBuilder {
    display_size: Size,
    scale: usize,
    pixel_spacing: usize,
    theme: BinaryColorTheme,
    title: String,
}

impl WindowBuilder {
    pub(crate) fn new(display_size: Size) -> Self {
        Self {
            display_size,
            scale: 1,
            pixel_spacing: 0,
            theme: BinaryColorTheme::Default,
            title: String::from("embedded-graphics-simulator"),
        }
    }

    /// Sets the pixel scale.
    ///
    /// A scale of `2` or higher is useful for viewing the simulator on high DPI displays.
    pub fn scale(&mut self, scale: usize) -> &mut Self {
        if scale == 0 {
            panic!("scale must be >= 0");
        }

        self.scale = scale;

        self
    }

    /// Sets the binary color theme for the display to use.
    pub fn theme(&mut self, theme: BinaryColorTheme) -> &mut Self {
        self.theme = theme;

        self.scale(3);
        self.pixel_spacing(1);

        self
    }

    /// Adds a gap between pixels, simulating the same effect of a physical display
    pub fn pixel_spacing(&mut self, pixel_spacing: usize) -> &mut Self {
        self.pixel_spacing = pixel_spacing;

        self
    }

    /// Sets the window title.
    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = title.to_owned();

        self
    }

    /// Builds the window.
    pub fn build(&self) -> Window {
        Window::new(
            self.display_size,
            self.scale,
            self.pixel_spacing,
            self.theme,
            &self.title,
        )
    }
}
