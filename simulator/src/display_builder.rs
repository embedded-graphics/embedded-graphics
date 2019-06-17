use crate::display_theme::BinaryColorTheme;
use crate::Display;
use embedded_graphics::pixelcolor::PixelColor;

/// Create a simulator display using the builder pattern
pub struct DisplayBuilder {
    width: usize,
    height: usize,
    scale: usize,
    pixel_spacing: usize,
    theme: BinaryColorTheme,
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

    /// Finish building the simulated display and open an SDL window to render it into
    pub fn build<C>(&self) -> Display<C>
    where
        C: PixelColor,
    {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window_width = self.width * self.scale + (self.width - 1) * self.pixel_spacing;
        let window_height = self.height * self.scale + (self.height - 1) * self.pixel_spacing;

        let window = video_subsystem
            .window(
                "graphics-emulator",
                window_width as u32,
                window_height as u32,
            )
            .position_centered()
            .build()
            .unwrap();

        let pixels = vec![C::DEFAULT_BG; self.width * self.height];
        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Display {
            width: self.width,
            height: self.height,
            scale: self.scale,
            pixel_spacing: self.pixel_spacing,
            theme: self.theme.clone(),
            pixels: pixels.into_boxed_slice(),
            canvas,
            event_pump,
        }
    }
}
