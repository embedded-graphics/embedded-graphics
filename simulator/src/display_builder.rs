use crate::display_theme::BinaryColorTheme;
use crate::window::Window;
use crate::{BinaryDisplay, PixelData, RgbDisplay};

/// Create a simulator display using the builder pattern
pub struct DisplayBuilder {
    width: usize,
    height: usize,
    scale: usize,
    pixel_spacing: usize,
    theme: BinaryColorTheme,
    title: String,
}

impl DisplayBuilder {
    /// Create a new display with default settings
    pub fn new() -> Self {
        Self {
            width: 256,
            height: 256,
            scale: 1,
            pixel_spacing: 0,
            theme: BinaryColorTheme::Default,
            title: String::from("embedded-graphics-simulator"),
        }
    }

    /// Set the width/height of the display in pixels
    pub fn size(&mut self, width: usize, height: usize) -> &mut Self {
        if width == 0 || height == 0 {
            panic!("with and height must be >= 0");
        }

        self.width = width;
        self.height = height;

        self
    }

    /// Set the pixel scale
    ///
    /// A scale of `2` or higher is useful for viewing the simulator on high DPI displays
    pub fn scale(&mut self, scale: usize) -> &mut Self {
        if scale == 0 {
            panic!("scale must be >= 0");
        }

        self.scale = scale;

        self
    }

    /// Set the binary color theme for the display to use
    pub fn theme(&mut self, theme: BinaryColorTheme) -> &mut Self {
        self.theme = theme;

        self.scale(3);
        self.pixel_spacing(1);

        self
    }

    /// Add a gap between pixels, simulating the same effect of a physical display
    pub fn pixel_spacing(&mut self, pixel_spacing: usize) -> &mut Self {
        self.pixel_spacing = pixel_spacing;

        self
    }

    /// Set the window title
    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = title.to_owned();

        self
    }

    fn build_window(&self) -> Window {
        Window::new(
            self.width,
            self.height,
            self.scale,
            self.pixel_spacing,
            &self.title,
        )
    }

    /// Finish building the simulated binary display and open an SDL window to render it into
    pub fn build_binary(&self) -> BinaryDisplay {
        let window = self.build_window();
        let pixels = PixelData {
            width: self.width,
            height: self.height,
        };

        BinaryDisplay {
            theme: self.theme.clone(),
            pixels,
            window,
        }
    }

    /// Finish building the simulated RGB display and open an SDL window to render it into
    pub fn build_rgb(&self) -> RgbDisplay {
        let window = self.build_window();
        let pixels = PixelData {
            width: self.width,
            height: self.height,
        };

        RgbDisplay { pixels, window }
    }
}

impl Default for DisplayBuilder {
    fn default() -> Self {
        Self::new()
    }
}
