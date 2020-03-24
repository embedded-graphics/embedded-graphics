use crate::{display::SimulatorDisplay, output_settings::OutputSettings};
use embedded_graphics::{
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point, Size},
    pixelcolor::{PixelColor, Rgb888, RgbColor},
    primitives::{Primitive, Rectangle},
    style::{PrimitiveStyle, Styled},
    DrawTarget,
};
use image::{ImageBuffer, Rgb};
use std::convert::TryFrom;

/// Rgb888 framebuffer
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Framebuffer {
    size: Size,
    pub(crate) data: Box<[u8]>,
    pub(crate) output_settings: OutputSettings,
}

impl Framebuffer {
    /// Creates a new framebuffer filled with `background_color`.
    pub fn new<C>(display: &SimulatorDisplay<C>, output_settings: &OutputSettings) -> Self
    where
        C: PixelColor + Into<Rgb888>,
    {
        let size = output_settings.framebuffer_size(display);

        // Create an empty pixel buffer.
        let pixel_count = size.width as usize * size.height as usize;
        let data = vec![0; pixel_count * 3].into_boxed_slice();

        let mut framebuffer = Self {
            size,
            data,
            output_settings: output_settings.clone(),
        };

        // Fill pixel buffer with background color.
        let background_color = output_settings.theme.convert(Rgb888::BLACK);
        framebuffer.clear(background_color).unwrap();

        // Update buffer.
        framebuffer.update(display);

        framebuffer
    }

    /// Updates the framebuffer from a `SimulatorDisplay`.
    pub fn update<C>(&mut self, display: &SimulatorDisplay<C>)
    where
        C: PixelColor + Into<Rgb888>,
    {
        let Size { width, height } = display.size();

        let pixel_pitch = (self.output_settings.scale + self.output_settings.pixel_spacing) as i32;
        let pixel_size = Size::new(self.output_settings.scale, self.output_settings.scale);

        for y in 0..height as i32 {
            for x in 0..width as i32 {
                let color = display.get_pixel(Point::new(x, y)).into();
                let p = Point::new(x * pixel_pitch, y * pixel_pitch);

                Rectangle::new(p, pixel_size)
                    .into_styled(PrimitiveStyle::with_fill(
                        self.output_settings.theme.convert(color),
                    ))
                    .draw(self)
                    .ok();
            }
        }
    }

    fn get_pixel_mut(&mut self, point: Point) -> Option<&mut [u8]> {
        if let Ok((x, y)) = <(u32, u32)>::try_from(point) {
            if x < self.size.width && y < self.size.height {
                let start_index = (x + y * self.size.width) as usize * 3;
                return self.data.get_mut(start_index..start_index + 3);
            }
        }

        None
    }

    /// Converts the framebuffer into an image.rs `ImageBuffer`.
    pub fn into_image_buffer(self) -> ImageBuffer<Rgb<u8>, Box<[u8]>> {
        ImageBuffer::from_raw(self.size.width, self.size.height, self.data).unwrap()
    }
}

impl DrawTarget<Rgb888> for Framebuffer {
    type Error = core::convert::Infallible;

    fn draw_pixel(&mut self, pixel: Pixel<Rgb888>) -> Result<(), Self::Error> {
        let Pixel(point, color) = pixel;

        if let Some(pixel) = self.get_pixel_mut(point) {
            pixel.copy_from_slice(&[color.r(), color.g(), color.b()]);
        }

        Ok(())
    }

    fn draw_rectangle(
        &mut self,
        item: &Styled<Rectangle, PrimitiveStyle<Rgb888>>,
    ) -> Result<(), Self::Error> {
        if item.style.stroke_color.is_some() && item.style.stroke_width != 0 {
            return self.draw_iter(item);
        }

        if let Some(fill_color) = item.style.fill_color {
            let color = &[fill_color.r(), fill_color.g(), fill_color.b()];

            let tl = item.primitive.top_left;
            let br = item.primitive.bottom_right();
            for y in tl.y..=br.y {
                for x in tl.x..=br.x {
                    let p = Point::new(x, y);
                    if let Some(pixel) = self.get_pixel_mut(p) {
                        pixel.copy_from_slice(color);
                    }
                }
            }
        }

        Ok(())
    }

    fn size(&self) -> Size {
        self.size
    }
}
