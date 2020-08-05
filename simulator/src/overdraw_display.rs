use crate::SimulatorDisplay;
use core::convert::TryFrom;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Size,
    pixelcolor::{raw::RawU8, Gray8},
    pixelcolor::{GrayColor, Rgb888},
    prelude::*,
    prelude::{OriginDimensions, Point},
    Pixel,
};

/// Overdraw display
pub struct OverdrawDisplay {
    size: Size,
    pixels: Box<[Gray8]>,
}

impl OverdrawDisplay {
    /// Creates a new display.
    pub fn new(size: Size) -> Self {
        let pixel_count = size.area() as usize;
        let pixels = vec![Gray8::BLACK; pixel_count].into_boxed_slice();

        OverdrawDisplay { size, pixels }
    }

    /// Convert to a normal display for drawing
    pub fn draw_to_display(
        &self,
        display: &mut SimulatorDisplay<Rgb888>,
    ) -> Result<(), core::convert::Infallible> {
        self.pixels
            .iter()
            .map(|p| Rgb888::new(p.luma(), p.luma(), p.luma()))
            .zip(self.bounding_box().points())
            .map(|(color, point)| Pixel(point, color))
            .draw(display)
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

impl DrawTarget for OverdrawDisplay {
    type Color = Gray8;
    type Error = core::convert::Infallible;

    fn clear(&mut self, color: Gray8) -> Result<(), Self::Error> {
        let pixel_count = self.size.area() as usize;
        self.pixels = vec![color; pixel_count].into_boxed_slice();

        Ok(())
    }

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, _color) in pixels.into_iter() {
            if let Some(index) = self.point_to_index(point) {
                let value = RawU8::from(self.pixels[index]).into_inner();

                let new = if value == 0 {
                    Gray8::new(127)
                } else {
                    Gray8::WHITE
                };

                // Increment each pixel by 1 each time it's written to
                self.pixels[index] = new;
            }
        }

        Ok(())
    }
}

impl OriginDimensions for OverdrawDisplay {
    fn size(&self) -> Size {
        self.size
    }
}
