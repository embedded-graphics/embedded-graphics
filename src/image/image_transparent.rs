use crate::draw_target::DrawTarget;
use crate::geometry::{Dimensions, OriginDimensions, Size};
use crate::pixelcolor::PixelColor;
use crate::prelude::ImageDrawable;
use crate::primitives::Rectangle;
use crate::Pixel;

/// A thin wrapper around an `ImageDrawable` to make it handle transparency.
///
/// The new object can be used inside an `Image` like the previous one.
///
/// `ImageTransparent` has the same content as its source `ImageDrawable`
/// but it doesn't draw anything when it encounters a transparent color.
///
/// This makes it possible to create transparent images from types that don't support it.
///
/// Example:
/// ```
/// # use embedded_graphics::{
/// # image::{Image, ImageRaw, ImageRawBE, ImageTransparent},
/// #     pixelcolor::Rgb565,
/// #     prelude::*,
/// # };
/// # use embedded_graphics::mock_display::MockDisplay as Display;
///
/// let mut display: Display<Rgb565> = Display::default();
/// // Example with an ImageRaw as ImageTransparent source
/// let data = [
///     0x00, 0x00, 0xF8, 0x00, 0x07, 0xE0, 0xFF, 0xE0, //
///     0x00, 0x1F, 0x07, 0xFF, 0xF8, 0x1F, 0xFF, 0xFF, //
/// ];
/// let raw: ImageRawBE<Rgb565> = ImageRaw::new(&data, Size::new(4, 2)).unwrap();
///
/// // Create the transparent object
/// let transparent = ImageTransparent::new(raw, Rgb565::WHITE);
///
/// // Create an Image object from ImageTransparent
/// let image = Image::new(&transparent, Point::zero());
/// image.draw(&mut display)?;
///
/// # Ok::<(), core::convert::Infallible>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct ImageTransparent<T, C> {
    drawable: T,
    transparent_color: C,
}

impl<T: ImageDrawable, C: PixelColor> ImageTransparent<T, C> {
    /// Creates a new `ImageTransparent` use it within `Image`
    pub fn new(drawable: T, transparent_color: C) -> Self {
        ImageTransparent {
            drawable,
            transparent_color,
        }
    }
}

impl<T: ImageDrawable, C: PixelColor> OriginDimensions for ImageTransparent<T, C> {
    fn size(&self) -> Size {
        self.drawable.size()
    }
}

impl<T: ImageDrawable<Color = C>, C: PixelColor> ImageDrawable for ImageTransparent<T, C> {
    type Color = C;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let mut drawer = Drawer {
            target,
            transparent_color: self.transparent_color,
        };
        self.drawable.draw(&mut drawer)
    }

    fn draw_sub_image<D>(&self, target: &mut D, area: &Rectangle) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let mut drawer = Drawer {
            target,
            transparent_color: self.transparent_color,
        };
        self.drawable.draw_sub_image(&mut drawer, area)
    }
}

struct Drawer<'a, T, C> {
    target: &'a mut T,
    transparent_color: C,
}

impl<'a, T: DrawTarget<Color = C>, C> Dimensions for Drawer<'a, T, C> {
    fn bounding_box(&self) -> Rectangle {
        self.target.bounding_box()
    }
}

impl<'a, T: DrawTarget<Color = C>, C: PixelColor> DrawTarget for Drawer<'a, T, C> {
    type Color = C;
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
