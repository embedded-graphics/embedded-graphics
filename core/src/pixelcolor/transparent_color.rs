use crate::pixelcolor::PixelColor;

/// Transparent color trait.
///
/// This trait is used to represent color that have an alpha channel to handle transparency level.
/// Transparent colors are a special kind of [`PixelColor`].
///
/// # Implementing transparent color types
///
/// Transparent color types can be added by implementing the [`TransparentColor`] trait with
/// an existing [`PixelColor`] as a matching opaque color. Please note that the transparent color
/// may have the same [`PixelColor::Raw`] type as its underlying opaque color.
///
/// A transparent color SHOULD implement [`From<Self::OpaqueColor>`] so that a color converted
/// target can easily be created. This makes sure it is always possible to draw with an opaque color
/// over a `DrawTarget` that supports transparency.
///
/// # Example
/// ```
/// use embedded_graphics::pixelcolor::{Gray8, TransparentColor};
/// use embedded_graphics::image::{Image, ImageRaw};
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::pixelcolor::raw::RawU16;
///
/// // A transparent color where alpha is handled as an f32
/// #[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
/// pub struct TransparentGray {
///     alpha: f32,
///     gray: Gray8,
/// }
///
/// // PixelColor is needed for TransparentColor
/// impl PixelColor for TransparentGray {
///     // this implementation doesn't expose raw data
///     type Raw = RawU16;
/// }
///
/// // From<RawU16> is needed for PixelColor
/// impl From<RawU16> for TransparentGray {
///     fn from(value: RawU16) -> Self {
///         let inner = value.into_inner();
///         let alpha = ((inner & 0xFF00) >> 8) as f32 / 255.;
///         TransparentGray { alpha, gray: Gray8::new((inner & 0xFF) as u8)}
///     }
/// }
///
/// // From<TransparentGray> is needed for PixelColor
/// impl From<TransparentGray> for RawU16 {
///     fn from(value: TransparentGray) -> Self {
///         let TransparentGray { alpha, gray } = value;
///         RawU16::new(((alpha * 255.) as u16) << 8 + gray.luma())
///     }
/// }
///
/// // Implement required methods
/// impl TransparentColor<Gray8> for TransparentGray {
///     fn blend_over(&self, other: Gray8) -> Gray8 {
///         let luma = (self.alpha * self.gray.luma() as f32) + (1.-self.alpha) * other.luma() as f32;
///         Gray8::new(luma as u8)
///     }
/// }
///
/// // Make conversion available for `DrawTargetExt::color_converted`
/// impl From<Gray8> for TransparentGray {
///     fn from(value: Gray8) -> Self {
///         TransparentGray { alpha: 1., gray: value }
///     }
/// }
///
/// // Mock transparent handling display
/// pub struct Display {}
/// impl DrawTarget for Display {
///     type Color = TransparentGray;
///     type Error = core::convert::Infallible;
///
///     fn draw_iter<I>(&mut self, pixels: I) -> Result<(), core::convert::Infallible>
///     where
///         I: IntoIterator<Item = Pixel<Self::Color>>,
///     {
///         for Pixel(point, color) in pixels.into_iter() {
///             // mock existing pixel as middle gray
///             let current_color = Gray8::new(127);
///             // blend the color with existing color
///             let final_color = color.blend_over(current_color);
///             // ... draw final color on display
///         }
///         Ok(())
///     }
/// }
/// impl OriginDimensions for Display {
///     fn size(&self) -> Size {
///         Size::new(300, 300)
///     }
/// }
///
/// // The image data.
/// const DATA: &[u8] = &[
///     0x80, 0xFF, 0xFF, 0x80, //
///     0xFF, 0x00, 0x00, 0xFF, //
///     0xFF, 0x00, 0x00, 0xFF, //
///     0x80, 0xFF, 0xFF, 0x80, //
/// ];
///
/// // Create a 4x4 non-transparent image
/// let raw_image = ImageRaw::<Gray8>::new(DATA, Size::new(4,4)).unwrap();
/// let image = Image::new(&raw_image, Point::zero());
///
/// // Create a transparent handling display
/// let mut display = Display{};
///
/// // Draw a non-transparent object on a transparent handling display
/// // Conversion is automatically handled by the `From` trait above and the `DrawTargetExt` trait
/// image.draw(&mut display.color_converted()).unwrap();
/// ```
pub trait TransparentColor<C: PixelColor=Self>: PixelColor {
    /// Blend this color over another underlying color
    fn blend_over(&self, other: C) -> C;
}

/// Trait for transparent colors that expose their alpha channel
pub trait AlphaColor: TransparentColor<Self::OpaqueColor> {
    /// Alpha channel type
    type Alpha;

    /// Base color type.
    /// Specifies the original color type that this transparent color is based on.
    type OpaqueColor: PixelColor;

    /// Create transparent color from opaque color
    fn from_opaque(opaque: Self::OpaqueColor, alpha: Self::Alpha) -> Self;

    /// Provide read access to alpha channel
    fn alpha(&self) -> Self::Alpha;

    /// Provide write access to alpha channel
    fn opaque(&self) -> Self::OpaqueColor;

    /// The maximum value in the alpha channel.
    const MAX_A: Self::Alpha;
}
