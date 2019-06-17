use crate::display_theme::DisplayTheme;
use crate::sim_pixel_color::SimPixelColor;
use crate::Display;

pub struct DisplayBuilder {
    width: usize,
    height: usize,
    scale: usize,
    pixel_spacing: usize,
    theme: DisplayTheme,
}

impl DisplayBuilder {
    pub fn new() -> Self {
        Self {
            width: 256,
            height: 256,
            scale: 1,
            pixel_spacing: 0,
            theme: DisplayTheme::Default,
        }
    }

    pub fn size(&mut self, width: usize, height: usize) -> &mut Self {
        if width == 0 || height == 0 {
            panic!("with and height must be >= 0");
        }

        self.width = width;
        self.height = height;

        self
    }

    pub fn scale(&mut self, scale: usize) -> &mut Self {
        if scale == 0 {
            panic!("scale must be >= 0");
        }

        self.scale = scale;

        self
    }

    pub fn theme(&mut self, theme: DisplayTheme) -> &mut Self {
        self.theme = theme;

        self.scale(3);
        self.pixel_spacing(1);

        self
    }

    pub fn pixel_spacing(&mut self, pixel_spacing: usize) -> &mut Self {
        self.pixel_spacing = pixel_spacing;

        self
    }

    pub fn build(&self) -> Display {
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

        let pixels = vec![SimPixelColor(0, 0, 0); self.width * self.height];
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
