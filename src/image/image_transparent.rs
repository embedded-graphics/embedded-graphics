use crate::{
    draw_target::DrawTarget,
    geometry::{Dimensions, OriginDimensions, Size},
    image::ImageDrawable,
    primitives::Rectangle,
    Pixel,
};

/// A wrapper to add basic transparency to an `ImageDrawable`.
///
/// `ImageTransparent` works by designating one color in the source
/// [`ImageDrawable`] as being transparent. All pixels with this color are
/// skipped during drawing, while all other pixels remain unchanged.
///
/// # Performance
///
/// When this wrapper is used, the image is drawn pixel by pixel to allow
/// transparent pixels to be skipped. This can have a negative impact on
/// performance.
///
/// # Examples
///
/// ```
/// use embedded_graphics::{
///     image::{Image, ImageRaw, ImageTransparent},
///     pixelcolor::Gray8,
///     prelude::*,
/// };
/// # use embedded_graphics::mock_display::MockDisplay as Display;
///
/// let mut display: Display<Gray8> = Display::default();
///
/// // Source image without transparency.
/// let data = [
///     0x00, 0x00, 0xF8, 0x00, //
///     0x07, 0xE0, 0xFF, 0xE0, //
///     0x00, 0x1F, 0x07, 0xFF, //
///     0xF8, 0x1F, 0xFF, 0xFF, //
/// ];
/// let source: ImageRaw<Gray8> = ImageRaw::new(&data, Size::new(4, 4)).unwrap();
///
/// // Make all white pixels (`0xFF`) in the source image transparent.
/// let transparent = ImageTransparent::new(source, Gray8::WHITE);
///
/// // Draw the transparent image at `(0, 0)`.
/// let image = Image::new(&transparent, Point::zero());
/// image.draw(&mut display)?;
///
/// # Ok::<(), core::convert::Infallible>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct ImageTransparent<T: ImageDrawable> {
    source: T,
    transparent_color: T::Color,
}

impl<T: ImageDrawable> ImageTransparent<T> {
    /// Creates a new `ImageTransparent` based on a source image.
    ///
    /// All pixels with the given transparent color will be skipped during drawing.
    pub fn new(source: T, transparent_color: T::Color) -> Self {
        ImageTransparent {
            source,
            transparent_color,
        }
    }
}

impl<T: ImageDrawable> OriginDimensions for ImageTransparent<T> {
    fn size(&self) -> Size {
        self.source.size()
    }
}

impl<T: ImageDrawable> ImageDrawable for ImageTransparent<T> {
    type Color = T::Color;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let mut draw_target = TransparentDrawTarget {
            target,
            transparent_color: self.transparent_color,
        };
        self.source.draw(&mut draw_target)
    }

    fn draw_sub_image<D>(&self, target: &mut D, area: &Rectangle) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let mut draw_target = TransparentDrawTarget {
            target,
            transparent_color: self.transparent_color,
        };
        self.source.draw_sub_image(&mut draw_target, area)
    }
}

struct TransparentDrawTarget<'a, T: DrawTarget> {
    target: &'a mut T,
    transparent_color: T::Color,
}

impl<'a, T: DrawTarget> Dimensions for TransparentDrawTarget<'a, T> {
    fn bounding_box(&self) -> Rectangle {
        self.target.bounding_box()
    }
}

impl<'a, T: DrawTarget> DrawTarget for TransparentDrawTarget<'a, T> {
    type Color = T::Color;
    type Error = T::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.target.draw_iter(
            pixels
                .into_iter()
                .filter(|pixel| pixel.1 != self.transparent_color),
        )
    }
}
