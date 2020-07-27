use crate::{framebuffer::Framebuffer, output_settings::OutputSettings};
use embedded_graphics::{
    pixelcolor::{BinaryColor, Rgb888},
    prelude::*,
};
use image::{ImageBuffer, Rgb};
use std::convert::TryFrom;

/// Simulator display.
pub struct SimulatorDisplay<C> {
    size: Size,
    pixels: Box<[C]>,
}

impl<C> SimulatorDisplay<C>
where
    C: PixelColor,
{
    /// Creates a new display filled with a color.
    ///
    /// This constructor can be used if `C` doesn't implement `From<BinaryColor>` or another
    /// default color is wanted.
    pub fn with_default_color(size: Size, default_color: C) -> Self {
        let pixel_count = size.width as usize * size.height as usize;
        let pixels = vec![default_color; pixel_count].into_boxed_slice();

        SimulatorDisplay { size, pixels }
    }

    /// Returns the color of the pixel at a point.
    ///
    /// # Panics
    ///
    /// Panics if `point` is outside the display.
    pub fn get_pixel(&self, point: Point) -> C {
        self.point_to_index(point)
            .and_then(|index| self.pixels.get(index).copied())
            .expect("can't get point outside of display")
    }

    fn point_to_index(&self, point: Point) -> Option<usize> {
        if let Ok((x, y)) = <(u32, u32)>::try_from(point) {
            if x < self.size.width && y < self.size.height {
                return Some((x + y * self.size.width) as usize);
            }
        }

        None
    }
}

impl<C> SimulatorDisplay<C>
where
    C: PixelColor + From<BinaryColor>,
{
    /// Creates a new display.
    ///
    /// The display is filled with `C::from(BinaryColor::Off)`.
    pub fn new(size: Size) -> Self {
        Self::with_default_color(size, C::from(BinaryColor::Off))
    }
}

impl<C> SimulatorDisplay<C>
where
    C: PixelColor + Into<Rgb888>,
{
    /// Converts the display contents into an image.rs `ImageBuffer`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
    /// use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};
    ///
    /// let output_settings = OutputSettingsBuilder::new().scale(2).build();
    ///
    /// let display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(128, 64));
    ///
    /// // draw something to the display
    ///
    /// let image_buffer = display.to_image_buffer(&output_settings);
    /// assert_eq!(image_buffer.width(), 256);
    /// assert_eq!(image_buffer.height(), 128);
    ///
    /// // use image buffer
    /// // example: image_buffer.save
    /// ```
    pub fn to_image_buffer(
        &self,
        output_settings: &OutputSettings,
    ) -> ImageBuffer<Rgb<u8>, Box<[u8]>> {
        let framebuffer = Framebuffer::new(self, output_settings);
        framebuffer.into_image_buffer()
    }
}

impl<C> DrawTarget for SimulatorDisplay<C>
where
    C: PixelColor,
{
    type Color = C;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels.into_iter() {
            if let Some(index) = self.point_to_index(point) {
                self.pixels[index] = color;
            }
        }

        Ok(())
    }
}

impl<C> OriginDimensions for SimulatorDisplay<C>
where
    C: PixelColor,
{
    fn size(&self) -> Size {
        self.size
    }
}
