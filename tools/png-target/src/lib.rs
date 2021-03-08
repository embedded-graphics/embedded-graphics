use embedded_graphics::{
    pixelcolor::{BinaryColor, Rgb888},
    prelude::*,
};
use image::{
    imageops::resize, png::PngEncoder, EncodableLayout, ImageBuffer, Luma, Pixel as _, Rgb,
};
use std::{convert::TryFrom, path::Path};

/// PNG draw target.
pub struct PngTarget<C: EgToImageColor> {
    image: ImageBuffer<C::Image, Vec<u8>>,
    scale: u32,
}

impl<C: EgToImageColor> PngTarget<C> {
    /// Creates a new PNG draw target.
    ///
    /// The image will be scaled up by the given factor when it is saved.
    pub fn new(size: Size, scale: u32) -> Self {
        assert!(scale > 0);

        Self {
            image: ImageBuffer::new(size.width, size.height),
            scale,
        }
    }

    fn scaled_image(&self) -> ImageBuffer<<C as EgToImageColor>::Image, Vec<u8>> {
        resize(
            &self.image,
            self.image.width() * self.scale,
            self.image.height() * self.scale,
            image::imageops::FilterType::Nearest,
        )
    }

    /// Saves the display content to a PNG file.
    pub fn save<PATH: AsRef<Path>>(&self, path: PATH) -> image::ImageResult<()> {
        self.scaled_image()
            .save_with_format(path, image::ImageFormat::Png)
    }

    /// Returns a base64 encoded version of the PNG file.
    pub fn to_base64(&self) -> String {
        let mut png = Vec::new();

        PngEncoder::new(&mut png)
            .encode(
                self.image.as_bytes(),
                self.image.width(),
                self.image.height(),
                C::Image::COLOR_TYPE,
            )
            .unwrap();

        base64::encode(&png)
    }
}

impl DrawTarget for PngTarget<BinaryColor> {
    type Color = BinaryColor;
    type Error = std::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(p, c) in pixels {
            if let (Ok(x), Ok(y)) = (u32::try_from(p.x), u32::try_from(p.y)) {
                if x < self.image.width() && y < self.image.height() {
                    self.image.put_pixel(
                        p.x as u32,
                        p.y as u32,
                        Luma([if c.is_on() { 255 } else { 0 }]),
                    );
                }
            }
        }

        Ok(())
    }
}

impl DrawTarget for PngTarget<Rgb888> {
    type Color = Rgb888;
    type Error = std::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(p, c) in pixels {
            if let (Ok(x), Ok(y)) = (u32::try_from(p.x), u32::try_from(p.y)) {
                if x < self.image.width() && y < self.image.height() {
                    self.image
                        .put_pixel(p.x as u32, p.y as u32, Rgb([c.r(), c.g(), c.b()]));
                }
            }
        }

        Ok(())
    }
}

impl<C: EgToImageColor> OriginDimensions for PngTarget<C> {
    fn size(&self) -> Size {
        Size::new(self.image.width(), self.image.height())
    }
}

/// Trait to convert `embedded-graphics` to `image` color types.
pub trait EgToImageColor {
    type Image: image::Pixel<Subpixel = u8> + 'static;
}

impl EgToImageColor for Rgb888 {
    type Image = Rgb<u8>;
}

impl EgToImageColor for BinaryColor {
    type Image = Luma<u8>;
}
