use embedded_graphics::prelude::*;
use std::convert::TryFrom;

const SIZE: usize = 256;

pub struct Framebuffer<C> {
    pixels: [[C; SIZE]; SIZE],
}

impl<C> Framebuffer<C>
where
    C: PixelColor + Default,
{
    pub fn new() -> Self {
        Self {
            pixels: [[C::default(); 256]; 256],
        }
    }

    fn set_pixel(&mut self, position: Point, color: C) {
        if let (Ok(x), Ok(y)) = (usize::try_from(position.x), usize::try_from(position.y)) {
            self.pixels
                .get_mut(y)
                .and_then(|row| row.get_mut(x))
                .map(|pixel| *pixel = color);
        } else {
            panic!(
                "tried to set pixel outside the framebuffer at {:?}",
                position
            );
        }
    }
}

impl<C> DrawTarget for Framebuffer<C>
where
    C: PixelColor + Default,
{
    type Color = C;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(position, color) in pixels {
            self.set_pixel(position, color);
        }

        Ok(())
    }
}

impl<C> OriginDimensions for Framebuffer<C> {
    fn size(&self) -> Size {
        Size::new_equal(SIZE as u32)
    }
}
